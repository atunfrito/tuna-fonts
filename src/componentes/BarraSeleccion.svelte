<script>
  // Barra flotante de la selección múltiple, abajo y centrada.
  import { CircleCheck, Circle, Heart, FolderPlus, Columns2, X } from "@lucide/svelte";
  import { t } from "../lib/i18n.svelte.js";

  let { n, colecciones, abierta = $bindable(), onactivar, ondesactivar, onfavoritas, oncoleccion, oncomparar, onlimpiar } = $props();
  const comparable = $derived(n >= 2 && n <= 4);
  const titCmp = $derived(n < 2 ? t("cmp.minimo") : n > 4 ? t("cmp.maximo") : t("compare"));
</script>

<div class="barra">
  <span class="cuenta">
    <span class="n num">{n}</span>
    <span>{n === 1 ? t("seleccionada") : t("seleccionadas")}</span>
  </span>
  <span class="sep"></span>
  <button type="button" class="principal" onclick={onactivar}><CircleCheck size={15} strokeWidth={1.9} />{t("activate")}</button>
  <button type="button" onclick={ondesactivar}><Circle size={15} strokeWidth={1.9} />{t("deactivate")}</button>
  <span class="sep"></span>
  <button type="button" onclick={onfavoritas}><Heart size={15} strokeWidth={1.9} />{t("favorites")}</button>
  <span class="pop">
    <button type="button" onclick={() => (abierta = !abierta)} aria-expanded={abierta}><FolderPlus size={15} strokeWidth={1.9} />{t("collection")}</button>
    {#if abierta}
      <span class="menu">
        <span class="tit">{t("addTo")}</span>
        {#each colecciones as c (c.id)}
          <button type="button" class="op" onclick={() => oncoleccion(c.id)}>{c.nombre}</button>
        {/each}
        <button type="button" class="op nueva" onclick={() => oncoleccion(null)}>+ {t("col.nueva")}</button>
      </span>
    {/if}
  </span>
  <button type="button" onclick={oncomparar} disabled={!comparable} title={titCmp} style:opacity={comparable ? 1 : 0.4}><Columns2 size={15} strokeWidth={1.9} />{t("compare")}</button>
  <span class="sep"></span>
  <button type="button" class="limpiar" title={t("clearSel")} aria-label={t("clearSel")} onclick={onlimpiar}><X size={12} strokeWidth={2.2} /></button>
</div>

<style>
  .barra {
    position: absolute;
    left: 50%;
    bottom: 20px;
    /* translate y no transform: la animación tf-up usa transform y, si no, la
       barra salía desplazada a la derecha y saltaba al centro al acabar. */
    translate: -50% 0;
    z-index: 6;
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 5px;
    border-radius: 14px;
    background: var(--win);
    color: var(--ink);
    box-shadow: 0 14px 34px rgba(0, 0, 0, 0.16), 0 0 0 1px var(--line2);
    animation: tf-up 0.22s cubic-bezier(0.2, 0.9, 0.3, 1);
    white-space: nowrap;
  }
  .cuenta { display: flex; align-items: center; gap: 8px; height: 32px; padding: 0 12px 0 8px; border-radius: 9px; background: var(--chip); font: 600 12.5px var(--sans); }
  .n { min-width: 20px; height: 20px; padding: 0 5px; border-radius: 999px; background: var(--red); color: #fff; font: 700 11.5px var(--sans); display: flex; align-items: center; justify-content: center; }
  .sep { width: 1px; height: 20px; background: var(--line2); margin: 0 4px; flex: none; }
  button {
    height: 32px;
    padding: 0 11px 0 9px;
    border: 0;
    border-radius: 9px;
    background: transparent;
    color: var(--ink);
    font: 600 12.5px var(--sans);
    display: flex;
    align-items: center;
    gap: 7px;
    transition: background-color 0.12s;
  }
  button:hover:not(:disabled) { background: var(--hov); }
  button:disabled { cursor: default; }
  .principal, .principal:hover:not(:disabled) { background: var(--ink); color: var(--win); padding: 0 13px 0 10px; }
  .limpiar { width: 32px; padding: 0; justify-content: center; color: var(--ink3); }
  .limpiar:hover { color: var(--ink); }
  .pop { position: relative; display: flex; }
  .menu {
    position: absolute;
    bottom: 42px;
    left: 0;
    min-width: 200px;
    display: flex;
    flex-direction: column;
    padding: 5px;
    border-radius: 12px;
    background: var(--win);
    box-shadow: 0 12px 30px rgba(0, 0, 0, 0.18), 0 0 0 1px var(--line2);
    animation: tf-up 0.15s ease-out;
  }
  .tit { padding: 6px 10px 4px; font: 600 10.5px var(--sans); letter-spacing: 0.08em; color: var(--ink3); }
  .op { height: 30px; padding: 0 10px; border-radius: 7px; font-weight: 500; text-align: left; }
  .nueva { color: var(--ink2); }
</style>
