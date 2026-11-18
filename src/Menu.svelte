<script>
  // El menú del clic derecho en la bandeja (ver src-tauri/src/bandeja.rs).
  // No importa ajustes ni i18n: esos guardan ajustes al cargarse y pisarían los
  // de la ventana principal. Los textos y el tema los manda ella.
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { AppWindow, Search, Compass, SlidersHorizontal, Power } from "@lucide/svelte";
  import es from "./i18n/es.js";
  import en from "./i18n/en.js";
  import zh from "./i18n/zh.js";

  let e = $state({ estado: "", carpetas: 0, tema: "", idioma: "" });
  const TEXTOS = { es, en, zh };
  const tr = (clave) => (TEXTOS[e.idioma] ?? es)[clave] ?? es[clave] ?? clave;
  let abierto = $state(0);

  const hacer = (accion) => invoke("accion_bandeja", { accion });

  $effect(() => {
    document.documentElement.style.background = document.body.style.background = "transparent";
    const aplicar = (x) => {
      e = x;
      if (x.tema) document.documentElement.dataset.theme = x.tema;
      document.documentElement.lang = ["en", "zh"].includes(x.idioma) ? x.idioma : "es";
    };
    invoke("estado_bandeja").then(aplicar);
    const quitar = [
      listen("menu-estado", ({ payload }) => aplicar(payload)),
      // Cada vez que se abre, la entrada vuelve a animarse.
      listen("menu-abierto", () => abierto++),
    ];
    const tecla = (ev) => ev.key === "Escape" && hacer("cerrar");
    window.addEventListener("keydown", tecla);
    return () => {
      quitar.forEach((q) => q.then((f) => f()));
      window.removeEventListener("keydown", tecla);
    };
  });

  const ITEMS = [
    { accion: "abrir", icono: AppWindow, clave: "menu.abrir" },
    { accion: "buscar", icono: Search, clave: "menu.buscar" },
    { accion: "descubrir", icono: Compass, clave: "menu.descubrir" },
    { accion: "ajustes", icono: SlidersHorizontal, clave: "settings" },
  ];
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="marco" onclick={(ev) => ev.target === ev.currentTarget && hacer("cerrar")}>
  {#key abierto}
    <div class="menu" role="menu">
      <div class="cabeza">
        <img src="/icon.svg" width="26" height="26" alt="" draggable="false" />
        <div class="textos">
          <span class="marca">tuna-fonts<i>.</i></span>
          <span class="estado num"><span class="punto" class:vivo={e.carpetas > 0}></span>{e.estado || "tuna-fonts"}</span>
        </div>
      </div>
      <div class="sep"></div>
      {#each ITEMS as it (it.accion)}
        <button type="button" role="menuitem" class="item" onclick={() => hacer(it.accion)}>
          <it.icono size={15} strokeWidth={1.9} />{tr(it.clave)}
        </button>
      {/each}
      <div class="sep"></div>
      <button type="button" role="menuitem" class="item salir" onclick={() => hacer("salir")}>
        <Power size={15} strokeWidth={1.9} />{tr("menu.salir")}
      </button>
    </div>
  {/key}
</div>

<style>
  /* Pegado abajo a la derecha: ahí está el cursor al abrirlo. */
  .marco { height: 100vh; display: flex; align-items: flex-end; justify-content: flex-end; padding: 8px; }
  .menu {
    width: 100%;
    padding: 5px;
    background: var(--win);
    color: var(--ink);
    border-radius: 12px;
    /* Sin sombra difuminada: en una ventana transparente de Windows sale como
       un halo claro alrededor. Solo un borde fino y opaco. */
    border: 1px solid #d9d6d0;
    animation: entra 0.16s cubic-bezier(0.2, 0.8, 0.2, 1);
    transform-origin: bottom right;
  }
  @keyframes entra { from { opacity: 0; transform: translateY(6px) scale(0.98); } }

  :global([data-theme="dark"]) .menu { border-color: #3a3835; }
  .cabeza { display: flex; align-items: center; gap: 10px; padding: 8px 9px 7px; }
  .cabeza img { display: block; flex: none; }
  .textos { display: flex; flex-direction: column; gap: 3px; min-width: 0; }
  .marca { font-size: 17px; }
  .estado { display: flex; align-items: center; gap: 6px; font-size: 11.5px; color: var(--ink3); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .punto { width: 6px; height: 6px; flex: none; border-radius: 50%; background: var(--off); }
  .punto.vivo { background: var(--ok); }

  .sep { height: 1px; margin: 4px 6px; background: var(--line); }
  .item {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    height: 32px;
    padding: 0 10px;
    border: 0;
    border-radius: 8px;
    background: transparent;
    color: var(--ink);
    font: 500 13px var(--sans);
    text-align: left;
    transition: background-color 0.12s, color 0.12s;
  }
  .item :global(svg) { color: var(--ink3); transition: color 0.12s; }
  .item:hover, .item:focus-visible { background: var(--chip); outline: none; }
  .item:hover :global(svg) { color: var(--ink); }
  .salir { color: var(--ink2); }
</style>
