<script>
  // Espécimen: escala de tamaños, juegos de caracteres, el mapa de caracteres
  // real de la fuente (tabla cmap; clic para copiar), los estilos y la ruta.
  import { ChevronLeft } from "@lucide/svelte";
  import Corazon from "../componentes/Corazon.svelte";
  import { estiloDe, caracteres } from "../lib/fuentes.svelte.js";
  import { avisar } from "../lib/avisos.svelte.js";
  import { t } from "../lib/i18n.svelte.js";

  let { f, muestra, fav, bloqueada, volver, onvolver, onalternar, onfav } = $props();

  const e = $derived(estiloDe(f));
  const otros = $derived(f.categoria === "otros");
  const meta = $derived(`${f.estilos.length} ${f.estilos.length === 1 ? t("estilo") : t("estilos")} · ${t("origen." + f.origen)}`);
  const juegos = $derived(
    otros
      ? [{ titulo: t("muestra.titulo"), chars: muestra }, { titulo: t("numeros.solo"), chars: "0123456789" }]
      : [
          { titulo: t("mayusculas"), chars: "ABCDEFGHIJKLMNÑOPQRSTUVWXYZ" },
          { titulo: t("minusculas"), chars: "abcdefghijklmnñopqrstuvwxyz" },
          { titulo: t("numeros"), chars: "0123456789 ¿?¡!@#€%&()«»" },
        ],
  );

  let glifos = $state([]);
  $effect(() => {
    const ruta = f.archivo;
    glifos = [];
    caracteres(ruta)
      .then((cs) => ruta === f.archivo && (glifos = cs.map((c) => String.fromCodePoint(c))))
      .catch(() => {});
  });

  async function copiar(c) {
    try {
      await navigator.clipboard.writeText(c);
    } catch {
      const a = document.createElement("textarea");
      a.value = c;
      document.body.appendChild(a);
      a.select();
      document.execCommand("copy");
      a.remove();
    }
    avisar("ok", t("ok.copiado", { c }), t("ok.pegalo"));
  }

</script>

<div class="esp">
  <button type="button" class="volver" onclick={onvolver}><ChevronLeft size={13} strokeWidth={2.2} />{volver}</button>

  <div class="cab">
    <div class="tit">
      <span class="meta">{meta}</span>
      <span class="nombre" style:font-family={e.familia} style:font-weight={e.peso} style:font-style={e.cursiva ? "italic" : "normal"}>{f.nombre}</span>
    </div>
    <button type="button" class="chip" onclick={onfav}>
      <span class="cor" class:fav><Corazon lleno={fav} tam={13} /></span>{fav ? t("fav.es") : t("fav.poner")}
    </button>
    {#if bloqueada}
      <span class="chip fijo">{t("alwaysOn")}</span>
    {:else}
      <button type="button" class="chip" class:principal={!f.activa} onclick={onalternar}>{f.activa ? t("deactivate") : t("activate")}</button>
    {/if}
  </div>

  <div class="escala">
    {#each [12, 18, 24, 36, 48, 72] as n (n)}
      <div class="paso">
        <span class="n num">{n}</span>
        <span class="linea" style:font-family={e.familia} style:font-weight={e.peso} style:font-style={e.cursiva ? "italic" : "normal"} style:font-size="{n}px">{muestra}</span>
      </div>
    {/each}
  </div>

  <div class="juegos">
    {#each juegos as j (j.titulo)}
      <div class="juego">
        <span class="etiqueta">{j.titulo}</span>
        <span class="chars" style:font-family={e.familia} style:font-weight={e.peso} style:font-style={e.cursiva ? "italic" : "normal"}>{j.chars}</span>
      </div>
    {/each}
  </div>

  {#if glifos.length}
    <div class="bloque">
      <span class="etiqueta">{t("allChars")} · {glifos.length}</span>
      <div class="glifos">
        {#each glifos as c, i (i)}
          <button type="button" class="glifo" title={t("copiar") + " U+" + c.codePointAt(0).toString(16).toUpperCase().padStart(4, "0")} style:font-family={e.familia} style:font-weight={e.peso} style:font-style={e.cursiva ? "italic" : "normal"} onclick={() => copiar(c)}>{c}</button>
        {/each}
      </div>
    </div>
  {/if}

  <div class="bloque">
    <span class="etiqueta">{t("estilos.titulo", { n: f.estilos.length })}</span>
    {#each f.estilos as s (s.nombre + s.peso + s.cursiva)}
      <div class="estilo">
        <span class="n">{s.nombre}</span>
        <span class="linea" style:font-family={estiloDe(f, s).familia} style:font-weight={estiloDe(f, s).peso} style:font-style={estiloDe(f, s).cursiva ? "italic" : "normal"} style:font-size="26px">{muestra}</span>
      </div>
    {/each}
  </div>

  <div class="ruta">
    {#each f.archivos as a (a)}<span>{a}</span>{/each}
  </div>
</div>

<style>
  .esp { flex: 1; min-height: 0; overflow-y: auto; padding: 4px 40px 70px 24px; display: flex; flex-direction: column; gap: 34px; animation: tf-up 0.25s ease-out; }
  .volver, .chip {
    height: 28px;
    padding: 0 12px 0 8px;
    border-radius: 999px;
    border: 0;
    background: var(--chip);
    color: var(--ink);
    font: 600 12.5px var(--sans);
    display: flex;
    align-items: center;
    gap: 4px;
    flex: none;
  }
  .volver { align-self: flex-start; }
  .chip { height: 30px; padding: 0 12px; gap: 6px; }
  .chip.principal { background: var(--ink); color: var(--win); padding: 0 14px; }
  .chip.fijo { color: var(--ink2); }
  .cor { display: flex; }
  .cor.fav { color: var(--red); }
  .cab { display: flex; align-items: flex-end; gap: 16px; }
  .tit { display: flex; flex-direction: column; gap: 6px; flex: 1; min-width: 0; }
  .meta { font-size: 12px; color: var(--ink3); }
  .nombre { font-size: 64px; line-height: 1.25; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .escala { display: flex; flex-direction: column; }
  .paso { display: grid; grid-template-columns: 40px minmax(0, 1fr); align-items: baseline; gap: 12px; padding: 4px 0; }
  .paso .n { font-size: 11px; color: var(--ink3); }
  .linea { line-height: 1.38; padding-bottom: 0.18em; margin-bottom: -0.18em; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .juegos { display: flex; flex-direction: column; gap: 18px; }
  .juego { display: flex; flex-direction: column; gap: 4px; }
  .chars { font-size: 30px; line-height: 1.3; letter-spacing: 0.02em; word-break: break-all; }
  .bloque { display: flex; flex-direction: column; gap: 8px; }
  .glifos { display: grid; grid-template-columns: repeat(auto-fill, minmax(52px, 1fr)); gap: 4px; }
  .glifo {
    height: 56px;
    border: 0;
    border-radius: 8px;
    background: var(--chip);
    color: var(--ink);
    font-size: 26px;
    line-height: 1;
    content-visibility: auto;
    contain-intrinsic-size: 52px 56px;
    transition: background-color 0.12s, color 0.12s, transform 0.12s;
  }
  .glifo:hover { background: var(--ink); color: var(--win); transform: scale(1.06); }
  .estilo { display: flex; flex-direction: column; gap: 1px; padding: 6px 0; }
  .estilo .n { font-size: 10.5px; color: var(--ink3); }
  .ruta { display: flex; flex-direction: column; gap: 4px; padding-top: 18px; border-top: 1px solid var(--line); font: 400 12px var(--mono); color: var(--ink2); user-select: text; }
</style>
