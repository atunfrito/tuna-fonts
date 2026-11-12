<script>
  import { SlidersHorizontal, Sun, Moon, Plus } from "@lucide/svelte";
  import Segmentado from "./Segmentado.svelte";
  import Actualizacion from "./Actualizacion.svelte";
  import { temaActual, ponerTema, version } from "../lib/ajustes.svelte.js";
  import { arrastre } from "../lib/arrastre.svelte.js";
  import { t } from "../lib/i18n.svelte.js";

  // seccion: instaladas | descubrir | favoritas | col:<id> | car:<ruta>
  let {
    seccion, cat, cuentas, catCuentas, colecciones, carpetas, pie, renombrando = $bindable(),
    onseccion, oncat, onnueva, onrenombrar, onanadirCarpeta, onajustes, onacerca,
  } = $props();

  const CATEGORIAS = ["sans", "serif", "script", "display", "mono", "otros"];

  let borrador = $state("");
  let campo = $state();

  $effect(() => {
    if (renombrando && campo) {
      borrador = colecciones.find((c) => c.id === renombrando)?.nombre ?? "";
      campo.focus();
      campo.select();
    }
  });

  function terminar(guardar) {
    if (guardar) onrenombrar(renombrando, borrador);
    renombrando = null;
  }

  // Colección nueva: campo en línea; solo se crea con Enter y un nombre.
  let creando = $state(false);
  let nombreNuevo = $state("");
  let campoNuevo = $state();
  $effect(() => {
    if (creando && campoNuevo) campoNuevo.focus();
  });
  function empezarNueva() {
    nombreNuevo = "";
    creando = true;
  }
  function acabarNueva(crear) {
    const n = nombreNuevo.trim();
    creando = false;
    if (crear && n) onnueva(n);
  }
</script>

