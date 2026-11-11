pub mod fuentes;
mod google;
pub mod win;

use fuentes::{cache::Cache, cambios, Entorno, Familia, Origen};
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    sync::{mpsc, Mutex},
    time::Duration,
};
use tauri::{AppHandle, Emitter, Manager, RunEvent, State, WebviewUrl, WebviewWindowBuilder};

/// Mismo archivo que usa tauri-plugin-store desde la interfaz (ver src/lib/ajustes.svelte.js).
const AJUSTES: &str = "ajustes.json";

/// Fondo de la ventana de cada tema (--win en src/app.css): así no hay
/// destello blanco mientras carga la página.
const FONDO_CLARO: (u8, u8, u8, u8) = (0xf7, 0xf6, 0xf3, 255);
const FONDO_OSCURO: (u8, u8, u8, u8) = (0x19, 0x18, 0x16, 255);

fn err(e: impl ToString) -> String {
    e.to_string()
}

pub struct Estado {
    ent: Entorno,
    cache: Mutex<Cache>,
    /// Carpetas vigiladas.
    carpetas: Mutex<Vec<PathBuf>>,
    /// Fuentes de carpetas vigiladas cargadas ahora en la sesión.
    cargadas: Mutex<HashSet<String>>,
    /// Las que se han apagado a mano en esta sesión.
    apagadas: Mutex<HashSet<String>>,
    vigilante: Mutex<Option<notify::RecommendedWatcher>>,
}

impl Estado {
    fn archivo_cache(&self) -> PathBuf {
        self.ent.datos.join("fonts-cache.json")
    }
    fn archivo_carpetas(&self) -> PathBuf {
        self.ent.datos.join("carpetas.json")
    }
    fn entradas(&self) -> Vec<fuentes::Entrada> {
        let carpetas = self.carpetas.lock().unwrap().clone();
        let apagadas = self.apagadas.lock().unwrap();
        let mut v = fuentes::entradas(&self.ent, &carpetas);
        for e in v.iter_mut().filter(|e| e.origen == Origen::Carpeta) {
            e.activa = !apagadas.contains(&fuentes::clave_ruta(&e.ruta));
        }
        v
    }
    /// Lee lo cambiado, guarda la caché y carga en la sesión las de carpetas vigiladas.
    fn refrescar(&self) -> (Vec<Familia>, usize) {
        let entradas = self.entradas();
        let mut cache = self.cache.lock().unwrap();
        let leidas = cache.actualizar(&entradas);
        cache.guardar(&self.archivo_cache());
        let familias = cache.familias.clone();
        drop(cache);
        self.sincronizar_carpetas(&entradas);
        (familias, leidas)
    }
    fn sincronizar_carpetas(&self, entradas: &[fuentes::Entrada]) {
        let mut cargadas = self.cargadas.lock().unwrap();
        let quiero: HashSet<String> = entradas
            .iter()
            .filter(|e| e.origen == Origen::Carpeta && e.activa)
            .map(|e| fuentes::clave_ruta(&e.ruta))
            .collect();
        let mut cambio = false;
        for e in entradas.iter().filter(|e| e.origen == Origen::Carpeta) {
            let k = fuentes::clave_ruta(&e.ruta);
            if quiero.contains(&k) && !cargadas.contains(&k) && win::cargar(&e.ruta, false) {
                cargadas.insert(k);
                cambio = true;
            }
        }
        for k in cargadas.iter().filter(|k| !quiero.contains(*k)).cloned().collect::<Vec<_>>() {
            win::descargar(Path::new(&k), false);
            cargadas.remove(&k);
            cambio = true;
        }
        if cambio {
            win::avisar_cambio();
        }
    }
    fn descargar_carpetas(&self) {
        let mut cargadas = self.cargadas.lock().unwrap();
        for k in cargadas.drain() {
            win::descargar(Path::new(&k), false);
        }
        win::avisar_cambio();
    }
}

async fn fondo<T: Send + 'static>(f: impl FnOnce() -> Result<T, String> + Send + 'static) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(f).await.map_err(err)?
}

#[derive(Serialize)]
struct Salida {
    familias: Vec<Familia>,
    resultado: cambios::Resultado,
}

/// La lista guardada de la última vez: se pinta al instante.
#[tauri::command]
fn fuentes_guardadas(estado: State<Estado>) -> Vec<Familia> {
    estado.cache.lock().unwrap().familias.clone()
}

/// Comprueba en segundo plano qué ha cambiado y lee solo eso.
#[tauri::command]
async fn fuentes_actualizar(app: AppHandle) -> Result<Value, String> {
    fondo(move || {
        let (familias, leidas) = app.state::<Estado>().refrescar();
        Ok(json!({ "familias": familias, "leidas": leidas }))
    })
    .await
}

