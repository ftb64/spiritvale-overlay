# SpiritVale Overlay — Handoff

## Current state

- Workspace: `C:\Users\hiimf\Documents\SpiritVale Overlay`
- GitHub: https://github.com/ftb64/spiritvale-overlay
- Current branch: `codex/v0.1.1-blank-window-fix`
- Latest local commit: `95f66a6 Fix blank overlay startup`
- `main` contains the initial preview from PR #1. PR #2 is still a **draft** and must be merged to bring the hotfix onto `main`: https://github.com/ftb64/spiritvale-overlay/pull/2
- GitHub prereleases:
  - v0.1.0 (broken; blank window): https://github.com/ftb64/spiritvale-overlay/releases/tag/v0.1.0
  - v0.1.1 (hotfix artifacts): https://github.com/ftb64/spiritvale-overlay/releases/tag/v0.1.1

## What exists

- Windows-first Tauri v2 + React + TypeScript desktop shell.
- Always-on-top, frameless window with `Ctrl+Alt+E` registered natively (the intended default was `Alt+E`; the UI label says `ALT+E`, so this discrepancy remains).
- Global fuzzy search over deliberately fictional demo records for monsters, bosses, cards, items, and maps.
- English default and an in-memory Thai UI toggle.
- Official source buttons, pin-state UI, and dark visual theme.
- Local build/release artifacts live in untracked `release/`; do not commit them. They contain installer, portable ZIP, and checksums for v0.1.0 and v0.1.1.

## Important hotfix

The v0.1.0 blank window was caused by `index.html` loading `src/main.tsx`, which exports `App` but never mounts React. Commit `95f66a6` fixes it by loading `src/bootstrap.tsx`, which calls `ReactDOM.createRoot(...)`. It also bumps `package.json`, `package-lock.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json` to `0.1.1`.

`npm run tauri -- build --bundles nsis` passed for v0.1.1. The packaged app was not launched for a visual runtime confirmation after the fix.

## Product decisions already made

- Scope: external, read-only SpiritVale companion; never inject into or inspect the game client.
- UI: hotkey-toggleable command palette, global search, compact details, optional pinned card intended to be click-through.
- Data: Official Catalog sourced from `https://spiritvale.info`, cached locally; update at launch and keep last verified data on failure. No current live adapter exists.
- Version 1 categories: monsters/bosses, items/cards, maps, source links.
- Platform: Windows only.
- Distribution: MIT code on GitHub, installer and portable ZIP releases. No accounts, cloud sync, or analytics.
- Language: English default, user-selectable Thai; preserve official item/stat names and avoid unlabelled machine translations.

## Known gaps / recommended next work

1. Merge PR #2, then begin new work from updated `main`.
2. Add a README with installation, v0.1.1 hotfix notice, preview limitations, data attribution, and contribution instructions.
3. Implement the real local settings model: persistent language and configurable global hotkey. Align default hotkey with the agreed `Alt+E`.
4. Implement actual pinned-window behavior and Windows click-through; current `Pin card` only toggles button text.
5. Add the official-data adapter responsibly. Inspect whether spiritvale.info offers a stable public endpoint first. If parsing pages, throttle, cache, preserve source URLs, and do not commit redistributed official catalog data without permission.
6. Add UI/runtime tests. The entry-point failure passed TypeScript/build validation because the unmounted component was valid; retain a test that asserts the HTML entry loads the React bootstrap and add a launch/smoke UI test.
7. Improve release process: build artifacts, checksum files, GitHub prerelease, and source PR. The GitHub CLI is installed at `C:\Program Files\GitHub CLI\gh.exe`; normal sandbox access may not see its keyring, so publishing commands previously required elevated execution.

## Suggested skills

- `diagnose` — use for any further blank/render/runtime failures; reproduce before changing code.
- `frontend-design` — use when improving the command palette, settings, and pinned-card UI.
- `github:yeet` — use for commit/push/PR/release work; it requires `gh` authentication and defaults to draft PRs.
- `context7-mcp` or `find-docs` — use for current Tauri v2 APIs, especially global shortcuts, window transparency, and click-through.
- `firecrawl-scrape` or `firecrawl-search` — only if needed to map the official site’s public data structure; respect terms and rate limits.
