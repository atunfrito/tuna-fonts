<script>
  import { onMount } from "svelte";
  import { SvelteSet } from "svelte/reactivity";
  import { Search, List, LayoutGrid, Folder, FolderOpen, Trash2, X, Heart, WifiOff, Layers, Type } from "@lucide/svelte";

  import Lateral from "./componentes/Lateral.svelte";
  import Controles from "./componentes/Controles.svelte";
  import Avisos from "./componentes/Avisos.svelte";
  import Segmentado from "./componentes/Segmentado.svelte";
  import ListaVirtual from "./componentes/ListaVirtual.svelte";
  import Fila from "./componentes/Fila.svelte";
  import Tarjeta from "./componentes/Tarjeta.svelte";
  import FilaGoogle from "./componentes/FilaGoogle.svelte";
  import Vacio from "./componentes/Vacio.svelte";
  import Esqueleto from "./componentes/Esqueleto.svelte";
  import BarraSeleccion from "./componentes/BarraSeleccion.svelte";
  import Ajustes from "./vistas/Ajustes.svelte";
  import Especimen from "./vistas/Especimen.svelte";
  import Comparar from "./vistas/Comparar.svelte";

  import { ajustes, enTauri } from "./lib/ajustes.svelte.js";
  import { t, locale } from "./lib/i18n.svelte.js";
  import { avisar } from "./lib/avisos.svelte.js";
  import { fuentes, iniciarFuentes, actualizarFuentes, alternar, instalar, estiloDe, ponerLista, llamar } from "./lib/fuentes.svelte.js";
  import { biblioteca, iniciarBiblioteca, alternarFavorita, aFavoritas, nuevaColeccion, renombrar, eliminarColeccion, aColeccion } from "./lib/biblioteca.svelte.js";
  import { descubrir, cargarCatalogo, pedirVistas, precargar, instalarGoogle } from "./lib/descubrir.svelte.js";
  import { arrastre, empezarArrastre } from "./lib/arrastre.svelte.js";
  import { comboDe } from "./lib/atajos.js";

  // ---- estado de la vista ----
  // instaladas | descubrir | favoritas | col:<id> | car:<ruta>
  let seccion = $state(ajustes.alAbrir === "favoritas" ? "favoritas" : "instaladas");
  let cat = $state(null);
  let q = $state("");
  let muestraPropia = $state("");
  let filtro = $state("todas");
  let orden = $state("popularidad");
  const sel = new SvelteSet();
  let detalle = $state(null);
  let comparar = $state([]);
  let ajustesEn = $state(null);
  let colAbierta = $state(false);
  let renombrando = $state(null);
  let borrando = $state(null);
  let soltando = $state(false);
  let carpetas = $state([]);
  let buscador = $state();

  const sinMovimiento = matchMedia("(prefers-reduced-motion: reduce)").matches;
  let splash = $state(!sinMovimiento);
  if (!sinMovimiento) setTimeout(() => (splash = false), 800);

  const muestraDefecto = $derived(ajustes.textoPrueba.trim() || t("defaultSample"));
  const muestra = $derived(muestraPropia || muestraDefecto);
  const tamano = $derived(ajustes.tamano);

  const enDescubrir = $derived(seccion === "descubrir");
  const coleccion = $derived(seccion.startsWith("col:") ? biblioteca.colecciones.find((c) => "col:" + c.id === seccion) : null);
  const carpeta = $derived(seccion.startsWith("car:") ? seccion.slice(4) : null);
  const detalleF = $derived(detalle ? fuentes.lista.find((f) => f.nombre === detalle) : null);
  const comparadas = $derived(comparar.map((n) => fuentes.lista.find((f) => f.nombre === n)).filter(Boolean));

  const bloqueada = (f) => f.origen === "sis" && (!ajustes.desbloqueadas || f.imprescindible);
  const motivo = (f) => (f.imprescindible ? t("candado.imprescindible") : t("candado.sistema"));
  // Las del sistema con candado no se enseñan salvo que se pida en ajustes
  // (en favoritas y colecciones sí: ahí las has puesto tú).
  const visible = (f) => ajustes.verBloqueadas || !bloqueada(f);
  const enCarpeta = (f, ruta) => f.origen === "car" && f.archivos.some((a) => a.toLowerCase().startsWith(ruta.toLowerCase() + "\\"));
  const favs = $derived(new Set(biblioteca.favoritas));

  // ---- listas ----
  const base = $derived.by(() => {
    const l = fuentes.lista;
    if (seccion === "favoritas") return l.filter((f) => favs.has(f.nombre));
    if (coleccion) return l.filter((f) => coleccion.fuentes.includes(f.nombre));
    if (carpeta) return l.filter((f) => enCarpeta(f, carpeta));
    return l.filter((f) => f.origen !== "car" && visible(f));
  });
  const ql = $derived(q.trim().toLowerCase());
  const lista = $derived(
    base.filter(
      (f) =>
        (!cat || f.categoria === cat) &&
        (!ql || f.nombre.toLowerCase().includes(ql)) &&
        (filtro === "todas" || (filtro === "activas") === (f.activa || bloqueada(f))),
    ),
  );
  const instaladasNombres = $derived(new Set(fuentes.lista.map((f) => f.nombre.toLowerCase())));
  const listaGoogle = $derived.by(() => {
    const l = descubrir.lista.filter((g) => (!cat || g.categoria === cat) && (!ql || g.nombre.toLowerCase().includes(ql)));
    if (orden === "popularidad") return [...l].sort((a, b) => a.popularidad - b.popularidad);
    if (orden === "recientes") return [...l].sort((a, b) => (b.fecha > a.fecha ? 1 : b.fecha < a.fecha ? -1 : 0));
    return [...l].sort((a, b) => a.nombre.localeCompare(b.nombre));
  });

  const catCuentas = $derived.by(() => {
    const c = {};
    for (const f of enDescubrir ? descubrir.lista : base) c[f.categoria] = (c[f.categoria] ?? 0) + 1;
    return c;
  });
  const instaladas = $derived(fuentes.lista.filter((f) => f.origen !== "car" && visible(f)));
  const activasN = $derived(instaladas.filter((f) => f.activa || bloqueada(f)).length);
  const cuentaGoogle = $derived(
    descubrir.lista.length ? (descubrir.lista.length >= 1000 ? (Math.floor(descubrir.lista.length / 100) / 10).toLocaleString(locale()) + "k" : descubrir.lista.length) : "",
  );
  const cuentas = $derived({ instaladas: instaladas.length, descubrir: cuentaGoogle, favoritas: biblioteca.favoritas.length });
  const carpetasCuenta = $derived(carpetas.map((c) => ({ ...c, cuenta: fuentes.lista.filter((f) => enCarpeta(f, c.ruta)).length })));

  const cabecera = $derived(
    comparadas.length
      ? t("comparar")
      : detalleF
        ? t("especimen")
        : cat
          ? t("cat." + cat)
          : enDescubrir
            ? t("google")
            : seccion === "favoritas"
              ? t("sec.favoritas")
              : coleccion
                ? coleccion.nombre
                : carpeta
                  ? (carpetas.find((c) => c.ruta === carpeta)?.nombre ?? "")
                  : t("sec.instaladas"),
  );
  const vistaNormal = $derived(!detalleF && !comparadas.length);

  // Vistas previas de Descubrir: solo lo que se ve.
  let vistasGoogle = [];
  function alVerGoogle(items) {
    vistasGoogle = items;
    pedirVistas(items.map((g) => g.nombre), muestra);
  }
  $effect(() => {
    // Si cambia el texto de prueba, se pide solo lo que falte de lo que se ve.
    const m = muestra;
    if (enDescubrir) pedirVistas(vistasGoogle.map((g) => g.nombre), m);
  });

  // ---- acciones ----
  function irA(s) {
    seccion = s;
    detalle = null;
    comparar = [];
    sel.clear();
    cat = null;
    colAbierta = false;
    if (s === "descubrir") cargarCatalogo();
  }

  async function alternarUna(f) {
    const activar = !f.activa;
    const r = await alternar([f.nombre], activar, ajustes.desbloqueadas);
    if (r?.cambiadas) avisar("ok", t(activar ? "ok.activada" : "ok.desactivada", { n: f.nombre }), activar ? t("ok.yaSale") : "");
  }

  async function alternarVarias(nombres, activar, limpiar = true) {
    const fs = nombres.map((n) => fuentes.lista.find((f) => f.nombre === n)).filter(Boolean);
    const bloq = fs.filter(bloqueada).length;
    const tocar = fs.filter((f) => !bloqueada(f) && f.activa !== activar).map((f) => f.nombre);
    if (limpiar) sel.clear();
    colAbierta = false;
    if (tocar.length) await alternar(tocar, activar, ajustes.desbloqueadas);
    const n = fs.length - bloq;
    if (bloq && !activar) avisar("warn", t("ok.desactivadas", { n }), bloq === 1 ? t("aviso.sistemaUna") : t("aviso.sistema", { n: bloq }));
    else avisar("ok", t(activar ? "ok.activadas" : "ok.desactivadas", { n }), "");
  }

  function aCol(id, nombres) {
    let c = biblioteca.colecciones.find((x) => x.id === id);
    if (!c) {
      id = nuevaColeccion(t("col.nombre", { n: biblioteca.colecciones.length + 1 }));
      c = biblioteca.colecciones.find((x) => x.id === id);
    }
    aColeccion(id, nombres);
    avisar("ok", t("ok.aColeccion", { n: nombres.length === 1 ? nombres[0] : t("fuentesN", { n: nombres.length }), c: c.nombre }), "");
  }

  function crearColeccion() {
    const id = nuevaColeccion(t("col.nombre", { n: biblioteca.colecciones.length + 1 }));
    irA("col:" + id);
    renombrando = id;
  }

  let temporizadorBorrar;
  function borrarColeccion() {
    if (!coleccion) return;
    if (borrando !== coleccion.id) {
      borrando = coleccion.id;
      clearTimeout(temporizadorBorrar);
      temporizadorBorrar = setTimeout(() => (borrando = null), 3000);
      return;
    }
    eliminarColeccion(coleccion.id);
    borrando = null;
    irA("instaladas");
    avisar("ok", t("ok.colEliminada"), t("ok.siguenInstaladas"));
  }

  async function activarColeccion(activar) {
    if (!coleccion) return;
    const n = coleccion.fuentes;
    const fs = n.map((x) => fuentes.lista.find((f) => f.nombre === x)).filter((f) => f && !bloqueada(f) && f.activa !== activar);
    if (fs.length) await alternar(fs.map((f) => f.nombre), activar, ajustes.desbloqueadas);
    if (activar) avisar("ok", t("ok.colActivada"), t("ok.colListas", { n: n.length }));
    else avisar("ok", t("ok.colDesactivada"), "");
  }

  async function anadirCarpeta() {
    if (!enTauri) return;
    const l = await llamar("anadir_carpeta");
    if (!l) return;
    const nueva = l.find((c) => !carpetas.some((x) => x.ruta === c.ruta));
    carpetas = l;
    await actualizarFuentes();
    if (nueva) irA("car:" + nueva.ruta);
    avisar("ok", t("ok.carpeta"), t("ok.carpetaTexto"));
  }

  async function quitarCarpeta() {
    if (!carpeta) return;
    carpetas = await llamar("quitar_carpeta", { ruta: carpeta });
    await actualizarFuentes();
    irA("instaladas");
    avisar("ok", t("ok.carpetaQuitada"), t("ok.carpetaQuitadaTexto"));
  }

  function seleccionar(f) {
    if (sel.has(f.nombre)) sel.delete(f.nombre);
    else sel.add(f.nombre);
    colAbierta = false;
  }

  function arrastrar(e, f) {
    empezarArrastre(
      e,
      () => (sel.has(f.nombre) ? [...sel] : [f.nombre]),
      (id, nombres) => {
        aCol(id, nombres);
        sel.clear();
      },
    );
  }

  // ---- teclado ----
  function tecla(e) {
    if (ajustesEn || splash) return;
    const combo = comboDe(e) ?? "";
    const tag = e.target?.tagName ?? "";
    const escribiendo = tag === "INPUT" || tag === "TEXTAREA";
    if (combo === ajustes.atajos.buscar) {
      e.preventDefault();
      buscador?.focus();
      buscador?.select();
      return;
    }
    if (e.key === "Escape") {
      if (escribiendo) return e.target.blur();
      if (colAbierta) colAbierta = false;
      else if (comparar.length) comparar = [];
      else if (detalle) detalle = null;
      else sel.clear();
      return;
    }
    if (escribiendo || !vistaNormal || enDescubrir) return;
    if (combo === ajustes.atajos.todas) {
      e.preventDefault();
      for (const f of lista) sel.add(f.nombre);
      return;
    }
    if (combo === ajustes.atajos.alternar && sel.size) {
      e.preventDefault();
      const n = [...sel];
      const algunaApagada = fuentes.lista.some((f) => sel.has(f.nombre) && !bloqueada(f) && !f.activa);
      alternarVarias(n, algunaApagada, false);
    }
  }

  // ---- arranque ----
  onMount(() => {
    iniciarBiblioteca();
    iniciarFuentes();
    precargar(muestra);
    if (!enTauri) return;
    llamar("carpetas").then((l) => (carpetas = l));
    let quitar;
    import("@tauri-apps/api/webview").then(async ({ getCurrentWebview }) => {
      // Archivos arrastrados desde fuera (el explorador). Lo de dentro de la
      // app no usa el arrastre del navegador, así que aquí nunca llega.
      quitar = await getCurrentWebview().onDragDropEvent(async ({ payload: p }) => {
        if (p.type === "enter") soltando = (p.paths ?? []).length > 0;
        else if (p.type === "leave") soltando = false;
        else if (p.type === "drop") {
          soltando = false;
          const rutas = (p.paths ?? []).filter((r) => /\.(ttf|otf|ttc|otc)$/i.test(r));
          if (!rutas.length) return avisar("err", t("err.noFuente"), t("err.noFuenteTexto"));
          const todos = ajustes.instalarPara === "todos";
          const r = await instalar(rutas, todos);
          if (!r || !r.instaladas.length) return;
          const titulo = r.instaladas.length === 1 ? t("ok.instalada", { n: r.instaladas[0] }) : t("ok.instaladas", { n: r.instaladas.length });
          avisar("ok", titulo, todos ? t("ok.paraTodos") : t("ok.soloTu"));
        }
      });
    });
    return () => quitar?.();
  });
