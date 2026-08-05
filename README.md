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

The deterministic closer completes the whole job in about 21.4 minutes: it
starts empty-handed, earns all five tools, repairs all 28 broken toys, and
shelves all 240. That leaves 8.6 modelled minutes inside the 30-minute deadline;
the margin is deliberately larger than the replay needs because a person has to
recognise toys, search, turn, and backtrack. Shelving the store perfectly ends
the run early with a "Store Restored" score screen.

Each mode keeps its own best run — most toys shelved, then fewest wrong
shelves, then fastest — and the score screen shows it so there is something to
beat. Tools do not carry between shifts; the record is the only thread from one
run to the next.

A first-shift guide teaches movement, pickup, category shelving, repairs,
display credits, and trolley cycling as each becomes relevant. Press `H` to
hide it; **Controls & How to Play** in Settings can replay it later.

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
- A warm wood-and-brass HUD keeps the clock, whole-store progress, current
  aisle, trolley, prompts, and store directory readable without hiding the room
- Two repair benches; roughly one toy in eight starts broken, split into a head
  and a body scattered into *different* zones, to be rejoined at one bench
  before either half can be shelved
- A one-toy carry limit to start with, a wrong-shelf penalty worth about one
  toy's work, and a score screen grading the run
- Tool credits from completed displays, spent on five tools and a bounded
  late-shift search service
- Toolkit save/load slot support
- 60+ FPS native via spatial-grid culling and distance LOD (F3 shows the debug
  overlay when enabled)

## Tools

Each restored display earns one credit. Tools unlock as displays are completed
and are bought from the shop screen (`T`); they last the shift, not beyond it.

| Tool | Unlocks at | Cost | Effect |
|---|---|---|---|
| Toy Scanner | 1 display | 1 | Recommends the nearest matching display with room, while still marking the alternatives, and pins the exact spot of a carried part's other half |
| Sorting Trolley | 2 displays | 2 | Carry three toys instead of one |
| Grippy Sneakers | 3 displays | 2 | A third faster across the floor |
| Long-Handled Grabber | 4 displays | 3 | Reach further into a pile |
| Manager's Nod | 5 displays | 3 | Stops the next twenty-five wrong placements before they leave your hands; they still count as mistakes |

After all five tools are owned, each spare credit can call a **Stockroom
Spotlight** for 60 seconds. It marks the nearest loose toy in the room and on
the minimap, stacking up to three minutes; it never moves or sorts the toy for
you.

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
- `H`: hide the first-shift guide (replay it from Settings)
- `Esc`: pause and open Settings
- `Ctrl+S` / `Ctrl+L`: save / load
- `R`: restart, in whichever mode is already running

Settings persist fullscreen, field of view, look sensitivity, UI text size,
and high-contrast mode separately from the current cleanup save.

## Validation

```powershell
cargo test                                    # session, interaction and replay suites
cargo clippy --all-targets --all-features -- -D warnings
.\publish.ps1                                 # Windows + WebGL build and deploy
```

Balance numbers come from deterministic replays in
`src/state/tests/replay.rs`, which drive the real `GameSession` API rather than
a model of it. The normal suite requires the earned-tool route to finish all
240 toys with every repair complete and at least 15% deadline headroom. Two
diagnostic reports are `#[ignore]`d because they are only wanted when retuning:

```powershell
cargo test --release shop_scale -- --ignored --nocapture   # run length vs shop size
cargo test --release full_shift -- --ignored --nocapture   # whole shop, start to finish
```

Outstanding work is tracked in [`TODO.md`](TODO.md).
