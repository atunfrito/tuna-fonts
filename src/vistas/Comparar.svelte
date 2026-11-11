<script>
  // De 2 a 4 fuentes, una debajo de otra, con la misma muestra y un párrafo.
  import { ChevronLeft, X } from "@lucide/svelte";
  import { estiloDe } from "../lib/fuentes.svelte.js";
  import { t } from "../lib/i18n.svelte.js";

  let { familias, muestra, tamano, onvolver, onquitar } = $props();
</script>

<div class="cmp">
  <button type="button" class="volver" onclick={onvolver}><ChevronLeft size={13} strokeWidth={2.2} />{t("back")}</button>
  {#each familias as f (f.nombre)}
    {@const e = estiloDe(f)}
    <div class="una">
      <div class="cab">
        <span class="nombre">{f.nombre}</span>
        <span class="meta">{f.estilos.length} {f.estilos.length === 1 ? t("estilo") : t("estilos")} · {t("origen." + f.origen)}</span>
        <button type="button" class="quitar" title={t("removeCmp")} aria-label={t("removeCmp")} onclick={() => onquitar(f.nombre)}><X size={12} strokeWidth={2} /></button>
      </div>
      <span class="linea" style:font-family={e.familia} style:font-weight={e.peso} style:font-style={e.cursiva ? "italic" : "normal"} style:font-size="{Math.max(tamano, 44)}px">{muestra}</span>
      <span class="parrafo" style:font-family={e.familia} style:font-weight={e.peso} style:font-style={e.cursiva ? "italic" : "normal"}>{t("paragraph")}</span>
    </div>
  {/each}
</div>

<style>
  .cmp { flex: 1; min-height: 0; overflow-y: auto; padding: 0 24px 60px; display: flex; flex-direction: column; gap: 4px; animation: tf-up 0.25s ease-out; }
  .volver {
    align-self: flex-start;
    margin-bottom: 14px;
    height: 28px;
    padding: 0 12px 0 8px;
    border-radius: 999px;
    border: 0;
    background: var(--chip);
    color: var(--ink);
    font: 600 12.5px var(--sans);
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .una { display: flex; flex-direction: column; gap: 4px; padding: 14px 0 18px; border-top: 1px solid var(--line); }
  .cab { display: flex; align-items: center; gap: 10px; }
  .nombre { font-size: 12px; font-weight: 600; }
  .meta { font-size: 11.5px; color: var(--ink3); }
  .quitar {
    margin-left: auto;
    width: 24px;
    height: 24px;
    border: 0;
    border-radius: 50%;
    background: transparent;
    color: var(--ink3);
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .quitar:hover { background: var(--hov); color: var(--ink); }
  .linea { line-height: 1.38; padding-bottom: 0.18em; margin-bottom: -0.18em; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .parrafo { font-size: 15px; line-height: 1.55; color: var(--ink2); max-width: 70ch; }
</style>