#[tauri::command]
async fn alternar(app: AppHandle, familias: Vec<String>, activar: bool, desbloqueadas: bool) -> Result<Salida, String> {
    fondo(move || {
        let estado = app.state::<Estado>();
        let entradas = estado.entradas();
        let resultado = {
            let mut cache = estado.cache.lock().unwrap();
            cache.actualizar(&entradas);
            let mut apagadas = estado.apagadas.lock().unwrap();
            cambios::alternar(&estado.ent, &entradas, &cache, &familias, activar, desbloqueadas, &mut apagadas)
        };
        let (familias, _) = estado.refrescar();
        Ok(Salida { familias, resultado })
    })
    .await
}

/// Instala archivos soltados en la ventana. `para_todos` pide permisos.
#[tauri::command]
async fn instalar(app: AppHandle, rutas: Vec<String>, para_todos: bool) -> Result<Salida, String> {
    fondo(move || {
        let estado = app.state::<Estado>();
        let r = instalar_archivos(&estado, &rutas.iter().map(PathBuf::from).collect::<Vec<_>>(), para_todos);
        let (familias, _) = estado.refrescar();
        Ok(Salida { familias, resultado: r })
    })
    .await
}

fn instalar_archivos(estado: &Estado, rutas: &[PathBuf], para_todos: bool) -> cambios::Resultado {
    let mut r = cambios::Resultado::default();
    let mut ops = Vec::new();
    for ruta in rutas {
        let nombre = ruta.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        if !fuentes::es_fuente(ruta) {
            r.errores.push(format!("{nombre}: no es una fuente"));
            continue;
        }
        let caras = match fuentes::leer::caras(ruta) {
            Ok(c) => c,
            Err(e) => {
                r.errores.push(format!("{nombre}: {e}"));
                continue;
            }
        };
        let familia = caras[0].familia.clone();
        if para_todos {
            let destino = cambios::libre(&estado.ent.fuentes_sistema, &nombre);
            let cff = ruta.extension().is_some_and(|e| e.eq_ignore_ascii_case("otf"));
            let valor = fuentes::registro::nombre_valor(&caras.iter().map(|c| c.completo.clone()).collect::<Vec<_>>(), cff);
            let datos = destino.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
            ops.push(fuentes::sistema::Op::Instalar { origen: ruta.clone(), ruta: destino, valor, datos });
            r.instaladas.push(familia);
        } else {
            match cambios::instalar_usuario(&estado.ent, ruta, &caras) {
                Ok(_) => {
                    r.cambiadas += 1;
                    if !r.instaladas.contains(&familia) {
                        r.instaladas.push(familia);
                    }
                }
                Err(e) => r.errores.push(format!("{nombre}: {e}")),
            }
        }
    }
    if !ops.is_empty() {
        let antes = r.cambiadas;
        cambios::aplicar_sistema(&estado.ent, &ops, &mut r);
        if r.cambiadas == antes {
            r.instaladas.clear();
        }
    }
    if r.cambiadas > 0 {
        win::avisar_cambio();
    }
    r
}

/// Ajustes → «restaurar todas las del sistema».
#[tauri::command]
async fn restaurar_sistema(app: AppHandle) -> Result<Salida, String> {
    fondo(move || {
        let estado = app.state::<Estado>();
        let ops: Vec<_> = fuentes::respaldadas(&estado.ent)
            .into_iter()
            .map(|g| fuentes::sistema::Op::Activar { respaldo: g.archivo, ruta: g.original, valor: g.valor, datos: g.datos })
            .collect();
        let mut r = cambios::Resultado::default();
        if !ops.is_empty() {
            cambios::aplicar_sistema(&estado.ent, &ops, &mut r);
            win::avisar_cambio();
        }
        let (familias, _) = estado.refrescar();
        Ok(Salida { familias, resultado: r })
    })
    .await
}

#[tauri::command]
fn respaldadas(estado: State<Estado>) -> usize {
    fuentes::respaldadas(&estado.ent).len()
}

/// Caracteres reales de la fuente (tabla cmap), para el mapa del espécimen.
#[tauri::command]
async fn caracteres(ruta: String) -> Result<Vec<u32>, String> {
    fondo(move || fuentes::leer::caracteres(Path::new(&ruta), 4000)).await
}

// ---- carpetas vigiladas ----

fn guardar_carpetas(estado: &Estado) {
    let c: Vec<String> = estado.carpetas.lock().unwrap().iter().map(|p| p.to_string_lossy().into_owned()).collect();
    let _ = fs::create_dir_all(&estado.ent.datos);
    let _ = fs::write(estado.archivo_carpetas(), serde_json::to_vec(&c).unwrap_or_default());
}

