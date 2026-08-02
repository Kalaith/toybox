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
- The WebGL shell is **generated**, not kept here: `publish.ps1` renders the shared `web/index.template.html` against this project's root `game_page.json` (title, prose, controls, details). Edit `game_page.json`, never a per-game `index.html`. `storage.js` — the localStorage bridge for toolkit persistence in WASM — is deployed once to `shared-assets/runtime/` and referenced from there; the deployed page carries no per-game copy. `dist/` holds built artifacts.
- Screenshot harness (`TOYBOX_CAPTURE_*`): scenes `toy_gallery` (one toy from front/right/back/left, selected via `TOYBOX_CAPTURE_TOY=<module slug>`), `gameplay`, `title` (the mode buttons and the caption distinguishing them), `repair_bench` (a bench holding one half of a broken toy, for the status beacon and the awaiting-match prompt), `mid_run` (one aisle shelved, another half done, for the HUD/minimap completion readouts — a fresh shop reads 0% everywhere and shows nothing), `closing_soon` (`mid_run` with 44s left, for the countdown and its red warning tint — a fresh shop shows a full clock in the neutral colour), `shift_over` (the score screen after the clock won, with aisles part-done — a fresh `TimeUp` session scores every row zero and proves nothing about layout), `broken_lineup` (six broken halves of different identities in a row, to compare the per-identity part accents; `TOYBOX_CAPTURE_PART_CATEGORY=plushies|dragons|robots|board_games|blocks` picks the category, since a plush head and a block top share no features to compare, and `TOYBOX_CAPTURE_PART_KIND=body` shows the other half — bodies are half the models the accents touch and had never been looked at), `carrying_a_half` and `carrying_a_half_scanned` (the two repair-hint tiers side by side — every other scene either carries a whole toy or owns the scanner, so neither hint appears), `tool_shop`, `checkout` (till clutter plus the night sky through the shopfront glass), and `lamp_contrast` (stands under a pendant lamp looking straight down — a diagnostic framing, because a lamp's floor pool is hidden behind a display fixture from anywhere further back). `.\scripts\capture_toys.ps1` sweeps every toy into `docs\verification\toys\<toy>.png` (`-Toys a,b` to filter, `-SkipBuild` to reuse the build). The toy list comes from the `ToyIdentity::X => x::draw` dispatch in `src\toys.rs`, not from listing `src\toys\*.rs` minus a denylist of helper modules — that denylist went stale the moment `part_accents.rs` landed, and the sweep captured a bear captioned "UNKNOWN TOY" and reported it as ok. Re-run the sweep after touching any toy renderer; the committed gallery is a reference and goes quietly stale otherwise.
- `.\scripts\check-captures.ps1` re-captures every scene into `dist\capture-check` and reports which committed images no longer match, without overwriting any of them; non-zero exit on drift. Run it after touching a renderer or the HUD — the committed gallery otherwise goes stale silently, which it did for three weeks. Drift is a **percentage of differing pixels**, not a hash: two scenes (`mid_run`, `carrying_a_half_scanned`) still wobble ~0.05% on anti-aliased text edges, while a real change repaints thousands of pixels (a stale reference measured 3.2%), so the 0.1% default sits an order of magnitude above the noise and far below anything meaningful. Needs Python with Pillow.
- `.\scripts\capture_scene.ps1` captures the whole-store scenes into `docs\verification\ui_<scene>.png`. It is a thin wrapper over the shared `..\macroquad-toolkit\scripts\capture_ui.ps1`, passing `-Release` (a debug capture at ~4500 toys takes minutes and in practice never finishes) and `-Prefix TOYBOX` (the shared script would otherwise derive `TOYBOX_AFTER_HOURS` from the package name, the capture vars would not match, and the exe would launch as a normal game that never exits). Scenes are staged in `src/capture_scenes.rs`, which sweeps the floor around the fixture under test so it is not buried in loose toys.
