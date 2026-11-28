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
    /// Funciones OpenType que interesan para filtrar (smcp, onum, liga…).
    #[serde(default)]
    pub rasgos: Vec<String>,
    /// La frase de muestra que esta cara sabe escribir (ver `muestra`): la de su
    /// idioma, o la única que sepa. Vacía en las latinas, que enseñan el texto
    /// de la interfaz. Si se pinta una frase con caracteres que la fuente no
    /// tiene, media fila sale con la fuente de reserva.
    #[serde(default)]
    pub muestra: String,
}

/// Las funciones OpenType (tabla GSUB) por las que se puede filtrar. Los
/// conjuntos estilísticos ss01…ss20 cuentan todos como «ss».
const RASGOS: &[&str] = &["smcp", "c2sc", "onum", "lnum", "tnum", "pnum", "liga", "dlig", "swsh", "salt", "frac", "zero", "ss"];

/// Lee la lista de funciones de una tabla GSUB (cabecera → FeatureList → etiquetas).
fn rasgos_gsub(gsub: Option<&[u8]>) -> Vec<String> {
    let Some(d) = gsub else { return vec![] };
    let Some(lista) = be16(d, 6).map(|o| o as usize) else { return vec![] };
    let Some(n) = be16(d, lista) else { return vec![] };
    let mut v: Vec<String> = Vec::new();
    for i in 0..n as usize {
        let Some(tag) = d.get(lista + 2 + i * 6..lista + 2 + i * 6 + 4) else { break };
        let tag = String::from_utf8_lossy(tag).to_string();
        let tag = if tag.starts_with("ss") && tag[2..].chars().all(|c| c.is_ascii_digit()) { "ss".to_string() } else { tag };
        if RASGOS.contains(&tag.as_str()) && !v.contains(&tag) {
            v.push(tag);
        }
    }
    v.sort();
    v
}

/// Lo que dice la propia fuente de sí misma (tabla name), para la ficha.
#[derive(Serialize, Default, Debug)]
pub struct Info {
    pub copyright: Option<String>,
    pub version: Option<String>,
    pub fabricante: Option<String>,
    pub disenador: Option<String>,
    pub url: Option<String>,
    pub licencia: Option<String>,
    pub licencia_url: Option<String>,
}

pub fn info(ruta: &Path) -> Result<Info, String> {
    let mut f = File::open(ruta).map_err(|e| e.to_string())?;
    let inicio = *inicios(&mut f)?.first().ok_or("vacío")?;
    let t = tablas(&mut f, inicio, &[b"name"])?;
    let name = t[0].as_deref().and_then(ttf_parser::name::Table::parse).ok_or("sin tabla name")?;
    let url = nombre(&name, 12).or_else(|| nombre(&name, 11));
    Ok(Info {
        copyright: nombre(&name, 0),
        version: nombre(&name, 5),
        fabricante: nombre(&name, 8),
        disenador: nombre(&name, 9),
        url,
        licencia: nombre(&name, 13),
        licencia_url: nombre(&name, 14),
    })
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

/// Las escrituras con las que se puede previsualizar una fuente. El orden es el
/// de preferencia cuando la fuente sabe escribir varias y no dice de qué idioma
/// es: las japonesas y las coreanas traen también los ideogramas tradicionales,
/// así que la tradicional va al final, y solo las coreanas traen hangul.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Escritura {
    China,
    Coreana,
    Japonesa,
    Tradicional,
}

impl Escritura {
    const TODAS: [Escritura; 4] = [Self::China, Self::Coreana, Self::Japonesa, Self::Tradicional];

    /// La frase que se pinta de muestra. La china tiene que ser la misma que
    /// `defaultSample` de `src/i18n/zh.js` (lo comprueba una prueba).
    pub fn muestra(self) -> &'static str {
        match self {
            Self::China => "轻快的金枪鱼跃过懒狗",
            Self::Tradicional => "輕快的金槍魚躍過懶狗",
            Self::Japonesa => "素早いマグロがのら犬を飛び越えた",
            Self::Coreana => "참치가 게으른 개를 뛰어넘었다",
        }
    }

    /// Las páginas de códigos de OS/2 que la delatan (bits de `ulCodePageRange1`):
    /// 17 japonés (932), 18 simplificado (936), 19 coreano (949), 20 tradicional
    /// (950) y 21 coreano Johab (1361).
    fn paginas(self) -> &'static [u32] {
        match self {
            Self::China => &[18],
            Self::Coreana => &[19, 21],
            Self::Japonesa => &[17],
            Self::Tradicional => &[20],
        }
    }

    /// El sufijo del nombre, que es lo que distingue a las pan-CJK entre sí
    /// («Noto Sans JP», «Source Han Sans SC», «… Sans K»).
    fn por_sufijo(trozo: &str, familia: &str) -> Option<Self> {
        Some(match trozo {
            "JP" | "JPN" => Self::Japonesa,
            "KR" | "KOR" => Self::Coreana,
            "SC" | "CN" | "GB" => Self::China,
            "TC" | "TW" | "HK" | "HC" => Self::Tradicional,
            "K" if familia.contains("HAN") => Self::Coreana,
            _ => return None,
        })
    }
}