fn vigilar(app: &AppHandle) {
    use notify::Watcher;
    let estado = app.state::<Estado>();
    let (tx, rx) = mpsc::channel::<()>();
    let w = notify::recommended_watcher(move |ev: notify::Result<notify::Event>| {
        if let Ok(ev) = ev {
            if ev.paths.iter().any(|p| fuentes::es_fuente(p) || p.extension().is_none()) {
                let _ = tx.send(());
            }
        }
    });
    let Ok(mut w) = w else { return };
    for c in estado.carpetas.lock().unwrap().iter() {
        let _ = w.watch(c, notify::RecursiveMode::Recursive);
    }
    *estado.vigilante.lock().unwrap() = Some(w);
    let app = app.clone();
    std::thread::spawn(move || {
        // Agrupa los cambios seguidos (copiar diez archivos = un aviso).
        while rx.recv().is_ok() {
            while rx.recv_timeout(Duration::from_millis(500)).is_ok() {}
            let _ = app.emit("fuentes-cambiadas", ());
        }
    });
}

#[derive(Serialize)]
struct Carpeta {
    ruta: String,
    nombre: String,
}

fn lista_carpetas(estado: &Estado) -> Vec<Carpeta> {
    estado
        .carpetas
        .lock()
        .unwrap()
        .iter()
        .map(|p| {
            let comps: Vec<String> = p.components().rev().take(2).map(|c| c.as_os_str().to_string_lossy().to_lowercase()).collect();
            let nombre = comps.into_iter().rev().collect::<Vec<_>>().join("\\");
            Carpeta { ruta: p.to_string_lossy().into_owned(), nombre }
        })
        .collect()
}

#[tauri::command]
fn carpetas(estado: State<Estado>) -> Vec<Carpeta> {
    lista_carpetas(&estado)
}

#[tauri::command]
async fn anadir_carpeta(app: AppHandle) -> Result<Option<Vec<Carpeta>>, String> {
    use tauri_plugin_dialog::DialogExt;
    let Some(elegida) = app.dialog().file().blocking_pick_folder() else { return Ok(None) };
    let ruta = elegida.into_path().map_err(err)?;
    let estado = app.state::<Estado>();
    {
        let mut c = estado.carpetas.lock().unwrap();
        if !c.iter().any(|x| fuentes::clave_ruta(x) == fuentes::clave_ruta(&ruta)) {
            c.push(ruta.clone());
        }
    }
    let _ = app.asset_protocol_scope().allow_directory(&ruta, true);
    guardar_carpetas(&estado);
    vigilar(&app);
    Ok(Some(lista_carpetas(&estado)))
}

#[tauri::command]
fn quitar_carpeta(app: AppHandle, ruta: String) -> Vec<Carpeta> {
    let estado = app.state::<Estado>();
    estado.carpetas.lock().unwrap().retain(|c| fuentes::clave_ruta(c) != fuentes::clave_ruta(Path::new(&ruta)));
    guardar_carpetas(&estado);
    vigilar(&app);
    // Descarga las de esa carpeta (refrescar ya no las ve).
    let _ = estado.refrescar();
    lista_carpetas(&estado)
}

// ---- google fonts ----

#[tauri::command]
async fn google_catalogo(app: AppHandle, forzar: bool) -> Result<Value, String> {
    let datos = app.state::<Estado>().ent.datos.clone();
    let lista = google::catalogo(&datos, forzar).await?;
    serde_json::to_value(lista).map_err(err)
}

#[tauri::command]
async fn google_instalar(app: AppHandle, familia: String) -> Result<Salida, String> {
    let datos = app.state::<Estado>().ent.datos.clone();
    let destino = datos.join("descargas").join(familia.replace(|c: char| !c.is_alphanumeric(), "_"));
    let a = app.clone();
    let f = familia.clone();
    let mut ultimo = -1.0;
    let archivos = google::bajar(&familia, &destino, move |p| {
        if p - ultimo >= 2.0 {
            ultimo = p;
            let _ = a.emit("descarga", json!({ "familia": f, "pct": p }));
        }
    })
    .await;
    let archivos = match archivos {
        Ok(a) => a,
        Err(e) => {
            let _ = app.emit("descarga", json!({ "familia": familia, "error": true }));
            return Err(e);
        }
    };
    let app2 = app.clone();
    let salida = fondo(move || {
        let estado = app2.state::<Estado>();
        let r = instalar_archivos(&estado, &archivos, false);
        let _ = fs::remove_dir_all(&destino);
        let (familias, _) = estado.refrescar();
        Ok(Salida { familias, resultado: r })
    })
    .await?;
    let _ = app.emit("descarga", json!({ "familia": familia, "pct": 100 }));
    Ok(salida)
}

