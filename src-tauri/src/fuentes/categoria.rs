//! Categoría de una fuente: sans, serif, script, display, mono u otros (otros
//! idiomas). Primero la tabla OS/2 (panose y sFamilyClass); si no basta, el
//! nombre de la familia. Si no está claro, display.

#[derive(Default, Clone, Debug)]
pub struct Datos {
    pub familia: String,
    pub panose: [u8; 10],
    /// sFamilyClass: clase en el byte alto, subclase en el bajo.
    pub clase: i16,
    pub unicode: [u32; 4],
    /// ulCodePageRange1 (OS/2 versión 1 o más).
    pub codepages: Option<u32>,
}

fn tiene(nombre: &str, palabras: &[&str]) -> bool {
    palabras.iter().any(|p| nombre.contains(p))
}

/// Palabras sueltas (no dentro de otra): " jp" no debe saltar con "jpeg".
fn tiene_palabra(nombre: &str, palabras: &[&str]) -> bool {
    nombre
        .split(|c: char| !c.is_alphanumeric())
        .any(|w| palabras.contains(&w))
}

const IDIOMAS: &[&str] = &[
    "arabic", "hebrew", "devanagari", "bengali", "tamil", "telugu", "kannada", "malayalam", "gujarati",
    "gurmukhi", "oriya", "sinhala", "thai", "lao", "khmer", "myanmar", "tibetan", "ethiopic", "georgian",
    "armenian", "mongolian", "naskh", "kufi", "nastaliq", "mincho", "hangul", "batang", "gulim", "dotum",
    "gungsuh", "malgun", "meiryo", "simsun", "simhei", "nsimsun", "mingliu", "pmingliu", "kaiti", "fangsong",
    "dengxian", "yahei", "jhenghei", "nirmala", "ebrima", "gadugi", "leelawadee", "javanese", "mongolian",
    "yi baiti", "himalaya", "estrangelo", "mangal", "kokila", "aparajita", "utsaah", "vani", "vijaya",
];

/// Bits de ulCodePageRange1 de alfabetos latinos, griego y cirílico.
const CP_LATINO: u32 = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 7) | (1 << 8);
/// Bits de ulCodePageRange1 de alfabetos de otros idiomas (tailandés, CJK, árabe, hebreo...).
const CP_OTROS: u32 = (1 << 5) | (1 << 6) | (1 << 16) | (1 << 17) | (1 << 18) | (1 << 19) | (1 << 20) | (1 << 21);

