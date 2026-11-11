//! Lee las fuentes de este equipo (sin tocar nada) y enseña cuánto tarda.
//! cargo run --release --example escanear
use std::time::Instant;
use tunafonts_lib::fuentes::{self, cache::Cache, Entorno};

fn main() {
    let tmp = std::env::temp_dir().join("tuna-fonts-escanear");
    let ent = Entorno { tocar_windows: false, ..Entorno::real(tmp) };
    let t = Instant::now();
    let entradas = fuentes::entradas(&ent, &[]);
    let t1 = t.elapsed();
    let mut c = Cache::default();
    let n = c.actualizar(&entradas);
    let t2 = t.elapsed();
    let mut c2 = Cache { version: c.version, archivos: c.archivos.clone(), familias: vec![] };
    let t3 = Instant::now();
    c2.actualizar(&fuentes::entradas(&ent, &[]));
    println!("{} archivos ({t1:?} en listar), {n} leídos en {t2:?}; segunda pasada {:?}", entradas.len(), t3.elapsed());
    println!("{} familias, {} con error", c.familias.len(), c.familias.iter().filter(|f| f.error.is_some()).count());
    let mut por = std::collections::BTreeMap::new();
    for f in &c.familias { *por.entry(f.categoria.clone()).or_insert(0) += 1; }
    println!("{por:?}");
    for cat in ["sans", "serif", "script", "display", "mono", "otros"] {
        let v: Vec<_> = c.familias.iter().filter(|f| f.categoria == cat).take(14).map(|f| f.nombre.as_str()).collect();
        println!("{cat}: {}", v.join(", "));
    }
    for f in c.familias.iter().filter(|f| f.error.is_some()).take(5) { println!("error: {} {:?}", f.nombre, f.error); }
}
