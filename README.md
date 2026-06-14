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
- One broken robot split into head and body pieces; carry both to the repair bench before shelving it
- Carry limit, active carried toy selection, mistakes, timer, and finish screen
- Tool credits from completed displays, with Toy Scanner and Small Trolley purchases
- Toolkit save/load slot support

## Controls

- `WASD`: move relative to the first-person view
- Mouse or arrow keys: look around, including up and down
- Click the shop view to lock mouse look; `Tab` or `Esc` releases it
- `E` or `Space`: pick up, shelf, or place the active toy on the floor
- `Q`: cycle carried toys
- `G`: quick-drop the active toy
- `T`: open or close the shop tools screen
- `Ctrl+S` / `Ctrl+L`: save / load
- `R`: restart

## Validation

```powershell
.\publish.ps1
```
