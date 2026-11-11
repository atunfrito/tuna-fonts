//! Activar, desactivar e instalar sin borrar nada.
//!
//! - Usuario: desactivar = descargarla de la sesión, quitar su entrada de HKCU
//!   y moverla a la carpeta de la app. Activar = devolverla a su sitio.
//! - Sistema: lo mismo con HKLM y C:\Windows\Fonts, pero con permisos de
//!   administrador (ver sistema.rs) y solo si se han desbloqueado en ajustes.
//! - Carpetas vigiladas: se cargan y descargan de la sesión; no se instalan.

use serde::Serialize;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use super::{
    cache::Cache, clave_ruta, de_la_app, es_imprescindible, guardadas, leer::Cara, poner_de_la_app, poner_guardadas,
    poner_respaldadas, registro, respaldadas, sistema, Entorno, Entrada, Guardada, Origen,
};
use crate::win;

#[derive(Serialize, Default, Debug)]
pub struct Resultado {
    pub cambiadas: usize,
    /// Del sistema que no se han tocado (bloqueadas o imprescindibles).
    pub bloqueadas: usize,
    pub errores: Vec<String>,
    /// Alguna del sistema se quitará del todo al reiniciar.
    pub reiniciar: bool,
    /// Se dijo que no al aviso de permisos de Windows.
    pub cancelado: bool,
    /// Familias instaladas (para el aviso).
    pub instaladas: Vec<String>,
}

fn nombre_archivo(r: &Path) -> String {
    r.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default()
}

/// Una ruta libre en `dir` para `nombre`: "x.ttf", "x (2).ttf"...
pub fn libre(dir: &Path, nombre: &str) -> PathBuf {
    let p = dir.join(nombre);
    if !p.exists() {
        return p;
    }
    let base = Path::new(nombre);
    let (tallo, ext) = (
        base.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default(),
        base.extension().map(|s| format!(".{}", s.to_string_lossy())).unwrap_or_default(),
    );
    (2..).map(|i| dir.join(format!("{tallo} ({i}){ext}"))).find(|p| !p.exists()).unwrap()
}

