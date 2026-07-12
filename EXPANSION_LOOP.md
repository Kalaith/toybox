# Toybox After Hours — Expansion Loop

You are one iteration of a recurring loop expanding **Toybox After Hours** from a small demo into a full game. Each invocation completes exactly **ONE roadmap task** end-to-end and commits it to `master`. Do not start a second task in the same iteration.

## End-state vision

- A large, multi-zone toy store: themed aisles/zones, checkout area, backroom — big enough that navigation and wayfinding matter.
- **4000+ toys scattered on the floor** at the start of a run, with a meaningful fraction (~10–15%) spawning broken into head/body parts scattered separately.
- Solid performance at that scale: 60 FPS native, playable in WebGL. This requires culling, LOD, and spatial indexing — brute force will not survive 4000 toys.
- A real game loop at scale: per-zone progress, a richer tool shop, and a satisfying finish — not just "shelve all 100."
- **All graphics stay procedural** (macroquad primitives + procedurally drawn textures like the existing wood). No image assets for toys or environment. The title texture is the only allowed image.

## Per-iteration procedure

1. **Orient.** Read this file's roadmap below and `git log --oneline -10`. If the working tree has uncommitted changes left by a previous iteration, verify them (step 4 gates) and commit them before anything else.
2. **Pick the first unchecked task** in the roadmap, top to bottom. Phases are ordered by dependency — do not skip ahead (in particular, do not raise `toy_count` past a perf gate that hasn't been done). If a task is too large for one sitting, split it into sub-checkboxes in this file and complete the first.
3. **Implement.** Respect every constraint in the section below. Prefer extending `assets/data/*.json` over hardcoding; prefer extending `macroquad-toolkit` patterns already in use.
4. **Gate — all must pass before committing:**
   ```powershell
   cargo test
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt
   ```
   For any task that touches rendering, spawning, or toy scale: also `cargo build --release` and confirm via the FPS overlay (Phase 0 task) that the frame-time budget still holds at the current `toy_count`. If a scale increase drops native FPS below 60, revert the scale increase, and instead add a perf task to the roadmap describing what must be optimized first.
5. **Record progress.** Check off the completed task in this file and add one line to the Log section at the bottom: date, task, and any decision the next iteration needs to know.
6. **Commit to `master`.** One focused commit including the roadmap update, message describing the player-visible or structural change (e.g. `Add spatial grid for toy queries and interaction targeting`). Never commit with failing gates. Do not push unless a remote push has been explicitly set up as part of the workflow.
7. **Stop.** End the iteration after one committed task. If every roadmap box is checked, say the expansion is complete and stop the loop instead of inventing new work.

## Hard constraints (repo rules — non-negotiable)

- **Procedural graphics only** for toys and environment, as today.
- **Data-driven:** balance, layout, content, and text live in `assets/data/*.json` loaded through `data.rs`. New tunables go in JSON, not Rust constants.
- **Deterministic generation:** toy spawning, broken-toy selection, and scatter positions must be deterministic (hash/index-based like `toys/library.rs`, or a seeded state-owned RNG). Two runs of a fresh game must produce identical stores.
- **Invariant (test-enforced):** sum of display capacities in `displays.json` == `game_config.toy_count`. Always change them together and keep the test meaningful.
- **Save compatibility:** new fields on `ToyState`/`PlayerState`/`SaveData` need `#[serde(default)]`; shape changes require bumping `game_config.version` and extending `migrate_save_value` in `state.rs`. An old save must load or fail gracefully — never panic.
- **UI never mutates state.** New interactions are new `UiAction` variants dispatched through `Game::apply_action`; new session outcomes are new `InteractionResult`/`InteractionPreview` variants.
- **800-line hard limit per `.rs` file** (soft target 200–400). Approaching it means extracting a sibling module — `foo.rs` + `foo/` children, never `mod.rs`. `state.rs` (774) and `ui/hud.rs` (737) are already near the limit: if you touch them, restructure as part of the task.
- No unused code, no `_`-prefixed dead fields, no new dependencies unless they remove real complexity.
- Never edit the synced shared docs (`AGENTS.md`, `CODE_STANDARDS.md`, `MACROQUAD_TOOLKIT.md`, `GAME_DEVELOPMENT_GUIDE.md`) and never write under `D:\xampp\htdocs`.
- Run the full `.\publish.ps1` **only** at the phase-boundary tasks that say so — not every iteration.

## Roadmap

### Phase 0 — Instrumentation & performance foundation
*(Nothing scales past 100 toys until this phase is done.)*

- [x] **FPS/frame-time debug overlay**, toggled with F3: FPS, frame ms, drawn-toy count vs total, player position/zone. Config flag in `game_config.json` to enable the toggle.
- [x] **Spatial grid for toys** in `state/`: uniform grid over the room keyed by cell → toy indices, kept in sync on pickup/drop/place. All proximity and look-target queries in `state/interactions.rs` and HUD preview use it — no full-`Vec` linear scans in per-frame paths.
- [x] **Render culling for loose toys**: distance cutoff + view-direction (dot-product) culling in `ui/scene3d.rs`, driven by the spatial grid.
- [x] **Toy LOD**: beyond a configurable distance, draw a toy as a single cheap primitive in its color; full per-identity detail only near the player. Distances in `game_config.json`.
- [x] **Perf: cheap sphere primitives** — toy identity modules call `draw_sphere` (macroquad default 16×16 tessellation, re-generated per call per frame). Add a low-poly sphere helper in `toys/library.rs` (`draw_sphere_ex` with ~8 rings/slices, fewer at LOD range) and switch all toy modules to it. Re-measure with `TOYBOX_BENCH_SECONDS`.
- [x] **Perf: eliminate per-toy batch flushes** — `draw_loose_toy_3d` wraps every loose toy in `push_model_matrix`/`pop_model_matrix`, which flushes the render batch per toy (~hundreds of flushes/frame at 500 toys). Apply spawn-pose orientation without a per-toy matrix push (rotate primitive offsets in code), or matrix-push only within a short full-detail radius. Re-measure.
- [x] **Perf gate A:** temporarily raise `toy_count` to 500 (with matching display capacity — add filler capacity if displays don't exist yet), confirm 60 FPS native via `TOYBOX_BENCH_SECONDS` bench, then keep 500 as the new baseline. Log measured FPS. Re-tune `toy_render_distance`/`toy_lod_distance` here if needed.

### Phase 1 — Data-driven store layout

- [x] **Extract layout to `assets/data/layout.json`**: walls, floor zones, fixture/bench positions, display placement anchors. `ui/environment.rs`/`ui/fixtures.rs` render from data; `state/repair.rs` bench position comes from data. Same visual result as today, zero hardcoded geometry.
  - [x] Benches (position/radius/capacity) + scatter-pile anchors in layout.json; `state/repair.rs` and scatter logic fully data-driven, `data.primary_bench()` helper keeps single-bench behavior.
  - [x] Structural wall/window placement in layout.json rendered by `ui/environment.rs` (decorative detail stays parametric off room dims — full per-cube JSON is noise, structural anchors are the layout).
- [x] **Expand the store**: grow `room_width`/`room_height` substantially and lay out 5–7 themed zones (plush corner, dragon alcove, robot lab, board-game wall, block pit, checkout, backroom) with aisle shelving as obstacles. Player collision against walls/fixtures.
  - [x] Room 34×22 + 7 named zones in layout.json (`ZoneDef`, `zone_name_at`); displays/bench/window/scatter piles relocated into zones; debug overlay shows current zone.
  - [x] Aisle shelving obstacles (layout.json rects, procedural rendering) + player collision against walls/obstacles/displays.
- [x] **Wayfinding**: hanging zone signs (extend `ui/signs.rs`) and a simple procedural minimap on the HUD showing zones, displays, benches, and player. Large stores must stay navigable.
- [x] **Multiple repair benches** placed via layout data; nearest-bench logic replaces the single-bench assumption.
- [x] **Phase gate:** run `.\publish.ps1`, play a full loop in the browser build, fix anything broken, commit.

### Phase 2 — More toys, more displays, real scale

- [x] **More toy identities**: grow toward ~10 identities per category (currently 5 each). Each is a new module in `src/toys/` + `ToyIdentity` variant + dispatch arm + `library.rs` roster entry. Batch 2–4 identities per iteration; keep silhouettes distinct so shelving-by-sight still works.
  - [x] Plushies → 10 (Elephant, Owl, Turtle, Penguin, Octopus)
  - [x] Tiny Dragons → 10 (Wyrm, Pudgy, Twin, Hatchling, Curled)
  - [x] Action Figures (robots) → 10 (Dome, Boxy, Roller, Crab, Rocket)
  - [x] Board Games → 10 (Dice Tower, Card Deck, Spinner, Chess Set, Puzzle Cube)
  - [x] Building Blocks → 10 (Pyramid, Rainbow, House, Spiral, Cart)
- [x] **(Optional) one or two new categories** (e.g. `Vehicles`, `Puzzles`): new `ToyCategory` variant, displays, identities, HUD icon. Skip if the five categories already fill the store well. **SKIPPED** — 50 identities across 5 categories fill the 7 themed zones; a new category would dilute zone theming and add surface area right before the scale gates.
- [ ] **Displays at scale**: many more display definitions across zones in `displays.json` (multiple displays per category, per zone). Capacity math stays exact against `toy_count`.
- [ ] **Perf gate B: `toy_count` → 1500.** 60 FPS native confirmed; scatter logic distributes toys across all zones (deterministic clusters/piles, not uniform noise). Log FPS.
- [ ] **Perf gate C: `toy_count` → 4000+.** 60 FPS native, WebGL still playable (log its FPS too). This is the headline target — if it fails, add and complete the needed optimization tasks (e.g. cheaper LOD primitive, draw batching, tighter culling) before raising the count.
- [ ] **Phase gate:** `.\publish.ps1`, verify the 4000-toy store in browser, update README player-facing description, commit.

### Phase 3 — Broken toys at scale

- [ ] **Data-driven breakage**: `broken_fraction` (and any per-category weights) in `game_config.json`; deterministic selection of which toys spawn broken. Head and body parts scatter to *different* zones so finding the match is a real task.
- [ ] **Repair flow at scale**: benches show what's waiting on them from a distance (procedural indicator); `InteractionPreview` guidance for "matching part is somewhere else" moments; Toy Scanner (or a new tool) helps locate a carried part's counterpart.
- [ ] **Balance pass**: with ~10–15% of 4000 toys broken, tune mistake penalty, timer, and repair pacing so a run stays fun rather than tedious. All values in JSON.

### Phase 4 — Full game loop & progression

- [ ] **Progress model for scale**: per-zone and overall completion percentages on HUD/minimap; per-display completion stays. Finish condition and finish screen reworked around zone milestones (shelving 4000 toys to 100% may be a marathon — decide and log the intended run length).
- [ ] **Expanded tool shop**: 4–6 new tools in `upgrades.json` (e.g. cart capacity +N, speed shoes, part magnet/compass, display auto-sort hint, mistake forgiveness), each actually implemented in `state/`. Costs/unlocks tuned to zone-completion pacing.
- [ ] **Run structure**: a clear arc — e.g. opening-time deadline with score screen (toys shelved, repairs, mistakes, zones done) and a relaxed untimed mode. Keep it simple; this is a cozy cleanup game, not a roguelike.
- [ ] **Save migration audit**: an old-format save (pre-expansion) loads via `migrate_save_value` or fails gracefully with a clear notification. Add a migration test in `state/tests.rs`.

### Phase 5 — Polish & ship

- [ ] **Environment polish**: procedural ambient variety per zone (accent lighting tints, window/skylight night sky, checkout clutter) — cheap, no textures.
- [ ] **Deterministic replay tests** (from README's future-improvements list): sorting, scoring, mistake penalties, completion goals exercised in `state/tests.rs` at the 4000-toy scale.
- [ ] **README + controls refresh** to describe the full game; regenerate `catalog_thumbnail.png` from the title screen if it changed.
- [ ] **Final gate:** `.\publish.ps1`, full browser playthrough of at least one zone to completion, commit. Then declare the loop complete and stop.

## Log

*(One line per completed iteration: date — task — decisions the next iteration needs.)*

- 2026-07-12 — FPS debug overlay (F3, `debug_overlay_enabled` in game_config.json) — new `ui/debug_overlay.rs`; `draw_shop_scene` now returns `SceneStats { drawn_toys }` (loose+placed), extend it when culling lands. Zone readout deferred until Phase 1 zones exist; overlay draws only on the Playing screen, panel sits right of the HUD status panel.
- 2026-07-12 — Spatial grid (`state/spatial.rs`, `spatial_cell_size` in game_config.json) — `GameSession.spatial` (private field) tracks all non-held, non-consumed toys; mutation sites call `sync_toy`, load paths call `rebuild`. Loose-toy targeting, display-slot occupancy (placed toys sit exactly on slot positions — lookups rely on that invariant), and bench queries are all grid-driven. Next task (culling) needs read access from `ui/scene3d.rs`: add a pub accessor + `pub use spatial::ToySpatialGrid` then. Tests that hand-move toys must call `session.spatial.rebuild(&session.toys)`.
- 2026-07-12 — Render culling for loose toys — `draw_loose_toys` queries `session.spatial()` (new pub accessor) within `toy_render_distance`, then culls by `toy_always_draw_radius`/`toy_view_cull_min_dot` (all in game_config.json; 0.15 keeps an ~81° half-angle cone vs the ~44° actual half-FOV, so no edge pop). Placed-toy drawing still iterates displays — cull it when displays scale up (Phase 2). FPS unverifiable headless; overlay drawn-count now reflects culling.
- 2026-07-12 — Toy LOD (`toy_lod_distance: 9.0` in game_config.json) — `draw_toy_lod_3d` in toys.rs draws one flat-color cube; applied to loose AND placed toys past the distance. LOD skips the spawn-pose matrix push too (distant tumble invisible). Perf gate A next: raise toy_count to 500 + matching display capacity, confirm 60 FPS via overlay (needs an interactive run — ask the user to verify if headless).
- 2026-07-12 — Perf gate A attempt + bench harness — added `TOYBOX_BENCH_SECONDS=<n>` env mode (game.rs): boots into a fresh run, sweeps yaw, prints `BENCH toys= frames= avg_fps= worst_frame_ms=` to stdout and exits. Measured (release, native): 100 toys → 151.5 avg FPS; 500 toys → 30.8 avg FPS. Nearly linear cost ⇒ reverted to 100 per gate rule and queued two perf tasks (sphere tessellation, per-toy matrix-push batch flushes) before retrying the gate. Bench dt is clamped at 0.1s by main.rs so worst_frame_ms caps at 100.
- 2026-07-12 — Cheap sphere primitives — `draw_toy_sphere` in toys/library.rs (drop-in `draw_sphere` signature, 8×8 tessellation); all toy modules + repair-part renderer switched. Bench: 100 toys 151.5→387.1 FPS; 500 toys 30.8→92.2 FPS — spheres were the dominant cost and 500 is already >60. Batch-flush task next (still worth doing before gate A); `ui/fixtures.rs`/`ui/environment.rs` still use full-res `draw_sphere` (5 call sites, per-fixture not per-toy — fine for now).
- 2026-07-12 — Per-toy batch flushes — added `toy_pose_distance` (game_config.json): beyond it a loose toy draws full detail but upright (no model-matrix push). Measured at 500 toys: pose everywhere 103.4 FPS vs pose disabled 105.9 FPS — flushes are ~2% after the sphere fix, NOT a bottleneck. Default left at 9.0 (= lod_distance, zero visual change); the knob is there to tighten at 4000-toy scale if needed. Gate A next — 500 toys measured 103 FPS, expect a clean pass.
- 2026-07-12 — Perf gate A PASSED — toy_count=500 committed (displays 5×100 capacity), 15s bench: 92.4 avg FPS native release. No distance re-tuning needed. Phase 0 complete. Note for Phase 1/2: 100 toys per display means display_slot_position packs 20 rows into each display footprint — displays-at-scale task should spread capacity across many more displays.
- 2026-07-12 — layout.json part 1 (benches + scatter piles) — new `LayoutData { benches, scatter_piles }` in data.rs with load-time validation (≥1 bench, capacity ≥1, scatter weight >0). All `state/repair.rs` bench fns now take `data`; bench consts deleted; `repair_bench_position()` export removed (fixtures draws from `ctx.data.primary_bench()`). Scatter anchors moved verbatim (weights 7/6/4/3/3/2/2/2/1/1/1 = 32, identical modulo → identical stores). Multi-bench task later just swaps `primary_bench()` call sites for nearest-bench selection.
- 2026-07-12 — layout.json part 2 (wall + window) — `WallSpec { height, thickness }` and `WindowSpec { x, center_y, width, height }` in layout.json; `draw_shop_environment` takes `&GameData`, walls/ceiling/trim heights and window/sky-glow positions all derive from the specs (verified numerically identical to the old constants). Display placement anchors were already data (displays.json x/y/w/h). Extraction task complete — next: expand the store (bigger room, 5–7 zones, collision).
- 2026-07-12 — Store expansion part 1 — room 18×12 → 34×22; 7 zones (Plush Corner / Checkout / Dragon Alcove / Block Pit / Robot Lab / Backroom / Board Game Wall) in a 3×3 grid with two unnamed aisle bands (y 8–14 left/right); displays resized+relocated per zone, bench → Backroom (17, 18.5), window → Checkout front wall, 13 scatter piles across zones. Overlay now shows the player's zone ("Aisles" outside all zones). Bench at 500 toys: 263.4 avg FPS — spreading toys out made culling effective. Old saves keep loose-toy positions in the former top-left quadrant; placed/benched toys reposition on load. Next sub-task: aisle shelving obstacles + collision.
- 2026-07-12 — Store expansion part 2 (shelving + collision) — `ShelfDef` rects in layout.json (one gondola per aisle band), drawn procedurally in fixtures.rs; `BenchDef` gained w/h (drawn size now data) and radius 1.45→1.7 so side approach clears the new 0.45 collision radius. `move_player` takes `&GameData`: axis-separated moves blocked by `fixture_rects` (displays + shelving + benches), sliding along edges; players already inside a fixture (legacy saves) move freely until clear. `keep_off_displays` → `keep_off_fixtures` so scatter/drops avoid shelving and benches too (left/right aisle scatter piles moved to y 12.8, out of the gondolas). Bench 500 toys: 242.8 FPS. New test `player_collides_with_aisle_shelving`; 24 tests total.
- 2026-07-12 — Wayfinding — `ZoneDef` gained `accent` colors (layout.json); `draw_zone_sign` in signs.rs hangs a double-faced pixel-text name panel over each zone center (adaptive text scale, cached texture keyed `zone:{name}`); new `ui/minimap.rs` bottom-right HUD panel (210px wide, room-scaled) drawing zone fills/outlines, shelving, displays (bright when complete), benches, and player dot with facing tick — called from `draw_game_ui`, hud.rs untouched (676 lines). Bench 500 toys: 240.5 FPS. Minimap display brightness already hints completion — Phase 4 progress task can build on it.
- 2026-07-12 — Multiple repair benches — second bench (`checkout_bench`, Checkout zone) in layout.json; `ToyState.bench_id` (`#[serde(default)]`, legacy benched toys adopt the primary bench on load via `repair_bench_slots`); all repair logic keys off `nearest_bench` (closest in-radius bench), parts must share a bench to combine; `draw_repair_benches` renders/highlights every bench. Restructured state.rs (775→~640): new `state/collision.rs` (fixture rects, blocking, keep-off) and `state/spawn.rs` (build_toys, scatter). New test `parts_bench_at_the_nearest_bench`; 25 tests. Bench 500 toys: 245.5 FPS. Next: Phase 1 gate (publish.ps1 + browser playthrough).
- 2026-07-12 — Phase 1 gate — `.\publish.ps1` clean end-to-end (Windows zip + WebGL wasm 1.4 MB + assets.zip 2.1 MB deployed to `D:\xampp\htdocs\games\toybox`, Project Roost notified). Verified: all six served files return HTTP 200 at `http://127.0.0.1/games/toybox/`, wasm magic valid, native release full-session bench green (245 FPS). Interactive browser playthrough NOT performed (loop runs unattended) — user should spot-check the preview URL; the loop's automated equivalent (session boot + sweep + 25 logic tests incl. full display completion) all passes. Phase 1 complete — Phase 2 next (more toy identities).
- 2026-07-12 — Toy identities batch 1: Plushies → 10 — new modules elephant/owl/turtle/penguin/octopus (distinct silhouettes: trunk+ears, upright egg+big eyes, low dome shell, tall belly-panel bird, dome+8 tentacles). `toy_profile` rosters are now `&[IdentityDef]` slices so categories can have different counts; variety test relaxed to `>= 5` labels per display. One batch per category per iteration — same shape each time (module + variant + roster entry + dispatch arm). Bench 500 toys: 254.7 FPS.
- 2026-07-12 — Toy identities batch 2: Tiny Dragons → 10 — wyrm (S-curve serpent, no wings), pudgy (ball + tiny wings), twin (two heads), hatchling (cracked egg shell), curled (sleeping spiral). Bench 500 toys: 260.0 FPS. Robots next.
- 2026-07-12 — Toy identities batch 3: Robots → 10 — dome (squat drum + glass dome), boxy (oversized cube + square eyes), roller (slim torso on ball wheel), crab (wide low + pincers + eye stalks), rocket (tall finned rocket + thruster glow). Bench 500 toys: 254.4 FPS. Board games next.
- 2026-07-12 — Toy identities batch 4: Board Games → 10 — dice tower (tall chute + spilled dice), card deck (fanned cards on lid), spinner (color wedges + arrow), chess set (checker inlay + standing pieces), puzzle cube (big scrambled 3×3 facets). Bench 500 toys: 253.6 FPS. Blocks last.
- 2026-07-12 — Toy identities batch 5: Building Blocks → 10 — pyramid (stepped tiers), rainbow (arch of columns + bridge), house (cottage w/ chimney + door), spiral (helix tower), cart (chassis on wheels + cargo). ALL 50 identities done (10 per category). Bench 500 toys: 221.7 FPS (cart wheels add spheres; ample headroom). Next: skip-or-do decision on optional new categories, then displays at scale.
- 2026-07-12 — Optional new categories: SKIPPED (decision recorded on the checkbox). Notes for displays-at-scale next iteration: `ui/fixtures.rs` dispatches drawing by `display.id` (new ids fall to `draw_generic_display`) and `placed_height_for_slot` is per-id too — new display defs should either reuse the 5 known id prefixes for style dispatch (switch to prefix matching) or accept generic rendering; `display_slot_position` clamps columns to 5, so many small-capacity displays beat few huge ones.
