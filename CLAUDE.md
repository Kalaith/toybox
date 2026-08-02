# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

**Toybox After Hours: Closing Shift** — a first-person 3D toy-store cleanup game (Rust + macroquad + `macroquad-toolkit`). The player picks up scattered toys and returns them to matching displays before opening; includes a one-toy carry limit, mistakes/timer, a repair bench (broken toys split into head/body parts), and a tool shop paid with credits from completed displays.

Workspace-wide rules (800-line file limit, no `mod.rs`, toolkit-first, data-driven JSON, doc-sync scripts) live in `../CLAUDE.md` and the synced `AGENTS.md`/`CODE_STANDARDS.md` here — this file only covers what's specific to toybox.

## Commands

```powershell
cargo test                    # main suite is src/state/tests.rs (session/interaction logic)
cargo test <name>             # single test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt -- --check
.\publish.ps1                 # end-to-end validation: Windows + WebGL build, deploy
```

## Architecture

Single-loop macroquad game; no state-machine enum of separate state structs. Instead `game.rs` owns one `GameSession` (the entire simulation) plus a `GameScreen` enum (`Title | Settings | ToolShop | Playing`) that only selects which screen draws and which inputs are read.

**Action flow (the core pattern):** keyboard input in `Game::update` and UI screens both emit `UiAction` variants into a toolkit `EventBus<UiAction>`; `Game::apply_action` is the single dispatcher that mutates `GameSession` and turns the typed results (`InteractionResult`, `ToolPurchaseResult`) into toolkit notifications. UI code never mutates state — to add an interaction, add a `UiAction` variant, emit it, and handle it in `apply_action`.

- `data.rs` — `GameData` loaded from JSON embedded via `include_str!` from `assets/data/` (`game_config.json`, `displays.json`, `upgrades.json`, `texture_manifest.json`). Balance/content changes go in these JSON files, not Rust constants. **Invariant (test-enforced):** the sum of display `capacity` values must equal `game_config.toy_count`.
- `state.rs` + `state/{interactions,repair}.rs` — `GameSession`: player pose, toys, displays, phase; pure logic, no rendering. `interactions.rs` resolves what `E`/`Space` does based on what the player carries and looks at (look-targeted shelf slots via dot-product thresholds); `repair.rs` handles the bench where two parts sharing a `repair_id` are rejoined. `InteractionPreview` (a parallel enum) drives the HUD prompt text without mutating anything.
- `toys.rs` + `toys/library.rs` + ~25 per-identity modules — toys are generated **deterministically** from display definitions (no runtime RNG); `library.rs` maps category + slot to a `ToyIdentity`, and each identity has its own module drawing the 3D primitives. Adding a toy = new identity module + `ToyIdentity` variant + dispatch arm in `toys.rs`.
- `ui.rs` + `ui/` — pure view layer returning `Vec<UiAction>`. `scene3d.rs` renders the 3D room (fixtures, signs, environment, procedural wood textures); `hud.rs` draws the 2D overlay in logical 1280×720 coordinates managed by `space.rs` (`begin_ui_frame`/`set_ui_camera`/`end_ui_frame`).

## Save/load and assets

- Saves use toolkit slot persistence with version migration: `migrate_save_value` in `state.rs` upgrades old save JSON. New fields on `ToyState`/`PlayerState`/`SaveData` need `#[serde(default)]` (or a migration step) so existing saves keep loading.
- `asset_packs.json` makes the publish pipeline zip `assets/` into `assets.zip`; the game loads via `AssetPack` with a loose-file fallback, so `cargo run` from source works without the pack.
- `index.html` + `storage.js` are the WebGL shell — `storage.js` is the localStorage bridge for toolkit persistence in WASM builds. `dist/` holds built artifacts.
- Screenshot harness (`TOYBOX_CAPTURE_*`): scenes `toy_gallery` (one toy from front/right/back/left, selected via `TOYBOX_CAPTURE_TOY=<module slug>`), `gameplay`, `repair_bench` (a bench holding one half of a broken toy, for the status beacon and the awaiting-match prompt), and `mid_run` (one aisle shelved, another half done, for the HUD/minimap completion readouts — a fresh shop reads 0% everywhere and shows nothing). `.\scripts\capture_toys.ps1` sweeps every toy module into `docs\verification\toys\<toy>.png` (`-Toys a,b` to filter, `-SkipBuild` to reuse the build).
- `.\scripts\capture_scene.ps1` captures the whole-store scenes into `docs\verification\ui_<scene>.png`. It is a thin wrapper over the shared `..\macroquad-toolkit\scripts\capture_ui.ps1`, passing `-Release` (a debug capture at ~4500 toys takes minutes and in practice never finishes) and `-Prefix TOYBOX` (the shared script would otherwise derive `TOYBOX_AFTER_HOURS` from the package name, the capture vars would not match, and the exe would launch as a normal game that never exits). Scenes are staged in `src/capture_scenes.rs`, which sweeps the floor around the fixture under test so it is not buried in loose toys.
