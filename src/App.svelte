<script>
  import Lateral from "./componentes/Lateral.svelte";
  import Controles from "./componentes/Controles.svelte";
  import Avisos from "./componentes/Avisos.svelte";
  import Ajustes from "./vistas/Ajustes.svelte";
  import { t } from "./lib/i18n.svelte.js";

  let seccion = $state("instaladas");
  let cat = $state(null);
  // null = cerrado; si no, la pestaña abierta.
  let ajustesEn = $state(null);

  // Splash: lo justo para la animación del icono. Sin movimiento, ni se ve.
  const sinMovimiento = matchMedia("(prefers-reduced-motion: reduce)").matches;
  let splash = $state(!sinMovimiento);
  if (!sinMovimiento) setTimeout(() => (splash = false), 800);

  const cabecera = $derived(cat ? t("cat." + cat) : t("sec." + seccion));
</script>

<div class="ventana">
  <Lateral
    {seccion}
    {cat}
    onseccion={(s) => ((seccion = s), (cat = null))}
    oncat={(c) => (cat = c)}
    onajustes={() => (ajustesEn = "ajustes")}
    onacerca={() => (ajustesEn = "acerca")}
  />

  <main>
    <header data-tauri-drag-region>
      <div class="hueco" data-tauri-drag-region></div>
      <Controles />
    </header>

    <div class="titulo">
      <span class="etiqueta">{cabecera}</span>
    </div>
  </main>

  {#if ajustesEn}
    <Ajustes bind:pestana={ajustesEn} oncerrar={() => (ajustesEn = null)} />
  {/if}

  <Avisos />

  {#if splash}
    <div class="splash">
      <img src="/icon.svg" width="140" height="140" alt="" draggable="false" />
      <span class="marca">tuna-fonts<i>.</i></span>
    </div>
  {/if}
</div>

<style>
  .ventana {
    position: relative;
    height: 100vh;
    overflow: hidden;
    display: grid;
    grid-template-columns: 216px minmax(0, 1fr);
  }
  main { position: relative; display: flex; flex-direction: column; min-width: 0; min-height: 0; }
  header {
    flex: none;
    height: 52px;
    display: flex;
    align-items: center;
    gap: 10px;
    padding-left: 24px;
  }
  .hueco { flex: 1; align-self: stretch; }
  .titulo { flex: none; display: flex; align-items: baseline; gap: 8px; padding: 10px 24px 8px; }

  .splash {
    position: absolute;
    inset: 0;
    z-index: 40;
    background: var(--win);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 22px;
  }
  .splash img {
    display: block;
    animation: tf-key 0.55s cubic-bezier(0.3, 1.2, 0.5, 1) both;
    filter: drop-shadow(0 18px 30px rgba(0, 0, 0, 0.25));
  }
  .splash .marca { font-size: 34px; animation: tf-fade 0.3s ease-out 0.45s both; }
</style>
