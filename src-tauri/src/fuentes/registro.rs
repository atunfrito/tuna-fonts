//! Las listas de fuentes de Windows en el registro (HKLM para todos, HKCU para
//! el usuario): nombre del valor → archivo.

use winreg::{enums::*, RegKey, HKEY};

pub const CLAVE: &str = r"Software\Microsoft\Windows NT\CurrentVersion\Fonts";

pub fn hklm() -> HKEY {
    HKEY_LOCAL_MACHINE
}
pub fn hkcu() -> HKEY {
    HKEY_CURRENT_USER
}

/// (nombre del valor, datos) de todas las entradas.
pub fn leer(raiz: HKEY, clave: &str) -> Vec<(String, String)> {
    let Ok(k) = RegKey::predef(raiz).open_subkey_with_flags(clave, KEY_READ) else { return vec![] };
    k.enum_values()
        .filter_map(Result::ok)
        .filter_map(|(nombre, v)| {
            let datos: String = winreg::types::FromRegValue::from_reg_value(&v).ok()?;
            Some((nombre, datos))
        })
        .collect()
}

pub fn poner(raiz: HKEY, clave: &str, nombre: &str, datos: &str) -> Result<(), String> {
    let (k, _) = RegKey::predef(raiz).create_subkey_with_flags(clave, KEY_SET_VALUE).map_err(|e| e.to_string())?;
    k.set_value(nombre, &datos).map_err(|e| e.to_string())
}

pub fn quitar(raiz: HKEY, clave: &str, nombre: &str) -> Result<(), String> {
    let k = RegKey::predef(raiz).open_subkey_with_flags(clave, KEY_SET_VALUE).map_err(|e| e.to_string())?;
    match k.delete_value(nombre) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

/// Nombre de valor al estilo de Windows: "Gochi Hand Regular (TrueType)".
pub fn nombre_valor(completos: &[String], cff: bool) -> String {
    let mut vistos: Vec<&str> = Vec::new();
    for c in completos {
        if !vistos.contains(&c.as_str()) {
            vistos.push(c);
        }
    }
    format!("{} ({})", vistos.join(" & "), if cff { "OpenType" } else { "TrueType" })
}
