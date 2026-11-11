// Favoritas y colecciones, guardadas con tauri-plugin-store en biblioteca.json.
import { LazyStore } from "@tauri-apps/plugin-store";
import { enTauri } from "./ajustes.svelte.js";

export const biblioteca = $state({
  favoritas: [],
  // [{ id, nombre, fuentes: [familia] }]
  colecciones: [],
  listo: false,
});

const store = enTauri ? new LazyStore("biblioteca.json", { autoSave: 150 }) : null;

export async function iniciarBiblioteca() {
  if (store) {
    biblioteca.favoritas = (await store.get("favoritas")) ?? [];
    biblioteca.colecciones = (await store.get("colecciones")) ?? [];
  }
  biblioteca.listo = true;
}

$effect.root(() => {
  $effect(() => {
    const f = $state.snapshot(biblioteca.favoritas);
    if (biblioteca.listo) store?.set("favoritas", f);
  });
  $effect(() => {
    const c = $state.snapshot(biblioteca.colecciones);
    if (biblioteca.listo) store?.set("colecciones", c);
  });
});

export const esFavorita = (n) => biblioteca.favoritas.includes(n);

export function alternarFavorita(n) {
  const i = biblioteca.favoritas.indexOf(n);
  if (i >= 0) biblioteca.favoritas.splice(i, 1);
  else biblioteca.favoritas.push(n);
}

export function aFavoritas(nombres) {
  for (const n of nombres) if (!biblioteca.favoritas.includes(n)) biblioteca.favoritas.push(n);
}

export function nuevaColeccion(nombre) {
  const id = "c" + Date.now().toString(36);
  biblioteca.colecciones.push({ id, nombre, fuentes: [] });
  return id;
}

export function renombrar(id, nombre) {
  const c = biblioteca.colecciones.find((c) => c.id === id);
  if (c && nombre.trim()) c.nombre = nombre.trim();
}

export function eliminarColeccion(id) {
  const i = biblioteca.colecciones.findIndex((c) => c.id === id);
  if (i >= 0) biblioteca.colecciones.splice(i, 1);
}

/** Añade fuentes a una colección; devuelve cuántas eran nuevas. */
export function aColeccion(id, nombres) {
  const c = biblioteca.colecciones.find((c) => c.id === id);
  if (!c) return 0;
  let n = 0;
  for (const x of nombres) {
    if (!c.fuentes.includes(x)) {
      c.fuentes.push(x);
      n++;
    }
  }
  return n;
}
