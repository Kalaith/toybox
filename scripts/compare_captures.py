"""Compare freshly taken captures against the committed reference images.

Used by scripts/check-captures.ps1. Prints one line per scene and exits
non-zero if any scene drifted.

Why a percentage rather than a hash: two of the nine scenes are not quite
byte-reproducible. Rendering no longer reads the wall clock (see
`rendering_animates_off_the_simulation_clock`), which fixed seven of them, but
`mid_run` and `carrying_a_half_scanned` still wobble by ~109 pixels of 860,784
— 0.013% — clustered on anti-aliased text edges, with channel deltas as high as
39. That rules out a delta tolerance and leaves a count tolerance, which works
because the two failure modes are far apart: font-edge noise moves a hundred
pixels, while a moved label, a resized panel or a changed colour moves tens of
thousands.
"""

from __future__ import annotations

import sys
from pathlib import Path

try:
    from PIL import Image
except ImportError:  # pragma: no cover - environment problem, not a drift
    print("compare_captures.py needs Pillow: pip install pillow", file=sys.stderr)
    raise SystemExit(2)


def differing_fraction(reference: Path, candidate: Path) -> tuple[float, int]:
    """Fraction of pixels that differ at all, and the worst channel delta."""
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
                if a != b:
                    differing += 1
                    worst = max(worst, max(abs(a[c] - b[c]) for c in range(3)))
    return differing / (width * height), worst


def main() -> int:
    if len(sys.argv) < 4:
        print(
            "usage: compare_captures.py <reference_dir> <candidate_dir> "
            "<max_percent> [relative_png ...]",
            file=sys.stderr,
        )
        return 2

    reference_dir = Path(sys.argv[1])
    candidate_dir = Path(sys.argv[2])
    max_percent = float(sys.argv[3])
    # Relative paths rather than bare names, so the same comparison serves the
    # whole-store scenes (`ui_mid_run.png`) and the toy gallery
    # (`toys/bear.png`) without either script knowing the other's layout.
    names = sys.argv[4:]

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

        fraction, worst = differing_fraction(reference, candidate)
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
