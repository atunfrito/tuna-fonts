<script>
  import { Check } from "@lucide/svelte";
  import { t } from "../lib/i18n.svelte.js";

  let { g, muestra, tamano, instalada, pct, oninstalar } = $props();
</script>

<div class="fila">
  <div class="txt">
    <span class="nombre">{g.nombre} · {g.estilos} {g.estilos === 1 ? t("estilo") : t("estilos")}</span>
    <span class="muestra" style:font-family={`"${g.nombre}", var(--sans)`} style:font-size="{tamano}px">{muestra}</span>
  </div>
  <div class="accion">
    {#if pct != null}
      <span class="pct num">{Math.round(pct)} %</span>
    {:else if instalada}
      <span class="hecha"><Check size={12} strokeWidth={2} />{t("g.instalada")}</span>
    {:else}
      <button type="button" onclick={oninstalar}>{t("g.instalar")}</button>
    {/if}
  </div>
  {#if pct != null}
    <span class="barra"><span style:width="{pct}%"></span></span>
  {/if}
</div>

<style>
  .fila {
    position: relative;
    display: grid;
    grid-template-columns: minmax(0, 1fr) 110px;
    align-items: end;
    gap: 0 16px;
    padding: 8px 24px 10px 60px;
    transition: background-color 0.12s;
  }
  .fila:hover { background: var(--hov); }
  .txt { display: flex; flex-direction: column; gap: 2px; min-width: 0; }
  .nombre { font-size: 10.5px; color: var(--ink3); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .muestra { line-height: 1.38; padding-bottom: 0.18em; margin-bottom: -0.18em; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .accion { justify-self: end; margin-bottom: 4px; }
  button {
    height: 26px;
    padding: 0 12px;
    border-radius: 999px;
    border: 0;
    background: var(--chip);
    color: var(--ink);
    font: 600 12px var(--sans);
    transition: background-color 0.15s, color 0.15s;
  }
  button:hover { background: var(--ink); color: var(--win); }
  .pct { font-size: 12px; font-weight: 600; color: var(--ink2); }
  .hecha { display: flex; align-items: center; gap: 5px; font-size: 12px; font-weight: 600; color: var(--ink3); }
  .barra { position: absolute; left: 60px; right: 24px; bottom: 0; height: 2px; background: var(--line); }
  .barra span { display: block; height: 100%; background: var(--red); transition: width 0.3s linear; }
</style>