/// Mover; si es a otro disco, copiar y borrar.
fn mover(de: &Path, a: &Path) -> Result<(), String> {
    if let Some(d) = a.parent() {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    if fs::rename(de, a).is_ok() {
        return Ok(());
    }
    fs::copy(de, a).map_err(|e| en_uso(e))?;
    if let Err(e) = fs::remove_file(de) {
        let _ = fs::remove_file(a);
        return Err(en_uso(e));
    }
    Ok(())
}

fn en_uso(e: std::io::Error) -> String {
    if e.raw_os_error() == Some(32) { "algún programa la tiene abierta".into() } else { e.to_string() }
}

fn es_cff(r: &Path) -> bool {
    r.extension().is_some_and(|e| e.eq_ignore_ascii_case("otf"))
}

pub fn desactivar_usuario(ent: &Entorno, e: &Entrada) -> Result<(), String> {
    if ent.tocar_windows {
        win::descargar(&e.ruta, false);
    }
    let destino = libre(&ent.desactivadas(), &nombre_archivo(&e.ruta));
    if let Err(err) = mover(&e.ruta, &destino) {
        if ent.tocar_windows {
            win::cargar(&e.ruta, false);
        }
        return Err(err);
    }
    let (valor, datos) = e.valor.clone().unwrap_or_default();
    if !valor.is_empty() {
        if let Err(err) = registro::quitar(registro::hkcu(), &ent.clave_usuario, &valor) {
            let _ = mover(&destino, &e.ruta);
            return Err(err);
        }
    }
    let mut g = guardadas(ent);
    g.push(Guardada { archivo: destino, original: e.ruta.clone(), valor, datos, origen: e.origen });
    poner_guardadas(ent, &g)
}

/// Activa una de usuario: una guardada por la app, o una suelta en la carpeta
/// de fuentes del usuario que no estaba en el registro.
pub fn activar_usuario(ent: &Entorno, e: &Entrada, caras: &[Cara]) -> Result<PathBuf, String> {
    let mut g = guardadas(ent);
    let guardada = g.iter().position(|x| clave_ruta(&x.archivo) == clave_ruta(&e.ruta));
    let (ruta, valor) = match guardada {
        Some(i) => {
            let x = &g[i];
            let ruta = if x.original.exists() {
                libre(x.original.parent().unwrap_or(&ent.fuentes_usuario), &nombre_archivo(&x.original))
            } else {
                x.original.clone()
            };
            mover(&x.archivo, &ruta)?;
            (ruta, x.valor.clone())
        }
        None => (e.ruta.clone(), String::new()),
    };
    let valor = if valor.is_empty() {
        registro::nombre_valor(&caras.iter().map(|c| c.completo.clone()).collect::<Vec<_>>(), es_cff(&ruta))
    } else {
        valor
    };
    registro::poner(registro::hkcu(), &ent.clave_usuario, &valor, &ruta.to_string_lossy())?;
    if let Some(i) = guardada {
        g.remove(i);
        poner_guardadas(ent, &g)?;
    }
    if ent.tocar_windows {
        win::cargar(&ruta, false);
    }
    Ok(ruta)
}

/// Instala para el usuario: sin pedir permisos.
pub fn instalar_usuario(ent: &Entorno, origen: &Path, caras: &[Cara]) -> Result<PathBuf, String> {
    let destino = libre(&ent.fuentes_usuario, &nombre_archivo(origen));
    fs::create_dir_all(&ent.fuentes_usuario).map_err(|e| e.to_string())?;
    fs::copy(origen, &destino).map_err(|e| e.to_string())?;
    let valor = registro::nombre_valor(&caras.iter().map(|c| c.completo.clone()).collect::<Vec<_>>(), es_cff(&destino));
    if let Err(e) = registro::poner(registro::hkcu(), &ent.clave_usuario, &valor, &destino.to_string_lossy()) {
        let _ = fs::remove_file(&destino);
        return Err(e);
    }
    let mut app = de_la_app(ent);
    app.insert(clave_ruta(&destino));
    poner_de_la_app(ent, &app)?;
    if ent.tocar_windows {
        win::cargar(&destino, false);
    }
    Ok(destino)
}

fn familias_de(cache: &Cache, e: &Entrada) -> Vec<String> {
    cache.caras(&e.ruta).map(|c| c.into_iter().map(|c| c.familia).collect()).unwrap_or_default()
}

/// Activa o desactiva las familias pedidas. `apagadas_carpeta`: fuentes de
/// carpetas vigiladas que se han descargado en esta sesión.
pub fn alternar(
    ent: &Entorno,
    entradas: &[Entrada],
    cache: &Cache,
    familias: &[String],
    activar: bool,
    desbloqueadas: bool,
    apagadas_carpeta: &mut HashSet<String>,
) -> Resultado {
    let quiero: HashSet<String> = familias.iter().map(|f| f.to_lowercase()).collect();
    let mut r = Resultado::default();
    let mut pendientes_sis: Vec<(&Entrada, sistema::Op)> = Vec::new();

    for e in entradas {
        let fams = familias_de(cache, e);
        if !fams.iter().any(|f| quiero.contains(&f.to_lowercase())) || e.activa == activar {
            continue;
        }
        let nombre = fams.first().cloned().unwrap_or_else(|| nombre_archivo(&e.ruta));
        let hecho = match e.origen {
            Origen::Usuario | Origen::Biblioteca if e.ruta.starts_with(&ent.fuentes_sistema) || respaldo_de(ent, e) => {
                // "Para todos" instalada por la app: vive en Windows\Fonts y va como las del sistema.
                if let Some(op) = op_sistema(ent, e, activar) {
                    pendientes_sis.push((e, op));
                }
                continue;
            }
            Origen::Usuario | Origen::Biblioteca => {
                if activar {
                    activar_usuario(ent, e, &cache.caras(&e.ruta).unwrap_or_default()).map(|_| ())
                } else {
                    desactivar_usuario(ent, e)
                }
            }
            Origen::Carpeta => {
                // La carga y descarga en la sesión la hace quien refresca (ver lib.rs).
                let k = clave_ruta(&e.ruta);
                if activar {
                    apagadas_carpeta.remove(&k);
                } else {
                    apagadas_carpeta.insert(k);
                }
                Ok(())
            }
            Origen::Sistema => {
                if !desbloqueadas || fams.iter().any(|f| es_imprescindible(f)) {
                    r.bloqueadas += 1;
                    continue;
                }
                if let Some(op) = op_sistema(ent, e, activar) {
                    pendientes_sis.push((e, op));
                }
                continue;
            }
        };
        match hecho {
            Ok(()) => r.cambiadas += 1,
            Err(err) => r.errores.push(format!("{nombre}: {err}")),
        }
    }

    if !pendientes_sis.is_empty() {
        for (e, _) in &pendientes_sis {
            if !activar && ent.tocar_windows {
                win::descargar(&e.ruta, false);
            }
        }
        let ops: Vec<sistema::Op> = pendientes_sis.iter().map(|(_, op)| op.clone()).collect();
        aplicar_sistema(ent, &ops, &mut r);
    }
    if ent.tocar_windows && (r.cambiadas > 0) {
        win::avisar_cambio();
    }
    r
}

fn respaldo_de(ent: &Entorno, e: &Entrada) -> bool {
    e.ruta.starts_with(ent.respaldo())
}

/// La operación con permisos para una del sistema.
fn op_sistema(ent: &Entorno, e: &Entrada, activar: bool) -> Option<sistema::Op> {
    let (valor, datos) = e.valor.clone().unwrap_or_default();
    if activar {
        let g = respaldadas(ent).into_iter().find(|g| clave_ruta(&g.archivo) == clave_ruta(&e.ruta))?;
        Some(sistema::Op::Activar { respaldo: g.archivo, ruta: g.original, valor: g.valor, datos: g.datos })
    } else {
        Some(sistema::Op::Desactivar { respaldo: libre(&ent.respaldo(), &nombre_archivo(&e.ruta)), ruta: e.ruta.clone(), valor, datos })
    }
}

/// Lanza las operaciones con permisos y deja apuntado lo que ha salido bien.
pub fn aplicar_sistema(ent: &Entorno, ops: &[sistema::Op], r: &mut Resultado) {
    let resp = match sistema::pedir(ent, ops) {
        Ok(x) => x,
        Err(e) => {
            if e == sistema::CANCELADO {
                r.cancelado = true;
            } else {
                r.errores.push(e);
            }
            // Lo descargado vuelve a cargarse: no ha cambiado nada.
            for op in ops {
                if let sistema::Op::Desactivar { ruta, .. } = op {
                    if ent.tocar_windows {
                        win::cargar(ruta, false);
                    }
                }
            }
            return;
        }
    };
    r.reiniciar |= resp.reiniciar;
    let mut resp_list = respaldadas(ent);
    let mut app = de_la_app(ent);
    for (op, err) in ops.iter().zip(resp.hechas.iter()) {
        if let Some(err) = err {
            let n = match op {
                sistema::Op::Desactivar { ruta, .. } | sistema::Op::Activar { ruta, .. } | sistema::Op::Instalar { ruta, .. } => nombre_archivo(ruta),
            };
            r.errores.push(format!("{n}: {err}"));
            if let sistema::Op::Desactivar { ruta, .. } = op {
                if ent.tocar_windows {
                    win::cargar(ruta, false);
                }
            }
            continue;
        }
        r.cambiadas += 1;
        match op {
            sistema::Op::Desactivar { ruta, respaldo, valor, datos } => {
                let origen = if app.contains(&clave_ruta(ruta)) { Origen::Biblioteca } else { Origen::Sistema };
                resp_list.push(Guardada { archivo: respaldo.clone(), original: ruta.clone(), valor: valor.clone(), datos: datos.clone(), origen });
            }
            sistema::Op::Activar { ruta, respaldo, .. } => {
                resp_list.retain(|g| clave_ruta(&g.archivo) != clave_ruta(respaldo));
                let _ = fs::remove_file(respaldo);
                let _ = fs::remove_file(respaldo.with_extension("reg.json"));
                if ent.tocar_windows {
                    win::cargar(ruta, false);
                }
            }
            sistema::Op::Instalar { ruta, .. } => {
                app.insert(clave_ruta(ruta));
                if ent.tocar_windows {
                    win::cargar(ruta, false);
                }
            }
        }
    }
    if let Err(e) = poner_respaldadas(ent, &resp_list) {
        r.errores.push(e);
    }
    let _ = poner_de_la_app(ent, &app);
}
