//! Inventario de fuentes: qué hay instalado (sistema y usuario), qué ha
//! desactivado tuna-fonts y qué hay en las carpetas vigiladas, agrupado por familia.

pub mod cache;
pub mod cambios;
pub mod categoria;
pub mod leer;
pub mod registro;
pub mod sistema;

use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use leer::Cara;

pub const EXTENSIONES: [&str; 4] = ["ttf", "otf", "ttc", "otc"];

/// Las que Windows necesita para verse bien: nunca se tocan.
const IMPRESCINDIBLES: [&str; 8] = [
    "segoe fluent icons", "segoe mdl2 assets", "marlett", "tahoma", "microsoft sans serif", "arial", "consolas", "segoe ui",
];

pub fn es_imprescindible(familia: &str) -> bool {
    let f = familia.to_lowercase();
    IMPRESCINDIBLES.iter().any(|i| f == *i) || f.starts_with("segoe ui")
}

pub fn es_fuente(ruta: &Path) -> bool {
    ruta.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| EXTENSIONES.contains(&e.to_ascii_lowercase().as_str()))
}

/// Dónde vive cada cosa. En los tests apunta a carpetas temporales y a una
/// clave de registro de prueba, y no carga nada en Windows.
#[derive(Clone, Debug)]
pub struct Entorno {
    pub fuentes_sistema: PathBuf,
    pub fuentes_usuario: PathBuf,
    pub clave_sistema: String,
    pub clave_usuario: String,
    /// Carpeta de datos de la app.
    pub datos: PathBuf,
    /// false en los tests: ni AddFontResourceEx ni WM_FONTCHANGE.
    pub tocar_windows: bool,
}

