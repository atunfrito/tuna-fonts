//! Llamadas a Windows: cargar y descargar fuentes en la sesión, avisar a los
//! programas de que han cambiado, mover archivos al reiniciar y relanzarse con
//! permisos de administrador (UAC).

use std::{ffi::OsStr, os::windows::ffi::OsStrExt, path::Path};
use windows_sys::Win32::{
    Foundation::{CloseHandle, WAIT_OBJECT_0},
    Graphics::Gdi::{AddFontResourceExW, RemoveFontResourceExW, FR_PRIVATE},
    Storage::FileSystem::{MoveFileExW, MOVEFILE_DELAY_UNTIL_REBOOT},
    System::Threading::{GetExitCodeProcess, WaitForSingleObject, INFINITE},
    UI::{
        Shell::{ShellExecuteExW, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW},
        WindowsAndMessaging::{SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, SW_HIDE, WM_FONTCHANGE},
    },
};

/// Ruta de una carpeta conocida (FOLDERID_*), pedida a Windows en vez de
/// montarla a mano con variables de entorno. None si Windows no la da.
pub fn carpeta_conocida(id: &windows_sys::core::GUID) -> Option<std::path::PathBuf> {
    use std::os::windows::ffi::OsStringExt;
    use windows_sys::Win32::{System::Com::CoTaskMemFree, UI::Shell::SHGetKnownFolderPath};
    let mut ptr: windows_sys::core::PWSTR = std::ptr::null_mut();
    // SAFETY: SHGetKnownFolderPath reserva la cadena; se copia y se libera
    // siempre con CoTaskMemFree, también si falla (puede venir no nula).
    unsafe {
        let hr = SHGetKnownFolderPath(id, 0, std::ptr::null_mut(), &mut ptr);
        let ruta = if hr == 0 && !ptr.is_null() {
            let mut n = 0;
            while *ptr.add(n) != 0 {
                n += 1;
            }
            Some(std::ffi::OsString::from_wide(std::slice::from_raw_parts(ptr, n)).into())
        } else {
            None
        };
        CoTaskMemFree(ptr as *const _);
        ruta
    }
}

pub fn ancho(s: impl AsRef<OsStr>) -> Vec<u16> {
    s.as_ref().encode_wide().chain(Some(0)).collect()
}

/// Carga la fuente en la sesión. `privada`: solo para este proceso.
pub fn cargar(ruta: &Path, privada: bool) -> bool {
    let w = ancho(ruta);
    unsafe { AddFontResourceExW(w.as_ptr(), if privada { FR_PRIVATE } else { 0 }, std::ptr::null()) > 0 }
}

/// La descarga de la sesión. Windows cuenta las cargas: se repite hasta que no quede ninguna.
pub fn descargar(ruta: &Path, privada: bool) {
    let w = ancho(ruta);
    for _ in 0..16 {
        if unsafe { RemoveFontResourceExW(w.as_ptr(), if privada { FR_PRIVATE } else { 0 }, std::ptr::null()) } == 0 {
            break;
        }
    }
}

/// WM_FONTCHANGE a todas las ventanas. Con tiempo límite: un programa colgado
/// no deja colgada a tuna-fonts.
pub fn avisar_cambio() {
    unsafe {
        SendMessageTimeoutW(HWND_BROADCAST, WM_FONTCHANGE, 0, 0, SMTO_ABORTIFHUNG, 1000, std::ptr::null_mut());
    }
}

/// Programa borrar (o mover) un archivo en el próximo arranque, para cuando Windows lo tiene en uso.
pub fn al_reiniciar(origen: &Path, destino: Option<&Path>) -> bool {
    let o = ancho(origen);
    let d = destino.map(ancho);
    unsafe { MoveFileExW(o.as_ptr(), d.as_ref().map_or(std::ptr::null(), |d| d.as_ptr()), MOVEFILE_DELAY_UNTIL_REBOOT) != 0 }
}

/// Lanza este mismo .exe con permisos de administrador y espera a que acabe.
/// Devuelve su código de salida, o None si el usuario dijo que no al UAC.
pub fn como_admin(args: &str) -> Option<u32> {
    let exe = std::env::current_exe().ok()?;
    let verbo = ancho("runas");
    let archivo = ancho(&exe);
    let parametros = ancho(args);
    unsafe {
        let mut info: SHELLEXECUTEINFOW = std::mem::zeroed();
        info.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
        info.fMask = SEE_MASK_NOCLOSEPROCESS;
        info.lpVerb = verbo.as_ptr();
        info.lpFile = archivo.as_ptr();
        info.lpParameters = parametros.as_ptr();
        info.nShow = SW_HIDE;
        if ShellExecuteExW(&mut info) == 0 || info.hProcess.is_null() {
            return None;
        }
        let mut codigo = 1u32;
        if WaitForSingleObject(info.hProcess, INFINITE) == WAIT_OBJECT_0 {
            GetExitCodeProcess(info.hProcess, &mut codigo);
        }
        CloseHandle(info.hProcess);
        Some(codigo)
    }
}
