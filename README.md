# SpiritVale Overlay

A Windows desktop companion for browsing the **SpiritVale** database without leaving your game setup. It is a read-only reference app: it does not inject into, read from, or modify the game client.

## Download

Get the current stable build from the [v1.0.0 release](https://github.com/ftb64/spiritvale-overlay/releases/tag/v1.0.0).

- **Windows installer** — recommended for most players.
- **Portable EXE** — run directly, with no installation.
- **Portable ZIP** — portable EXE packaged as a ZIP download.
- **SHA-256 checksums** — verify downloaded files if needed.

## Features

- Live catalog sync with a local verified-cache fallback.
- Search across names, stats, drops, crafting, and item details.
- Catalogs for Artifacts, Cards, Consumables, Gems, Maps, Materials, Monsters, Equipment, and Skills.
- Type filters for Cards, Gems, and Equipment.
- Item detail pages with images, stats, sources, and optional pinned selection.
- Four built-in color themes plus a custom theme editor.
- Adjustable window resolutions, animation controls, and English/Thai interface toggle.

## Controls

| Control | Action |
| --- | --- |
| `Alt + E` | Minimize or restore the overlay |
| `−` | Minimize the window |
| `×` | Close the overlay |
| Settings | Change theme, resolution, animation, and refresh data |

You can drag the top header to move the window. Selecting an item returns its details view to the top.

## Data and attribution

Catalog information and source links are synced from [SpiritVale.info](https://www.spiritvale.info/). SpiritVale Overlay caches the last verified response locally so the reference remains available when a sync cannot finish.

## Development

Requirements: Node.js, Rust, and the Windows prerequisites for [Tauri v2](https://v2.tauri.app/start/prerequisites/).

```bash
npm install
npm run tauri dev
```

Build a production installer:

```bash
npm run tauri build
```

## License

MIT. See [LICENSE](LICENSE).