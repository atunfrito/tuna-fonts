//! Descubrir: catálogo de Google Fonts (con caché local) e instalación.
//! La única red de la app, junto con nada más: solo fonts.google.com y fonts.gstatic.com.

use futures_util::StreamExt;
use serde::Serialize;
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

const METADATA: &str = "https://fonts.google.com/metadata/fonts";
const LISTA: &str = "https://fonts.google.com/download/list?family=";
/// El catálogo guardado vale una semana; si no hay red, vale el que haya.
const VIGENCIA: Duration = Duration::from_secs(7 * 24 * 3600);

fn cliente() -> Result<reqwest::Client, String> {
    let _ = rustls::crypto::ring::default_provider().install_default();
    reqwest::Client::builder()
        .user_agent("tuna-fonts")
        .connect_timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::custom(|intento| {
            // Solo dentro de Google Fonts.
            let ok = intento.url().host_str().is_some_and(|h| h == "fonts.google.com" || h == "fonts.gstatic.com");
            if ok && intento.previous().len() < 5 { intento.follow() } else { intento.stop() }
        }))
        .build()
        .map_err(|e| e.to_string())
}

/// Google antepone `)]}'` a sus JSON.
fn json_de(texto: &str) -> Result<Value, String> {
    let t = texto.trim_start();
    let t = t.strip_prefix(")]}'").unwrap_or(t);
    serde_json::from_str(t).map_err(|e| e.to_string())
}

#[derive(Serialize, Clone)]
pub struct Entrada {
    nombre: String,
    categoria: &'static str,
    estilos: usize,
    popularidad: i64,
    fecha: String,
}

fn categoria(f: &Value) -> &'static str {
    let script = f["primaryScript"].as_str().unwrap_or("");
    if !script.is_empty() && script != "Latn" {
        return "otros";
    }
    match f["category"].as_str().unwrap_or("") {
        "Sans Serif" => "sans",
        "Serif" => "serif",
        "Handwriting" => "script",
        "Monospace" => "mono",
        _ => "display",
    }
}

fn compactar(v: &Value) -> Vec<Entrada> {
    v["familyMetadataList"]
        .as_array()
        .map(|l| {
            l.iter()
                .filter_map(|f| {
                    Some(Entrada {
                        nombre: f["family"].as_str()?.to_string(),
                        categoria: categoria(f),
                        estilos: f["fonts"].as_object().map_or(1, |o| o.len()),
                        popularidad: f["popularity"].as_i64().unwrap_or(i64::MAX),
                        fecha: f["dateAdded"].as_str().unwrap_or("").to_string(),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

pub async fn catalogo(datos: &Path, forzar: bool) -> Result<Vec<Entrada>, String> {
    let archivo = datos.join("google-metadata.json");
    let fresco = fs::metadata(&archivo)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| SystemTime::now().duration_since(t).ok())
        .is_some_and(|d| d < VIGENCIA);
    if fresco && !forzar {
        if let Some(v) = fs::read_to_string(&archivo).ok().and_then(|t| json_de(&t).ok()) {
            return Ok(compactar(&v));
        }
    }
    let bajado = async {
        let t = cliente()?.get(METADATA).send().await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?.text().await.map_err(|e| e.to_string())?;
        let v = json_de(&t)?;
        fs::create_dir_all(datos).map_err(|e| e.to_string())?;
        let _ = fs::write(&archivo, &t);
        Ok::<_, String>(v)
    }
    .await;
    match bajado {
        Ok(v) => Ok(compactar(&v)),
        // Sin red: lo último que se bajó, aunque sea viejo.
        Err(e) => fs::read_to_string(&archivo).ok().and_then(|t| json_de(&t).ok()).map(|v| compactar(&v)).ok_or(e),
    }
}

/// Archivos .ttf de la familia: los de la raíz (variables si los hay), sin la carpeta static/ duplicada.
fn elegir(refs: &[Value]) -> Vec<(String, String)> {
    let todos: Vec<(String, String)> = refs
        .iter()
        .filter_map(|r| Some((r["filename"].as_str()?.to_string(), r["url"].as_str()?.to_string())))
        .filter(|(n, u)| {
            let n = n.to_lowercase();
            (n.ends_with(".ttf") || n.ends_with(".otf")) && u.starts_with("https://fonts.gstatic.com/")
        })
        .collect();
    let raiz: Vec<_> = todos.iter().filter(|(n, _)| !n.contains('/')).cloned().collect();
    if raiz.is_empty() { todos } else { raiz }
}

/// Baja los archivos de una familia a `destino`. `progreso(0..=100)`.
pub async fn bajar(familia: &str, destino: &Path, mut progreso: impl FnMut(f64)) -> Result<Vec<PathBuf>, String> {
    let c = cliente()?;
    let url = format!("{LISTA}{}", familia.replace(' ', "+"));
    let t = c.get(&url).send().await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?.text().await.map_err(|e| e.to_string())?;
    let v = json_de(&t)?;
    let archivos = elegir(v["manifest"]["fileRefs"].as_array().map(|a| a.as_slice()).unwrap_or(&[]));
    if archivos.is_empty() {
        return Err("google fonts no da archivos para esta familia".into());
    }
    let _ = fs::remove_dir_all(destino);
    fs::create_dir_all(destino).map_err(|e| e.to_string())?;
    let n = archivos.len() as f64;
    let mut fuera = Vec::new();
    for (i, (nombre, url)) in archivos.iter().enumerate() {
        let resp = c.get(url).send().await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?;
        let total = resp.content_length().unwrap_or(0) as f64;
        let mut bytes = Vec::new();
        let mut flujo = resp.bytes_stream();
        while let Some(trozo) = flujo.next().await {
            bytes.extend_from_slice(&trozo.map_err(|e| e.to_string())?);
            let parte = if total > 0.0 { (bytes.len() as f64 / total).min(1.0) } else { 0.5 };
            progreso(((i as f64 + parte) / n * 100.0).min(99.0));
        }
        let base = Path::new(nombre).file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| format!("{i}.ttf"));
        // Los nombres de los variables llevan corchetes: fuera, que Windows no se líe.
        let base: String = base.chars().map(|c| if c == '[' || c == ']' || c == ',' { '_' } else { c }).collect();
        let ruta = destino.join(base);
        fs::write(&ruta, &bytes).map_err(|e| e.to_string())?;
        fuera.push(ruta);
    }
    Ok(fuera)
}
