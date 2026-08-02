# TODO — Toybox After Hours

## Repair flow at scale

- The cross-zone scatter is **deliberate** — hunting scattered halves and rejoining them is a pillar of the game alongside sorting, and must not be shortened to make repairs cheaper. Measured on its own terms in `state/tests/replay.rs` (`Strategy::Restorer`), the errand is fully winnable: at the 240-toy shop that is all 28 broken pairs in 56 actions with nothing deferred, ~18s per pair. Cost per pair is unchanged by the shop retune; there are simply fewer pairs. The earlier "zero repairs" figure came from a nearest-first closer, which measures the sorting loop and never runs the errand at all.
- **The scanner gate is softened, not removed.** A carried half now always names the aisle its counterpart landed in ("Other half: head in Checkout"); the scanner upgrades that to distance plus the beacon column ("Scanner: head in Checkout, 17m"). Unaided the errand is a real search of one zone rather than a sweep of the whole shop for one object among hundreds, which was a wall rather than a journey. The scanner's `upgrades.json` description was rewritten to sell what it now actually adds.
- Mistake penalty and timer are still untuned against `assets/data/*.json`.
- Repair parts render per *category* but not per *identity* — every plush head is the same model, so a broken Bear and a broken Octopus are indistinguishable while split. Decide whether identity-level parts are worth 50×2 renderers or whether identity-derived detail on the existing 10 is enough.

## Game loop and progression

- **Run length is decided: 240 toys, 12 per display.** A shift is ~28 min bare-handed and ~17 min fully equipped, and 12 displays complete against the 11 credits all five tools cost — so the tool economy closes with a little room, and buying tools visibly shortens the run. Measured by `shop_scale_sets_the_length_of_a_shift` (`cargo test --release shop_scale -- --ignored --nocapture`), which sweeps capacity and is the thing to re-run before changing `toy_count` again:

  | capacity | toys | bare-handed | equipped | displays done |
  |---|---|---|---|---|
  | 8 | 160 | 19.4 min | 11.5 min | 13 |
  | **12** | **240** | **28.0 min** | **16.8 min** | **12** |
  | 20 | 400 | 47.4 min | 27.3 min | 8 |
  | 40 | 800 | 93.0 min | 56.3 min | 6 |
  | 200 | 4000 | 462.8 min | 328.1 min | 1 |

  Cost per toy is ~7.6s bare-handed at every size, so length is linear in `toy_count`; what is *not* linear is completed displays, which collapse as capacity grows because one unrepaired toy holds a whole display open. The old 4000-toy shop completed a single display in nearly eight hours, which put every one of the five tools out of reach for the entire game.
- **Deadline and relaxed mode are in.** `shift_seconds` (1800) ends a timed run at `GamePhase::TimeUp`; the title offers *Closing Shift* against the clock and *Relaxed Run* without one, `ShiftMode` persists in the save, and the HUD counts down (amber under 5 min, red under 1). The mistake penalty now bites, because pushing `elapsed_seconds` can end the shift outright — pinned by `state/tests/shift_clock.rs`.
- **The score screen is in** (`ui/score.rs`): grade badge, toys shelved, repairs, wrong shelves, time, and a per-aisle bar table, driven by `GameSession::shift_summary`. Finishing an aisle now announces itself mid-run via `InteractionResult::Placed.completed_zone`. The HUD hides behind the panel so the score is the whole message.
- **The ~88% zone cap is now legible rather than weakened.** `ZoneProgress::broken` counts an aisle's toys currently in halves and `still_to_find()` the rest of its shortfall, so the score screen reads "41 to find - 5 to mend" and the HUD "Plush Corner - 5 to mend / 90%". An aisle with toys in pieces is genuinely not restored, so `is_restored` was left alone; what was wrong was that a player who had shelved every whole toy in an aisle had no way to tell the remainder from toys they had missed. Pinned by `every_aisle_slot_is_accounted_for_as_shelved_broken_or_missing`.
- `shift_seconds` is set from replay minutes, which use an untuned 0.6s-per-interaction constant. Worth a real playthrough before trusting 30 minutes as the deadline.

## Polish

- Refresh the README and controls once the full loop lands, and regenerate `catalog_thumbnail.png` if the title screen changes.

## Engineering

- Balance the five tools against measured pacing. Over a whole 240-toy shift: bare-handed 28.0 min, trolley+scanner 21.1, fully equipped 16.8 — so a full toolset is worth about 40% of the run. The trolley now clearly pays (it used to measure *worse* than bare hands); the speed tools stack on top.
- The replay closer used to walk every toy to the **first** display of its category, ignoring the other three. That fills a quarter of the shop and then spins, carrying each remaining toy to a shelf that has been full for hours and dropping it again — 9700 wasted actions in an 8000-action shift. Every "the shop is unfinishable" number before this was measuring that, not the game. `home_display_index` now picks the nearest matching display *with room*, and `the_closer_clears_the_floor_without_misshelving` asserts the run ends because it ran out of work rather than out of budget.
- Placement still goes through `place_active_toy` rather than aiming at a shelf slot, so the numbers cover the cost of *finding* a toy but not of *placing* it.
- The "crosshair misses half the time" reading was **two different things added together**, and the report now splits them:
  - *Whiffed* — the crosshair delivered nothing. Bare-handed this is **zero**; the cone is fine. What produced 175 whiffs was the Sorting Trolley having no input path at all (fixed: `E` on a loose toy now loads the armful when there is room). The 66 that remain are the closer aiming at a toy that sits in front of a free shelf slot, where `E` correctly shelves instead — real friction, but the right precedence.
  - *Neighbour* — the crosshair handed over a different toy than the one aimed at, ~175–195 of 400. The closer still walks away holding a toy, so this is pile texture rather than lost work, and it is arguably the point of a floor buried in toys. Leaving as-is.
