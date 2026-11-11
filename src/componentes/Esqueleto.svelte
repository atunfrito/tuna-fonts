<div class="esq" aria-hidden="true">
  {#each [58, 44, 66, 52, 61, 38, 55, 47] as w, i (i)}
    <div class="fila">
      <span class="circ"></span>
      <span class="txt"><span class="nombre"></span><span class="muestra" style:width="{w}%"></span></span>
    </div>
  {/each}
</div>

<style>
  @keyframes tf-shim { 0% { background-position: -320px 0; } 100% { background-position: 320px 0; } }
  .esq { display: flex; flex-direction: column; }
  .fila { display: grid; grid-template-columns: 24px minmax(0, 1fr); gap: 0 12px; align-items: end; padding: 10px 24px; }
  .circ { width: 15px; height: 15px; border-radius: 50%; background: var(--hov); margin-bottom: 6px; }
  .txt { display: flex; flex-direction: column; gap: 7px; }
  .nombre { width: 90px; height: 7px; border-radius: 4px; background: var(--hov); }
  .muestra { height: 24px; border-radius: 5px; background: linear-gradient(90deg, var(--hov) 0%, var(--line2) 50%, var(--hov) 100%) 0 0 / 320px 100% repeat-x; animation: tf-shim 1.2s linear infinite; }
</style>
