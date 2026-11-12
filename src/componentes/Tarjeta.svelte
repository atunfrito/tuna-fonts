<script>
  // Una tarjeta de la cuadrícula: "Aa" grande y el nombre debajo.
  import Interruptor from "./Interruptor.svelte";
  import Corazon from "./Corazon.svelte";
  import { t } from "../lib/i18n.svelte.js";
  import { encajar } from "../lib/encajar.js";

  let { f, estilo, sel, fav, bloqueada, motivo, onsel, onabrir, onalternar, onfav, onarrastre } = $props();
  const on = $derived(f.activa || bloqueada);
  const aa = $derived(f.categoria === "otros" ? "あ" : "Aa");
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="tarjeta" class:sel onclick={onsel} ondblclick={onabrir} onpointerdown={onarrastre} data-familia={f.nombre}>
  <div
    class="aa"
    class:off={!on}
    style:font-family={estilo.familia}
    style:font-weight={estilo.peso}
    style:font-style={estilo.cursiva ? "italic" : "normal"}
  >
    <span use:encajar={{ texto: aa, familia: estilo.familia, peso: estilo.peso, cursiva: estilo.cursiva, base: 64, margen: 12 }}>{aa}</span>
  </div>
  <div class="pie">
    <Interruptor activa={f.activa} {bloqueada} titulo={bloqueada ? motivo : f.activa ? t("deactivate") : t("activate")} onclick={onalternar} tam={16} />
    <span class="nombre">{f.nombre}</span>
    <button
      type="button"
      class="corazon"
      class:fav
      title={fav ? t("fav.quitar") : t("fav.poner")}
      aria-label={fav ? t("fav.quitar") : t("fav.poner")}
      onclick={(e) => (e.stopPropagation(), onfav())}
      ondblclick={(e) => e.stopPropagation()}
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
    box-shadow: inset 0 0 0 1px var(--line);
    transition: background-color 0.12s, box-shadow 0.12s;
  }
  .tarjeta:hover { background: var(--hov); }
  .tarjeta.sel, .tarjeta.sel:hover { background: var(--sel); box-shadow: inset 0 0 0 1.5px var(--red); }
  .aa {
    height: 92px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 64px;
    line-height: 1;
    color: var(--ink);
    overflow: hidden;
    white-space: nowrap;
  }
  .aa span { display: block; line-height: 1; }
  .aa.off { color: var(--off); }
  .pie { display: flex; align-items: center; gap: 8px; }
  .nombre { flex: 1; min-width: 0; font-size: 12px; font-weight: 500; color: var(--ink2); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
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
