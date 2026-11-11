<script>
  // Selector segmentado de tuna-mono: opciones de igual ancho y una pastilla
  // (--win) que se desliza hasta la elegida. Es el único selector de la app.
  //
  // opciones: [{ valor, etiqueta?, icono?, titulo? }]
  // ancho: px de cada opción (si no, el de la más ancha); alto: px de cada opción.
  let { opciones, valor, onelegir, ancho = null, alto = 26, relleno = 14, fuente = "600 12px", etiqueta = "" } = $props();

  const indice = $derived(Math.max(0, opciones.findIndex((o) => o.valor === valor)));
  const n = $derived(opciones.length);
</script>

<div
  class="seg"
  role="radiogroup"
  aria-label={etiqueta}
  style:grid-template-columns={ancho ? `repeat(${n}, ${ancho}px)` : `repeat(${n}, minmax(0, 1fr))`}
>
  <span
    class="pastilla"
    aria-hidden="true"
    style:left="calc(3px + (100% - 6px) * {indice} / {n})"
    style:width="calc((100% - 6px) / {n})"
  ></span>
  {#each opciones as o (o.valor)}
    {@const elegida = o.valor === valor}
    <button
      type="button"
      role="radio"
      aria-checked={elegida}
      aria-label={o.titulo ?? o.etiqueta}
      title={o.titulo}
      class:elegida
      style:height="{alto}px"
      style:padding={o.icono ? "0" : `0 ${relleno}px`}
      style:font="{fuente} var(--sans)"
      onclick={() => !elegida && onelegir(o.valor)}
    >
      {#if o.icono}
        <o.icono size={13} strokeWidth={2} />
      {:else}
        {o.etiqueta}
      {/if}
    </button>
  {/each}
</div>

<style>
  .seg {
    position: relative;
    flex: none;
    display: grid;
    padding: 3px;
    border-radius: 999px;
    background: var(--chip);
  }
  .pastilla {
    position: absolute;
    top: 3px;
    bottom: 3px;
    border-radius: 999px;
    background: var(--win);
    box-shadow: var(--pastilla);
    transition: var(--desliza), background-color 0.3s;
    pointer-events: none;
  }
  button {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    white-space: nowrap;
    border: 0;
    border-radius: 999px;
    background: transparent;
    color: var(--ink2);
    transition: color 0.15s;
  }
  button:hover,
  button.elegida { color: var(--ink); }
  button.elegida { cursor: default; }
</style>
