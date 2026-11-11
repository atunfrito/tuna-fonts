<div align="center">

<img src="public/icon.svg" width="88" alt="">

# tuna-fonts

**A font manager for Windows 10 and 11.**
Your fonts, tidy and at hand. No accounts, no ads, nothing sent anywhere.

[English](#what-it-does) · [Español](#en-español)

</div>

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/list-dark.png">
  <img src="docs/list-light.png" alt="tuna-fonts showing the installed fonts as a list, each one previewed with the sample text">
</picture>

## What it does

- **See every font you have**, grouped by family, as a list or a grid. Each one is previewed with your own text, at the size you pick.
- **Activate and deactivate without deleting.** A deactivated font disappears from your apps but stays safe in tuna-fonts, ready to come back with one click.
- **Install by dropping** `.ttf`, `.otf` or `.ttc` files on the window — just for you (no admin rights) or for every user.
- **Discover** the whole Google Fonts catalog (about 1,900 families) and install any of them in one click.
- **Favorites and collections** to keep the fonts for each project together, and a selection bar to act on many at once.
- **Specimen** with the real character map of the font (click a glyph to copy it), and **compare** 2 to 4 fonts side by side.
- **Watched folders**: point it at a folder and its fonts are available while the app is open, without installing them.
- **Categories** worked out from the font itself: sans serif, serif, script, display, monospace and other scripts.
- Light and dark theme, Spanish and English, editable keyboard shortcuts.

### Type and see it everywhere

<img src="docs/sample-text.gif" alt="Typing a sample text and dragging the size slider; every font in the list updates live">

### Light or dark, list or grid

<img src="docs/theme-and-view.gif" alt="Switching between light and dark theme and between list and grid view">

### More screenshots

| | |
|---|---|
| <img src="docs/grid-dark.png" alt="Grid view in the dark theme"> | <img src="docs/specimen.png" alt="Specimen of Instrument Serif at several sizes"> |
| **Grid** — an "Aa" card for each family | **Specimen** — sizes, alphabet, numbers and every character |
| <img src="docs/compare.png" alt="Three fonts compared one under another"> | <img src="docs/discover.png" alt="Google Fonts catalog with an install button for each family"> |
| **Compare** — 2 to 4 fonts with the same text | **Discover** — Google Fonts, sorted by popularity, newest or A–Z |
| <img src="docs/selection.png" alt="Five fonts selected and the menu to add them to a collection"> | <img src="docs/settings.png" alt="Settings window"> |
| **Selection bar** — activate, favorite, collect or compare many at once | **Settings** — theme, where to install, system fonts, language |

## System fonts, safely

Fonts that come with Windows are **locked by default** and hidden from the list, so you can't break anything by accident. If you really want to, you can unlock them in settings:

- Changing a system font asks for admin rights (UAC) each time.
- Before removing one, tuna-fonts keeps a copy of the file and its registry entry, and **"restore all system fonts"** puts everything back.
- The fonts Windows itself needs (Segoe UI and friends) stay locked no matter what.

## Privacy

Everything happens on your computer. The only network traffic is to Google Fonts (`fonts.google.com` and `fonts.gstatic.com`), and only when you open Discover or install from it. There is no telemetry, no account and no auto-updater phoning home.

## Install

There are no prebuilt releases yet, so for now you build it yourself. You need [Node 22+](https://nodejs.org), [Rust](https://rustup.rs) stable and the WebView2 runtime (already included in Windows 11).

```bash
git clone https://github.com/atunfrito/tuna-fonts
cd tuna-fonts
npm install
npm run instalador
```

The installer ends up in `src-tauri/target/release/bundle/nsis/`. It installs for your user only and doesn't need admin rights.

Other commands:

```bash
npm run dev     # run in development, with hot reload
npm run build   # just tuna-fonts.exe, in src-tauri/target/release
cargo test --manifest-path src-tauri/Cargo.toml
```

The tests cover category detection and a full install → deactivate → activate cycle with a bundled test font and a throwaway registry key.

## How it's built

[Tauri 2](https://tauri.app) with a Rust backend and a [Svelte 5](https://svelte.dev) interface.

| Where | What |
|---|---|
| `src/` | Interface. Texts live in `src/i18n/es.js` and `en.js`. |
| `src-tauri/src/fuentes/leer.rs` | Reads the name, OS/2 and cmap tables straight from the font file (TTF, OTF and TTC). |
| `src-tauri/src/fuentes/categoria.rs` | Picks the category from panose, sFamilyClass, code pages and, as a last resort, the name. |
| `src-tauri/src/fuentes/cache.rs` | Caches what it has read, so opening the app is instant; new or changed files are read in parallel. |
| `src-tauri/src/fuentes/cambios.rs` | Activate, deactivate and install for the current user (files + `HKCU` registry). |
| `src-tauri/src/fuentes/sistema.rs` | The same for system fonts, run in an elevated copy of the app, with backups. |
| `src-tauri/src/google.rs` | Google Fonts catalog and downloads. |

App data (cache, deactivated fonts, system font backups) is kept in `%APPDATA%\es.atunfrito.tunafonts`.

## License

[MIT](LICENSE). The fonts bundled for the interface (Archivo, JetBrains Mono and Gochi Hand) are under the SIL Open Font License; see [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md).

---

## En español

**Gestor de fuentes para Windows 10 y 11.** Tus fuentes, ordenadas y a mano. Sin cuentas, sin anuncios y sin enviar nada a ningún sitio.

- **Todas tus fuentes**, agrupadas por familia, en lista o cuadrícula, con tu propio texto de prueba y al tamaño que quieras.
- **Activar y desactivar sin borrar.** Una fuente desactivada deja de salir en tus programas, pero se queda guardada en tuna-fonts y vuelve con un clic.
- **Instalar arrastrando** `.ttf`, `.otf` o `.ttc` a la ventana: solo para ti (sin permisos) o para todos los usuarios.
- **Descubrir**: el catálogo entero de Google Fonts, con instalación en un clic.
- **Favoritas y colecciones** para tener juntas las fuentes de cada proyecto, y una barra de selección para hacer cosas con varias a la vez.
- **Espécimen** con el mapa de caracteres real (clic en un carácter para copiarlo) y **comparar** de 2 a 4 fuentes.
- **Carpetas vigiladas**: sus fuentes están disponibles mientras la app está abierta, sin instalarlas.
- Tema claro y oscuro, español e inglés, y atajos de teclado editables.

Las fuentes del sistema vienen **bloqueadas y ocultas** por defecto. Se pueden desbloquear en ajustes: pide permisos cada vez, guarda una copia antes de quitar nada y hay un botón para restaurarlas todas. Las imprescindibles para Windows no se tocan nunca.

Todavía no hay versiones publicadas para descargar. Para compilarla hace falta Node 22+, Rust y WebView2:

```bash
npm install
npm run instalador
```

El instalador queda en `src-tauri/target/release/bundle/nsis/`.
