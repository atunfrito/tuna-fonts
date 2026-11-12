// Actualizar la propia app con tauri-plugin-updater. Solo se instalan paquetes
// firmados con la clave cuya parte pública está en tauri.conf.json: si la firma
// no cuadra, el plugin no instala nada.
//
// La única consulta es a GitHub (latest.json de la última release) y solo si
// «buscar actualizaciones al abrir» está activado, o al pulsar «buscar ahora».
import { ajustes, enTauri } from "./ajustes.svelte.js";

// estado: idle | buscando | nada | available | downloading | installing | error
export const nueva = $state({ estado: "idle", version: "", bajado: 0, total: 0, error: "", portable: false, cerrado: false });

let pendiente = null;

export async function buscarApp(aMano = false) {
  if (!enTauri) return;
  const { invoke } = await import("@tauri-apps/api/core");
  const { check } = await import("@tauri-apps/plugin-updater");
  if (aMano) nueva.estado = "buscando";
  try {
    nueva.portable = await invoke("es_portable");
    pendiente = await check();
    if (pendiente) {
      nueva.version = pendiente.version;
      nueva.estado = "available";
      nueva.cerrado = false;
    } else if (aMano) nueva.estado = "nada";
  } catch (e) {
    // Sin internet, o la release aún no trae latest.json: al abrir no se molesta a nadie.
    if (aMano) {
      nueva.error = String(e);
      nueva.estado = "error";
    } else nueva.estado = "idle";
  }
}

export async function instalarApp() {
  const { invoke } = await import("@tauri-apps/api/core");
  // La portable no tiene instalador al lado: se baja a mano de la release.
  if (nueva.portable) return invoke("abrir_web", { cual: "release" });
  if (!pendiente) return;
  const { relaunch } = await import("@tauri-apps/plugin-process");
  nueva.estado = "downloading";
  nueva.bajado = 0;
  nueva.total = 0;
  try {
    await pendiente.downloadAndInstall((e) => {
      if (e.event === "Started") nueva.total = e.data.contentLength ?? 0;
      else if (e.event === "Progress") nueva.bajado += e.data.chunkLength;
      else if (e.event === "Finished") nueva.estado = "installing";
    });
    await relaunch();
  } catch (e) {
    nueva.error = String(e);
    nueva.estado = "error";
  }
}

export function iniciarActualizador() {
  if (enTauri && ajustes.buscarActualizaciones) {
    // Un poco después de abrir, para no competir con la lista de fuentes.
    setTimeout(() => buscarApp(false), 2500);
  }
}
