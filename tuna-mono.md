# tuna-mono — sistema visual de apps (AtunFrito)

Tercer estilo de la familia, junto a **tuna-ink** (escritorio de noche, terminal) y **tuna-paper** (cuaderno, scrapbook). tuna-mono es el de **apps de escritorio** donde el contenido manda: minimal, neutro, tranquilo, con un único acento rojo y un guiño a mano en la marca. Referencia: `tuna-fonts v2.dc.html`.

Idea: la interfaz se quita de en medio. Grises cálidos, mucho aire, sin bordes duros, sin papel ni celo. Lo único con personalidad es el nombre escrito a mano y el punto rojo.

## Color
Claro
- ventana `#f7f6f3` · barra lateral `#efede9` · campos `#ffffff`
- tinta `#1b1a19` · secundaria `rgba(27,26,25,0.62)` · terciaria `rgba(27,26,25,0.42)`
- línea `rgba(27,26,25,0.08)` · línea fuerte `rgba(27,26,25,0.18)`
- hover `rgba(27,26,25,0.035)` · pastilla/chip `rgba(27,26,25,0.06)`
- seleccionado `rgba(200,16,46,0.055)` · apagado/inactivo `rgba(27,26,25,0.22)`
- acento ÚNICO `#c8102e`

Oscuro
- ventana `#191816` · barra lateral `#1f1e1b` · campos `#232220`
- tinta `#eeece8` · secundaria `rgba(238,236,232,0.62)` · terciaria `rgba(238,236,232,0.4)`
- línea `rgba(238,236,232,0.07)` · línea fuerte `rgba(238,236,232,0.16)`
- hover `rgba(238,236,232,0.04)` · chip `rgba(238,236,232,0.08)`
- seleccionado `rgba(229,56,74,0.08)` · apagado `rgba(238,236,232,0.2)`
- acento `#e5384a`

Estados auxiliares: ok `#2f7a4c` · aviso `#b7791f` · error = acento.
El rojo solo para: el punto/letra del logotipo, contadores de selección, avisos de error. Nunca en botones normales ni en decoración.

## Tipografía
- **Archivo** (400–700) → toda la interfaz. 13px cuerpo, 12–12.5px secundario, 10.5px etiquetas en MAYÚSCULAS con tracking 0.1em y peso 600.
- **Gochi Hand** → SOLO el nombre de la app (`tuna-fonts.` con el punto rojo) y notas tipo «hecho en la cueva».
- **JetBrains Mono** → rutas, versiones, datos técnicos.
- Títulos de panel: Archivo 18px, peso 700, tracking -0.02em.
- Números en contadores con `font-variant-numeric: tabular-nums`.

## Formas
- Radios: ventana 12px · tarjetas/modales 12–14px · campos y menús 8px · items de lista 6px · **pastillas y selectores 999px**.
- Sombras suaves: elegido `0 1px 2px rgba(0,0,0,0.12)` · flotantes `0 12px 30px rgba(0,0,0,0.2), 0 0 0 1px var(--line2)` · ventana `0 20px 50px rgba(0,0,0,0.12)`.
- Sin bordes gruesos: separación con líneas de 1px o con cambio de fondo.

## Componentes clave
- **Selector segmentado** (tema, vista, filtros, todos los ajustes): fondo chip, padding 3px, radio 999px; opciones de igual ancho; la elegida es una pastilla blanca (`--win`) con sombra suave que **se desliza** (`left .26s cubic-bezier(.3,1.25,.5,1)`). Sin aro de foco ni subrayado al pulsar con ratón.
- **Tema claro/oscuro**: segmentado con sol y luna de trazo fino (stroke 2), nunca interruptor con bolita.
- **Lista vs cuadrícula**: segmentado de iconos, mismo estilo.
- **Botón**: 28–30px alto, radio 999px. Principal = tinta sobre fondo (`--ink` / `--win`). Secundario = chip.
- **Item de barra lateral**: 28px, radio 6px, activo con fondo chip y peso 600, contador a la derecha en terciaria.
- **Barra de selección flotante**: oscura (tinta), pastilla, abajo centrada.
- **Toast**: arriba o abajo a la derecha, con punto de color de estado.
- **Ajustes**: modal con pestañas laterales (ajustes · atajos · acerca de), sin scroll si se puede; scrollbar fina (`scrollbar-width: thin`).
- **Atajos editables**: pulsas el atajo, «pulsa la combinación…», Esc cancela, «volver a los de siempre».

## Movimiento
- Rápido y suave: 0.15–0.28s. Hovers de fondo a 0.15s. Pastillas que se deslizan con un rebote mínimo.
- Respetar siempre `prefers-reduced-motion`.

## Voz
Minúsculas, frases cortas, cercano, sin marketing. «nada con "comic sans"», «pulsa el corazón de cualquier fuente y la tendrás aquí.»

## Reglas para la IA
- No Fluent UI ni aspecto nativo de Windows. Ventana sin bordes con barra propia.
- Nada de emojis. Iconos de trazo fino (Lucide, stroke 1.75–2).
- Un solo acento rojo. No inventar colores.
- Todos los selectores iguales (segmentado deslizante).
- La interfaz nunca compite con el contenido.
- Tema claro y oscuro siempre. Español e inglés.
