// Encaja un texto corto (el «Aa» de las tarjetas) en su caja: mide la tinta real
// de los glifos con canvas (no la caja de la línea, que en las caligráficas se
// queda corta), lo reduce si se pasa y lo centra por la tinta, con margen.
//
// Uso: <span use:encajar={{ texto, familia, peso, cursiva, base: 64, margen: 14 }}>

const lienzo = document.createElement("canvas").getContext("2d");
const REF = 100;

function medir(texto, fuente) {
  lienzo.font = fuente;
  const m = lienzo.measureText(texto);
  return {
    ancho: m.width,
    izq: m.actualBoundingBoxLeft,
    der: m.actualBoundingBoxRight,
    sube: m.actualBoundingBoxAscent,
    baja: m.actualBoundingBoxDescent,
    fSube: m.fontBoundingBoxAscent,
    fBaja: m.fontBoundingBoxDescent,
  };
}

export function encajar(nodo, opciones) {
  let o = opciones;
  let vivo = true;

  async function aplicar() {
    const { texto, familia, peso = 400, cursiva = false, base = 64, margen = 14 } = o;
    const caja = nodo.parentElement;
    if (!caja) return;
    // canvas no entiende var(--…): se cambia por una genérica.
    const pila = familia.replace(/var\([^)]*\)/g, "sans-serif");
    const fuente = `${cursiva ? "italic " : ""}${peso} ${REF}px ${pila}`;
    try {
      await document.fonts.load(fuente, texto);
    } catch {
      // Sin cargar todavía: se mide con lo que haya y se repite al llegar.
    }
    if (!vivo) return;
    const m = medir(texto, fuente);
    const W = caja.clientWidth - margen * 2;
    const H = caja.clientHeight - margen * 2;
    const tintaAncho = m.izq + m.der;
    const tintaAlto = m.sube + m.baja;
    if (!(W > 0 && H > 0 && tintaAncho > 0 && tintaAlto > 0)) return;

    const s = Math.min(base, (W / tintaAncho) * REF, (H / tintaAlto) * REF);
    const k = s / REF;
    // Horizontal: del centro de la caja de avance al centro de la tinta.
    const dx = (m.ancho / 2 - (m.der - m.izq) / 2) * k;
    // Vertical (line-height: 1): de la línea base que pone el navegador al centro de la tinta.
    const dy = (-(m.fSube - m.fBaja) / 2 - (m.baja - m.sube) / 2) * k;
    nodo.style.fontSize = s.toFixed(1) + "px";
    nodo.style.transform = `translate(${dx.toFixed(1)}px, ${dy.toFixed(1)}px)`;
  }

  const alCargar = () => aplicar();
  document.fonts.addEventListener("loadingdone", alCargar);
  aplicar();

  return {
    update(nuevas) {
      o = nuevas;
      aplicar();
    },
    destroy() {
      vivo = false;
      document.fonts.removeEventListener("loadingdone", alCargar);
    },
  };
}
