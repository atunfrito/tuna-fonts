//! Activar y desactivar una fuente de usuario de prueba, sin tocar las de
//! verdad: carpetas temporales y una clave de registro propia en HKCU que se
//! borra al acabar. No se carga nada en Windows (tocar_windows = false).

use std::{collections::HashSet, path::Path};
use tunafonts_lib::fuentes::{self, cache::Cache, cambios, leer, registro, Entorno, Origen};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

struct Limpieza(String);
impl Drop for Limpieza {
    fn drop(&mut self) {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let _ = hkcu.delete_subkey_all(&self.0);
        // Y la carpeta de pruebas, si se ha quedado vacía.
        let _ = hkcu.delete_subkey(r"Software	una-fonts-pruebas");
    }
}

#[test]
fn instalar_desactivar_y_activar_una_de_usuario() {
    let tmp = tempfile::tempdir().unwrap();
    let base = format!(r"Software\tuna-fonts-pruebas\{}", std::process::id());
    let _limpia = Limpieza(base.clone());
    let ent = Entorno {
        fuentes_sistema: tmp.path().join("sistema"),
        fuentes_usuario: tmp.path().join("usuario"),
        clave_sistema: format!(r"{base}\Sistema"),
        clave_usuario: format!(r"{base}\Fonts"),
        datos: tmp.path().join("datos"),
        tocar_windows: false,
    };
    let fuente = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/GochiHand-Regular.ttf");

    // Leer: familia y categoría.
    let caras = leer::caras(&fuente).unwrap();
    assert_eq!(caras[0].familia, "Gochi Hand");
    assert_eq!(caras[0].categoria, "script");
    assert!(!leer::caracteres(&fuente, 500).unwrap().is_empty());

    // Instalar para el usuario.
    let instalada = cambios::instalar_usuario(&ent, &fuente, &caras).unwrap();
    assert!(instalada.is_file());
    let reg = registro::leer(registro::hkcu(), &ent.clave_usuario);
    assert_eq!(reg.len(), 1);
    assert_eq!(reg[0].0, "Gochi Hand (TrueType)");
    assert_eq!(Path::new(&reg[0].1), instalada);

    let mut cache = Cache::default();
    let entradas = fuentes::entradas(&ent, &[]);
    cache.actualizar(&entradas);
    let f = cache.familias.iter().find(|f| f.nombre == "Gochi Hand").unwrap();
    assert!(f.activa);
    assert_eq!(f.origen, Origen::Biblioteca);

    // Desactivar: fuera del registro y a la carpeta de la app, sin borrar.
    let mut apagadas = HashSet::new();
    let r = cambios::alternar(&ent, &entradas, &cache, &["Gochi Hand".into()], false, false, &mut apagadas);
    assert_eq!(r.cambiadas, 1, "{:?}", r.errores);
    assert!(!instalada.exists());
    assert!(registro::leer(registro::hkcu(), &ent.clave_usuario).is_empty());
    let g = fuentes::guardadas(&ent);
    assert_eq!(g.len(), 1);
    assert!(g[0].archivo.starts_with(ent.desactivadas()) && g[0].archivo.is_file());

    let entradas = fuentes::entradas(&ent, &[]);
    cache.actualizar(&entradas);
    let f = cache.familias.iter().find(|f| f.nombre == "Gochi Hand").unwrap();
    assert!(!f.activa);

    // Desactivar otra vez no hace nada.
    let r = cambios::alternar(&ent, &entradas, &cache, &["Gochi Hand".into()], false, false, &mut apagadas);
    assert_eq!(r.cambiadas, 0);

    // Activar: vuelve a su sitio y a su entrada del registro.
    let r = cambios::alternar(&ent, &entradas, &cache, &["Gochi Hand".into()], true, false, &mut apagadas);
    assert_eq!(r.cambiadas, 1, "{:?}", r.errores);
    assert!(instalada.is_file());
    let reg = registro::leer(registro::hkcu(), &ent.clave_usuario);
    assert_eq!(reg, vec![("Gochi Hand (TrueType)".to_string(), instalada.to_string_lossy().into_owned())]);
    assert!(fuentes::guardadas(&ent).is_empty());

    let entradas = fuentes::entradas(&ent, &[]);
    cache.actualizar(&entradas);
    assert!(cache.familias.iter().find(|f| f.nombre == "Gochi Hand").unwrap().activa);
}

#[test]
fn las_imprescindibles_no_se_tocan() {
    for f in ["Segoe UI", "Segoe UI Variable", "Segoe UI Emoji", "Segoe Fluent Icons", "Segoe MDL2 Assets", "Marlett", "Tahoma", "Microsoft Sans Serif", "Arial", "Consolas"] {
        assert!(fuentes::es_imprescindible(f), "{f}");
    }
    for f in ["Arial Narrow", "Georgia", "Times New Roman", "Segoe Print"] {
        assert!(!fuentes::es_imprescindible(f), "{f}");
    }
}
