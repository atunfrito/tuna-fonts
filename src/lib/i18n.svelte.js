// t("clave", { var }) en cualquier componente. Lee ajustes.idioma, así que lo
// que lo use se repinta solo al cambiar de idioma.
import { ajustes } from "./ajustes.svelte.js";
import es from "../i18n/es.js";
import en from "../i18n/en.js";
import zh from "../i18n/zh.js";

const TEXTOS = { es, en, zh };

// Cualquier valor que no sea una traducción conocida cae al español, que es el
// idioma original del proyecto.
export const idioma = () => (ajustes.idioma in TEXTOS ? ajustes.idioma : "es");

export function t(clave, vars) {
  let texto = TEXTOS[idioma()][clave] ?? es[clave] ?? clave;
  if (vars) for (const [k, v] of Object.entries(vars)) texto = texto.replaceAll(`{${k}}`, v);
  return texto;
}

const LOCALES = { es: "es-ES", en: "en-US", zh: "zh-CN" };

export const locale = () => LOCALES[idioma()];

$effect.root(() => {
  $effect(() => {
    document.documentElement.lang = idioma();
  });
});
