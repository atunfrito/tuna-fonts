// t("clave", { var }) en cualquier componente. Lee ajustes.idioma, así que lo
// que lo use se repinta solo al cambiar de idioma.
import { ajustes } from "./ajustes.svelte.js";
import es from "../i18n/es.js";
import en from "../i18n/en.js";

const TEXTOS = { es, en };

export const idioma = () => (ajustes.idioma === "en" ? "en" : "es");

export function t(clave, vars) {
  let texto = TEXTOS[idioma()][clave] ?? es[clave] ?? clave;
  if (vars) for (const [k, v] of Object.entries(vars)) texto = texto.replaceAll(`{${k}}`, v);
  return texto;
}

export const locale = () => (idioma() === "es" ? "es-ES" : "en-US");

$effect.root(() => {
  $effect(() => {
    document.documentElement.lang = idioma();
  });
});
