<script>
  import { X } from "@lucide/svelte";
  import { avisos, quitarAviso } from "../lib/avisos.svelte.js";
  import { t } from "../lib/i18n.svelte.js";

  const COLOR = { ok: "var(--ok)", warn: "var(--warn)", err: "var(--red)" };
</script>

<div class="pila" aria-live="polite">
  {#each avisos as a (a.id)}
    <div class="aviso">
      <span class="punto" style:background={COLOR[a.tipo]}></span>
      <span class="txt"><b>{a.titulo}</b> <span>{a.texto}</span></span>
      <button type="button" aria-label={t("close")} onclick={() => quitarAviso(a.id)}><X size={11} strokeWidth={2} /></button>
    </div>
  {/each}
</div>

<style>
  .pila {
    position: absolute;
    right: 18px;
    bottom: 18px;
    z-index: 30;
    display: flex;
    flex-direction: column;
    gap: 8px;
    align-items: flex-end;
    pointer-events: none;
  }
  .aviso {
    pointer-events: auto;
    max-width: 340px;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 10px 10px 12px;
    border-radius: 10px;
    background: var(--win);
    box-shadow: 0 10px 28px rgba(0, 0, 0, 0.16), 0 0 0 1px var(--line2);
    animation: tf-up 0.2s ease-out;
  }
  .punto { flex: none; width: 8px; height: 8px; border-radius: 50%; }
  .txt { flex: 1; font-size: 12.5px; line-height: 1.4; }
  .txt b { font-weight: 600; }
  .txt span { color: var(--ink2); }
  button {
    width: 20px;
    height: 20px;
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--ink3);
    display: flex;
    align-items: center;
    justify-content: center;
  }
  button:hover { color: var(--ink); }
</style>