/// El idioma que dice ser la fuente, si lo dice: la página de códigos de OS/2
/// cuando declara una sola, o si no el sufijo del nombre. Las pan-CJK declaran
/// las cuatro páginas a la vez, y entonces solo queda el sufijo.
fn escritura_declarada(os2: Option<&[u8]>, familia: &str) -> Option<Escritura> {
    let paginas = os2.filter(|o| be16(o, 0).unwrap_or(0) >= 1).and_then(|o| be32(o, 78)).unwrap_or(0);
    let mut citadas = Escritura::TODAS.iter().filter(|e| e.paginas().iter().any(|b| paginas & (1 << b) != 0));
    // Una sola página de códigos y ninguna más: esa es. Con varias no dice nada.
    if let (Some(solo), None) = (citadas.next(), citadas.next()) {
        return Some(*solo);
    }
    let arriba = familia.to_uppercase();
    arriba.split(|c: char| !c.is_ascii_alphanumeric()).rev().find_map(|trozo| Escritura::por_sufijo(trozo, &arriba))
}

/// La muestra que se le pide a la vista previa de esta cara: la de su escritura
/// si la sabe escribir; si no, la primera que sepa, en el orden de
/// `Escritura::TODAS`; si ninguna, vacía, y la fila enseña el texto de la
/// interfaz.
///
/// Se pregunta a la tabla cmap, que es la que sabe de verdad qué caracteres
/// tiene la fuente (las banderas de OS/2 mienten a menudo). Y se pregunta la
/// frase entera, no tres ideogramas sueltos: un japonés trae 中 / 文 / 字 pero
/// no 轻 ni 跃, y con esos tres bastaba para colar la muestra china, así que la
/// fila salía medio con la fuente y medio con la de reserva.
fn muestra_de(cmap: Option<&[u8]>, os2: Option<&[u8]>, familia: &str) -> &'static str {
    let Some(cmap) = cmap.and_then(ttf_parser::cmap::Table::parse) else { return "" };
    let subs: Vec<_> = cmap.subtables.into_iter().filter(|sub| sub.is_unicode()).collect();
    let sabe = |e: Escritura| subs.iter().any(|sub| e.muestra().chars().all(|c| sub.glyph_index(u32::from(c)).is_some()));
    let puede: Vec<Escritura> = Escritura::TODAS.into_iter().filter(|e| sabe(*e)).collect();
    let declarada = escritura_declarada(os2, familia).filter(|e| puede.contains(e));
    declarada.or(puede.first().copied()).map_or("", Escritura::muestra)
}