pub fn clasificar(d: &Datos) -> &'static str {
    let n = d.familia.to_lowercase();
    let familia_panose = d.panose[0];
    let clase = (d.clase >> 8) as u8;

    // Otros idiomas: el nombre lo dice, o no cubre ningún alfabeto latino y sí otros.
    let sin_latino = d.codepages.is_some_and(|c| c & CP_LATINO == 0 && c & CP_OTROS != 0);
    let unicode_latino = d.unicode[0] & 0b11 != 0;
    if tiene(&n, IDIOMAS) || tiene_palabra(&n, &["jp", "kr", "sc", "tc", "hk", "cjk", "yu"]) || n.starts_with("ms gothic") || n.starts_with("ms mincho") || (sin_latino && !unicode_latino) {
        return "otros";
    }

    // Monoespaciada: panose (texto latino con proporción "monospaced") o el nombre
    // ("JetBrainsMono" sí; "Monotype Corsiva" y "Lucida Sans Unicode" no).
    let mono_nombre = n.split(|c: char| !c.is_alphanumeric()).any(|w| {
        (w.ends_with("mono") && w != "monotype")
            || ["code", "courier", "typewriter", "terminal", "fixedsys"].contains(&w)
            || w.starts_with("consol")
    });
    if (familia_panose == 2 && d.panose[3] == 9) || mono_nombre {
        return "mono";
    }

    // Símbolos, dingbats y decorativas.
    if familia_panose == 5 || clase == 12 || tiene(&n, &["dings", "symbol", "icons", "mdl2", "marlett", "webdings", "emoji"]) {
        return "display";
    }

    // Escritas a mano.
    if familia_panose == 3 || clase == 10 || tiene(&n, &["script", "hand", "brush", "marker", "caveat", "pacifico", "signature", "cursive", "kalam", "calligraph", "chancery", "gochi"]) {
        return "script";
    }
    if familia_panose == 4 || clase == 9 {
        return "display";
    }

    // Por panose de texto latino (estilo de serifa); si no lo trae, por sFamilyClass.
    if familia_panose == 2 {
        match d.panose[1] {
            2..=10 => return "serif",
            11..=15 => return "sans",
            _ => {}
        }
    }
    match clase {
        1..=5 | 7 => return "serif",
        8 => return "sans",
        _ => {}
    }

    // Por el nombre.
    if tiene(&n, &["sans", "grotesk", "grotesque", "gothic", "helvetica", "arial", "verdana", "segoe", "inter", "roboto", "calibri", "tahoma"]) {
        return "sans";
    }
    if tiene(&n, &["serif", "roman", "garamond", "baskerville", "times", "georgia", "bodoni", "caslon", "didot", "slab", "antiqua", "book"]) {
        return "serif";
    }
    "display"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(familia: &str) -> Datos {
        Datos { familia: familia.into(), codepages: Some(1), unicode: [3, 0, 0, 0], ..Default::default() }
    }

    #[test]
    fn por_panose() {
        let mut x = d("Algo");
        x.panose = [2, 11, 6, 4, 2, 2, 2, 2, 2, 4]; // Arial
        assert_eq!(clasificar(&x), "sans");
        x.panose = [2, 2, 6, 3, 5, 4, 5, 2, 3, 4]; // Times New Roman
        assert_eq!(clasificar(&x), "serif");
        x.panose = [2, 11, 6, 9, 2, 2, 4, 3, 2, 4]; // Consolas: proporción 9
        assert_eq!(clasificar(&x), "mono");
        x.panose = [3, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(clasificar(&x), "script");
        x.panose = [4, 2, 2, 2, 2, 2, 2, 2, 2, 2];
        assert_eq!(clasificar(&x), "display");
        x.panose = [5, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(clasificar(&x), "display");
    }

    #[test]
    fn por_clase() {
        let mut x = d("Algo");
        x.clase = 8 << 8;
        assert_eq!(clasificar(&x), "sans");
        x.clase = (1 << 8) | 4;
        assert_eq!(clasificar(&x), "serif");
        x.clase = 5 << 8; // slab
        assert_eq!(clasificar(&x), "serif");
        x.clase = 10 << 8;
        assert_eq!(clasificar(&x), "script");
    }

    #[test]
    fn por_nombre() {
        assert_eq!(clasificar(&d("JetBrains Mono")), "mono");
        assert_eq!(clasificar(&d("Courier New")), "mono");
        assert_eq!(clasificar(&d("Open Sans")), "sans");
        assert_eq!(clasificar(&d("Space Grotesk")), "sans");
        assert_eq!(clasificar(&d("Libre Baskerville")), "serif");
        assert_eq!(clasificar(&d("Instrument Serif")), "serif");
        assert_eq!(clasificar(&d("Dancing Script")), "script");
        assert_eq!(clasificar(&d("Gochi Hand")), "script");
        assert_eq!(clasificar(&d("Wingdings")), "display");
        assert_eq!(clasificar(&d("Segoe MDL2 Assets")), "display");
    }

    #[test]
    fn el_nombre_manda_sobre_el_panose_en_mono_e_idiomas() {
        let mut x = d("Fira Code");
        x.panose = [2, 11, 5, 9, 5, 0, 0, 2, 0, 4];
        assert_eq!(clasificar(&x), "mono");
        let mut y = d("Noto Sans JP");
        y.panose = [2, 11, 5, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(clasificar(&y), "otros");
    }

    #[test]
    fn otros_idiomas() {
        assert_eq!(clasificar(&d("Noto Naskh Arabic")), "otros");
        assert_eq!(clasificar(&d("Yu Gothic")), "otros");
        assert_eq!(clasificar(&d("Malgun Gothic")), "otros");
        assert_eq!(clasificar(&d("MS Gothic")), "otros");
        assert_eq!(clasificar(&d("Microsoft YaHei")), "otros");
        // Sin nombre que lo delate: solo cubre japonés.
        let x = Datos { familia: "Algo".into(), codepages: Some(1 << 17), ..Default::default() };
        assert_eq!(clasificar(&x), "otros");
    }

    #[test]
    fn panose_antes_que_la_clase() {
        // Century Gothic: sFamilyClass dice serif de transición, panose dice sans.
        let mut x = d("Century Gothic");
        x.clase = (2 << 8) | 4;
        x.panose = [2, 11, 5, 2, 2, 2, 2, 2, 2, 4];
        assert_eq!(clasificar(&x), "sans");
        // Sin "mono" de "Monotype" ni "code" de "Unicode".
        let mut y = d("Lucida Sans Unicode");
        y.panose = [2, 11, 6, 2, 3, 5, 4, 2, 2, 4];
        assert_eq!(clasificar(&y), "sans");
        assert_ne!(clasificar(&d("Monotype Corsiva")), "mono");
        assert_eq!(clasificar(&d("JetBrainsMono Nerd Font")), "mono");
    }

    #[test]
    fn sin_pistas_va_a_display() {
        let x = Datos { familia: "Zzyzx".into(), codepages: Some(1), unicode: [3, 0, 0, 0], ..Default::default() };
        assert_eq!(clasificar(&x), "display");
        // "Scope" no es "sc" de chino simplificado.
        assert_ne!(clasificar(&d("Scope One")), "otros");
    }
}
