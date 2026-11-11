<script>
  // Modal de ajustes con tres pestañas: ajustes · atajos · acerca de.
  // Fase 1: tema e idioma. El resto de opciones llega en su fase.
  import { SlidersHorizontal, Keyboard, Info, X } from "@lucide/svelte";
  import Segmentado from "../componentes/Segmentado.svelte";
  import { ajustes, temaActual, ponerTema, version } from "../lib/ajustes.svelte.js";
  import { t } from "../lib/i18n.svelte.js";

  let { pestana = $bindable(), oncerrar } = $props();

  const PESTANAS = [
    { id: "ajustes", texto: () => t("settings"), icono: SlidersHorizontal },
    { id: "atajos", texto: () => t("shortcuts"), icono: Keyboard },
    { id: "acerca", texto: () => t("about"), icono: Info },
  ];

  const filas = $derived([
    {
      etiqueta: t("aj.tema"),
      pista: t("aj.temaHint"),
      valor: temaActual(),
      elegir: ponerTema,
      opciones: [
        { valor: "light", etiqueta: t("aj.claro") },
        { valor: "dark", etiqueta: t("aj.oscuro") },
      ],
    },
    {
      etiqueta: t("aj.idioma"),
      pista: "",
      valor: ajustes.idioma,
      elegir: (v) => (ajustes.idioma = v),
      opciones: [
        { valor: "es", etiqueta: "español" },
        { valor: "en", etiqueta: "english" },
      ],
    },
  ]);

  // Por defecto; en la fase 6 se podrán cambiar desde aquí.
  const atajos = $derived([
    { teclas: ["Ctrl", "F"], que: t("atajos.buscar") },
    { teclas: ["Ctrl", "A"], que: t("atajos.todas") },
    { teclas: [t("atajos.espacio")], que: t("atajos.alternar") },
    { teclas: [t("atajos.dobleClic")], que: t("atajos.doble"), fijo: true },
    { teclas: ["Esc"], que: t("atajos.esc"), fijo: true },
  ]);

  function tecla(e) {
    if (e.key === "Escape") {
      e.preventDefault();
      oncerrar();
    }
  }
</script>

<svelte:window onkeydown={tecla} />

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="velo" onclick={oncerrar}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="modal" role="dialog" aria-modal="true" aria-label={t("settings")} tabindex="-1" onclick={(e) => e.stopPropagation()}>
    <div class="lado">
      <span class="marca">tuna-fonts<i>.</i></span>
      {#each PESTANAS as p (p.id)}
        <button type="button" class="tab" class:activa={pestana === p.id} onclick={() => (pestana = p.id)}>
          <p.icono size={14} strokeWidth={1.9} />{p.texto()}
        </button>
      {/each}
    </div>

    <div class="cuerpo">
      <button type="button" class="cerrar" title={t("close")} aria-label={t("close")} onclick={oncerrar}>
        <X size={13} strokeWidth={2} />
      </button>

      {#if pestana === "ajustes"}
        <h2>{t("settings")}</h2>
        {#each filas as f (f.etiqueta)}
          <div class="fila">
            <span class="txt">
              <span class="eti">{f.etiqueta}</span>
              {#if f.pista}<span class="pista">{f.pista}</span>{/if}
            </span>
            <Segmentado etiqueta={f.etiqueta} opciones={f.opciones} valor={f.valor} onelegir={f.elegir} />
          </div>
        {/each}
      {:else if pestana === "atajos"}
        <h2>{t("shortcuts")}</h2>
        <div class="atajos">
          {#each atajos as a (a.que)}
            <span class="teclas" title={a.fijo ? t("atajos.fijo") : undefined}>
              {#each a.teclas as k (k)}<kbd>{k}</kbd>{/each}
            </span>
            <span class="que">{a.que}</span>
          {/each}
        </div>
      {:else}
        <div class="acerca">
          <img src="/icon.svg" width="64" height="64" alt="" draggable="false" />
          <div class="nombre">
            <span class="marca grande">tuna-fonts<i>.</i></span>
            <span class="ver">{t("version", { v: version })}</span>
          </div>
        </div>
        <p class="tagline">{t("tagline")}</p>
        <div class="firma">
          <span class="marca cueva">{t("madeIn")}</span>
          <span class="copy">© 2026 atunfrito · MIT</span>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .velo {
    position: absolute;
    inset: 0;
    z-index: 15;
    background: var(--velo);
    display: flex;
    align-items: center;
    justify-content: center;
    animation: tf-fade 0.15s ease-out;
  }
  .modal {
    width: 680px;
    height: 600px;
    max-height: calc(100% - 40px);
    display: grid;
    grid-template-columns: 170px minmax(0, 1fr);
    border-radius: 14px;
    overflow: hidden;
    background: var(--win);
    box-shadow: 0 30px 70px rgba(0, 0, 0, 0.3), 0 0 0 1px var(--line2);
    animation: tf-up 0.22s ease-out;
    outline: none;
  }
  .lado {
    background: var(--side);
    border-right: 1px solid var(--line);
    padding: 16px 10px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .lado .marca { padding: 0 10px 10px; font-size: 20px; }
  .tab {
    display: flex;
    align-items: center;
    gap: 9px;
    height: 30px;
    padding: 0 10px;
    border: 0;
    border-radius: 6px;
    background: transparent;
    color: var(--ink2);
    font: 500 13px var(--sans);
    text-align: left;
    transition: background-color 0.15s, color 0.15s;
  }
  .tab :global(svg) { opacity: 0.7; }
  .tab:hover { background: var(--hov); }
  .tab.activa { background: var(--chip); color: var(--ink); font-weight: 600; }
  .tab.activa :global(svg) { opacity: 1; }

  .cuerpo {
    position: relative;
    padding: 22px 26px 18px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }
  .cerrar {
    position: absolute;
    top: 12px;
    right: 12px;
    width: 28px;
    height: 28px;
    border: 0;
    border-radius: 50%;
    background: var(--chip);
    color: var(--ink2);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: color 0.15s;
  }
  .cerrar:hover { color: var(--ink); }
  h2 { margin: 0; font-size: 18px; font-weight: 700; letter-spacing: -0.02em; }

  .fila {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding-bottom: 14px;
    border-bottom: 1px solid var(--line);
  }
  .txt { display: flex; flex-direction: column; gap: 3px; }
  .eti { font-size: 13.5px; font-weight: 600; }
  .pista { font-size: 12px; color: var(--ink2); }

  .atajos {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 10px 14px;
    font-size: 13px;
    color: var(--ink2);
  }
  .teclas { justify-self: start; display: flex; gap: 3px; }
  .que { align-self: center; }
  kbd {
    min-width: 20px;
    height: 20px;
    padding: 0 5px;
    border-radius: 5px;
    background: var(--chip);
    box-shadow: inset 0 -1px 0 var(--line2);
    font: 600 11px var(--sans);
    color: var(--ink);
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .acerca { display: flex; align-items: center; gap: 16px; }
  .acerca img { display: block; }
  .nombre { display: flex; flex-direction: column; gap: 4px; }
  .grande { font-size: 32px; }
  .ver { font: 500 12px var(--mono); color: var(--ink2); }
  .tagline { margin: 0; font-size: 14px; line-height: 1.5; color: var(--ink2); }
  .firma { margin-top: auto; display: flex; align-items: flex-end; justify-content: space-between; }
  .cueva { font-size: 18px; color: var(--red); }
  .copy { font: 400 10.5px var(--mono); color: var(--ink3); }
</style>
