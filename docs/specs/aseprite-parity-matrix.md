# Aseprite v1.3.x Feature Parity Matrix — PixelForge

**Baseline:** [Aseprite v1.3.17.x](https://www.aseprite.org/release-notes/) stable  
**Research sources:** [docs](https://www.aseprite.org/docs/), [gui.xml](https://github.com/aseprite/aseprite/blob/main/data/gui.xml), [API v40](https://www.aseprite.org/api/), [ASE spec](https://github.com/aseprite/aseprite/blob/main/docs/ase-file-specs.md)  
**Targets:** Web (WASM) + Android tablet (NDK) — identical requirements on both  
**Machine-readable:** [`aseprite-parity-matrix.json`](./aseprite-parity-matrix.json)

---

## Summary

| Section | Items | P0 | P1 |
|---------|------:|---:|---:|
| Drawing tools (24 + quicktools) | 27 | 27 | 0 |
| Ink modes | 7 | 7 | 0 |
| Brushes & dynamics | 7 | 7 | 0 |
| Sprite document model | 14 | 14 | 0 |
| Timeline & animation | 16 | 16 | 0 |
| Selection & transform | 9 | 9 | 0 |
| Color, palette, adjustments | 14 | 14 | 0 |
| Tilemap system (v1.3) | 12 | 12 | 0 |
| View, guides, helpers | 11 | 10 | 1 |
| File I/O & export | 12 | 12 | 0 |
| Slices & 9-patch | 5 | 5 | 0 |
| Menus & commands | 10 | 10 | 0 |
| Edit operations | 10 | 10 | 0 |
| Customization & extensibility | 12 | 10 | 2 |
| Platform & input | 8 | 8 | 0 |
| **Total (Aseprite parity)** | **174** | **171** | **3** |
| CLI batch flags | 11 | 10 | 1 |
| AI delta (separate track) | 10 | — | — |

**Pass criteria:** P0 rows must reach `pass` on **both** web and tablet before Phase 1 AI GA.  
**Test bar:** 50+ `.aseprite` fixtures; pixel-identical output vs Aseprite v1.3.17.x or documented exception.

### Status legend

| Symbol | Meaning |
|--------|---------|
| ☐ | Not started |
| ◐ | In progress |
| ☑ | Pass |
| ✗ | Fail (fix required) |
| ⚠ | Exception (documented delta + timeline) |
| — | N/A on this platform |

---

## 1. Drawing tools (24 + quicktools)

Source: Aseprite tool IDs in `gui.xml`. Each tool must match behavior, modifiers, and cursor.

| ID | Tool | Shortcut | Priority | Web | Tablet | Key behaviors |
|----|------|----------|----------|-----|--------|---------------|
| TOOL-01 | Rectangular Marquee | M | P0 | ☐ | ☐ | Replace / Add / Subtract / Intersect |
| TOOL-02 | Elliptical Marquee | Shift+M | P0 | ☐ | ☐ | Same selection modifiers as rectangular |
| TOOL-03 | Lasso | Q | P0 | ☐ | ☐ | Freehand selection |
| TOOL-04 | Polygonal Lasso | Shift+Q | P0 | ☐ | ☐ | Point-by-point selection |
| TOOL-05 | Magic Wand | W | P0 | ☐ | ☐ | Tolerance; refer active/visible layers; 4/8 connectivity |
| TOOL-06 | Pencil | B | P0 | ☐ | ☐ | Pixel-perfect; Shift straight-line preview |
| TOOL-07 | Spray | Shift+B | P0 | ☐ | ☐ | Spray density and size |
| TOOL-08 | Eraser | E | P0 | ☐ | ☐ | Simple; replace fg↔bg modes |
| TOOL-09 | Eyedropper | I / Alt | P0 | ☐ | ☐ | Fg/Bg pick; pick from desktop (v1.3) |
| TOOL-10 | Paint Bucket | G | P0 | ☐ | ☐ | Tolerance; contiguous; stop-at-grid; refer layer |
| TOOL-11 | Gradient | Shift+G | P0 | ☐ | ☐ | Linear/radial; dithering matrix |
| TOOL-12 | Line | L | P0 | ☐ | ☐ | — |
| TOOL-13 | Curve | Shift+L | P0 | ☐ | ☐ | Bézier |
| TOOL-14 | Rectangle / Filled | U | P0 | ☐ | ☐ | Optional fill |
| TOOL-15 | Ellipse / Filled | Shift+U | P0 | ☐ | ☐ | Improved ellipse algorithm |
| TOOL-16 | Contour | D | P0 | ☐ | ☐ | — |
| TOOL-17 | Polygon | Shift+D | P0 | ☐ | ☐ | — |
| TOOL-18 | Blur | Shift+R | P0 | ☐ | ☐ | Indexed palette snapping |
| TOOL-19 | Jumble | R | P0 | ☐ | ☐ | Pixel shuffle; no new colors |
| TOOL-20 | Text | T | P0 | ☐ | ☐ | Font load; hinting; antialias; fill/stroke; ligatures |
| TOOL-21 | Hand | H / Space | P0 | ☐ | ☐ | Pan canvas |
| TOOL-22 | Move | V / Ctrl | P0 | ☐ | ☐ | Move cel / selection |
| TOOL-23 | Slice | Shift+C | P0 | ☐ | ☐ | 9-slice; pivot; named regions |
| TOOL-24 | Zoom | Z | P0 | ☐ | ☐ | Integer zoom 50%–3200% |

### Quicktools (must not change active tool)

| ID | Feature | Shortcut | Web | Tablet |
|----|---------|----------|-----|--------|
| TOOL-QT-01 | Alt → Eyedropper | Alt | ☐ | ☐ |
| TOOL-QT-02 | Ctrl → Move | Ctrl | ☐ | ☐ |
| TOOL-QT-03 | Space → Hand | Space | ☐ | ☐ |

---

## 2. Ink modes (7)

| ID | Ink mode | Priority | Web | Tablet | Notes |
|----|----------|----------|-----|--------|-------|
| INK-01 | Simple Ink | P0 | ☐ | ☐ | Opaque / semi-transparent / mask eraser |
| INK-02 | Alpha Compositing | P0 | ☐ | ☐ | |
| INK-03 | Copy Alpha+Color | P0 | ☐ | ☐ | |
| INK-04 | Lock Alpha | P0 | ☐ | ☐ | |
| INK-05 | Shading Ink | P0 | ☐ | ☐ | L/R click along palette ramp |
| INK-06 | Gradient Ink | P0 | ☐ | ☐ | |
| INK-07 | Eraser variants | P0 | ☐ | ☐ | Replace fg↔bg |

---

## 3. Brushes and dynamics

| ID | Feature | Priority | Web | Tablet | Notes |
|----|---------|----------|-----|--------|-------|
| BRU-01 | Brush types: round, square, line | P0 | ☐ | ☐ | Line brush angle |
| BRU-02 | Custom saved brushes | P0 | ☐ | ☐ | Lock size, angle, colors, ink, opacity, pixel-perfect |
| BRU-03 | Pattern brushes | P0 | ☐ | ☐ | Source/dest/paint align; Ctrl+B new brush |
| BRU-04 | Flip/rotate while painting | P0 | ☐ | ☐ | Space+H/X/V/Y/D/R |
| BRU-05 | Dynamics: pressure + velocity | P0 | ☐ | ☐ | Min/max; threshold handles |
| BRU-06 | Stabilizer (mouse/pen) | P0 | ☐ | ☐ | v1.3 |
| BRU-07 | Pixel-perfect intertwine | P0 | ☐ | ☐ | Freehand tools |

---

## 4. Sprite document model

| ID | Feature | Priority | Web | Tablet | Notes |
|----|---------|----------|-----|--------|-------|
| DOC-01 | Canvas size W×H | P0 | ☐ | ☐ | |
| DOC-02 | Color mode RGB / Indexed / Grayscale | P0 | ☐ | ☐ | Indexed 256 |
| DOC-03 | Color profile sRGB | P0 | ☐ | ☐ | |
| DOC-04 | Pixel aspect ratio | P0 | ☐ | ☐ | |
| DOC-05 | Global user data on sprite | P0 | ☐ | ☐ | v1.3 |
| DOC-06 | Background layer rules | P0 | ☐ | ☐ | Single; opaque; immovable |
| DOC-07 | Layer groups | P0 | ☐ | ☐ | Collapse/expand |
| DOC-08 | Reference layers | P0 | ☐ | ☐ | From file or clipboard |
| DOC-09 | Tilemap layers | P0 | ☐ | ☐ | v1.3 flagship |
| DOC-10 | Cel = layer × frame + x/y | P0 | ☐ | ☐ | |
| DOC-11 | Cel z-index per frame | P0 | ☐ | ☐ | v1.3 |
| DOC-12 | Cel opacity (RGB) | P0 | ☐ | ☐ | |
| DOC-13 | Layer blend modes | P0 | ☐ | ☐ | |
| DOC-14 | Linked cels / continuous layers | P0 | ☐ | ☐ | Unlink support |

---

## 5. Timeline and animation

| ID | Feature | Priority | Web | Tablet | Notes |
|----|---------|----------|-----|--------|-------|
| TL-01 | Frame grid (layers × frames) | P0 | ☐ | ☐ | |
| TL-02 | Per-frame duration (ms) | P0 | ☐ | ☐ | |
| TL-03 | New / copy / remove / reverse frames | P0 | ☐ | ☐ | |
| TL-04 | Drag-drop move/copy cels and frames | P0 | ☐ | ☐ | |
| TL-05 | Tags: name, color, from/to | P0 | ☐ | ☐ | |
| TL-06 | Tag direction Fwd / Rev / Ping-pong | P0 | ☐ | ☐ | |
| TL-07 | Tag repeat count | P0 | ☐ | ☐ | v1.3 |
| TL-08 | Tag resize by dragging borders | P0 | ☐ | ☐ | |
| TL-09 | Subtags | P0 | ☐ | ☐ | |
| TL-10 | Playback Enter / Shift+Enter preview | P0 | ☐ | ☐ | Subtags+repeats default (v1.3-rc2) |
| TL-11 | Onion skin F3 | P0 | ☐ | ☐ | Prev/next count; tint; loop tag |
| TL-12 | Timeline thumbnails F6 | P0 | ☐ | ☐ | |
| TL-13 | Timeline selection customization | P0 | ☐ | ☐ | Keep-selection option |
| TL-14 | Copy/paste between open documents | P0 | ☐ | ☐ | Layers, frames, cels |
| TL-15 | Drop files onto timeline | P0 | ☐ | ☐ | Sprites, tilemaps |
| TL-16 | Frame goto Home/End , . Alt+G | P0 | ☐ | ☐ | Tag navigation |

---

## 6. Selection and transformation

| ID | Feature | Priority | Web | Tablet | Notes |
|----|---------|----------|-----|--------|-------|
| SEL-01 | Selection modes | P0 | ☐ | ☐ | Replace; Add (Shift); Sub (Alt+Shift); Intersect (Ctrl+Shift) |
| SEL-02 | Select All / Cel Content / Invert | P0 | ☐ | ☐ | Ctrl+T cel content |
| SEL-03 | Floating transform handles | P0 | ☐ | ☐ | Move, scale, rotate, **skew** (v1.3) |
| SEL-04 | RotSprite rotation | P0 | ☐ | ☐ | Force RotSprite for 90° option |
| SEL-05 | Pivot numeric control | P0 | ☐ | ☐ | v1.3 |
| SEL-06 | Snap to grid on move | P0 | ☐ | ☐ | Toggle (v1.3-rc9) |
| SEL-07 | Transform multi layer/frame/cel | P0 | ☐ | ☐ | |
| SEL-08 | Shift wrap pixels in selection | P0 | ☐ | ☐ | |
| SEL-09 | Esc cancel / Enter apply | P0 | ☐ | ☐ | Configurable keys (v1.3.15) |

---

## 7. Color, palette, and adjustments

| ID | Feature | Shortcut | Priority | Web | Tablet |
|----|---------|----------|----------|-----|--------|
| COL-01 | Color bar fg/bg; swap X | X | P0 | ☐ | ☐ |
| COL-02 | Palette editor | F4 | P0 | ☐ | ☐ |
| COL-03 | RGB/HSB popup | — | P0 | ☐ | ☐ |
| COL-04 | Replace Color | Shift+R | P0 | ☐ | ☐ |
| COL-05 | Brightness/Contrast | — | P0 | ☐ | ☐ |
| COL-06 | Hue/Saturation | Ctrl+U | P0 | ☐ | ☐ |
| COL-07 | Color Curve | Ctrl+M | P0 | ☐ | ☐ |
| COL-08 | Invert (per-channel RGBA) | — | P0 | ☐ | ☐ |
| COL-09 | FX Outline | Shift+O | P0 | ☐ | ☐ |
| COL-10 | FX Convolution Matrix | F9 | P0 | ☐ | ☐ |
| COL-11 | FX Despeckle | — | P0 | ☐ | ☐ |
| COL-12 | Color mode conversion + dither | — | P0 | ☐ | ☐ |
| COL-13 | Opacity range 0–255 vs 0–100% | — | P0 | ☐ | ☐ |
| COL-14 | Copy palette hex to clipboard | — | P0 | ☐ | ☐ |

---

## 8. Tilemap system (v1.3 — non-optional)

| ID | Feature | Shortcut | Priority | Web | Tablet |
|----|---------|----------|----------|-----|--------|
| MAP-01 | New Tilemap Layer | Space+N | P0 | ☐ | ☐ |
| MAP-02 | Convert layer to tilemap | — | P0 | ☐ | ☐ |
| MAP-03 | Draw Pixels vs Draw Tiles | Space+Tab | P0 | ☐ | ☐ |
| MAP-04 | Manual tileset mode | Space+1 | P0 | ☐ | ☐ |
| MAP-05 | Auto tileset mode | Space+2 | P0 | ☐ | ☐ |
| MAP-06 | Stack tileset mode | Space+3 | P0 | ☐ | ☐ |
| MAP-07 | Tileset indices (0 = empty) | — | P0 | ☐ | ☐ |
| MAP-08 | External tilesets | — | P0 | ☐ | ☐ |
| MAP-09 | Tile pick fg/bg (color bar) | — | P0 | ☐ | ☐ |
| MAP-10 | Brush on tilemaps | — | P0 | ☐ | ☐ |
| MAP-11 | Export tileset | — | P0 | ☐ | ☐ |
| MAP-12 | CLI `--export-tileset` `--split-grid` | — | P0 | ☐ | — |

---

## 9. View, guides, and helpers

| ID | Feature | Shortcut | Priority | Web | Tablet | Notes |
|----|---------|----------|----------|-----|--------|-------|
| VIEW-01 | Tiled Mode | View menu | P0 | ☐ | ☐ | Pattern repeat (not tilemaps) |
| VIEW-02 | Symmetry H/V | View menu | P0 | ☐ | ☐ | Draggable handles; mid-pixel |
| VIEW-03 | Grid | Ctrl+' | P0 | ☐ | ☐ | |
| VIEW-04 | Pixel Grid | Ctrl+Shift+' | P0 | ☐ | ☐ | |
| VIEW-05 | Snap to Grid | Shift+S | P0 | ☐ | ☐ | |
| VIEW-06 | Guides + auto guides | — | P0 | ☐ | ☐ | Layer edge; custom colors |
| VIEW-07 | Preview window | F7 | P0 | ☐ | ☐ | Shift+F7 hide other layers |
| VIEW-08 | Fullscreen preview / mode | F8 / F11 | P0 | ☐ | ☐ | |
| VIEW-09 | Brush preview | Pref | P0 | ☐ | ☐ | v1.3.12 |
| VIEW-10 | Extras/overlays | Ctrl+H | P0 | ☐ | ☐ | |
| VIEW-11 | Pick color from desktop | — | P1 | ⚠ | ⚠ | Limited on web/tablet; document delta |

---

## 10. File I/O and export

| ID | Feature | Shortcut | Priority | Web | Tablet | Notes |
|----|---------|----------|----------|-----|--------|-------|
| IO-01 | `.aseprite` / `.ase` read/write | — | P0 | ☐ | ☐ | Full ASE spec |
| IO-02 | Import PNG, GIF, BMP, WebP, JPEG, FLI | — | P0 | ☐ | ☐ | |
| IO-03 | Import image sequences | — | P0 | ☐ | ☐ | frame0.png, frame1.png… |
| IO-04 | Import sprite sheet | Ctrl+I | P0 | ☐ | ☐ | |
| IO-05 | Paste as new sprite | — | P0 | ☐ | ☐ | |
| IO-06 | Export As | Ctrl+Alt+Shift+S | P0 | ☐ | ☐ | GIF, PNG seq, resize, anim direction |
| IO-07 | Export Sprite Sheet | Ctrl+E | P0 | ☐ | ☐ | Packed; JSON hash/array; trim; extrude |
| IO-08 | Export tileset | — | P0 | ☐ | ☐ | |
| IO-09 | Export selection only | — | P0 | ☐ | ☐ | |
| IO-10 | Repeat last export | Ctrl+Shift+X | P0 | ☐ | ☐ | |
| IO-11 | Pinned/recent export folders | — | P0 | ☐ | ☐ | v1.3.15 |
| IO-12 | CLI `--batch` flag parity | — | P0 | — | — | Server API; see CLI table below |

### CLI batch flags (server-side parity)

| Flag | Required | Notes |
|------|----------|-------|
| `--sheet` | P0 | Sprite sheet output |
| `--data` | P0 | JSON metadata |
| `--split-tags` | P0 | Per-tag export |
| `--split-slices` | P0 | Per-slice export |
| `--split-grid` | P0 | Grid split |
| `--play-subtags` | P0 | Animation playback |
| `--list-layer-hierarchy` | P0 | Layer tree |
| `--export-tileset` | P0 | Tileset export |
| `--extrude` | P0 | Texture extrude |
| `--trim-by-grid` | P0 | Grid trim |
| `--script` | P1 | Lua script execution |

---

## 11. Slices and 9-patch

| ID | Feature | Priority | Web | Tablet | Notes |
|----|---------|----------|-----|--------|-------|
| SLC-01 | Named slice regions + bounds | P0 | ☐ | ☐ | |
| SLC-02 | 9-slice center rect + pivot | P0 | ☐ | ☐ | |
| SLC-03 | Export `--split-slices` | P0 | ☐ | — | Server CLI |
| SLC-04 | Slice metadata in sheet JSON | P0 | ☐ | ☐ | |
| SLC-05 | Slice tool on tilemap (manual) | P0 | ☐ | ☐ | v1.3.5 fix |

---

## 12. Menus and commands (~480 bindings)

All eight menus plus Run Command. Shortcut map derived from Aseprite `gui.xml`.

| ID | Menu / command | Priority | Web | Tablet |
|----|----------------|----------|-----|--------|
| MENU-01 | **File** — New, Open, Save/Save As/Save Copy, Import/Export, Close, Exit | P0 | ☐ | ☐ |
| MENU-02 | **Edit** — Undo/Redo/History, Clipboard, Fill/Stroke, Flip, Rotate, Transform, FX, Prefs | P0 | ☐ | ☐ |
| MENU-03 | **Sprite** — Properties, Color Mode, Canvas Size, Rotate Canvas, Crop, Trim | P0 | ☐ | ☐ |
| MENU-04 | **Layer** — New Layer/Group/Tilemap/Reference, Convert, Merge, Arrange | P0 | ☐ | ☐ |
| MENU-05 | **Frame** — New/Remove/Duplicate, Tags, Properties, Playback, Reverse | P0 | ☐ | ☐ |
| MENU-06 | **Select** — All, Color Range, Deselect, Reselect, Invert, Mask | P0 | ☐ | ☐ |
| MENU-07 | **View** — Zoom, Grid, Tiled, Symmetry, Onion, Preview, Timeline | P0 | ☐ | ☐ |
| MENU-08 | **Help** — Docs, Quick Reference, About | P0 | ☐ | ☐ |
| MENU-09 | **Run Command** Ctrl+Space | P0 | ☐ | ☐ | Search, math, inline Lua |
| MENU-10 | Full keyboard shortcut map (~480) | P0 | ☐ | ☐ | Rebindable |

---

## 13. Edit operations

| ID | Feature | Priority | Web | Tablet |
|----|---------|----------|-----|--------|
| EDT-01 | Undo / Redo | P0 | ☐ | ☐ |
| EDT-02 | Undo History panel | P0 | ☐ | ☐ |
| EDT-03 | Cut / Copy / Copy Merged / Paste | P0 | ☐ | ☐ |
| EDT-04 | Paste Special | P0 | ☐ | ☐ |
| EDT-05 | Clear / Fill / Stroke | P0 | ☐ | ☐ |
| EDT-06 | Flip horizontal / vertical | P0 | ☐ | ☐ |
| EDT-07 | Rotate 90° / 180° / custom | P0 | ☐ | ☐ |
| EDT-08 | Canvas size / crop / trim | P0 | ☐ | ☐ |
| EDT-09 | Layer merge / arrange | P0 | ☐ | ☐ |
| EDT-10 | Insert Text | P0 | ☐ | ☐ |

---

## 14. Customization and extensibility

| ID | Feature | Priority | Web | Tablet | Notes |
|----|---------|----------|-----|--------|-------|
| PRF-01 | Preferences: General | P0 | ☐ | ☐ | |
| PRF-02 | Preferences: Tablet | P0 | ☐ | ☐ | |
| PRF-03 | Preferences: Files, Color, Alerts, Editor | P0 | ☐ | ☐ | |
| PRF-04 | Preferences: Selection, Timeline, Cursors | P0 | ☐ | ☐ | |
| PRF-05 | Preferences: Grid, Guides, Undo, Theme | P0 | ☐ | ☐ | |
| PRF-06 | Keyboard rebind + action modifiers | P0 | ☐ | ☐ | |
| PRF-07 | Dockable workspace panels | P0 | ☐ | ☐ | |
| PRF-08 | Themes light/dark/custom | P0 | ☐ | ☐ | Font hinting v1.3.14 |
| PRF-09 | Session recovery + undo limit (MB) | P0 | ☐ | ☐ | |
| PRF-10 | Non-linear undo history | P0 | ☐ | ☐ | |
| PRF-11 | Lua scripting API v40 | P1 | ☐ | ☐ | Sandboxed WASM or server |
| PRF-12 | `.aseprite-extension` install | P1 | ☐ | ☐ | |

Preference sub-panels must match [Aseprite preferences.md](https://www.aseprite.org/docs/preferences/).

---

## 15. Platform and input (PixelForge targets)

| ID | Feature | Priority | Web | Tablet | Notes |
|----|---------|----------|-----|--------|-------|
| PLT-01 | Browser support Chrome/Edge/Firefox | P0 | ☐ | — | WASM core |
| PLT-02 | Stylus pressure + dynamics | P0 | — | ☐ | Advan `elan_pen` |
| PLT-03 | BT keyboard full shortcut map | P0 | — | ☐ | |
| PLT-04 | Touch long-press tool submenus | P0 | — | ☐ | Marquee variants, filled shapes |
| PLT-05 | Timeline touch drag | P0 | — | ☐ | |
| PLT-06 | Pinch zoom on preview | P0 | — | ☐ | |
| PLT-07 | Tablet-only gate (≥600dp) | P0 | — | ☐ | Block phones |
| PLT-08 | Offline edit + sync when online | P0 | ☐ | ☐ | |

### Advan Tab Sketsa 3 performance budget

| Metric | Target |
|--------|--------|
| Editor cold start (64×64) | &lt;2s |
| Pencil stroke latency | &lt;16ms |
| 3200% zoom pan | No UI freeze |
| Core RAM | &lt;4 GB (6 GB device) |

---

## 16. PixelForge AI delta (NOT Aseprite — separate track)

These extend Aseprite; tracked here for completeness but **do not count** toward Aseprite parity %.

| ID | Feature | Phase | Web | Tablet |
|----|---------|-------|-----|--------|
| AI-01 | Text → pixel generation | 1 | ☐ | ☐ |
| AI-02 | JPG/PNG/WebP → pixel art | 1 | ☐ | ☐ |
| AI-03 | Sketch → pixel (img2img) | 1 | ☐ | ☐ |
| AI-04 | Inpaint / region regen | 1 | ☐ | ☐ |
| AI-05 | Palette-locked output | 1 | ☐ | ☐ |
| AI-06 | Style bible + reference board | 1 | ☐ | ☐ |
| AI-07 | PC-offline AI banner | 1 | ☐ | ☐ |
| AI-08 | Fast quantize (no diffusion) | 1 | ☐ | ☐ |
| AI-09 | Batch convert | 2 | ☐ | ☐ |
| AI-10 | Generation history | 1 | ☐ | ☐ |

---

## Parity testing

### Harness

| Asset | Path |
|-------|------|
| Fixtures | `tests/parity/fixtures/*.aseprite` |
| Golden images | `tests/parity/golden/` |
| Runner | `tests/parity/run_parity.py` |
| Matrix source | `docs/specs/aseprite-parity-matrix.json` |
| Report | `tests/parity/output/report.json` |

### Commands

```bash
python tests/parity/run_parity.py --list
python tests/parity/run_parity.py --matrix docs/specs/aseprite-parity-matrix.json --target wasm
python tests/parity/run_parity.py --target wasm   # also writes matrix_report.json
python tests/parity/run_parity.py --matrix docs/specs/aseprite-parity-matrix.json --target ndk --device 1029P002505S070810
```

### Fixture categories (target 50+)

| Category | Min fixtures | Example IDs |
|----------|-------------|-------------|
| Round-trip ASE | 10 | RT-001…RT-010 |
| Per-tool ops | 24 | OP-TOOL-01…24 |
| Timeline/tags | 8 | TL-001…008 |
| Tilemaps | 6 | MAP-001…006 |
| Export/CLI | 6 | EX-001…006 |
| Indexed/RGB/Gray | 6 | CM-001…006 |

---

## Exception process

1. Open issue referencing matrix ID (e.g. `VIEW-11`)
2. Describe behavioral delta vs Aseprite v1.3.17.x with repro steps
3. Assign fix phase (P0 blocker vs P1 deferral)
4. Mark row `⚠` with link to issue — **no silent P0 failures**

---

## Related documents

- [Round-trip spec](./aseprite-roundtrip.md)
- [PRD](../PRD.md)
- [JSON matrix](./aseprite-parity-matrix.json)