impl Entorno {
    pub fn real(datos: PathBuf) -> Self {
        use windows_sys::Win32::UI::Shell::{FOLDERID_Fonts, FOLDERID_LocalAppData};
        // Carpetas conocidas; las variables de entorno solo como último recurso.
        let fuentes_sistema = crate::win::carpeta_conocida(&FOLDERID_Fonts).unwrap_or_else(|| {
            std::env::var_os("WINDIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(r"C:\Windows")).join("Fonts")
        });
        let local = crate::win::carpeta_conocida(&FOLDERID_LocalAppData)
            .or_else(|| std::env::var_os("LOCALAPPDATA").map(PathBuf::from))
            .unwrap_or_default();
        Entorno {
            fuentes_sistema,
            fuentes_usuario: local.join(r"Microsoft\Windows\Fonts"),
            clave_sistema: registro::CLAVE.into(),
            clave_usuario: registro::CLAVE.into(),
            datos,
            tocar_windows: true,
        }
    }
    pub fn desactivadas(&self) -> PathBuf {
        self.datos.join("desactivadas")
    }
    pub fn respaldo(&self) -> PathBuf {
        self.datos.join("respaldo-sistema")
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub enum Origen {
    #[serde(rename = "sis")]
    Sistema,
    #[serde(rename = "bib")]
    Biblioteca,
    #[serde(rename = "usu")]
    Usuario,
    #[serde(rename = "car")]
    Carpeta,
}

/// Una fuente que tuna-fonts ha quitado de Windows y guarda en su carpeta.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Guardada {
    /// Dónde está ahora (carpeta de la app).
    pub archivo: PathBuf,
    /// Dónde estaba.
    pub original: PathBuf,
    /// Su entrada del registro, para dejarla igual.
    pub valor: String,
    pub datos: String,
    pub origen: Origen,
}

fn leer_json<T: for<'a> Deserialize<'a> + Default>(ruta: &Path) -> T {
    fs::read(ruta).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default()
}
fn escribir_json<T: Serialize>(ruta: &Path, v: &T) -> Result<(), String> {
    if let Some(d) = ruta.parent() {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    // Primero a un temporal: si se corta a medias, el bueno sigue entero.
    let tmp = ruta.with_extension("tmp");
    fs::write(&tmp, serde_json::to_vec_pretty(v).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    fs::rename(&tmp, ruta).map_err(|e| e.to_string())
}

pub fn guardadas(ent: &Entorno) -> Vec<Guardada> {
    leer_json(&ent.datos.join("desactivadas.json"))
}
pub fn poner_guardadas(ent: &Entorno, v: &[Guardada]) -> Result<(), String> {
    escribir_json(&ent.datos.join("desactivadas.json"), &v)
}
pub fn respaldadas(ent: &Entorno) -> Vec<Guardada> {
    leer_json(&ent.datos.join("respaldo-sistema.json"))
}
pub fn poner_respaldadas(ent: &Entorno, v: &[Guardada]) -> Result<(), String> {
    escribir_json(&ent.datos.join("respaldo-sistema.json"), &v)
}
/// Archivos que ha instalado tuna-fonts (soltados o de Google Fonts): origen "biblioteca".
pub fn de_la_app(ent: &Entorno) -> HashSet<String> {
    leer_json(&ent.datos.join("instaladas-por-app.json"))
}
pub fn poner_de_la_app(ent: &Entorno, v: &HashSet<String>) -> Result<(), String> {
    escribir_json(&ent.datos.join("instaladas-por-app.json"), v)
}
pub fn clave_ruta(r: &Path) -> String {
    r.to_string_lossy().to_lowercase()
}

/// Un archivo de fuente con su sitio en el inventario.
#[derive(Clone, Debug)]
pub struct Entrada {
    pub ruta: PathBuf,
    pub origen: Origen,
    pub activa: bool,
    /// Entrada del registro (nombre, datos), si la tiene.
    pub valor: Option<(String, String)>,
}

/// Datos del registro: relativos a la carpeta de fuentes, o rutas completas.
fn resolver(datos: &str, carpeta: &Path) -> PathBuf {
    let p = PathBuf::from(datos);
    if p.is_absolute() { p } else { carpeta.join(p) }
}

/// Todos los archivos, sin leerlos: registro, carpetas de fuentes, lo que
/// guarda la app y las carpetas vigiladas.
pub fn entradas(ent: &Entorno, vigiladas: &[PathBuf]) -> Vec<Entrada> {
    let mut fuera: Vec<Entrada> = Vec::new();
    let mut vistas: HashSet<String> = HashSet::new();
    let app = de_la_app(ent);
    // Del sistema desactivadas cuyo archivo aún no se ha podido borrar (hasta reiniciar).
    for g in respaldadas(ent) {
        vistas.insert(clave_ruta(&g.original));
    }
    let mut meter = |e: Entrada, fuera: &mut Vec<Entrada>| {
        if vistas.insert(clave_ruta(&e.ruta)) {
            fuera.push(e);
        }
    };

    // Sistema: lo registrado en HKLM y lo que haya suelto en la carpeta.
    for (nombre, datos) in registro::leer(registro::hklm(), &ent.clave_sistema) {
        let ruta = resolver(&datos, &ent.fuentes_sistema);
        if es_fuente(&ruta) && ruta.is_file() {
            let origen = if app.contains(&clave_ruta(&ruta)) { Origen::Biblioteca } else { Origen::Sistema };
            meter(Entrada { ruta, origen, activa: true, valor: Some((nombre, datos)) }, &mut fuera);
        }
    }
    for ruta in listar(&ent.fuentes_sistema, false) {
        meter(Entrada { ruta, origen: Origen::Sistema, activa: true, valor: None }, &mut fuera);
    }

    // Usuario: lo registrado en HKCU está activo; lo suelto en su carpeta, no.
    for (nombre, datos) in registro::leer(registro::hkcu(), &ent.clave_usuario) {
        let ruta = resolver(&datos, &ent.fuentes_usuario);
        if es_fuente(&ruta) && ruta.is_file() {
            let origen = if app.contains(&clave_ruta(&ruta)) { Origen::Biblioteca } else { Origen::Usuario };
            meter(Entrada { ruta, origen, activa: true, valor: Some((nombre, datos)) }, &mut fuera);
        }
    }
    for ruta in listar(&ent.fuentes_usuario, false) {
        let origen = if app.contains(&clave_ruta(&ruta)) { Origen::Biblioteca } else { Origen::Usuario };
        meter(Entrada { ruta, origen, activa: false, valor: None }, &mut fuera);
    }

    // Desactivadas por la app.
    for g in guardadas(ent).into_iter().chain(respaldadas(ent)) {
        if g.archivo.is_file() {
            meter(Entrada { ruta: g.archivo, origen: g.origen, activa: false, valor: Some((g.valor, g.datos)) }, &mut fuera);
        }
    }

    for c in vigiladas {
        for ruta in listar(c, true) {
            meter(Entrada { ruta, origen: Origen::Carpeta, activa: true, valor: None }, &mut fuera);
        }
    }
    fuera
}

pub fn listar(carpeta: &Path, recursivo: bool) -> Vec<PathBuf> {
    let mut fuera = Vec::new();
    let mut pendientes = vec![carpeta.to_path_buf()];
    while let Some(d) = pendientes.pop() {
        let Ok(it) = fs::read_dir(&d) else { continue };
        for e in it.flatten() {
            let p = e.path();
            match e.file_type() {
                Ok(t) if t.is_dir() && recursivo => pendientes.push(p),
                Ok(t) if t.is_file() && es_fuente(&p) => fuera.push(p),
                _ => {}
            }
        }
    }
    fuera
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Estilo {
    pub nombre: String,
    pub peso: u16,
    pub cursiva: bool,
    pub archivo: String,
    /// Nombre con el que Windows conoce esta cara (para pintarla sin cargar nada).
    pub familia_win: String,
    /// Nombre completo (name 4): el que entiende `local()` en CSS.
    #[serde(default)]
    pub completo: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Familia {
    pub nombre: String,
    pub categoria: String,
    pub origen: Origen,
    pub activa: bool,
    pub imprescindible: bool,
    pub estilos: Vec<Estilo>,
    /// La cara normal (o la primera): con ella se pinta la vista previa.
    pub archivo: String,
    pub archivos: Vec<String>,
    /// Si el archivo no se ha podido leer.
    pub error: Option<String>,
}

/// Agrupa las entradas ya leídas por familia.
pub fn agrupar(entradas: &[Entrada], caras: &dyn Fn(&Path) -> Result<Vec<Cara>, String>) -> Vec<Familia> {
    let mut por: BTreeMap<String, Familia> = BTreeMap::new();
    let mut ordenadas: Vec<&Entrada> = entradas.iter().collect();
    ordenadas.sort_by_key(|e| e.origen);
    for e in ordenadas {
        let archivo = e.ruta.to_string_lossy().into_owned();
        match caras(&e.ruta) {
            Err(err) => {
                let nombre = e.ruta.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
                por.insert(
                    format!("\u{0}{}", archivo.to_lowercase()),
                    Familia {
                        nombre,
                        categoria: "display".into(),
                        origen: e.origen,
                        activa: e.activa,
                        imprescindible: false,
                        estilos: vec![],
                        archivo: archivo.clone(),
                        archivos: vec![archivo],
                        error: Some(err),
                    },
                );
            }
            Ok(cs) => {
                for c in cs {
                    let f = por.entry(c.familia.to_lowercase()).or_insert_with(|| Familia {
                        nombre: c.familia.clone(),
                        categoria: c.categoria.clone(),
                        origen: e.origen,
                        activa: false,
                        imprescindible: es_imprescindible(&c.familia),
                        estilos: vec![],
                        archivo: archivo.clone(),
                        archivos: vec![],
                        error: None,
                    });
                    f.activa |= e.activa;
                    if !f.archivos.contains(&archivo) {
                        f.archivos.push(archivo.clone());
                    }
                    let normal = !c.cursiva && c.peso == 400;
                    if normal && !f.estilos.iter().any(|s| !s.cursiva && s.peso == 400) {
                        f.archivo = archivo.clone();
                        f.categoria = c.categoria.clone();
                    }
                    if !f.estilos.iter().any(|s| s.nombre == c.estilo && s.peso == c.peso && s.cursiva == c.cursiva) {
                        f.estilos.push(Estilo { nombre: c.estilo, peso: c.peso, cursiva: c.cursiva, archivo: archivo.clone(), familia_win: c.familia_win, completo: c.completo });
                    }
                }
            }
        }
    }
    let mut v: Vec<Familia> = por.into_values().collect();
    for f in &mut v {
        f.estilos.sort_by_key(|s| (s.cursiva, (s.peso as i32 - 400).abs(), s.peso));
    }
    v.sort_by_key(|f| f.nombre.to_lowercase());
    v
}
