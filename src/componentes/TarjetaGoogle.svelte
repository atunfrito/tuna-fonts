<script>
  // Tarjeta de una favorita de Google Fonts que aún no está instalada.
  import Corazon from "./Corazon.svelte";
  import { t } from "../lib/i18n.svelte.js";
  import { encajar } from "../lib/encajar.js";

  let { g, pct, fav, onfav, oninstalar } = $props();
  const familia = $derived(`"${g.nombre}", var(--sans)`);
  const aa = $derived(g.categoria === "otros" ? "あ" : "Aa");
</script>

<div class="tarjeta">
  <div class="aa" style:font-family={familia}>
    <span use:encajar={{ texto: aa, familia, base: 64, margen: 12 }}>{aa}</span>
  </div>
  <div class="pie">
    {#if pct != null}
      <span class="pct num">{Math.round(pct)} %</span>
    {:else}
      <button type="button" class="instalar" onclick={oninstalar} title={t("g.noInstalada")}>{t("g.instalar")}</button>
    {/if}
    <span class="nombre">{g.nombre}</span>
    <button
      type="button"
      class="corazon"
      class:fav
      title={fav ? t("fav.quitar") : t("fav.deseo")}
      aria-label={fav ? t("fav.quitar") : t("fav.deseo")}
      onclick={onfav}
    >
      <Corazon lleno={fav} tam={16} />
    </button>
  </div>
</div>

<style>
  .tarjeta {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
    border-radius: 8px;
    /* Discontinua: es una favorita que aún no está en el equipo. */
    border: 1px dashed var(--line2);
    transition: background-color 0.12s;
  }
  .tarjeta:hover { background-color: var(--hov); }
  .aa {
    height: 92px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 64px;
    line-height: 1;
    color: var(--ink2);
    overflow: hidden;
    white-space: nowrap;
  }
  .aa span { display: block; line-height: 1; }
  .pie { display: flex; align-items: center; gap: 8px; }
  .nombre { flex: 1; min-width: 0; font-size: 12px; font-weight: 500; color: var(--ink2); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .instalar {
    flex: none;
    height: 20px;
    padding: 0 8px;
    border-radius: 999px;
    border: 0;
    background: var(--chip);
    color: var(--ink);
    font: 600 11px var(--sans);
    transition: background-color 0.15s, color 0.15s;
  }
  .instalar:hover { background: var(--ink); color: var(--win); }
  .pct { flex: none; font-size: 11px; font-weight: 600; color: var(--ink2); }
  .corazon {
    width: 22px;
    height: 22px;
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--ink3);
    opacity: 0.6;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .corazon:hover { opacity: 1; }
  .corazon.fav { color: var(--red); opacity: 1; }
</style>
