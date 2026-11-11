<script>
  // Modal de ajustes con tres pestañas: ajustes · atajos · acerca de.
  import { SlidersHorizontal, Keyboard, Info, X } from "@lucide/svelte";
  import Segmentado from "../componentes/Segmentado.svelte";
  import { ajustes, temaActual, ponerTema, version, ATAJOS } from "../lib/ajustes.svelte.js";
  import { t } from "../lib/i18n.svelte.js";
  import { avisar } from "../lib/avisos.svelte.js";
  import { respaldadas, restaurarSistema, fuentes } from "../lib/fuentes.svelte.js";
  import { comboDe } from "../lib/atajos.js";

  let { pestana = $bindable(), oncerrar, cuentas } = $props();

  const PESTANAS = [
    { id: "ajustes", texto: () => t("settings"), icono: SlidersHorizontal },
    { id: "atajos", texto: () => t("shortcuts"), icono: Keyboard },
    { id: "acerca", texto: () => t("about"), icono: Info },
  ];

  let nRespaldo = $state(0);
  respaldadas().then((n) => (nRespaldo = n)).catch(() => {});

  function desbloquear(v) {
    ajustes.desbloqueadas = v;
    if (v) avisar("warn", t("aviso.desbloqueadas"), t("aviso.desbloqueadasTexto"));
    else avisar("ok", t("ok.bloqueadas"), t("ok.bloqueadasTexto"));
  }

  async function restaurar() {
    const r = await restaurarSistema();
    nRespaldo = await respaldadas().catch(() => 0);
    if (r && r.cambiadas) avisar("ok", t("ok.restauradas"), r.reiniciar ? t("aviso.reiniciarTexto") : "");
  }

  const filas = $derived([
    { etiqueta: t("aj.tema"), pista: t("aj.temaHint"), valor: temaActual(), elegir: ponerTema, opciones: [["light", t("aj.claro")], ["dark", t("aj.oscuro")]] },
    { etiqueta: t("aj.instalar"), pista: t("aj.instalarHint"), valor: ajustes.instalarPara, elegir: (v) => (ajustes.instalarPara = v), opciones: [["usuario", t("aj.soloYo")], ["todos", t("aj.todos")]] },
    { etiqueta: t("aj.abrir"), pista: t("aj.abrirHint"), valor: ajustes.alAbrir, elegir: (v) => (ajustes.alAbrir = v), opciones: [["instaladas", t("sec.instaladas")], ["favoritas", t("sec.favoritas")]] },
    { etiqueta: t("aj.sistema"), pista: t("aj.sistemaHint"), valor: ajustes.desbloqueadas, elegir: desbloquear, opciones: [[false, t("aj.bloqueadas")], [true, t("aj.desbloquear")]] },
    { etiqueta: t("aj.idioma"), pista: "", valor: ajustes.idioma, elegir: (v) => (ajustes.idioma = v), opciones: [["es", "español"], ["en", "english"]] },
  ]);

  // ---- atajos ----
  let grabando = $state(null);
  const bonita = (k) => (k === "Space" ? t("atajos.espacio") : k);
  const atajos = $derived([
    { id: "buscar", que: t("atajos.buscar") },
    { id: "todas", que: t("atajos.todas") },
    { id: "alternar", que: t("atajos.alternar") },
    { fijo: t("atajos.dobleClic"), que: t("atajos.doble") },
    { fijo: "Esc", que: t("atajos.esc") },
  ]);
  const cambiados = $derived(JSON.stringify(ajustes.atajos) !== JSON.stringify(ATAJOS));

  function tecla(e) {
    if (grabando) {
      e.preventDefault();
      e.stopPropagation();
      if (e.key === "Escape") return (grabando = null);
      const c = comboDe(e);
      if (!c) return;
      const choca = Object.keys(ajustes.atajos).find((k) => k !== grabando && ajustes.atajos[k] === c);
      const antes = ajustes.atajos[grabando];
      ajustes.atajos = { ...ajustes.atajos, [grabando]: c, ...(choca ? { [choca]: antes } : {}) };
      grabando = null;
      avisar("ok", t("ok.atajo", { c: c.split("+").map(bonita).join("+") }), choca ? t("ok.atajoCambio") : "");
      return;
    }
    if (e.key === "Escape") {
      e.preventDefault();
      oncerrar();
    }
  }

  function resetear() {
    ajustes.atajos = { ...ATAJOS };
    grabando = null;
    avisar("ok", t("ok.atajosRestaurados"), "");
  }

  const novedades = $derived([t("nov.1"), t("nov.2"), t("nov.3"), t("nov.4")]);
</script>

