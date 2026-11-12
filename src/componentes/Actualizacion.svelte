<script>
  // Aviso de versión nueva, abajo en la barra lateral. «ahora no» lo cierra
  // hasta la próxima vez que se abra la app.
  import { ArrowDownToLine } from "@lucide/svelte";
  import { nueva, instalarApp } from "../lib/actualizador.svelte.js";
  import { t } from "../lib/i18n.svelte.js";

  const pct = $derived(nueva.total > 0 ? Math.min(100, Math.round((nueva.bajado / nueva.total) * 100)) : 0);
  const bajando = $derived(nueva.estado === "downloading" || nueva.estado === "installing");
  const ver = $derived(!nueva.cerrado && (nueva.estado === "available" || bajando || (nueva.estado === "error" && nueva.version)));
  // El plugin dice "signature" si el paquete no está firmado con nuestra clave.
  const firmaMala = $derived(/signature/i.test(nueva.error));
</script>

{#if ver}
  <div class="act" role="status">
    <span class="ico"><ArrowDownToLine size={14} strokeWidth={2} /></span>
    <div class="txt">
      <b>{t("act.hay", { v: nueva.version })}</b>
      <span>
        {#if nueva.estado === "installing"}{t("act.instalando")}
        {:else if nueva.estado === "downloading"}{t("act.bajando", { pct })}
        {:else if nueva.estado === "error"}{firmaMala ? t("act.firma") : t("act.fallo")}
        {:else if nueva.portable}{t("act.portable")}
        {:else}{t("act.pista")}{/if}
      </span>
    </div>
    {#if !bajando}
      <div class="botones">
        <button type="button" class="no" onclick={() => (nueva.cerrado = true)}>{t("act.ahoraNo")}</button>
        <button type="button" class="si" onclick={instalarApp}>
          {nueva.portable ? t("act.descargar") : nueva.estado === "error" ? t("act.reintentar") : t("act.actualizar")}
        </button>
      </div>
    {:else}
      <span class="barra"><span style:width="{nueva.total ? pct : 100}%" class:sin={!nueva.total}></span></span>
    {/if}
  </div>
{/if}

<style>
  .act {
    position: relative;
    overflow: hidden;
    display: grid;
    grid-template-columns: 18px minmax(0, 1fr);
    gap: 8px 8px;
    padding: 10px 10px 10px 10px;
    border-radius: 10px;
    background: var(--win);
    box-shadow: 0 0 0 1px var(--line2);
    animation: tf-up 0.25s ease-out;
  }
  .ico { color: var(--ink2); display: flex; padding-top: 1px; }
  .txt { display: flex; flex-direction: column; gap: 2px; font-size: 12px; line-height: 1.35; }
  .txt b { font-weight: 600; }
  .txt span { color: var(--ink2); }
  .botones { grid-column: 1 / -1; display: flex; justify-content: flex-end; gap: 4px; }
  button { height: 24px; padding: 0 10px; border-radius: 999px; border: 0; font: 600 11.5px var(--sans); transition: background-color 0.15s, color 0.15s; }
  .no { background: transparent; color: var(--ink2); }
  .no:hover { background: var(--hov); color: var(--ink); }
  .si { background: var(--ink); color: var(--win); }
  .barra { grid-column: 1 / -1; height: 2px; background: var(--line); border-radius: 2px; overflow: hidden; }
  .barra span { display: block; height: 100%; background: var(--ink); transition: width 0.25s linear; }
  .barra span.sin { animation: late 1.2s ease-in-out infinite; }
  @keyframes late { 50% { opacity: 0.35; } }
</style>
