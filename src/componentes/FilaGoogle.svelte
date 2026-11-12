<script>
  // Una familia de Google Fonts: en Descubrir y, si no está instalada, en favoritas.
  import { Check } from "@lucide/svelte";
  import Corazon from "./Corazon.svelte";
  import { t } from "../lib/i18n.svelte.js";

  let { g, muestra, tamano, instalada, pct, fav, onfav, oninstalar, sinInstalar = false } = $props();
</script>

<div class="fila">
  <div class="txt">
    <span class="nombre"
      >{g.nombre} · {g.estilos} {g.estilos === 1 ? t("estilo") : t("estilos")}{#if sinInstalar}<span class="falta">{" · " + t("g.noInstalada")}</span>{/if}</span
    >
    <span class="muestra" style:font-family={`"${g.nombre}", var(--sans)`} style:font-size="{tamano}px">{muestra}</span>
  </div>
  <button
    type="button"
    class="ico corazon"
    class:fav
    title={fav ? t("fav.quitar") : t("fav.deseo")}
    aria-label={fav ? t("fav.quitar") : t("fav.deseo")}
    onclick={onfav}
  >
    <Corazon lleno={fav} />
  </button>
  <div class="accion">
    {#if pct != null}
      <span class="pct num">{Math.round(pct)} %</span>
    {:else if instalada}
      <span class="hecha"><Check size={12} strokeWidth={2} />{t("g.instalada")}</span>
    {:else}
      <button type="button" class="instalar" onclick={oninstalar}>{t("g.instalar")}</button>
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
    grid-template-columns: minmax(0, 1fr) 34px 110px;
    /* El corazón y el botón, centrados con toda la fila. */
    align-items: center;
    gap: 0 12px;
    padding: 8px 24px 10px 60px;
    transition: background-color 0.12s;
  }
  .fila:hover { background: var(--hov); }
  .txt { display: flex; flex-direction: column; gap: 2px; min-width: 0; }
  .nombre { font-size: 10.5px; color: var(--ink3); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .muestra { line-height: 1.38; padding-bottom: 0.18em; margin-bottom: -0.18em; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .falta { color: var(--ink2); font-weight: 600; }
  .accion { justify-self: end; }
  .ico {
    width: 32px; height: 32px; border: 0; border-radius: 8px; background: transparent;
    display: flex; align-items: center; justify-content: center;
    color: var(--ink3); opacity: 0.6; transition: background-color 0.15s, opacity 0.15s, color 0.15s;
  }
  .ico:hover { opacity: 1; background: var(--hov); }
  .corazon.fav { color: var(--red); opacity: 1; }
  .instalar {
    height: 26px;
    padding: 0 12px;
    border-radius: 999px;
    border: 0;
    background: var(--chip);
    color: var(--ink);
    font: 600 12px var(--sans);
    transition: background-color 0.15s, color 0.15s;
  }
  .instalar:hover { background: var(--ink); color: var(--win); }
  .pct { font-size: 12px; font-weight: 600; color: var(--ink2); }
  .hecha { display: flex; align-items: center; gap: 5px; font-size: 12px; font-weight: 600; color: var(--ink3); }
  .barra { position: absolute; left: 60px; right: 24px; bottom: 0; height: 2px; background: var(--line); }
  .barra span { display: block; height: 100%; background: var(--red); transition: width 0.3s linear; }
</style>