// ---- arranque ----

fn leer_ajustes(app: &AppHandle) -> Value {
    app.path()
        .app_data_dir()
        .ok()
        .and_then(|d| fs::read_to_string(d.join(AJUSTES)).ok())
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .filter(Value::is_object)
        .unwrap_or_else(|| json!({}))
}

/// Antes de la primera línea de la página: los ajustes guardados y el tema
/// puesto en <html>, para que el primer fotograma ya salga con su color e idioma.
fn script_inicio(inicio: &Value) -> String {
    format!(
        "window.__TUNA__ = {inicio};
        (function (t) {{
          if (t !== 'light' && t !== 'dark')
            t = matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
          var d = document, s = function () {{ d.documentElement.dataset.theme = t; }};
          if (d.documentElement) s();
          else new MutationObserver(function (m, o) {{
            if (d.documentElement) {{ s(); o.disconnect(); }}
          }}).observe(d, {{ childList: true }});
        }})(window.__TUNA__.ajustes.tema);"
    )
}

/// Tema oscuro de Windows (apps), para la primera vez que se abre.
fn windows_oscuro() -> bool {
    winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
        .open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize")
        .and_then(|k| k.get_value::<u32, _>("AppsUseLightTheme"))
        .map(|v| v == 0)
        .unwrap_or(false)
}

/// Con `--elevado <peticion.json>`: la copia con permisos de administrador.
pub fn modo_elevado() -> Option<i32> {
    let mut args = std::env::args_os().skip(1);
    if args.next()?.to_str()? != "--elevado" {
        return None;
    }
    let peticion = PathBuf::from(args.next()?);
    Some(fuentes::sistema::atender(&peticion))
}

pub fn run() {
    let app = tauri::Builder::default()
        // Una sola tuna-fonts a la vez: dos moviendo las mismas fuentes se pisarían.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(v) = app.get_webview_window("main") {
                let _ = v.unminimize();
                let _ = v.set_focus();
            }
        }))
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            fuentes_guardadas,
            fuentes_actualizar,
            alternar,
            instalar,
            restaurar_sistema,
            respaldadas,
            caracteres,
            carpetas,
            anadir_carpeta,
            quitar_carpeta,
            google_catalogo,
            google_instalar,
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            let datos = app.path().app_data_dir()?;
            let ent = Entorno::real(datos.clone());
            let cache = Cache::cargar(&datos.join("fonts-cache.json"));
            let carpetas: Vec<PathBuf> = fs::read(datos.join("carpetas.json"))
                .ok()
                .and_then(|b| serde_json::from_slice::<Vec<String>>(&b).ok())
                .unwrap_or_default()
                .into_iter()
                .map(PathBuf::from)
                .collect();

            // Vistas previas de las que no están instaladas ahora (desactivadas,
            // de carpetas, recién instaladas): se cargan desde su archivo.
            let alcance = app.asset_protocol_scope();
            for d in [ent.desactivadas(), ent.respaldo(), ent.fuentes_usuario.clone(), ent.fuentes_sistema.clone()] {
                let _ = alcance.allow_directory(d, true);
            }
            for c in &carpetas {
                let _ = alcance.allow_directory(c, true);
            }

            app.manage(Estado {
                ent,
                cache: Mutex::new(cache),
                carpetas: Mutex::new(carpetas),
                cargadas: Mutex::default(),
                apagadas: Mutex::default(),
                vigilante: Mutex::default(),
            });
            vigilar(&handle);

            let ajustes = leer_ajustes(&handle);
            let oscuro = match ajustes["tema"].as_str() {
                Some("dark") => true,
                Some("light") => false,
                _ => windows_oscuro(),
            };
            let inicio = json!({ "ajustes": ajustes, "version": app.package_info().version.to_string() });

            WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("tuna-fonts")
                .inner_size(1280.0, 820.0)
                .min_inner_size(960.0, 600.0)
                .center()
                .decorations(false)
                .shadow(true)
                .zoom_hotkeys_enabled(false)
                .background_color(if oscuro { FONDO_OSCURO } else { FONDO_CLARO }.into())
                .initialization_script(script_inicio(&inicio))
                .build()?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error al arrancar tuna-fonts");

    app.run(|app, ev| {
        // Al cerrar: las de carpetas vigiladas solo estaban mientras la app estuviera abierta.
        if let RunEvent::Exit = ev {
            app.state::<Estado>().descargar_carpetas();
        }
    });
}
