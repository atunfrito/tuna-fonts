use serde_json::{json, Value};
use std::fs;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

/// Mismo archivo que usa tauri-plugin-store desde la interfaz (ver src/lib/ajustes.svelte.js).
const AJUSTES: &str = "ajustes.json";

/// Fondo de la ventana de cada tema (--win en src/app.css): así no hay
/// destello blanco mientras carga la página.
const FONDO_CLARO: (u8, u8, u8, u8) = (0xf7, 0xf6, 0xf3, 255);
const FONDO_OSCURO: (u8, u8, u8, u8) = (0x19, 0x18, 0x16, 255);

fn leer_ajustes(app: &AppHandle) -> Value {
    app.path()
        .app_data_dir()
        .ok()
        .and_then(|d| fs::read_to_string(d.join(AJUSTES)).ok())
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .filter(Value::is_object)
        .unwrap_or_else(|| json!({}))
}

/// Antes de la primera línea de la página: los ajustes guardados y el tema
/// puesto en <html>, para que el primer fotograma ya salga con su color e idioma.
fn script_inicio(inicio: &Value) -> String {
    format!(
        "window.__TUNA__ = {inicio};
        (function (t) {{
          if (t !== 'light' && t !== 'dark')
            t = matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
          var d = document, s = function () {{ d.documentElement.dataset.theme = t; }};
          if (d.documentElement) s();
          else new MutationObserver(function (m, o) {{
            if (d.documentElement) {{ s(); o.disconnect(); }}
          }}).observe(d, {{ childList: true }});
        }})(window.__TUNA__.ajustes.tema);"
    )
}

/// Tema oscuro de Windows (apps), para la primera vez que se abre.
fn windows_oscuro() -> bool {
    use std::os::windows::process::CommandExt;
    std::process::Command::new("reg")
        .args([
            "query",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "/v",
            "AppsUseLightTheme",
        ])
        .creation_flags(0x0800_0000)
        .output()
        .map(|s| String::from_utf8_lossy(&s.stdout).contains("0x0"))
        .unwrap_or(false)
}

pub fn run() {
    tauri::Builder::default()
        // Una sola tuna-fonts a la vez: dos moviendo las mismas fuentes se pisarían.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(v) = app.get_webview_window("main") {
                let _ = v.unminimize();
                let _ = v.set_focus();
            }
        }))
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();
            let ajustes = leer_ajustes(&handle);
            let oscuro = match ajustes["tema"].as_str() {
                Some("dark") => true,
                Some("light") => false,
                _ => windows_oscuro(),
            };
            let inicio = json!({ "ajustes": ajustes, "version": app.package_info().version.to_string() });

            WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("tuna-fonts")
                .inner_size(1280.0, 820.0)
                .min_inner_size(960.0, 600.0)
                .center()
                .decorations(false)
                .shadow(true)
                .zoom_hotkeys_enabled(false)
                .background_color(if oscuro { FONDO_OSCURO } else { FONDO_CLARO }.into())
                .initialization_script(script_inicio(&inicio))
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error al arrancar tuna-fonts");
}
