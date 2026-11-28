// Descubrir: catálogo de Google Fonts y sus vistas previas.
// Las vistas previas usan la API CSS de Google Fonts con &text= (solo los
// caracteres de la muestra), por lotes de 15 y solo de lo que se ve.
import { enTauri } from "./ajustes.svelte.js";
import { avisar } from "./avisos.svelte.js";
import { t } from "./i18n.svelte.js";

export const descubrir = $state({
  lista: [],
  cargando: false,
  error: "",
  // familia -> porcentaje de descarga
  descargas: {},
});

let pedido = null;
export function cargarCatalogo(forzar = false) {
  if (pedido && !forzar) return pedido;
  if (!enTauri) {
    if (window.__DEMO__?.google) descubrir.lista = window.__DEMO__.google;
    return Promise.resolve();
  }
  descubrir.cargando = true;
  descubrir.error = "";
  pedido = import("@tauri-apps/api/core")
    .then(({ invoke }) => invoke("google_catalogo", { forzar }))
    .then((l) => {
      descubrir.lista = l;
    })
    .catch((e) => {
      descubrir.error = String(e);
      pedido = null;
    })
    .finally(() => (descubrir.cargando = false));
  return pedido;
}

// familia -> caracteres ya pedidos
const pedidos = new Map();
const LOTE = 15;

/** Pide (si falta algo) la vista previa de estas familias con este texto. */
export function pedirVistas(familias, texto) {
  const chars = new Set([...(texto + "Aa")].filter((c) => c.trim()));
  const faltan = [];
  for (const f of familias) {
    const ya = pedidos.get(f) ?? new Set();
    if ([...chars].some((c) => !ya.has(c))) faltan.push(f);
  }
  for (let i = 0; i < faltan.length; i += LOTE) {
    const lote = faltan.slice(i, i + LOTE);
    // El texto del lote: la muestra más lo que ya se pidió antes (la nueva regla sustituye a la vieja).
    const todo = new Set(chars);
    for (const f of lote) for (const c of pedidos.get(f) ?? []) todo.add(c);
    for (const f of lote) pedidos.set(f, new Set(todo));
    const l = document.createElement("link");
    l.rel = "stylesheet";
    l.href =
      "https://fonts.googleapis.com/css2?" +
      lote.map((f) => "family=" + encodeURIComponent(f).replace(/%20/g, "+")).join("&") +
      "&display=swap&text=" +
      encodeURIComponent([...todo].join(""));
    document.head.appendChild(l);
  }
}

/** Pide las vistas de estas familias agrupadas por la muestra que le toca a cada
 *  una (un lote por texto distinto): las latinas con el texto de la interfaz, la
 *  japonesa con el japonés… `textoDe(g)` es quien lo decide. */
export function pedirVistasPorTexto(familias, textoDe) {
  const grupos = new Map();
  for (const g of familias) {
    const texto = textoDe(g);
    if (!grupos.has(texto)) grupos.set(texto, []);
    grupos.get(texto).push(g.nombre);
  }
  for (const [texto, lote] of grupos) pedirVistas(lote, texto);
}

/** En segundo plano tras abrir: el catálogo y las vistas de las más populares.
 *  `textoDe` es el mismo que el de `pedirVistasPorTexto`. */
export function precargar(textoDe) {
  const idle = window.requestIdleCallback ?? ((f) => setTimeout(f, 1200));
  idle(() =>
    cargarCatalogo().then(() => {
      const top = [...descubrir.lista].sort((a, b) => a.popularidad - b.popularidad).slice(0, 45);
      pedirVistasPorTexto(top, textoDe);
    }),
  );
}

let escuchando = false;
export async function instalarGoogle(familia, alTerminar) {
  if (!enTauri || descubrir.descargas[familia] != null) return;
  const { invoke } = await import("@tauri-apps/api/core");
  if (!escuchando) {
    escuchando = true;
    const { listen } = await import("@tauri-apps/api/event");
    listen("descarga", ({ payload }) => {
      if (payload.error || payload.pct >= 100) return;
      if (descubrir.descargas[payload.familia] != null) descubrir.descargas[payload.familia] = payload.pct;
    });
  }
  descubrir.descargas[familia] = 1;
  try {
    const r = await invoke("google_instalar", { familia });
    alTerminar(r);
    if (r.resultado.errores.length) avisar("err", t("err.instalar"), r.resultado.errores[0]);
    else avisar("ok", t("ok.instalada", { n: familia }), t("ok.yaSale"));
  } catch (e) {
    avisar("err", t("err.descargar", { n: familia }), String(e));
  } finally {
    delete descubrir.descargas[familia];
  }
}
