// Arrastrar fuentes a una colección de la barra lateral. Va con eventos de
// puntero y no con el arrastre del navegador: así nunca sale la capa de
// «suéltalas» (que es solo para archivos de fuera) al arrastrar algo de dentro.
export const arrastre = $state({ activo: false, x: 0, y: 0, nombres: [], sobre: null });

/**
 * e: el pointerdown. nombres(): qué se arrastra (la fila o toda la selección).
 * alSoltar(idColeccion, nombres).
 */
export function empezarArrastre(e, nombres, alSoltar) {
  if (e.button !== 0 || e.target.closest("button, input")) return;
  const x0 = e.clientX;
  const y0 = e.clientY;
  let vivo = false;

  const mover = (ev) => {
    if (!vivo && Math.hypot(ev.clientX - x0, ev.clientY - y0) > 6) {
      vivo = true;
      arrastre.nombres = nombres();
      arrastre.activo = true;
      document.body.style.cursor = "grabbing";
    }
    if (!vivo) return;
    arrastre.x = ev.clientX;
    arrastre.y = ev.clientY;
    const el = document.elementFromPoint(ev.clientX, ev.clientY)?.closest("[data-coleccion]");
    arrastre.sobre = el?.dataset.coleccion ?? null;
  };
  const fin = () => {
    removeEventListener("pointermove", mover);
    removeEventListener("pointerup", fin);
    removeEventListener("pointercancel", fin);
    if (!vivo) return;
    document.body.style.cursor = "";
    const id = arrastre.sobre;
    const n = arrastre.nombres;
    arrastre.activo = false;
    arrastre.sobre = null;
    if (id) alSoltar(id, n);
    // El clic que viene detrás del arrastre no debe (de)seleccionar la fila.
    const parar = (ev) => ev.stopPropagation();
    addEventListener("click", parar, { capture: true, once: true });
    setTimeout(() => removeEventListener("click", parar, { capture: true }), 0);
  };
  addEventListener("pointermove", mover);
  addEventListener("pointerup", fin);
  addEventListener("pointercancel", fin);
}
