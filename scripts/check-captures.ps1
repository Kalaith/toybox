<#
.SYNOPSIS
    Reports whether the committed verification screenshots still match the game.

.DESCRIPTION
    Re-captures every whole-store scene into a temporary directory and compares
    each against docs\verification\ui_<scene>.png, failing if any drifted. Read
    only: it never overwrites a committed image, so a drift is something you
    look at and then decide about.

    This exists because the committed gallery goes stale silently. Fifty toy
    captures sat three weeks out of date behind two commits that changed the
    renderers, and nothing said so — the only check was a person remembering to
    look.

    Drift is measured as the percentage of differing pixels, not a hash. Two
    scenes are not byte-reproducible: rendering no longer reads the wall clock,
    which fixed seven of nine, but mid_run and carrying_a_half_scanned still
    wobble by about 0.013% of pixels on anti-aliased text edges. The default
    threshold sits an order of magnitude above that noise and far below any real
    change, since a moved label or a resized panel repaints tens of thousands of
    pixels.

    Needs Python with Pillow for the comparison.

.EXAMPLE
    ./scripts/check-captures.ps1                 # every scene
    ./scripts/check-captures.ps1 -SkipBuild      # reuse the last release build
    ./scripts/check-captures.ps1 -Scenes mid_run,tool_shop
#>
param(
    [string[]]$Scenes = @(
        "gameplay", "title", "mid_run", "closing_soon", "shift_over",
        "tool_shop", "checkout", "lamp_contrast", "repair_bench",
        "carrying_a_half", "carrying_a_half_scanned", "broken_lineup"
    ),
    [double]$MaxDiffPercent = 0.10,
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
Push-Location $gameDir
try {
    $reference = Join-Path $gameDir "docs\verification"
    # Relative on purpose: the shared capture script resolves -OutputDir against
    # the game directory, so an absolute temp path gets concatenated onto it and
    # fails. dist\ is already gitignored and already holds build output.
    $relative = "dist\capture-check"
    $candidate = Join-Path $gameDir $relative
    New-Item -ItemType Directory -Force $candidate | Out-Null

    try {
        Write-Host "Capturing $($Scenes.Count) scene(s) for comparison..."
        & (Join-Path $PSScriptRoot "capture_scene.ps1") -Scenes $Scenes -OutputDir $relative -SkipBuild:$SkipBuild | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "capture failed" }

        Write-Host ""
        & python (Join-Path $PSScriptRoot "compare_captures.py") $reference $candidate $MaxDiffPercent @Scenes
        $comparison = $LASTEXITCODE

        Write-Host ""
        if ($comparison -eq 0) {
            Write-Host "All captures match within $MaxDiffPercent%."
        }
        exit $comparison
    }
    finally {
        Remove-Item $candidate -Recurse -Force -ErrorAction SilentlyContinue
    }
}
finally {
    Pop-Location
}
