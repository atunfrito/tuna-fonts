//! fonts-cache.json: lo leído de cada archivo (con su fecha y tamaño) y la
//! última lista de familias. Al abrir se pinta la lista guardada; luego solo se
//! vuelven a leer los archivos nuevos o cambiados, en paralelo.

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path, time::UNIX_EPOCH};

use super::{agrupar, clave_ruta, leer, Entrada, Familia};

#[derive(Serialize, Deserialize, Clone)]
pub struct Registro {
    pub mtime: u64,
    pub tam: u64,
    pub caras: Result<Vec<leer::Cara>, String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Cache {
    /// Versión del formato: si cambia la forma de leer, se vuelve a leer todo.
    #[serde(default)]
    pub version: u32,
    pub archivos: HashMap<String, Registro>,
    pub familias: Vec<Familia>,
}

const VERSION: u32 = 1;

fn sello(ruta: &Path) -> Option<(u64, u64)> {
    let m = fs::metadata(ruta).ok()?;
    let t = m.modified().ok()?.duration_since(UNIX_EPOCH).ok()?.as_secs();
    Some((t, m.len()))
}

impl Cache {
    pub fn cargar(ruta: &Path) -> Cache {
        let c: Cache = fs::read(ruta).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
        if c.version != VERSION { Cache { version: VERSION, ..Default::default() } } else { c }
    }

    pub fn guardar(&self, ruta: &Path) {
        if let Ok(b) = serde_json::to_vec(self) {
            let tmp = ruta.with_extension("tmp");
            if fs::write(&tmp, b).is_ok() {
                let _ = fs::rename(&tmp, ruta);
            }
        }
    }

    /// Lee lo nuevo o cambiado, olvida lo que ya no está y rehace las familias.
    /// Devuelve cuántos archivos ha tenido que leer.
    pub fn actualizar(&mut self, entradas: &[Entrada]) -> usize {
        let pendientes: Vec<(String, &Path, (u64, u64))> = entradas
            .iter()
            .filter_map(|e| {
                let s = sello(&e.ruta)?;
                let k = clave_ruta(&e.ruta);
                match self.archivos.get(&k) {
                    Some(r) if (r.mtime, r.tam) == s => None,
                    _ => Some((k, e.ruta.as_path(), s)),
                }
            })
            .collect();
        let leidos: Vec<(String, Registro)> = pendientes
            .par_iter()
            .map(|(k, ruta, (mtime, tam))| (k.clone(), Registro { mtime: *mtime, tam: *tam, caras: leer::caras(ruta) }))
            .collect();
        let n = leidos.len();
        self.archivos.extend(leidos);

        let vivas: std::collections::HashSet<String> = entradas.iter().map(|e| clave_ruta(&e.ruta)).collect();
        self.archivos.retain(|k, _| vivas.contains(k));

        let archivos = &self.archivos;
        self.familias = agrupar(entradas, &|p: &Path| {
            archivos.get(&clave_ruta(p)).map(|r| r.caras.clone()).unwrap_or_else(|| Err("no se ha podido leer".into()))
        });
        n
    }

    pub fn caras(&self, ruta: &Path) -> Option<Vec<leer::Cara>> {
        self.archivos.get(&clave_ruta(ruta)).and_then(|r| r.caras.clone().ok())
    }
}
