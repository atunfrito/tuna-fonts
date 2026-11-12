// Ayudas al pasar el ratón, con el estilo de la app en vez del recuadro nativo
// de Windows. Vale para cualquier elemento con `title`: al entrar se guarda el
// texto en data-ayuda (y se quita el title, para que no salga el nativo), y
// tras un momento aparece la ayuda propia debajo (o encima, si no cabe).

const RETRASO = 450;
const MARGEN = 8;

let caja = null;
let actual = null;
let temporizador = 0;

function crear() {
  caja = document.createElement("div");
  caja.className = "ayuda";
  caja.setAttribute("role", "tooltip");
  document.body.appendChild(caja);
}

function texto(el) {
  const t = el.getAttribute("title");
  if (t) {
    el.dataset.ayuda = t;
    el.removeAttribute("title");
    // Para lectores de pantalla, el texto sigue estando.
    if (!el.hasAttribute("aria-label") && !el.textContent.trim()) el.setAttribute("aria-label", t);
  }
  return el.dataset.ayuda ?? "";
}

function colocar(el) {
  const r = el.getBoundingClientRect();
  const c = caja.getBoundingClientRect();
  let x = r.left + r.width / 2 - c.width / 2;
  x = Math.max(MARGEN, Math.min(x, innerWidth - c.width - MARGEN));
  let y = r.bottom + 6;
  let arriba = false;
  if (y + c.height > innerHeight - MARGEN) {
    y = r.top - c.height - 6;
    arriba = true;
  }
  caja.style.transform = `translate(${Math.round(x)}px, ${Math.round(y)}px)`;
  caja.classList.toggle("arriba", arriba);
}

function ensenar(el) {
  const t = texto(el);
  if (!t) return;
  if (!caja) crear();
  caja.textContent = t;
  caja.classList.remove("vista");
  colocar(el);
  requestAnimationFrame(() => caja.classList.add("vista"));
}

function esconder() {
  clearTimeout(temporizador);
  actual = null;
  caja?.classList.remove("vista");
}

export function iniciarAyudas() {
  document.addEventListener("pointerover", (e) => {
    const el = e.target.closest?.("[title], [data-ayuda]");
    if (el === actual) return;
    esconder();
    if (!el) return;
    actual = el;
    texto(el);
    temporizador = setTimeout(() => actual === el && el.isConnected && ensenar(el), RETRASO);
  });
  document.addEventListener("pointerout", (e) => {
    if (actual && !actual.contains(e.relatedTarget)) esconder();
  });
  for (const ev of ["pointerdown", "keydown", "wheel", "scroll"]) document.addEventListener(ev, esconder, true);
  addEventListener("blur", esconder);
}
