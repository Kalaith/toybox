"""Compare freshly taken captures against the committed reference images.

Used by scripts/check-captures.ps1. Prints one line per scene and exits
non-zero if any scene drifted.

Why a percentage rather than a hash: the scenes are not quite byte-reproducible.
Rendering no longer reads the wall clock (see
`rendering_animates_off_the_simulation_clock`), which settled almost all of it,
but `carrying_armful` still wobbles a few pixels per capture and occasionally
~300, clustered on anti-aliased text edges.

Why `--min-delta` on top of that: a plain count could not separate the two
failure modes after all. That wobble reaches 0.035% at its worst, while a real
change of the kind that matters — adding a 12px caption to the HUD — measured
0.019% and slipped straight through a 0.1% gate. No single any-pixel threshold
sits between them.

Channel *contrast* does separate them, cleanly. Measured over five repeat
captures, the noise never exceeds a channel delta of 39 and vanishes entirely
above 64. A real HUD text change (mid_run vs relaxed_run, which differ only in
the clock digits and the LEFT/ELAPSED caption) puts **838 pixels** past delta
64. Counting only pixels above `--min-delta` therefore drops the scene noise
floor to a flat 0.000% and lets the threshold come down an order of magnitude.

An earlier version of this file claimed the 39-delta noise "rules out a delta
tolerance". That was drawn from measuring the noise alone; the signal sits far
above it, which is exactly what makes the tolerance work.

The toy gallery deliberately keeps `--min-delta 0`. Those captures are exact, so
any-pixel counting is the most sensitive test available, and a subtle geometry
shift moves a smooth-shaded edge by *small* deltas that a contrast gate would
discard.
"""

from __future__ import annotations

import sys
from pathlib import Path

try:
    from PIL import Image
except ImportError:  # pragma: no cover - environment problem, not a drift
    print("compare_captures.py needs Pillow: pip install pillow", file=sys.stderr)
    raise SystemExit(2)


def differing_fraction(
    reference: Path, candidate: Path, min_delta: int
) -> tuple[float, int]:
    """Fraction of pixels differing by more than `min_delta` on any channel.

    Also returns the worst delta seen anywhere, including below the gate, so a
    scene sitting just under it still says so in the report.
    """
    with Image.open(reference) as ref_img, Image.open(candidate) as new_img:
        ref = ref_img.convert("RGB")
        new = new_img.convert("RGB")
        if ref.size != new.size:
            return 1.0, 255
        ref_px = ref.load()
        new_px = new.load()
        width, height = ref.size
        differing = 0
        worst = 0
        for y in range(height):
            for x in range(width):
                a = ref_px[x, y]
                b = new_px[x, y]
                if a == b:
                    continue
                delta = max(abs(a[c] - b[c]) for c in range(3))
                worst = max(worst, delta)
                if delta > min_delta:
                    differing += 1
    return differing / (width * height), worst


def main() -> int:
    argv = sys.argv[1:]
    min_delta = 0
    if "--min-delta" in argv:
        index = argv.index("--min-delta")
        min_delta = int(argv[index + 1])
        del argv[index : index + 2]

    if len(argv) < 3:
        print(
            "usage: compare_captures.py [--min-delta N] <reference_dir> "
            "<candidate_dir> <max_percent> [relative_png ...]",
            file=sys.stderr,
        )
        return 2

    reference_dir = Path(argv[0])
    candidate_dir = Path(argv[1])
    max_percent = float(argv[2])
    # Relative paths rather than bare names, so the same comparison serves the
    # whole-store scenes (`ui_mid_run.png`) and the toy gallery
    # (`toys/bear.png`) without either script knowing the other's layout.
    names = argv[3:]

    drifted: list[str] = []
    missing: list[str] = []

    for filename in names:
        label = filename.removeprefix("ui_").removesuffix(".png")
        reference = reference_dir / filename
        candidate = candidate_dir / filename
        if not candidate.exists():
            print(f"  ??    {label:<24} capture missing")
            missing.append(label)
            continue
        if not reference.exists():
            print(f"  NEW   {label:<24} no committed reference")
            drifted.append(label)
            continue

        fraction, worst = differing_fraction(reference, candidate, min_delta)
        percent = fraction * 100.0
        if percent > max_percent:
            print(f"  DRIFT {label:<24} {percent:6.3f}% of pixels (max delta {worst})")
            drifted.append(label)
        else:
            print(f"  ok    {label:<24} {percent:6.3f}% of pixels")

    if missing:
        print(f"\ncapture failed for: {', '.join(missing)}", file=sys.stderr)
    if drifted:
        print(
            f"\n{len(drifted)} scene(s) drifted past {max_percent}%: "
            f"{', '.join(drifted)}\n"
            "Re-run scripts\\capture_scene.ps1, look at the new images, and "
            "commit them if the change was intended.",
            file=sys.stderr,
        )
    return 1 if (drifted or missing) else 0


if __name__ == "__main__":
    raise SystemExit(main())
