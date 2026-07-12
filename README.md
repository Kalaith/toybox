# Toybox After Hours: Closing Shift

A Rust + Macroquad first-person 3D cleanup game for Web Hatchery Games.

You are the night-shift worker in a magical toy store after closing. Pick up
scattered toys, read their visual clues, and return every plushie, block set,
action figure, and board game to its matching display before opening.

## The Store

- A large multi-zone toy store (34×22 m): Plush Corner, Checkout, Dragon
  Alcove, Block Pit, Robot Lab, Backroom, and the Board Game Wall, connected
  by shelving-lined aisles with real player collision
- **4000 deterministic toys** scattered across the zones in messy piles —
  identical store every fresh run
- 50 distinct procedural toy designs (10 per category) built entirely from
  primitives: no image assets, everything drawn in code
- 20 themed displays (walls, pegboards, bins, shelves, tables) to shelve
  toys onto, four per category
- Hanging zone signs and a live minimap keep the store navigable
- Two repair benches; broken toys split into head and body pieces that must
  be rejoined at the same bench before shelving
- One-toy carry limit, mistakes, timer, and finish screen
- Tool credits from completed displays, with a Toy Scanner purchase
- Toolkit save/load slot support
- Runs at 60+ FPS native at full 4000-toy scale via spatial-grid culling and
  distance LOD (F3 shows the debug overlay when enabled)

## Controls

- `WASD`: move relative to the first-person view
- Mouse or arrow keys: look around, including up and down
- Click the shop view to lock mouse look; `Tab` or `Esc` releases it
- `E` or `Space`: pick up, shelf, place repair parts on the bench, or place the active toy on the floor
- `G`: quick-drop the active toy
- `T`: open or close the shop tools screen
- `Ctrl+S` / `Ctrl+L`: save / load
- `R`: restart

## Validation

```powershell
.\publish.ps1
```
# Practical Future Improvements

- Add deterministic replay tests for sorting, scoring, mistake penalties, timer acceleration, and completion goals.
- Validate upgrade availability and challenge metadata before run start so new toy types cannot break progression.
- Separate 3D scene rendering from game-state mutation so camera and visual effects do not affect scoring.
- Add scenario fixtures for beginner, mid-upgrade, and high-pressure sorting runs.