/// Todas las caras de un archivo, con su categoría.
pub fn caras(ruta: &Path) -> Result<Vec<Cara>, String> {
    let mut f = File::open(ruta).map_err(|e| e.to_string())?;
    let mut fuera = Vec::new();
    for inicio in inicios(&mut f)? {
        let t = tablas(&mut f, inicio, &[b"name", b"OS/2", b"GSUB", b"cmap"])?;
        let name = t[0].as_deref().and_then(ttf_parser::name::Table::parse).ok_or("sin tabla name")?;
        let os2 = t[1].as_deref();
        let rasgos = rasgos_gsub(t[2].as_deref());

        let familia_win = nombre(&name, 1).unwrap_or_else(|| sin_extension(ruta));
        let familia = nombre(&name, 16).unwrap_or_else(|| familia_win.clone());
        let muestra = muestra_de(t[3].as_deref(), os2, &familia).to_string();
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
        fuera.push(Cara { familia, familia_win, estilo, completo, peso, cursiva, categoria: categoria::clasificar(&d).into(), rasgos: rasgos.clone(), muestra });
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

#[cfg(test)]
mod pruebas {
    use super::*;

    /// Gochi Hand, la que usa el repo para probar, solo trae alfabeto latino.
    #[test]
    fn una_fuente_latina_no_trae_ideogramas() {
        let ruta = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/GochiHand-Regular.ttf");
        let cs = caras(&ruta).unwrap();
        assert!(cs.iter().all(|c| c.muestra.is_empty()), "Gochi Hand no sabe escribir ninguna de las muestras");
    }

    /// Una fuente china pide la muestra china. Se salta si este Windows no la tiene.
    #[test]
    fn una_fuente_china_trae_la_suya() {
        let ruta = Path::new(r"C:\Windows\Fonts\msyh.ttc");
        if !ruta.is_file() {
            return;
        }
        let cs = caras(ruta).unwrap();
        assert_eq!(cs[0].muestra, Escritura::China.muestra(), "Microsoft YaHei es china");
    }

    /// Cada escritura pide la suya: una japonesa no se conforma con la china
    /// (le faltan 轻枪跃过懒) ni una tradicional con la japonesa. Se saltan las
    /// que este Windows no traiga.
    #[test]
    fn cada_escritura_pide_su_muestra() {
        for (ruta, esperada) in [
            (r"C:\Windows\Fonts\YuGothR.ttc", Escritura::Japonesa.muestra()),
            (r"C:\Windows\Fonts\msgothic.ttc", Escritura::Japonesa.muestra()),
            (r"C:\Windows\Fonts\msjh.ttc", Escritura::Tradicional.muestra()),
        ] {
            let ruta = Path::new(ruta);
            if !ruta.is_file() {
                continue;
            }
            let cs = caras(ruta).unwrap();
            assert_eq!(cs[0].muestra, esperada, "{} no pide la muestra de su escritura", ruta.display());
        }
    }

    /// El sufijo del nombre es lo que distingue a las pan-CJK, que declaran las
    /// cuatro páginas de códigos y no dicen nada por ahí.
    #[test]
    fn el_sufijo_del_nombre_dice_el_idioma() {
        assert_eq!(escritura_declarada(None, "Noto Sans JP"), Some(Escritura::Japonesa));
        assert_eq!(escritura_declarada(None, "Noto Sans KR"), Some(Escritura::Coreana));
        assert_eq!(escritura_declarada(None, "Noto Sans SC"), Some(Escritura::China));
        assert_eq!(escritura_declarada(None, "Noto Sans HK"), Some(Escritura::Tradicional));
        assert_eq!(escritura_declarada(None, "Source Han Sans K VF"), Some(Escritura::Coreana));
        assert_eq!(escritura_declarada(None, "Source Han Serif JP SemiBold"), Some(Escritura::Japonesa));
        assert_eq!(escritura_declarada(None, "Helvetica Neue"), None);
    }

    /// La página de códigos de OS/2 manda sobre el sufijo, y si declara dos
    /// escrituras no elige ninguna: las pan-CJK las declaran todas.
    #[test]
    fn la_pagina_de_codigos_manda() {
        let os2 = |bits: u32| {
            let mut t = vec![0u8; 82];
            t[..2].copy_from_slice(&1u16.to_be_bytes());
            t[78..82].copy_from_slice(&bits.to_be_bytes());
            t
        };
        // bit 17 = 932, japonés.
        assert_eq!(escritura_declarada(Some(&os2(1 << 17)), "Noto Sans SC"), Some(Escritura::Japonesa));
        // 17 y 20: japonés y tradicional a la vez, como las pan-CJK.
        assert_eq!(escritura_declarada(Some(&os2((1 << 17) | (1 << 20))), "Cualquiera"), None);
        // Sin tabla OS/2 se cae al sufijo del nombre.
        assert_eq!(escritura_declarada(None, "Noto Sans TC"), Some(Escritura::Tradicional));
    }

    /// La muestra de aquí y la de `src/i18n/zh.js` tienen que ser la misma: si
    /// alguien cambia el texto, salta esta prueba en vez de que el hueco de
    /// Ajustes y la vista previa digan cosas distintas.
    #[test]
    fn la_muestra_coincide_con_la_del_idioma() {
        let ruta = Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/i18n/zh.js");
        let Ok(texto) = std::fs::read_to_string(ruta) else { return };
        let china = Escritura::China.muestra();
        assert!(texto.contains(china), "la muestra china ya no está en src/i18n/zh.js: {china}");
    }
}
