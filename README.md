# Toybox After Hours: Closing Shift

A Rust + Macroquad first-person 3D cleanup game for Web Hatchery Games.

You are the night-shift worker in a magical toy store after closing. Pick up
scattered toys, read their visual clues, and return every plushie, block set,
action figure, and board game to its matching display before opening.

## A Shift

Two ways to start, chosen on the title screen:

- **Closing Shift** — 30 minutes until the doors open. The HUD clock counts
  down, ambering under five minutes and reddening under one. When it runs out
  the shift ends where it stands and the score screen reports the damage.
- **Relaxed Run** — the same store with no deadline. The clock still counts so
  a run can be compared against a timed one, but it never ends the shift.

A shift measures at about 28 minutes bare-handed and 20 for a closer that buys
tools as it earns them, so the deadline is comfortable played well and tight
played carelessly. Shelving the store perfectly ends the run early with a
"Store Restored" score screen.

## The Store

- A multi-zone toy store (34×22 m): Plush Corner, Checkout, Dragon Alcove,
  Block Pit, Robot Lab, Backroom, and the Board Game Wall, connected by
  shelving-lined aisles with real player collision
- **240 deterministic toys** scattered across the zones in messy piles —
  identical store every fresh run
- 50 distinct procedural toy designs (10 per category) built entirely from
  primitives: no image assets, everything drawn in code
- 20 themed displays (walls, pegboards, bins, shelves, tables) to shelve toys
  onto, four per category, twelve slots each
- Hanging zone signs and a live minimap keep the store navigable
- Two repair benches; roughly one toy in eight starts broken, split into a head
  and a body scattered into *different* zones, to be rejoined at one bench
  before either half can be shelved
- A one-toy carry limit to start with, a wrong-shelf penalty worth about one
  toy's work, and a score screen grading the run
- Tool credits from completed displays, spent on five tools
- Toolkit save/load slot support
- 60+ FPS native via spatial-grid culling and distance LOD (F3 shows the debug
  overlay when enabled)

## Tools

Each restored display earns one credit. Tools unlock as displays are completed
and are bought from the shop screen (`T`); they last the shift, not beyond it.

| Tool | Unlocks at | Cost | Effect |
|---|---|---|---|
| Toy Scanner | 1 display | 1 | Names the display a held toy belongs to, and pins the exact spot of a carried part's other half instead of just its aisle |
| Sorting Trolley | 2 displays | 2 | Carry three toys instead of one |
| Grippy Sneakers | 3 displays | 2 | A third faster across the floor |
| Long-Handled Grabber | 4 displays | 3 | Reach further into a pile |
| Manager's Nod | 5 displays | 3 | The next twenty-five wrong shelves cost no time |

Without the scanner a carried repair part still names the aisle its other half
landed in — enough to make the errand a search rather than a sweep of the whole
store.

## Controls

- `WASD`: move relative to the first-person view
- Mouse or arrow keys: look around, including up and down
- Click the shop view to lock mouse look; `Tab` or `Esc` releases it
- `E` or `Space`: pick up, load another toy onto the trolley, shelf, place
  repair parts on the bench, or put the active toy on the floor
- `Q`: cycle which carried toy is active
- `G`: quick-drop the active toy
- `T`: open or close the shop tools screen
- `Ctrl+S` / `Ctrl+L`: save / load
- `R`: restart, in whichever mode is already running

## Validation

```powershell
cargo test                                    # session, interaction and replay suites
cargo clippy --all-targets --all-features -- -D warnings
.\publish.ps1                                 # Windows + WebGL build and deploy
```

Balance numbers come from the deterministic replays in
`src/state/tests/replay.rs`, which drive the real `GameSession` API rather than
a model of it. Two are `#[ignore]`d because they are slow and only wanted when
retuning:

```powershell
cargo test --release shop_scale -- --ignored --nocapture   # run length vs shop size
cargo test --release full_shift -- --ignored --nocapture   # whole shop, start to finish
```

Outstanding work is tracked in [`TODO.md`](TODO.md).
