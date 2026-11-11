// Las fuentes instaladas: se pintan al momento desde la caché de Rust y luego
// se comprueba en segundo plano si ha cambiado algo.
import { enTauri } from "./ajustes.svelte.js";
import { avisar } from "./avisos.svelte.js";
import { t } from "./i18n.svelte.js";

export const fuentes = $state({
  lista: [],
  // true hasta tener algo que enseñar (primera vez sin caché).
  cargando: true,
  // Tarda más de 150 ms: ahora sí, esqueleto.
  esqueleto: false,
  // Ocupado activando/desactivando/instalando.
  ocupado: false,
});

let invocar = null;
async function llamar(cmd, args) {
  if (!enTauri) return demo(cmd, args);
  if (!invocar) invocar = (await import("@tauri-apps/api/core")).invoke;
  return invocar(cmd, args);
}

// Las que Windows ya tenía cargadas al abrir: esas se pintan por su nombre,
// sin cargar nada. Las demás (desactivadas, de carpetas, recién instaladas)
// se pintan desde su archivo.
let vistasAlAbrir = null;
const caras = new Map();
let convertir = null;

function registrarCara(f) {
  if (caras.has(f.archivo)) return caras.get(f.archivo);
  const id = "tf" + caras.size;
  caras.set(f.archivo, id);
  if (convertir) {
    const url = convertir(f.archivo);
    const e = f.estilos[0];
    const regla = `@font-face{font-family:"${id}";src:url("${url}");font-display:block;font-weight:${e?.peso ?? 400};font-style:${e?.cursiva ? "italic" : "normal"};}`;
    hoja().insertRule(regla, hoja().cssRules.length);
  }
  return id;
}

let _hoja = null;
function hoja() {
  if (!_hoja) {
    const el = document.createElement("style");
    document.head.appendChild(el);
    _hoja = el.sheet;
  }
  return _hoja;
}

const css = (n) => `"${n.replace(/"/g, "")}"`;

/** font-family (y peso/estilo) para pintar la vista previa de una familia. */
export function estiloDe(f, estilo = null) {
  const e = estilo ?? f.estilos[0];
  const local = f.activa && f.origen !== "car" && vistasAlAbrir?.has(f.nombre);
  const familia = local
    ? `${css(f.nombre)}, ${css(e?.familia_win ?? f.nombre)}, var(--sans)`
    : `${css(registrarCaraDe(f, e))}, var(--sans)`;
  return { familia, peso: e?.peso ?? 400, cursiva: !!e?.cursiva };
}

function registrarCaraDe(f, e) {
  if (!e || e.archivo === f.archivo) return registrarCara(f);
  return registrarCara({ ...f, archivo: e.archivo, estilos: [e] });
}

function poner(lista) {
  fuentes.lista = lista;
  if (!vistasAlAbrir) vistasAlAbrir = new Set(lista.filter((f) => f.activa && f.origen !== "car").map((f) => f.nombre));
  fuentes.cargando = false;
  fuentes.esqueleto = false;
}

const avisadas = new Set();
function avisarErrores(lista) {
  const nuevas = lista.filter((f) => f.error && !avisadas.has(f.archivo));
  nuevas.forEach((f) => avisadas.add(f.archivo));
  if (nuevas.length === 1) avisar("err", t("err.leer", { n: nuevas[0].nombre }), nuevas[0].error);
  else if (nuevas.length > 1) avisar("err", t("err.leerVarias", { n: nuevas.length }), t("err.leerTexto"));
}

export async function iniciarFuentes() {
  if (enTauri) convertir = (await import("@tauri-apps/api/core")).convertFileSrc;
  const tarde = setTimeout(() => fuentes.cargando && (fuentes.esqueleto = true), 150);
  try {
    const guardadas = await llamar("fuentes_guardadas");
    if (guardadas.length) poner(guardadas);
  } catch {}
  await actualizarFuentes();
  clearTimeout(tarde);
  if (enTauri) {
    const { listen } = await import("@tauri-apps/api/event");
    listen("fuentes-cambiadas", () => actualizarFuentes());
  }
}

export async function actualizarFuentes() {
  try {
    const r = await llamar("fuentes_actualizar");
    poner(r.familias);
    avisarErrores(r.familias);
  } catch (e) {
    fuentes.cargando = false;
    avisar("err", t("err.lista"), String(e));
  }
}

function resumen(r, accion) {
  const x = r.resultado;
  if (x.cancelado) {
    avisar("warn", t("aviso.sinPermisos"), t("aviso.sinPermisosTexto"));
  }
  if (x.errores.length) {
    avisar("err", x.errores.length === 1 ? t("err.una") : t("err.varias", { n: x.errores.length }), x.errores[0]);
  }
  return x;
}

/** Activa o desactiva familias. Devuelve el resultado de Rust. */
export async function alternar(nombres, activar, desbloqueadas) {
  fuentes.ocupado = true;
  try {
    const r = await llamar("alternar", { familias: nombres, activar, desbloqueadas });
    poner(r.familias);
    const x = resumen(r);
    if (x.reiniciar) avisar("warn", t("aviso.reiniciar"), t("aviso.reiniciarTexto"));
    return x;
  } catch (e) {
    avisar("err", t("err.una"), String(e));
    return null;
  } finally {
    fuentes.ocupado = false;
  }
}

export async function instalar(rutas, paraTodos) {
  fuentes.ocupado = true;
  try {
    const r = await llamar("instalar", { rutas, paraTodos });
    poner(r.familias);
    return resumen(r);
  } catch (e) {
    avisar("err", t("err.instalar"), String(e));
    return null;
  } finally {
    fuentes.ocupado = false;
  }
}

export async function restaurarSistema() {
  fuentes.ocupado = true;
  try {
    const r = await llamar("restaurar_sistema");
    poner(r.familias);
    return resumen(r);
  } finally {
    fuentes.ocupado = false;
  }
}

/** Para quien instale por su cuenta (Descubrir). */
export const ponerLista = (r) => poner(r.familias);

export const respaldadas = () => llamar("respaldadas");
export const caracteres = (ruta) => llamar("caracteres", { ruta });
export { llamar };

// Fuera de Tauri (vite en el navegador): unos datos de muestra para ver la interfaz.
function demo(cmd) {
  const fam = (nombre, categoria, origen, activa) => ({
    nombre, categoria, origen, activa, imprescindible: ["Arial", "Segoe UI", "Consolas"].includes(nombre),
    estilos: [{ nombre: "Regular", peso: 400, cursiva: false, archivo: nombre, familia_win: nombre }],
    archivo: nombre, archivos: [nombre], error: null,
  });
  const lista = [
    fam("Segoe UI", "sans", "sis", true), fam("Arial", "sans", "sis", true), fam("Georgia", "serif", "sis", true),
    fam("Consolas", "mono", "sis", true), fam("Comic Sans MS", "script", "sis", true), fam("Impact", "display", "sis", true),
    fam("Courier New", "mono", "sis", true), fam("Times New Roman", "serif", "sis", true), fam("Gochi Hand", "script", "usu", true),
    fam("Archivo", "sans", "bib", false),
  ];
  if (cmd === "fuentes_guardadas") return Promise.resolve(lista);
  if (cmd === "fuentes_actualizar") return Promise.resolve({ familias: lista, leidas: 0 });
  if (cmd === "carpetas") return Promise.resolve([]);
  if (cmd === "respaldadas") return Promise.resolve(0);
  if (cmd === "caracteres") return Promise.resolve([...Array(94)].map((_, i) => 33 + i));
  return Promise.reject("solo en la app");
}
