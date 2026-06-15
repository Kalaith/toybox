# Toybox After Hours: Closing Shift

A Rust + Macroquad 3D cleanup prototype for Web Hatchery Games.

You are the night-shift worker in a magical toy store after closing. Pick up
scattered toys, read their visual clues, and return every plushie, block set,
action figure, and board game to its matching display before opening.

## Current Prototype

- One small 3D toy store room
- 5 physical display areas
- 100 deterministic toys generated from display data
- Five categories: plushies, tiny dragons, robots, board games, building blocks
- One broken robot split into head and body pieces; place both matching pieces on the repair bench before shelving it
- One-toy carry limit, mistakes, timer, and finish screen
- Tool credits from completed displays, with a Toy Scanner purchase
- Toolkit save/load slot support

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

