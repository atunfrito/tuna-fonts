<script>
  // Minimizar, maximizar/restaurar y cerrar: la ventana no tiene bordes de Windows.
  import { enTauri } from "../lib/ajustes.svelte.js";
  import { t } from "../lib/i18n.svelte.js";

  let ventana = null;
  let maximizada = $state(false);

  $effect(() => {
    if (!enTauri) return;
    let quitar;
    import("@tauri-apps/api/window").then(async ({ getCurrentWindow }) => {
      ventana = getCurrentWindow();
      const leer = () => ventana.isMaximized().then((m) => (maximizada = m));
      leer();
      quitar = await ventana.onResized(leer);
    });
    return () => quitar?.();
  });
</script>

<div class="controles">
  <button type="button" title={t("barra.minimizar")} aria-label={t("barra.minimizar")} onclick={() => ventana?.minimize()}>
    <svg width="10" height="10" viewBox="0 0 10 10"><path d="M1 5h8" stroke="currentColor" stroke-width="1.1" /></svg>
  </button>
  <button
    type="button"
    title={maximizada ? t("barra.restaurar") : t("barra.maximizar")}
    aria-label={maximizada ? t("barra.restaurar") : t("barra.maximizar")}
    onclick={() => ventana?.toggleMaximize()}
  >
    {#if maximizada}
      <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.1">
        <rect x="1" y="3" width="6" height="6" rx="1" /><path d="M3 3V2a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v4a1 1 0 0 1-1 1H7" />
      </svg>
    {:else}
      <svg width="10" height="10" viewBox="0 0 10 10"><rect x="1" y="1" width="8" height="8" rx="1" fill="none" stroke="currentColor" stroke-width="1.1" /></svg>
    {/if}
  </button>
  <button type="button" class="cerrar" title={t("close")} aria-label={t("close")} onclick={() => ventana?.close()}>
    <svg width="10" height="10" viewBox="0 0 10 10"><path d="M1 1l8 8M9 1l-8 8" stroke="currentColor" stroke-width="1.1" /></svg>
  </button>
</div>

<style>
  /* Botones redondeados como el resto de la app, centrados en la barra de 52 px. */
  .controles { display: flex; align-items: center; gap: 2px; margin-right: 10px; }
  button {
    width: 34px;
    height: 28px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 0;
    border-radius: 7px;
    background: transparent;
    color: var(--ink3);
    transition: background-color 0.15s, color 0.15s;
  }
  /* Cerrar también en gris: el rojo es solo para el acento (docs/design.md). */
  button:hover { background: var(--chip); color: var(--ink); }
  button:active { background: var(--line2); }
</style>
