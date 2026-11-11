<script>
  // Una fila de la lista: nombre pequeño encima y la fuente debajo.
  import { ChevronRight } from "@lucide/svelte";
  import Interruptor from "./Interruptor.svelte";
  import Corazon from "./Corazon.svelte";
  import { t } from "../lib/i18n.svelte.js";

  let { f, estilo, muestra, tamano, sel, fav, bloqueada, motivo, onsel, onabrir, onalternar, onfav, onarrastre } = $props();

  const on = $derived(f.activa || bloqueada);
  const mb = $derived(Math.round(tamano * 0.32) + "px");
  const mbBtn = $derived(Math.max(0, Math.round(tamano * 0.32) - 7) + "px");
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="fila" class:sel onclick={onsel} ondblclick={onabrir} onpointerdown={onarrastre} data-familia={f.nombre}>
  <Interruptor activa={f.activa} {bloqueada} titulo={bloqueada ? motivo : f.activa ? t("deactivate") : t("activate")} onclick={onalternar} abajo={mb} />
  <div class="txt">
    <span class="nombre">{f.nombre}{#if f.error}<span class="roto"> · {t("err.ilegible")}</span>{/if}</span>
    <span
      class="muestra"
      class:off={!on}
      style:font-family={estilo.familia}
      style:font-weight={estilo.peso}
      style:font-style={estilo.cursiva ? "italic" : "normal"}
      style:font-size="{tamano}px">{muestra}</span
    >
  </div>
  <button
    type="button"
    class="ico corazon"
    class:fav
    style:margin-bottom={mbBtn}
    title={fav ? t("fav.quitar") : t("fav.poner")}
    aria-label={fav ? t("fav.quitar") : t("fav.poner")}
    onclick={(e) => (e.stopPropagation(), onfav())}
    ondblclick={(e) => e.stopPropagation()}
  >
    <Corazon lleno={fav} />
  </button>
  <button
    type="button"
    class="ico abrir"
    style:margin-bottom={mbBtn}
    title={t("specimen")}
    aria-label={t("specimen")}
    onclick={(e) => (e.stopPropagation(), onabrir())}
    ondblclick={(e) => e.stopPropagation()}
  >
    <ChevronRight size={17} strokeWidth={2} />
  </button>
</div>

<style>
  .fila {
    display: grid;
    grid-template-columns: 24px minmax(0, 1fr) 34px 34px;
    align-items: end;
    gap: 0 12px;
    padding: 8px 18px 8px 24px;
    transition: background-color 0.12s;
  }
  .fila:hover { background: var(--hov); }
  .fila.sel, .fila.sel:hover { background: var(--sel); }
  .txt { display: flex; flex-direction: column; gap: 2px; min-width: 0; }
  .nombre { font-size: 10.5px; color: var(--ink3); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .roto { color: var(--red); }
  .muestra {
    line-height: 1.38;
    padding-bottom: 0.18em;
    margin-bottom: -0.18em;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--ink);
    transition: color 0.25s;
  }
  .muestra.off { color: var(--off); text-shadow: 0 1px 0 rgba(255, 255, 255, 0.9); }
  :global([data-theme="dark"]) .muestra.off { text-shadow: 0 1px 0 rgba(0, 0, 0, 0.6); }
  .ico {
    width: 32px;
    height: 32px;
    border: 0;
    border-radius: 8px;
    background: transparent;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--ink3);
    opacity: 0.6;
    transition: background-color 0.15s, opacity 0.15s, color 0.15s;
  }
  .ico:hover { opacity: 1; background: var(--hov); }
  .abrir { opacity: 0.7; }
  .abrir:hover { color: var(--ink); }
  .corazon.fav { color: var(--red); opacity: 1; }
</style>
