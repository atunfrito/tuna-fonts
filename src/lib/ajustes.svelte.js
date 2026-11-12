// Ajustes de la app. Se guardan con tauri-plugin-store en ajustes.json; Rust
// lee ese mismo archivo al arrancar y lo inyecta en window.__TUNA__ (ver
// src-tauri/src/lib.rs), así que el primer fotograma ya sale con el tema y el
// idioma buenos, sin esperar a nada.
import { LazyStore } from "@tauri-apps/plugin-store";
import { version as versionPaquete } from "../../package.json";

// Fuera de Tauri (vista previa en el navegador) la versión sale de package.json.
const inicio = window.__TUNA__ ?? { ajustes: {}, version: versionPaquete };

export const enTauri = "__TAURI_INTERNALS__" in window;
export const version = inicio.version;

export const ATAJOS = { buscar: "Ctrl+F", todas: "Ctrl+A", alternar: "Space" };

const POR_DEFECTO = {
  // null = el de Windows la primera vez; luego, el que elijas.
  tema: null,
  // El de Windows la primera vez (es si está en español, en para lo demás); luego, el que elijas.
  idioma: inicio.idiomaSistema ?? "en",
  // usuario | todos (pide permisos)
  instalarPara: "usuario",
  // instaladas | favoritas
  alAbrir: "instaladas",
  // Fuentes del sistema desbloqueadas (las imprescindibles, nunca).
  desbloqueadas: false,
  // Ver en la lista las del sistema que llevan candado (por defecto, ocultas).
  verBloqueadas: false,
  // Una consulta a GitHub al abrir para ver si hay versión nueva (desactivable).
  buscarActualizaciones: true,
  // Texto de prueba por defecto ("" = el de siempre del idioma).
  textoPrueba: "",
  atajos: { ...ATAJOS },
  vista: "lista",
  tamano: 30,
};

export const ajustes = $state({ ...POR_DEFECTO, ...inicio.ajustes });
ajustes.atajos = { ...ATAJOS, ...(ajustes.atajos || {}) };

// Tema que se ve ahora mismo (resuelve el null con el de Windows).
const oscuroSistema = matchMedia("(prefers-color-scheme: dark)");
export const temaActual = () => ajustes.tema ?? (oscuroSistema.matches ? "dark" : "light");

const store = enTauri ? new LazyStore("ajustes.json", { autoSave: 150 }) : null;

$effect.root(() => {
  $effect(() => {
    document.documentElement.dataset.theme = temaActual();
  });
  // Cada ajuste que cambia se guarda; el plugin agrupa las escrituras seguidas.
  for (const clave of Object.keys(POR_DEFECTO)) {
    $effect(() => {
      const valor = $state.snapshot(ajustes[clave]);
      store?.set(clave, valor);
    });
  }
});

export function ponerTema(tema) {
  ajustes.tema = tema;
  if (!enTauri) return;
  // El fondo nativo de la ventana también, para que al redimensionar no asome el otro color.
  import("@tauri-apps/api/window").then(({ getCurrentWindow }) =>
    getCurrentWindow().setBackgroundColor(tema === "dark" ? [0x19, 0x18, 0x16, 255] : [0xf7, 0xf6, 0xf3, 255]),
  );
}