</script>

<svelte:window onkeydown={tecla} />

{#snippet fila(f)}
  <Fila
    {f}
    estilo={estiloDe(f)}
    {muestra}
    {tamano}
    sel={sel.has(f.nombre)}
    fav={favs.has(f.nombre)}
    bloqueada={bloqueada(f)}
    motivo={motivo(f)}
    onsel={() => seleccionar(f)}
    onabrir={() => ((detalle = f.nombre), sel.clear())}
    onalternar={() => alternarUna(f)}
    onfav={() => alternarFavorita(f.nombre)}
    onarrastre={(e) => arrastrar(e, f)}
  />
{/snippet}

{#snippet tarjeta(f)}
  <Tarjeta
    {f}
    estilo={estiloDe(f)}
    sel={sel.has(f.nombre)}
    fav={favs.has(f.nombre)}
    bloqueada={bloqueada(f)}
    motivo={motivo(f)}
    onsel={() => seleccionar(f)}
    onabrir={() => ((detalle = f.nombre), sel.clear())}
    onalternar={() => alternarUna(f)}
    onfav={() => alternarFavorita(f.nombre)}
    onarrastre={(e) => arrastrar(e, f)}
  />
{/snippet}

{#snippet filaGoogle(g)}
  <FilaGoogle
    {g}
    {muestra}
    {tamano}
    instalada={instaladasNombres.has(g.nombre.toLowerCase())}
    pct={descubrir.descargas[g.nombre]}
    oninstalar={() => instalarGoogle(g.nombre, ponerLista)}
  />
{/snippet}

<div class="ventana">
  <Lateral
    {seccion}
    {cat}
    {cuentas}
    {catCuentas}
    colecciones={biblioteca.colecciones}
    carpetas={carpetasCuenta}
    pie={t("activas", { n: activasN, total: instaladas.length })}
    bind:renombrando
    onseccion={irA}
    oncat={(c) => ((cat = c), (detalle = null), (comparar = []))}
    onnueva={crearColeccion}
    onrenombrar={renombrar}
    onanadirCarpeta={anadirCarpeta}
    onajustes={() => (ajustesEn = "ajustes")}
    onacerca={() => (ajustesEn = "acerca")}
  />

  <main>
    <header data-tauri-drag-region>
      {#if vistaNormal}
        <label class="campo buscar">
          <Search size={13} strokeWidth={2} />
          <input bind:this={buscador} bind:value={q} placeholder={enDescubrir ? t("searchGoogle") : t("search")} spellcheck="false" />
        </label>
        <label class="campo prueba">
          <span class="a">A</span>
          <input bind:value={muestraPropia} placeholder={t("typeSomething")} spellcheck="false" />
          {#if muestraPropia}
            <button type="button" class="reset" title={t("resetText")} aria-label={t("resetText")} onclick={() => (muestraPropia = "")}><X size={9} strokeWidth={3} /></button>
          {/if}
        </label>
        <div class="tamano" title={t("tamano")}>
          <span class="a pq">A</span>
          <input class="rango" type="range" min="16" max="96" bind:value={ajustes.tamano} aria-label={t("tamano")} />
          <span class="a gr">A</span>
          <span class="num n">{tamano}</span>
        </div>
      {/if}
      <div class="hueco" data-tauri-drag-region></div>
      {#if vistaNormal && !enDescubrir}
        <div class="vista">
          <Segmentado
            etiqueta={t("list")}
            opciones={[
              { valor: "lista", icono: List, titulo: t("list") },
              { valor: "cuadricula", icono: LayoutGrid, titulo: t("grid") },
            ]}
            valor={ajustes.vista}
            onelegir={(v) => (ajustes.vista = v)}
            ancho={34}
            alto={24}
          />
        </div>
      {/if}
      <Controles />
    </header>

    <div class="titulo">
      <span class="etiqueta">{cabecera}</span>
      {#if vistaNormal}<span class="cuenta num">{enDescubrir ? listaGoogle.length : lista.length}</span>{/if}
      <div class="derecha">
        {#if vistaNormal && !enDescubrir}
          <Segmentado
            etiqueta={t("vista.todas")}
            opciones={["todas", "activas", "inactivas"].map((v) => ({ valor: v, etiqueta: t("vista." + v) }))}
            valor={filtro}
            onelegir={(v) => (filtro = v)}
            ancho={74}
            alto={22}
            fuente="600 11.5px"
          />
        {:else if vistaNormal}
          <Segmentado
            etiqueta={t("orden.popularidad")}
            opciones={["popularidad", "recientes", "alfabetico"].map((v) => ({ valor: v, etiqueta: t("orden." + v) }))}
            valor={orden}
            onelegir={(v) => (orden = v)}
            ancho={84}
            alto={22}
            fuente="600 11.5px"
          />
        {/if}
        {#if coleccion && vistaNormal}
          <span class="sep"></span>
          <button type="button" class="pild tinta" onclick={() => activarColeccion(true)}>{t("activateAll")}</button>
          <button type="button" class="pild" onclick={() => activarColeccion(false)}>{t("deactivate")}</button>
          <button type="button" class="pild borrar" class:seguro={borrando === coleccion.id} title={t("col.eliminarPista")} onclick={borrarColeccion}>
            <Trash2 size={12} strokeWidth={2} />{borrando === coleccion.id ? t("col.seguro") : t("col.eliminar")}
          </button>
        {/if}
      </div>
    </div>

    {#if carpeta && vistaNormal}
      <div class="aviso-carpeta">
        <Folder size={16} strokeWidth={1.9} />
        <span>{t("folderInfo")}</span>
        <button type="button" class="pild blanca" onclick={anadirCarpeta}>{t("addAnother")}</button>
        <button type="button" class="pild clara" onclick={quitarCarpeta}>{t("stopWatching")}</button>
      </div>
    {/if}

    {#if detalleF}
      <Especimen
        f={detalleF}
        {muestra}
        fav={favs.has(detalleF.nombre)}
        bloqueada={bloqueada(detalleF)}
        volver={seccion === "favoritas" ? t("favorites") : coleccion ? coleccion.nombre : t("sec.instaladas")}
        onvolver={() => (detalle = null)}
        onalternar={() => alternarUna(detalleF)}
        onfav={() => alternarFavorita(detalleF.nombre)}
      />
    {:else if comparadas.length}
      <Comparar
        familias={comparadas}
        {muestra}
        {tamano}
        onvolver={() => (comparar = [])}
        onquitar={(n) => (comparar = comparar.length <= 2 ? [] : comparar.filter((x) => x !== n))}
      />
    {:else if enDescubrir}
      {#if descubrir.lista.length && listaGoogle.length}
        <ListaVirtual items={listaGoogle} alto={Math.ceil(34 + tamano * 1.38)} clave={(g) => g.nombre} fila={filaGoogle} alVer={alVerGoogle} padBottom={60} />
      {:else if descubrir.lista.length}
        <Vacio titulo={t("vacio.nada", { q: q.trim() })} texto={t("vacio.nadaGoogle")} boton={t("vacio.borrar")} onboton={() => (q = "")} />
      {:else if descubrir.error}
        <Vacio icono={WifiOff} titulo={t("g.sinRed")} texto={descubrir.error} boton={t("g.reintentar")} onboton={() => cargarCatalogo(true)} />
      {:else}
        <Esqueleto />
      {/if}
    {:else if fuentes.cargando}
      {#if fuentes.esqueleto}<Esqueleto />{/if}
    {:else if !lista.length}
      {#if seccion === "favoritas" && !ql && !base.length}
        <Vacio icono={Heart} titulo={t("vacio.favTitulo")} texto={t("vacio.favTexto")} boton={t("vacio.favBoton")} onboton={() => irA("instaladas")} />
      {:else if ql}
        <Vacio
          titulo={t("vacio.nada", { q: q.trim() })}
          texto={t("vacio.nadaTexto")}
          boton={t("vacio.buscarGoogle")}
          onboton={() => {
            const b = q;
            irA("descubrir");
            q = b;
          }}
        />
      {:else}
        <Vacio icono={coleccion ? Layers : carpeta ? FolderOpen : Type} titulo={t("vacio.lista")} texto={coleccion ? t("vacio.colTexto") : carpeta ? t("vacio.carTexto") : ""} />
      {/if}
    {:else if ajustes.vista === "cuadricula"}
      {#key seccion + cat}
        <ListaVirtual items={lista} alto={146} minAncho={176} gap={10} padX={24} padTop={4} clave={(f) => f.nombre} fila={tarjeta} />
      {/key}
    {:else}
      {#key seccion + cat}
        <ListaVirtual items={lista} alto={Math.ceil(32 + tamano * 1.38)} clave={(f) => f.nombre} fila={fila} padBottom={80} />
      {/key}
    {/if}

    {#if sel.size && vistaNormal && !enDescubrir}
      <BarraSeleccion
        n={sel.size}
        colecciones={biblioteca.colecciones}
        bind:abierta={colAbierta}
        onactivar={() => alternarVarias([...sel], true)}
        ondesactivar={() => alternarVarias([...sel], false)}
        onfavoritas={() => {
          const n = [...sel];
          aFavoritas(n);
          sel.clear();
          avisar("ok", t("ok.aFavoritas", { n: n.length }), "");
        }}
        oncoleccion={(id) => {
          aCol(id, [...sel]);
          sel.clear();
          colAbierta = false;
        }}
        oncomparar={() => {
          if (sel.size < 2 || sel.size > 4) return;
          comparar = [...sel];
          sel.clear();
        }}
        onlimpiar={() => sel.clear()}
      />
    {/if}

    {#if soltando}
      <div class="soltar">
        <div class="marco">
          <img src="/icon.svg" width="92" height="92" alt="" draggable="false" />
          <span class="st">{t("dropTitle")}</span>
          <span class="sx">{ajustes.instalarPara === "todos" ? t("dropTextTodos") : t("dropText")}</span>
        </div>
      </div>
    {/if}
  </main>

  {#if arrastre.activo}
    <div class="fantasma" style:left="{arrastre.x + 12}px" style:top="{arrastre.y + 10}px">
      {arrastre.nombres.length === 1 ? arrastre.nombres[0] : t("fuentesN", { n: arrastre.nombres.length })}
    </div>
  {/if}

  {#if ajustesEn}
    <Ajustes bind:pestana={ajustesEn} oncerrar={() => (ajustesEn = null)} cuentas={{ instaladas: instaladas.length, activas: activasN, favoritas: biblioteca.favoritas.length }} />
  {/if}

  <Avisos />

  {#if splash}
    <div class="splash">
      <img src="/icon.svg" width="140" height="140" alt="" draggable="false" />
      <span class="marca">tuna-fonts<i>.</i></span>
    </div>
  {/if}
</div>

<style>
  .ventana { position: relative; height: 100vh; overflow: hidden; display: grid; grid-template-columns: 216px minmax(0, 1fr); }
  main { position: relative; display: flex; flex-direction: column; min-width: 0; min-height: 0; }
  header { flex: none; height: 52px; display: flex; align-items: center; gap: 10px; padding-left: 24px; }
  .hueco { flex: 1; align-self: stretch; }
  .campo {
    height: 28px;
    display: flex;
    align-items: center;
    gap: 7px;
    padding: 0 9px;
    border-radius: 6px;
    background: var(--field);
    box-shadow: 0 0 0 1px var(--line);
    color: var(--ink3);
  }
  .buscar { flex: 0 0 190px; }
  .prueba { flex: 1 1 auto; min-width: 0; max-width: 360px; }
  .campo input { flex: 1; min-width: 0; border: 0; outline: 0; background: transparent; font: 500 12.5px var(--sans); color: var(--ink); }
  .a { font: 700 12px var(--sans); color: var(--ink3); }
  .reset { width: 16px; height: 16px; border: 0; border-radius: 50%; background: var(--line2); color: var(--win); padding: 0; display: flex; align-items: center; justify-content: center; flex: none; }
  .tamano { flex: none; display: flex; align-items: center; gap: 8px; margin-left: 8px; }
  .tamano .pq { font: 600 10px var(--sans); }
  .tamano .gr { font: 600 15px var(--sans); }
  .tamano .n { width: 22px; font-size: 11.5px; color: var(--ink3); }
  .rango { -webkit-appearance: none; appearance: none; width: 90px; height: 2px; background: var(--line2); border-radius: 2px; outline: none; }
  .rango::-webkit-slider-thumb { -webkit-appearance: none; width: 12px; height: 12px; border-radius: 50%; background: var(--ink); border: 0; cursor: pointer; }
  .vista { margin-right: 12px; display: flex; }

  .titulo { flex: none; display: flex; align-items: center; gap: 8px; padding: 10px 24px 8px; min-height: 46px; }
  .cuenta { font-size: 10.5px; color: var(--ink3); }
  .derecha { margin-left: auto; display: flex; align-items: center; gap: 2px; }
  .sep { width: 1px; height: 16px; background: var(--line2); margin: 0 6px; }
  .pild {
    height: 24px;
    padding: 0 10px;
    border: 0;
    border-radius: 999px;
    background: var(--chip);
    color: var(--ink);
    font: 600 11.5px var(--sans);
    display: flex;
    align-items: center;
    gap: 5px;
    flex: none;
    transition: background-color 0.15s, color 0.15s;
  }
  .pild.tinta { background: var(--ink); color: var(--win); }
  .pild.borrar { background: transparent; color: var(--ink2); padding: 0 10px 0 8px; }
  .pild.borrar:hover { background: var(--hov); }
  .pild.borrar.seguro { background: var(--red); color: #fff; }
  .pild.blanca { height: 26px; padding: 0 12px; background: var(--win); box-shadow: 0 0 0 1px var(--line2); font-size: 12px; }
  .pild.clara { height: 26px; padding: 0 12px; background: transparent; color: var(--ink2); font-size: 12px; }
  .pild.clara:hover { background: var(--hov); color: var(--ink); }
  .aviso-carpeta {
    flex: none;
    margin: 0 24px 6px;
    padding: 10px 12px;
    border-radius: 10px;
    background: var(--chip);
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 12.5px;
    line-height: 1.45;
    color: var(--ink2);
  }
  .aviso-carpeta span { flex: 1; }

  .soltar { position: absolute; inset: 0; z-index: 9; padding: 14px; background: var(--win); animation: tf-fade 0.15s ease-out; }
  .marco { height: 100%; border: 1.5px dashed var(--line2); border-radius: 12px; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px; text-align: center; }
  .marco img { display: block; }
  .st { font-size: 17px; font-weight: 700; }
  .sx { font-size: 13.5px; color: var(--ink2); }

  .fantasma {
    position: fixed;
    z-index: 50;
    pointer-events: none;
    padding: 6px 10px;
    border-radius: 999px;
    background: var(--ink);
    color: var(--win);
    font: 600 12px var(--sans);
    white-space: nowrap;
  }

  .splash { position: absolute; inset: 0; z-index: 40; background: var(--win); display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 22px; }
  .splash img { display: block; animation: tf-key 0.55s cubic-bezier(0.3, 1.2, 0.5, 1) both; filter: drop-shadow(0 18px 30px rgba(0, 0, 0, 0.25)); }
  .splash .marca { font-size: 34px; animation: tf-fade 0.3s ease-out 0.45s both; }
</style>
