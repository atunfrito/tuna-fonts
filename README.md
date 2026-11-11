# tuna-fonts

A font manager for Windows 10 and 11. Everything runs locally: no accounts, no ads, no telemetry.
Part of the AtunFrito family (tuna-dlp, tuna-send).

- **Installed fonts** from `C:\Windows\Fonts`, your user fonts folder and the registry, grouped by family, in a list or a grid.
- **Categories**: sans, serif, script, display, monospace and other scripts, read from the OS/2 table (panose and sFamilyClass) and, when that is not enough, from the name.
- **Activate and deactivate without deleting**. User fonts move to the app's folder and leave the registry; activating puts them back. System fonts are locked by default; you can unlock them in settings (asks for admin rights, keeps a backup, never touches the fonts Windows needs). "Restore all system fonts" is in settings.
- **Install** by dropping `.ttf`, `.otf` or `.ttc` on the window, just for you (no admin) or for everyone.
- **Watched folders**: fonts in them are available while the app is open, without installing them.
- **Discover**: the Google Fonts catalog, with previews that only load what is on screen, and one-click install.
- **Favorites and collections**, multi-select bar, compare 2 to 4 fonts, specimen with the real character map (click to copy).
- **Editable shortcuts**, light and dark theme, Spanish and English.

The only network traffic is Google Fonts (fonts.google.com and fonts.gstatic.com) when you use Discover or install from it.

## Build

Needs Node 22+, Rust stable and the WebView2 runtime (already in Windows 11).

```bash
npm install
npm run dev          # run in development
npm run build        # tuna-fonts.exe in src-tauri/target/release
npm run instalador   # NSIS installer in src-tauri/target/release/bundle/nsis
```

Tests (category detection and activating/deactivating a test user font with a throwaway registry key):

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

## Where things are

- `src/` — interface (Svelte 5). Texts in `src/i18n/es.js` and `en.js`.
- `src-tauri/src/fuentes/` — reading fonts (`leer.rs`), categories (`categoria.rs`), cache (`cache.rs`), registry (`registro.rs`), activate/deactivate/install (`cambios.rs`) and system fonts with admin rights (`sistema.rs`).
- `src-tauri/src/google.rs` — Google Fonts catalog and downloads.
- App data (cache, deactivated fonts, system backup): `%APPDATA%\es.atunfrito.tunafonts`.

## License

MIT. See `THIRD-PARTY-NOTICES.md` for bundled fonts and libraries.

---

## En español

Gestor de fuentes para Windows 10 y 11. Todo en local: sin cuentas, sin anuncios y sin telemetría.

- Lista o cuadrícula de las fuentes instaladas, agrupadas por familia y por categoría.
- Activar y desactivar sin borrar nada. Las del sistema llevan candado; se pueden desbloquear en ajustes (pide permisos, hace copia de respaldo y nunca toca las imprescindibles para Windows).
- Instalar arrastrando `.ttf`, `.otf` o `.ttc` a la ventana: solo para ti (sin permisos) o para todos.
- Carpetas vigiladas: sus fuentes están disponibles mientras la app está abierta, sin instalarlas.
- Descubrir: catálogo de Google Fonts con instalación en un clic.
- Favoritas, colecciones, comparar de 2 a 4, espécimen con el mapa de caracteres real y atajos editables.

Para compilar: `npm install` y luego `npm run instalador` (instalador) o `npm run build` (solo el .exe).
Los datos de la app están en `%APPDATA%\es.atunfrito.tunafonts`.