{#snippet item(texto, activo, ir, cuenta, extra = {})}
  <button
    type="button"
    class="item"
    class:activo
    class:diana={extra.diana}
    onclick={ir}
    ondblclick={extra.doble}
    title={extra.titulo}
    data-coleccion={extra.coleccion}
    aria-current={activo ? "page" : undefined}
  >
    <span class="punto" class:rojo={extra.rojo}></span>
    <span class="texto">{texto}</span>
    {#if cuenta != null && cuenta !== ""}<span class="cuenta num">{cuenta}</span>{/if}
  </button>
{/snippet}

{#snippet cabecera(texto, titulo, alPulsar)}
  <div class="titulo con-mas">
    <span>{texto}</span>
    <button type="button" class="mas" onclick={alPulsar} title={titulo} aria-label={titulo}>
      <Plus size={13} strokeWidth={2} />
    </button>
  </div>
{/snippet}

<aside>
  <div class="cabeza" data-tauri-drag-region>
    <img src="/icon.svg" width="24" height="24" alt="" draggable="false" />
    <span class="marca" data-tauri-drag-region>tuna-fonts<i>.</i></span>
  </div>

  <nav>
    <div class="grupo">
      <span class="titulo">{t("grupo.biblioteca")}</span>
      {@render item(t("sec.instaladas"), seccion === "instaladas", () => onseccion("instaladas"), cuentas.instaladas)}
      {@render item(t("sec.descubrir"), seccion === "descubrir", () => onseccion("descubrir"), cuentas.descubrir)}
      {@render item(t("sec.favoritas"), seccion === "favoritas", () => onseccion("favoritas"), cuentas.favoritas, { rojo: true })}
    </div>

    <div class="grupo">
      {@render cabecera(t("grupo.colecciones"), t("col.nueva"), empezarNueva)}
      {#each colecciones as c (c.id)}
        {#if renombrando === c.id}
          <input
            class="renombrar"
            bind:this={campo}
            bind:value={borrador}
            onkeydown={(e) => {
              if (e.key === "Enter") terminar(true);
              if (e.key === "Escape") (e.stopPropagation(), terminar(false));
            }}
            onblur={() => terminar(true)}
          />
        {:else}
          {@render item(c.nombre, seccion === "col:" + c.id, () => onseccion("col:" + c.id), c.fuentes.length, {
            doble: () => (renombrando = c.id),
            coleccion: c.id,
            diana: arrastre.activo && arrastre.sobre === c.id,
            titulo: t("col.pista"),
          })}
        {/if}
      {/each}
      {#if creando}
        <input
          class="renombrar"
          bind:this={campoNuevo}
          bind:value={nombreNuevo}
          placeholder={t("col.nuevaNombre")}
          aria-label={t("col.nueva")}
          onkeydown={(e) => {
            if (e.key === "Enter") acabarNueva(true);
            if (e.key === "Escape") (e.stopPropagation(), acabarNueva(false));
          }}
          onblur={() => acabarNueva(false)}
        />
      {:else if !colecciones.length}
        <span class="vacio">{t("col.vacio")}</span>
      {/if}
    </div>

    <div class="grupo">
      {@render cabecera(t("grupo.carpetas"), t("car.anadir"), onanadirCarpeta)}
      {#each carpetas as c (c.ruta)}
        {@render item(c.nombre, seccion === "car:" + c.ruta, () => onseccion("car:" + c.ruta), c.cuenta, { titulo: c.ruta })}
      {/each}
      {#if !carpetas.length}
        <span class="vacio">{t("car.vacio")}</span>
      {/if}
    </div>

    <div class="grupo">
      <span class="titulo">{t("grupo.categorias")}</span>
      {#each CATEGORIAS as c (c)}
        {@render item(t("cat." + c), cat === c, () => oncat(cat === c ? null : c), catCuentas[c] ?? 0)}
      {/each}
    </div>
  </nav>

  <div class="pie">
    <Actualizacion />
    <span class="nota num">{pie}</span>
    <div class="fila">
      <button type="button" class="ajustes" onclick={onajustes}>
        <SlidersHorizontal size={15} strokeWidth={1.9} />{t("settings")}
      </button>
      <Segmentado
        etiqueta={t("aj.tema")}
        opciones={[
          { valor: "light", icono: Sun, titulo: t("light") },
          { valor: "dark", icono: Moon, titulo: t("dark") },
        ]}
        valor={temaActual()}
        onelegir={ponerTema}
        ancho={30}
        alto={22}
      />
    </div>
    <button type="button" class="version" onclick={onacerca}>v{version} · {t("about")}</button>
  </div>
</aside>

<style>
  aside {
    background: var(--side);
    border-right: 1px solid var(--line);
    display: flex;
    flex-direction: column;
    min-height: 0;
    padding: 0 10px 14px;
    transition: background-color 0.3s;
  }
  .cabeza { flex: none; height: 52px; display: flex; align-items: center; gap: 8px; padding: 0 8px; }
  .cabeza img { display: block; flex: none; }
  .cabeza .marca { display: flex; align-items: center; height: 24px; font-size: 18px; transform: translateY(1px); }
  nav { flex: 1 1 auto; min-height: 0; overflow-y: auto; margin: 0 -6px; padding: 0 6px 6px; }
  .grupo { display: flex; flex-direction: column; gap: 1px; margin-top: 14px; }
  .grupo:first-child { margin-top: 2px; }
  .titulo { padding: 0 10px 6px; font-size: 10.5px; font-weight: 600; letter-spacing: 0.08em; text-transform: uppercase; color: var(--ink3); }
  .item {
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
    transition: background-color 0.15s, color 0.15s, box-shadow 0.15s;
  }
  .item:hover { background: var(--hov); }
  .item.activo { background: var(--chip); color: var(--ink); font-weight: 600; }
  .item.diana { box-shadow: inset 0 0 0 1.5px var(--red); }
  .punto { width: 7px; height: 7px; flex: none; border-radius: 50%; box-shadow: inset 0 0 0 1.5px var(--line2); }
  .activo .punto { background: var(--ink); box-shadow: none; }
  .activo .punto.rojo { background: var(--red); }
  /* La cabecera mide lo mismo que el botón y el hueco de abajo va fuera
     (margen, no relleno), así el fondo del «+» queda centrado con el texto. */
  .con-mas { display: flex; align-items: center; justify-content: space-between; height: 20px; padding: 0 2px 0 10px; margin-bottom: 4px; }
  .mas {
    flex: none; width: 20px; height: 20px; padding: 0; display: grid; place-items: center;
    border: 0; border-radius: 5px; background: transparent; color: var(--ink3);
    transition: background-color 0.15s, color 0.15s;
  }
  .mas :global(svg) { display: block; }
  .mas:hover { background: var(--hov); color: var(--ink); }
  .vacio { padding: 2px 10px 4px; font-size: 12px; line-height: 1.4; color: var(--ink3); opacity: 0.8; }
  .texto { flex: 1 1 auto; min-width: 0; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; padding-right: 1px; }
  .cuenta { margin-left: auto; font-size: 11.5px; color: var(--ink3); }
  .renombrar {
    height: 30px;
    padding: 0 10px;
    border: 0;
    border-radius: 6px;
    background: var(--field);
    box-shadow: 0 0 0 1.5px var(--red);
    font: 600 13px var(--sans);
    color: var(--ink);
    outline: 0;
    width: 100%;
  }
  .renombrar::placeholder { color: var(--ink3); font-weight: 500; }
  .pie { flex: none; display: flex; flex-direction: column; gap: 10px; padding: 10px 4px 0; }
  .nota { font-size: 11.5px; color: var(--ink3); padding: 0 6px; }
  .fila { display: flex; align-items: center; gap: 6px; }
  .ajustes {
    flex: 1;
    height: 32px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 10px;
    border: 0;
    border-radius: 8px;
    background: transparent;
    color: var(--ink2);
    font: 600 12.5px var(--sans);
    transition: background-color 0.15s, color 0.15s;
  }
  .ajustes:hover { background: var(--hov); color: var(--ink); }
  .version { align-self: flex-start; padding: 0 6px; border: 0; background: transparent; color: var(--ink3); font: 500 11px var(--mono); transition: color 0.15s; }
  .version:hover { color: var(--ink); }
</style>
