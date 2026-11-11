//! Fuentes del sistema (o "para todos"): hace falta ser administrador.
//!
//! tuna-fonts se relanza a sí misma con UAC (`--elevado <peticion.json>`), esa
//! copia hace solo las operaciones pedidas y deja el resultado en un JSON.
//! Antes de tocar nada copia el archivo y su entrada del registro al respaldo.
//! Si Windows tiene el archivo en uso, se borra al reiniciar (MoveFileEx).

use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::{es_imprescindible, leer, registro, Entorno};
use crate::win;

pub const CANCELADO: &str = "cancelado";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "tipo")]
pub enum Op {
    /// Quitar de HKLM y mover el archivo al respaldo.
    Desactivar { ruta: PathBuf, respaldo: PathBuf, valor: String, datos: String },
    /// Devolver del respaldo a su sitio y a HKLM.
    Activar { respaldo: PathBuf, ruta: PathBuf, valor: String, datos: String },
    /// Copiar a C:\Windows\Fonts y registrar en HKLM.
    Instalar { origen: PathBuf, ruta: PathBuf, valor: String, datos: String },
}

#[derive(Serialize, Deserialize)]
struct Peticion {
    clave: String,
    fuentes: PathBuf,
    respaldo: PathBuf,
    ops: Vec<Op>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Respuesta {
    /// Una por operación: None si salió bien, o el error.
    pub hechas: Vec<Option<String>>,
    pub reiniciar: bool,
}

/// Desde la app normal: pide permisos y espera.
pub fn pedir(ent: &Entorno, ops: &[Op]) -> Result<Respuesta, String> {
    fs::create_dir_all(ent.respaldo()).map_err(|e| e.to_string())?;
    let peticion = ent.datos.join("elevado-peticion.json");
    let respuesta = ent.datos.join("elevado-respuesta.json");
    let _ = fs::remove_file(&respuesta);
    let p = Peticion { clave: ent.clave_sistema.clone(), fuentes: ent.fuentes_sistema.clone(), respaldo: ent.respaldo(), ops: ops.to_vec() };
    fs::write(&peticion, serde_json::to_vec(&p).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let codigo = win::como_admin(&format!("--elevado \"{}\"", peticion.display()));
    let _ = fs::remove_file(&peticion);
    if codigo.is_none() {
        return Err(CANCELADO.into());
    }
    let r = fs::read(&respuesta).map_err(|_| "no se ha podido terminar con permisos de administrador".to_string())?;
    let _ = fs::remove_file(&respuesta);
    serde_json::from_slice(&r).map_err(|e| e.to_string())
}

fn dentro(ruta: &Path, carpeta: &Path) -> bool {
    ruta.parent().is_some_and(|p| p.to_string_lossy().eq_ignore_ascii_case(&carpeta.to_string_lossy()))
}

/// Las imprescindibles no se tocan nunca, aunque lo pida la petición.
fn protegida(ruta: &Path) -> bool {
    leer::caras(ruta).map(|cs| cs.iter().any(|c| es_imprescindible(&c.familia))).unwrap_or(false)
}

fn una(p: &Peticion, op: &Op, reiniciar: &mut bool) -> Result<(), String> {
    let hklm = registro::hklm();
    match op {
        Op::Desactivar { ruta, respaldo, valor, datos } => {
            if !dentro(ruta, &p.fuentes) || !dentro(respaldo, &p.respaldo) {
                return Err("ruta no permitida".into());
            }
            if protegida(ruta) {
                return Err("windows la necesita".into());
            }
            // Respaldo primero: el archivo y su entrada del registro.
            fs::copy(ruta, respaldo).map_err(|e| e.to_string())?;
            let reg = serde_json::json!({ "clave": format!(r"HKLM\{}", p.clave), "valor": valor, "datos": datos });
            fs::write(respaldo.with_extension("reg.json"), reg.to_string()).map_err(|e| e.to_string())?;
            if !valor.is_empty() {
                registro::quitar(hklm, &p.clave, valor)?;
            }
            if fs::remove_file(ruta).is_err() {
                // En uso: se borra al reiniciar (la copia ya está en el respaldo).
                if !win::al_reiniciar(ruta, None) {
                    let _ = registro::poner(hklm, &p.clave, valor, datos);
                    return Err("windows la tiene en uso".into());
                }
                *reiniciar = true;
            }
            Ok(())
        }
        Op::Activar { respaldo, ruta, valor, datos } => {
            if !dentro(ruta, &p.fuentes) || !dentro(respaldo, &p.respaldo) {
                return Err("ruta no permitida".into());
            }
            if ruta.exists() {
                // Pendiente de borrar al reiniciar: hay que reiniciar antes.
                return Err("reinicia windows antes de volver a activarla".into());
            }
            fs::copy(respaldo, ruta).map_err(|e| e.to_string())?;
            if !valor.is_empty() {
                registro::poner(hklm, &p.clave, valor, datos)?;
            }
            Ok(())
        }
        Op::Instalar { origen, ruta, valor, datos } => {
            if !dentro(ruta, &p.fuentes) || ruta.exists() {
                return Err("ruta no permitida".into());
            }
            leer::caras(origen)?;
            fs::copy(origen, ruta).map_err(|e| e.to_string())?;
            registro::poner(hklm, &p.clave, valor, datos)
        }
    }
}

/// En la copia con permisos: hace lo pedido y escribe la respuesta.
pub fn atender(peticion: &Path) -> i32 {
    let Ok(b) = fs::read(peticion) else { return 2 };
    let Ok(p) = serde_json::from_slice::<Peticion>(&b) else { return 2 };
    let mut r = Respuesta::default();
    for op in &p.ops {
        r.hechas.push(una(&p, op, &mut r.reiniciar).err());
    }
    let destino = peticion.with_file_name("elevado-respuesta.json");
    match serde_json::to_vec(&r).ok().and_then(|b| fs::write(destino, b).ok()) {
        Some(()) => 0,
        None => 3,
    }
}
