# TODO — Toybox After Hours

## Repair flow at scale

- The cross-zone scatter is **deliberate** — hunting scattered halves and rejoining them is a pillar of the game alongside sorting, and must not be shortened to make repairs cheaper. Measured on its own terms in `state/tests/replay.rs` (`Strategy::Restorer`), the errand is fully winnable: at the 240-toy shop that is all 28 broken pairs in 56 actions with nothing deferred, ~18s per pair. Cost per pair is unchanged by the shop retune; there are simply fewer pairs. The earlier "zero repairs" figure came from a nearest-first closer, which measures the sorting loop and never runs the errand at all.
- **The scanner gate is softened, not removed.** A carried half now always names the aisle its counterpart landed in ("Other half: head in Checkout"); the scanner upgrades that to distance plus the beacon column ("Scanner: head in Checkout, 17m"). Unaided the errand is a real search of one zone rather than a sweep of the whole shop for one object among hundreds, which was a wall rather than a journey. The scanner's `upgrades.json` description was rewritten to sell what it now actually adds.
- **Mistake penalty and timer are measured now, and neither number needed changing.** A wrong shelf costs 8.0s against a measured 7.7s per toy — a ratio of 1.03, i.e. exactly one more toy's work, which is also the natural cost of having to collect and re-shelve it. `a_wrong_shelf_costs_about_one_toys_worth_of_time` pins the *relationship* rather than the constant, so a retune of shop size, walking speed or tools shows up instead of quietly making the penalty trivial or savage.
- **Decided: identity-derived detail, not 50×2 renderers.** A hundred new models is a great deal of work for an object the player sees for the length of one errand, and they would have to be kept in step with the fifty whole-toy renderers forever. Instead `toys/part_accents.rs` gives each identity a `PartAccent` — a crest shape and scale, a muzzle length, an eye spread — and the ten category renderers read it. Fifty table rows and one `draw_crest` helper. `no_two_identities_in_a_category_break_into_the_same_part` compares only the fields a category actually draws, so a copy-pasted row fails rather than silently shipping two identical halves.

## Game loop and progression

- **Run length is decided: 240 toys, 12 per display.** A shift is ~28 min bare-handed and ~17 min fully equipped, and 12 displays complete against the 11 credits all five tools cost — so the tool economy closes with a little room, and buying tools visibly shortens the run. Measured by `shop_scale_sets_the_length_of_a_shift` (`cargo test --release shop_scale -- --ignored --nocapture`), which sweeps capacity and is the thing to re-run before changing `toy_count` again:

  | capacity | toys | bare-handed | equipped | displays done | shelf refusals |
  |---|---|---|---|---|---|
  | 8 | 160 | 19.1 min | 12.2 min | 14 | 0 |
  | **12** | **240** | **28.0 min** | **17.7 min** | **13** | **2** |
  | 20 | 400 | 46.9 min | 53.3 min | 14 | 1133 |
  | 40 | 800 | 92.5 min | 63.8 min | 12 | 280 |
  | 100 | 2000 | 231.7 min | 161.7 min | 3 | 7173 |
  | 200 | 4000 | 377.5 min | 333.1 min | 0 | 15793 |

  (last column: times a run walked to a gap on a shelf and `E` refused, bare-handed)

  Cost per toy is ~7.7s bare-handed at the sizes that work, so length is roughly linear in `toy_count`. Two things are *not* linear. Completed displays collapse as capacity grows, because one unrepaired toy holds a whole display open. And **displays become unfillable past their front rows**: slots are laid out five to a row, so capacity sets depth, and shelf targeting always offers the nearest slot in the crosshair — once row one is full, rows two and beyond are shadowed by it. At capacity 12 (three rows) that costs 2 refusals in a run; at 200 (forty rows) the shop cannot be filled at all. `a_display_stays_fillable_to_its_back_row` guards the shipped depth, because nothing else would fail if displays got deeper — they would just quietly stop accepting toys.
