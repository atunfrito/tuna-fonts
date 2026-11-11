<script>
  let { glifo = "?", titulo, texto, boton = "", onboton = null } = $props();
</script>

<div class="vacio">
  <span class="caja">{glifo}<span class="punto"></span></span>
  <div class="txt">
    <span class="titulo">{titulo}</span>
    <span class="texto">{texto}</span>
  </div>
  {#if boton}<button type="button" onclick={onboton}>{boton}</button>{/if}
</div>

<style>
  .vacio { flex: 1; min-height: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 18px; text-align: center; padding-bottom: 60px; animation: tf-up 0.3s ease-out; }
  .caja {
    position: relative;
    width: 84px;
    height: 84px;
    border-radius: 20px;
    box-shadow: inset 0 0 0 2px var(--line2);
    display: flex;
    align-items: center;
    justify-content: center;
    font: 700 34px var(--sans);
    letter-spacing: -0.04em;
    color: var(--off);
  }
  .punto { position: absolute; right: 13px; bottom: 13px; width: 8px; height: 8px; border-radius: 50%; background: var(--red); }
  .txt { display: flex; flex-direction: column; gap: 6px; max-width: 360px; }
  .titulo { font-size: 17px; font-weight: 700; letter-spacing: -0.01em; }
  .texto { font-size: 13.5px; line-height: 1.5; color: var(--ink2); text-wrap: pretty; }
  button { height: 30px; padding: 0 14px; border-radius: 999px; border: 0; background: var(--ink); color: var(--win); font: 600 12.5px var(--sans); }
</style>