<svelte:window onkeydowncapture={tecla} />

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="velo" onclick={oncerrar}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="modal" role="dialog" aria-modal="true" aria-label={t("settings")} tabindex="-1" onclick={(e) => e.stopPropagation()}>
    <div class="lado">
      <span class="marca">tuna-fonts<i>.</i></span>
      {#each PESTANAS as p (p.id)}
        <button type="button" class="tab" class:activa={pestana === p.id} onclick={() => ((pestana = p.id), (grabando = null))}>
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
            <Segmentado etiqueta={f.etiqueta} opciones={f.opciones.map(([valor, etiqueta]) => ({ valor, etiqueta }))} valor={f.valor} onelegir={f.elegir} />
          </div>
        {/each}
        {#if nRespaldo > 0}
          <div class="fila">
            <span class="txt">
              <span class="eti">{t("aj.restaurar")}</span>
              <span class="pista">{t("aj.restaurarHint", { n: nRespaldo })}</span>
            </span>
            <button type="button" class="boton" disabled={fuentes.ocupado} onclick={restaurar}>{t("aj.restaurar")}</button>
          </div>
        {/if}
        <div class="prueba">
          <span class="eti">{t("defSample")}</span>
          <input bind:value={ajustes.textoPrueba} placeholder={t("defaultSample")} spellcheck="false" />
        </div>
      {:else if pestana === "atajos"}
        <h2>{t("shortcuts")}</h2>
        <span class="pista menos">{t("atajos.hint")}</span>
        <div class="atajos">
          {#each atajos as a (a.que)}
            {#if a.id}
              <button type="button" class="teclas" class:grabando={grabando === a.id} title={t("atajos.cambiar")} onclick={() => (grabando = grabando === a.id ? null : a.id)}>
                {#if grabando === a.id}
                  <kbd>{t("atajos.pulsa")}</kbd>
                {:else}
                  {#each ajustes.atajos[a.id].split("+") as k (k)}<kbd>{bonita(k)}</kbd>{/each}
                {/if}
              </button>
            {:else}
              <span class="teclas fijo" title={t("atajos.fijo")}><kbd>{a.fijo}</kbd></span>
            {/if}
            <span class="que">{a.que}</span>
          {/each}
        </div>
        {#if cambiados}
          <button type="button" class="boton suelto" onclick={resetear}>{t("atajos.reset")}</button>
        {/if}
      {:else}
        <div class="acerca">
          <img src="/icon.svg" width="64" height="64" alt="" draggable="false" />
          <div class="nombre">
            <span class="marca grande">tuna-fonts<i>.</i></span>
            <span class="ver">{t("version", { v: version })}</span>
          </div>
        </div>
        <p class="tagline">{t("tagline")}</p>
        <div class="numeros">
          {#each [[cuentas.instaladas, t("acerca.instaladas")], [cuentas.activas, t("acerca.activas")], [cuentas.favoritas, t("acerca.favoritas")]] as [n, l] (l)}
            <div class="dato"><span class="n num">{n}</span><span class="l">{l}</span></div>
          {/each}
        </div>
        <div class="nov">
          <span class="etiqueta">{t("whatsNew", { v: version })}</span>
          {#each novedades as n (n)}
            <div class="linea"><span class="punto"></span><span>{n}</span></div>
          {/each}
        </div>
        <div class="firma">
          <span class="marca cueva">{t("madeIn")}</span>
          <span class="copy">© 2026 atunfrito · MIT</span>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .velo { position: absolute; inset: 0; z-index: 15; background: var(--velo); display: flex; align-items: center; justify-content: center; animation: tf-fade 0.15s ease-out; }
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
  .lado { background: var(--side); border-right: 1px solid var(--line); padding: 16px 10px; display: flex; flex-direction: column; gap: 2px; }
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
  .cuerpo { position: relative; padding: 22px 26px 18px; overflow-y: auto; display: flex; flex-direction: column; gap: 14px; }
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
  }
  .cerrar:hover { color: var(--ink); }
  h2 { margin: 0; font-size: 18px; font-weight: 700; letter-spacing: -0.02em; }
  .fila { display: flex; align-items: center; justify-content: space-between; gap: 16px; padding-bottom: 14px; border-bottom: 1px solid var(--line); }
  .txt { display: flex; flex-direction: column; gap: 3px; }
  .eti { font-size: 13.5px; font-weight: 600; }
  .pista { font-size: 12px; color: var(--ink2); }
  .pista.menos { font-size: 12.5px; margin-top: -6px; }
  .prueba { display: flex; flex-direction: column; gap: 8px; }
  .prueba input { height: 32px; padding: 0 10px; border-radius: 8px; border: 0; box-shadow: 0 0 0 1px var(--line2); background: var(--field); font: 500 13px var(--sans); color: var(--ink); outline: 0; }
  .boton { flex: none; height: 28px; padding: 0 12px; border: 0; border-radius: 999px; background: var(--chip); color: var(--ink); font: 600 12px var(--sans); }
  .boton:hover:not(:disabled) { background: var(--ink); color: var(--win); }
  .boton.suelto { align-self: flex-start; }
  .atajos { display: grid; grid-template-columns: auto 1fr; gap: 10px 14px; font-size: 13px; color: var(--ink2); }
  .teclas { justify-self: start; display: flex; gap: 3px; padding: 4px; margin: -4px; border: 0; border-radius: 8px; background: transparent; transition: background-color 0.15s, box-shadow 0.15s; }
  button.teclas:hover { background: var(--hov); }
  .teclas.grabando { box-shadow: 0 0 0 2px var(--ink); }
  .teclas.fijo { cursor: default; }
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
  .numeros { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 8px; }
  .dato { display: flex; flex-direction: column; gap: 2px; padding: 8px 12px; border-radius: 10px; background: var(--chip); }
  .dato .n { font-size: 20px; font-weight: 700; letter-spacing: -0.03em; line-height: 1.1; }
  .dato .l { font-size: 11.5px; color: var(--ink2); }
  .nov { display: flex; flex-direction: column; gap: 6px; }
  .linea { display: flex; gap: 9px; align-items: baseline; font-size: 12.5px; line-height: 1.45; }
  .punto { flex: none; width: 5px; height: 5px; border-radius: 50%; background: var(--red); transform: translateY(-2px); }
  .firma { margin-top: auto; display: flex; align-items: flex-end; justify-content: space-between; }
  .cueva { font-size: 18px; color: var(--red); }
  .copy { font: 400 10.5px var(--mono); color: var(--ink3); }
</style>
