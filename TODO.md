# TODO — Toybox After Hours

## Repair flow at scale

- The cross-zone scatter is **deliberate** — hunting scattered halves and rejoining them is a pillar of the game alongside sorting, and must not be shortened to make repairs cheaper. Measured on its own terms in `state/tests/replay.rs` (`Strategy::Restorer`), the errand is fully winnable: 100 repairs in 200 actions with nothing deferred, at ~55m walked and ~19s per pair. The earlier "zero repairs" figure came from a nearest-first closer, which measures the sorting loop and never runs the errand at all.
- Repairs are gated on owning the Toy Scanner in practice — without it there is no way to learn where a carried part's other half went, so the errand is a blind search. Decide whether that gate is intended, or whether an unaided player needs some weaker signal.
- Mistake penalty and timer are still untuned against `assets/data/*.json`.
- Repair parts render per *category* but not per *identity* — every plush head is the same model, so a broken Bear and a broken Octopus are indistinguishable while split. Decide whether identity-level parts are worth 50×2 renderers or whether identity-derived detail on the existing 10 is enough.

## Game loop and progression

- Rework the finish condition and finish screen around zone milestones now that per-zone completion exists (`GameSession::zone_progress`). Note a zone caps at ~88% until its broken toys are repaired.
- **Decide an intended run length — the current one is far too long.** Replay measures ~342 toys shelved per 400 actions at 18.8 min; the full 4000-toy shop extrapolates to roughly 3.7 hours, and no display completes in the first 400 actions. Either cut `toy_count`, raise display throughput, or make a shift a slice of the shop.
- Give a run a clear arc: an opening-time deadline with a score screen (toys shelved, repairs, mistakes, zones done) plus a relaxed untimed mode.

## Polish

- Refresh the README and controls once the full loop lands, and regenerate `catalog_thumbnail.png` if the title screen changes.

## Engineering

- Balance the five tools against measured pacing. Now that pickups run through real crosshair input, the earlier "the Sorting Trolley is probably too strong" reading is **wrong** — that came from assuming every pickup succeeds. Per minute over 400 actions: bare-handed 8.3 toys, trolley+scanner 7.9, fully equipped 9.7. The trolley alone is roughly neutral, because gathering a same-display armful means targeting *specific* toys and the crosshair misses most of them. The speed tools are what actually pay.
- Placement still goes through `place_active_toy` rather than aiming at a shelf slot, so the numbers cover the cost of *finding* a toy but not of *placing* it.
- **Crosshair pickup misses roughly half the time** in a full shop (193 of 400 attempts bare-handed, 264 of 400 when gathering a same-display armful). That is measured against a closer that walks to arm's length and aims straight at the toy, so it is a floor, not a worst case. Worth deciding whether that is acceptable friction or whether pickup wants a wider cone / a nearest-toy fallback when the crosshair is empty.
