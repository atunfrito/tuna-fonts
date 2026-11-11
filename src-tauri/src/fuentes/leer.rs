//! Lee de un .ttf/.otf/.ttc solo las tablas que hacen falta (name, OS/2 y,
//! para el mapa de caracteres, cmap). Nunca carga el archivo entero: lee la
//! cabecera, busca las tablas en el directorio y salta a cada una.

use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

use super::categoria::{self, Datos};

/// Más que esto en una tabla name/OS/2/cmap es un archivo roto o malicioso.
const TOPE_TABLA: u32 = 32 * 1024 * 1024;
const TOPE_CARAS: u32 = 64;

/// Una cara (estilo) dentro de un archivo.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Cara {
    /// Familia tipográfica (name 16, o 1 si no hay).
    pub familia: String,
    /// Familia "de siempre" (name 1): la que entiende Windows al pedirla por nombre.
    pub familia_win: String,
    /// Estilo (name 17, o 2).
    pub estilo: String,
    pub completo: String,
    pub peso: u16,
    pub cursiva: bool,
    pub categoria: String,
}

fn be16(b: &[u8], i: usize) -> Option<u16> {
    b.get(i..i + 2).map(|s| u16::from_be_bytes([s[0], s[1]]))
}
fn be32(b: &[u8], i: usize) -> Option<u32> {
    b.get(i..i + 4).map(|s| u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
}

fn leer_en(f: &mut File, desde: u64, largo: usize) -> std::io::Result<Vec<u8>> {
    let mut v = vec![0; largo];
    f.seek(SeekFrom::Start(desde))?;
    f.read_exact(&mut v)?;
    Ok(v)
}

/// Las tablas pedidas (por etiqueta) de la fuente que empieza en `inicio`.
fn tablas(f: &mut File, inicio: u64, quiero: &[&[u8; 4]]) -> Result<Vec<Option<Vec<u8>>>, String> {
    let cab = leer_en(f, inicio, 12).map_err(|e| e.to_string())?;
    let n = be16(&cab, 4).ok_or("cabecera corta")? as usize;
    if n == 0 || n > 512 {
        return Err("directorio de tablas raro".into());
    }
    let dir = leer_en(f, inicio + 12, n * 16).map_err(|e| e.to_string())?;
    let mut fuera = vec![None; quiero.len()];
    for r in dir.chunks_exact(16) {
        let Some(i) = quiero.iter().position(|t| &r[0..4] == *t) else { continue };
        let (off, len) = (be32(r, 8).unwrap_or(0), be32(r, 12).unwrap_or(0));
        if len == 0 || len > TOPE_TABLA {
            continue;
        }
        fuera[i] = Some(leer_en(f, off as u64, len as usize).map_err(|e| e.to_string())?);
    }
    Ok(fuera)
}

/// Desplazamientos de cada fuente del archivo (una, o varias si es .ttc).
fn inicios(f: &mut File) -> Result<Vec<u64>, String> {
    let cab = leer_en(f, 0, 12).map_err(|_| "no es una fuente".to_string())?;
    match &cab[0..4] {
        b"ttcf" => {
            let n = be32(&cab, 8).unwrap_or(0).min(TOPE_CARAS) as usize;
            let offs = leer_en(f, 12, n * 4).map_err(|e| e.to_string())?;
            Ok(offs.chunks_exact(4).map(|c| u32::from_be_bytes([c[0], c[1], c[2], c[3]]) as u64).collect())
        }
        [0, 1, 0, 0] | b"OTTO" | b"true" => Ok(vec![0]),
        _ => Err("no es una fuente".into()),
    }
}

/// Texto de un nombre: primero Windows en inglés, luego cualquier Windows, luego lo que haya.
fn nombre(t: &ttf_parser::name::Table, id: u16) -> Option<String> {
    let mut mejor: Option<(u8, String)> = None;
    for n in t.names {
        if n.name_id != id {
            continue;
        }
        let Some(s) = n.to_string() else { continue };
        let s = s.trim().to_string();
        if s.is_empty() {
            continue;
        }
        let nota = match (n.platform_id, n.language_id) {
            (ttf_parser::PlatformId::Windows, 0x409) => 3,
            (ttf_parser::PlatformId::Windows, _) => 2,
            _ => 1,
        };
        if mejor.as_ref().is_none_or(|(m, _)| nota > *m) {
            mejor = Some((nota, s));
        }
    }
    mejor.map(|(_, s)| s)
}

fn sin_extension(ruta: &Path) -> String {
    ruta.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default()
}

/// Todas las caras de un archivo, con su categoría.
pub fn caras(ruta: &Path) -> Result<Vec<Cara>, String> {
    let mut f = File::open(ruta).map_err(|e| e.to_string())?;
    let mut fuera = Vec::new();
    for inicio in inicios(&mut f)? {
        let t = tablas(&mut f, inicio, &[b"name", b"OS/2"])?;
        let name = t[0].as_deref().and_then(ttf_parser::name::Table::parse).ok_or("sin tabla name")?;
        let os2 = t[1].as_deref();

        let familia_win = nombre(&name, 1).unwrap_or_else(|| sin_extension(ruta));
        let familia = nombre(&name, 16).unwrap_or_else(|| familia_win.clone());
        let estilo = nombre(&name, 17).or_else(|| nombre(&name, 2)).unwrap_or_else(|| "Regular".into());
        let completo = nombre(&name, 4).unwrap_or_else(|| format!("{familia} {estilo}"));

        let mut d = Datos { familia: familia.clone(), ..Default::default() };
        let (mut peso, mut cursiva) = (400, estilo.to_lowercase().contains("italic"));
        if let Some(o) = os2 {
            peso = be16(o, 4).filter(|p| (1..=1000).contains(p)).unwrap_or(400);
            let sel = be16(o, 62).unwrap_or(0);
            cursiva = sel & 1 != 0 || sel & (1 << 9) != 0;
            d.clase = be16(o, 30).unwrap_or(0) as i16;
            if let Some(p) = o.get(32..42) {
                d.panose.copy_from_slice(p);
            }
            for (i, r) in d.unicode.iter_mut().enumerate() {
                *r = be32(o, 42 + i * 4).unwrap_or(0);
            }
            if be16(o, 0).unwrap_or(0) >= 1 {
                d.codepages = be32(o, 78);
            }
        }
        fuera.push(Cara { familia, familia_win, estilo, completo, peso, cursiva, categoria: categoria::clasificar(&d).into() });
    }
    if fuera.is_empty() {
        return Err("el archivo no trae ninguna fuente".into());
    }
    Ok(fuera)
}

/// Caracteres que tiene de verdad la primera fuente del archivo (tabla cmap),
/// sin controles ni espacios, como mucho `tope`.
pub fn caracteres(ruta: &Path, tope: usize) -> Result<Vec<u32>, String> {
    let mut f = File::open(ruta).map_err(|e| e.to_string())?;
    let inicio = *inicios(&mut f)?.first().ok_or("vacío")?;
    let t = tablas(&mut f, inicio, &[b"cmap"])?;
    let cmap = t[0].as_deref().and_then(ttf_parser::cmap::Table::parse).ok_or("sin tabla cmap")?;
    let mut set = std::collections::BTreeSet::new();
    for sub in cmap.subtables {
        if !sub.is_unicode() {
            continue;
        }
        sub.codepoints(|c| {
            if let Some(ch) = char::from_u32(c) {
                if !ch.is_control() && !ch.is_whitespace() && !(0xE000..=0xF8FF).contains(&c) {
                    set.insert(c);
                }
            }
        });
    }
    // Fuentes de símbolos (Wingdings y compañía): solo cmap de símbolo, en la zona privada.
    if set.is_empty() {
        for sub in cmap.subtables {
            sub.codepoints(|c| {
                if (0xF020..=0xF0FF).contains(&c) {
                    set.insert(c);
                }
            });
        }
    }
    Ok(set.into_iter().take(tope).collect())
}
