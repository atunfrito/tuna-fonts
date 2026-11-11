#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // La copia con permisos de administrador (fuentes del sistema) no abre ventana.
    if let Some(codigo) = tunafonts_lib::modo_elevado() {
        std::process::exit(codigo);
    }
    tunafonts_lib::run()
}
