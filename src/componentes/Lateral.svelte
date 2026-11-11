<script>
  import { SlidersHorizontal, Sun, Moon } from "@lucide/svelte";
  import Segmentado from "./Segmentado.svelte";
  import { ajustes, temaActual, ponerTema, version } from "../lib/ajustes.svelte.js";
  import { t } from "../lib/i18n.svelte.js";

  // seccion: instaladas | descubrir | favoritas. cat: null o la categoría.
  let { seccion, cat, onseccion, oncat, onajustes, onacerca } = $props();

  const CATEGORIAS = ["sans", "serif", "script", "display", "mono", "otros"];

  const grupos = $derived([
    {
      etiqueta: t("grupo.biblioteca"),
      items: [
        { id: "instaladas", texto: t("sec.instaladas"), activo: seccion === "instaladas", ir: () => onseccion("instaladas") },
        { id: "descubrir", texto: t("sec.descubrir"), activo: seccion === "descubrir", ir: () => onseccion("descubrir") },
        { id: "favoritas", texto: t("sec.favoritas"), activo: seccion === "favoritas", ir: () => onseccion("favoritas"), rojo: true },
      ],
    },
    {
      etiqueta: t("grupo.categorias"),
      items: CATEGORIAS.map((c) => ({ id: c, texto: t("cat." + c), activo: cat === c, ir: () => oncat(cat === c ? null : c) })),
    },
  ]);
</script>

<aside>
  <div class="cabeza" data-tauri-drag-region>
    <img src="/icon.svg" width="24" height="24" alt="" draggable="false" />
    <span class="marca" data-tauri-drag-region>tuna-fonts<i>.</i></span>
  </div>

  <nav>
    {#each grupos as g (g.etiqueta)}
      <div class="grupo">
        <span class="titulo">{g.etiqueta}</span>
        {#each g.items as it (it.id)}
          <button type="button" class="item" class:activo={it.activo} onclick={it.ir} aria-current={it.activo ? "page" : undefined}>
            <span class="punto" class:rojo={it.rojo}></span>
            <span class="texto">{it.texto}</span>
            {#if it.cuenta != null}<span class="cuenta num">{it.cuenta}</span>{/if}
          </button>
        {/each}
      </div>
    {/each}
  </nav>

  <div class="pie">
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
  .cabeza {
    flex: none;
    height: 52px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 8px;
  }
  .cabeza img { display: block; flex: none; }
  .cabeza .marca {
    display: flex;
    align-items: center;
    height: 24px;
    font-size: 18px;
    transform: translateY(1px);
  }
  nav { flex: 1 1 auto; min-height: 0; overflow-y: auto; margin: 0 -4px; padding: 0 4px; }
  .grupo { display: flex; flex-direction: column; gap: 1px; margin-top: 14px; }
  .titulo {
    padding: 0 10px 6px;
    font-size: 10.5px;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--ink3);
  }
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
    transition: background-color 0.15s, color 0.15s;
  }
  .item:hover { background: var(--hov); }
  .item.activo { background: var(--chip); color: var(--ink); font-weight: 600; }
  .punto {
    width: 7px;
    height: 7px;
    flex: none;
    border-radius: 50%;
    box-shadow: inset 0 0 0 1.5px var(--line2);
  }
  .activo .punto { background: var(--ink); box-shadow: none; }
  .activo .punto.rojo { background: var(--red); }
  .texto { flex: 1 1 auto; min-width: 0; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; padding-right: 1px; }
  .cuenta { margin-left: auto; font-size: 11.5px; color: var(--ink3); }

  .pie { flex: none; display: flex; flex-direction: column; gap: 10px; padding: 10px 4px 0; }
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
  .version {
    align-self: flex-start;
    padding: 0 6px;
    border: 0;
    background: transparent;
    color: var(--ink3);
    font: 500 11px var(--mono);
    transition: color 0.15s;
  }
  .version:hover { color: var(--ink); }
</style>
