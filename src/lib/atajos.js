// Combinación de teclas de un evento, como "Ctrl+F" o "Space". null si solo
// se ha pulsado un modificador.
export function comboDe(e) {
  if (["Control", "Shift", "Alt", "Meta"].includes(e.key)) return null;
  const k = e.key === " " ? "Space" : e.key.length === 1 ? e.key.toUpperCase() : e.key;
  return [(e.ctrlKey || e.metaKey) && "Ctrl", e.shiftKey && "Shift", e.altKey && "Alt", k].filter(Boolean).join("+");
}
