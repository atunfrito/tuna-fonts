<script>
  // Lista (o cuadrícula) virtualizada: solo existen en la página las filas que
  // se ven, más unas pocas de margen. Con 500 familias o 1.900 de Google Fonts
  // va igual de fluida.
  //
  // alto: px de cada fila. minAncho: si se da, cuadrícula con columnas de al menos ese ancho.
  let { items, alto, clave, fila, minAncho = 0, gap = 0, padX = 0, padTop = 0, padBottom = 90, alVer = null } = $props();

  let ancho = $state(0);
  let altoVista = $state(0);
  let scroll = $state(0);

  const cols = $derived(minAncho ? Math.max(1, Math.floor((ancho - padX * 2 + gap) / (minAncho + gap))) : 1);
  const paso = $derived(alto + gap);
  const filas = $derived(Math.ceil(items.length / cols));
  const desde = $derived(Math.max(0, Math.floor((scroll - padTop) / paso) - 3));
  const hasta = $derived(Math.min(filas, Math.ceil((scroll + altoVista - padTop) / paso) + 3));
  const visibles = $derived(items.slice(desde * cols, hasta * cols));

  // Para quien quiera saber qué se ve (Descubrir pide las vistas previas así).
  $effect(() => {
    alVer?.(items.slice(Math.max(0, desde + 3) * cols, Math.max(0, hasta - 3) * cols));
  });
</script>

<div class="caja" bind:clientWidth={ancho} bind:clientHeight={altoVista} onscroll={(e) => (scroll = e.currentTarget.scrollTop)}>
  <div class="todo" style:height="{padTop + Math.max(0, filas * paso - gap) + padBottom}px">
    <div
      class="vista"
      style:top="{padTop + desde * paso}px"
      style:left="{padX}px"
      style:right="{padX}px"
      style:grid-template-columns="repeat({cols}, minmax(0, 1fr))"
      style:grid-auto-rows="{alto}px"
      style:gap="{gap}px"
    >
      {#each visibles as it (clave(it))}
        {@render fila(it)}
      {/each}
    </div>
  </div>
</div>

<style>
  .caja { flex: 1 1 auto; min-height: 0; overflow-y: auto; overflow-x: hidden; }
  .todo { position: relative; }
  .vista { position: absolute; display: grid; }
</style>
