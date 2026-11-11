import { mount } from "svelte";
// Las fuentes de la interfaz van dentro de la app: nada se pide a Google al arrancar.
import "@fontsource/archivo/latin-400.css";
import "@fontsource/archivo/latin-500.css";
import "@fontsource/archivo/latin-600.css";
import "@fontsource/archivo/latin-700.css";
import "@fontsource/jetbrains-mono/latin-400.css";
import "@fontsource/jetbrains-mono/latin-500.css";
import "@fontsource/gochi-hand/latin-400.css";
import "./app.css";
import App from "./App.svelte";

// Que parezca una app y no una web: sin menú contextual del navegador (salvo
// en los campos de texto, para poder pegar) ni atajos de recarga o impresión.
document.addEventListener("contextmenu", (e) => {
  if (!e.target.closest("input, textarea")) e.preventDefault();
});
if (import.meta.env.PROD) {
  document.addEventListener("keydown", (e) => {
    const k = e.key.toLowerCase();
    if (k === "f5" || k === "f7" || (e.ctrlKey && ["r", "p", "g", "u", "j"].includes(k))) e.preventDefault();
  });
}

mount(App, { target: document.getElementById("app") });