- **Deadline and relaxed mode are in.** `shift_seconds` (1800) ends a timed run at `GamePhase::TimeUp`; the title offers *Closing Shift* against the clock and *Relaxed Run* without one, `ShiftMode` persists in the save, and the HUD counts down (amber under 5 min, red under 1). The mistake penalty now bites, because pushing `elapsed_seconds` can end the shift outright — pinned by `state/tests/shift_clock.rs`.
- **The score screen is in** (`ui/score.rs`): grade badge, toys shelved, repairs, wrong shelves, time, and a per-aisle bar table, driven by `GameSession::shift_summary`. Finishing an aisle now announces itself mid-run via `InteractionResult::Placed.completed_zone`. The HUD hides behind the panel so the score is the whole message.
- **The ~88% zone cap is now legible rather than weakened.** `ZoneProgress::broken` counts an aisle's toys currently in halves and `still_to_find()` the rest of its shortfall, so the score screen reads "41 to find - 5 to mend" and the HUD "Plush Corner - 5 to mend / 90%". An aisle with toys in pieces is genuinely not restored, so `is_restored` was left alone; what was wrong was that a player who had shelved every whole toy in an aisle had no way to tell the remainder from toys they had missed. Pinned by `every_aisle_slot_is_accounted_for_as_shelved_broken_or_missing`.
- **The deadline is reachable.** `the_deadline_is_reachable_by_a_closer_who_buys_tools` adds `Strategy::Earner` — the only loadout a timed run ever really has, since tools do not carry between shifts — and it clears the shop in 20.1 min against the 30 min deadline, versus 28.0 bare-handed. Ten minutes of slack for a near-optimal closer that teleports in straight lines and never backtracks, so a real player should find 30 minutes comfortable played well and tight played carelessly. Still worth a human playthrough, since the replay's 0.6s-per-interaction constant is untuned.

## Polish

- README refreshed for the 240-toy shop, both shift modes, the tool table and the `Q` key. `catalog_thumbnail.png` needed no regeneration: it is a crop of the title art, not a screenshot of the menu, so the new buttons do not appear in it.

## Engineering

- Balance the five tools against measured pacing. Over a whole 240-toy shift: bare-handed 28.0 min, trolley+scanner 21.1, fully equipped 16.8 — so a full toolset is worth about 40% of the run. The trolley now clearly pays (it used to measure *worse* than bare hands); the speed tools stack on top.
- The replay closer used to walk every toy to the **first** display of its category, ignoring the other three. That fills a quarter of the shop and then spins, carrying each remaining toy to a shelf that has been full for hours and dropping it again — 9700 wasted actions in an 8000-action shift. Every "the shop is unfinishable" number before this was measuring that, not the game. `home_display_index` now picks the nearest matching display *with room*, and `the_closer_clears_the_floor_without_misshelving` asserts the run ends because it ran out of work rather than out of budget.
- **Placement now runs through real aiming** (`aim_and_place`): the closer walks to a gap, faces it, and presses `E` only once `interaction_preview` says it will shelve — so a run is charged for putting each toy away, not just for finding it. With a trolley that is a separate walk per toy in the armful, because the slot just filled is no longer the one the crosshair offers. Doing this uncovered two `interact` ordering bugs and the back-row shadowing above.
- The "crosshair misses half the time" reading was **two different things added together**, and the report now splits them:
  - *Whiffed* — the crosshair delivered nothing. Bare-handed this is **zero**; the cone is fine. What produced 175 whiffs was the Sorting Trolley having no input path at all (fixed: `E` on a loose toy now loads the armful when there is room). The 66 that remain are the closer aiming at a toy that sits in front of a free shelf slot, where `E` correctly shelves instead — real friction, but the right precedence.
  - *Neighbour* — the crosshair handed over a different toy than the one aimed at, ~175–195 of 400. The closer still walks away holding a toy, so this is pile texture rather than lost work, and it is arguably the point of a floor buried in toys. Leaving as-is.
