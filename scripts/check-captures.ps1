<#
.SYNOPSIS
    Reports whether the committed verification screenshots still match the game.

.DESCRIPTION
    Re-captures the whole-store scenes and the 50-toy gallery into a temporary
    directory and compares each against docs\verification\, failing if any
    drifted. Read only: it never overwrites a committed image, so a drift is
    something you look at and then decide about.

    This exists because the committed gallery goes stale silently. Fifty toy
    captures sat three weeks out of date behind two commits that changed the
    renderers, and nothing said so — the only check was a person remembering to
    look.

    Drift is measured as the percentage of differing pixels, not a hash. The toy
    gallery is byte-reproducible, but two scenes are not: rendering no longer
    reads the wall clock, which fixed seven of nine, while mid_run and
    carrying_a_half_scanned still wobble by about 0.05% of pixels on
    anti-aliased text edges. The default threshold sits an order of magnitude
    above that noise and far below any real change, since a moved label or a
    resized panel repaints tens of thousands of pixels — a stale reference
    caught during development measured 3.2%.

    Needs Python with Pillow for the comparison.

.EXAMPLE
    ./scripts/check-captures.ps1                 # scenes and toys
    ./scripts/check-captures.ps1 -SkipBuild      # reuse the last builds
    ./scripts/check-captures.ps1 -ScenesOnly     # skip the 50 toy launches
#>
param(
    [string[]]$Scenes = @(
        "gameplay", "title", "mid_run", "closing_soon", "shift_over",
        "tool_shop", "checkout", "lamp_contrast", "repair_bench",
        "carrying_a_half", "carrying_a_half_scanned", "broken_lineup"
    ),
    [double]$MaxDiffPercent = 0.10,
    # The toy gallery is byte-reproducible, unlike two of the scenes, so it gets
    # a threshold ten times tighter. Spending the scenes' noise allowance on it
    # would waste real sensitivity: moving one primitive in the bear by 0.06
    # registers as 0.179%, which a 0.1% gate only just catches.
    [double]$ToyMaxDiffPercent = 0.01,
    [switch]$ScenesOnly,
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
        $compare = Join-Path $PSScriptRoot "compare_captures.py"
        $failed = 0
        $checked = 0

        Write-Host "Capturing $($Scenes.Count) scene(s)..."
        & (Join-Path $PSScriptRoot "capture_scene.ps1") -Scenes $Scenes -OutputDir $relative -SkipBuild:$SkipBuild | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "scene capture failed" }
        $sceneFiles = $Scenes | ForEach-Object { "ui_$_.png" }

        if (-not $ScenesOnly) {
            Write-Host "Capturing the toy gallery..."
            $toyOut = Join-Path $relative "toys"
            & (Join-Path $PSScriptRoot "capture_toys.ps1") -OutputDir $toyOut -SkipBuild:$SkipBuild | Out-Null
            if ($LASTEXITCODE -ne 0) { throw "toy capture failed" }
            $toyFiles = Get-ChildItem (Join-Path $candidate "toys\*.png") |
                ForEach-Object { "toys/$($_.Name)" }
        } else {
            $toyFiles = @()
        }

        Write-Host ""
        Write-Host "Scenes (threshold $MaxDiffPercent%):"
        & python $compare $reference $candidate $MaxDiffPercent @sceneFiles
        if ($LASTEXITCODE -ne 0) { $failed = 1 }
        $checked += $sceneFiles.Count

        if ($toyFiles.Count -gt 0) {
            Write-Host ""
            Write-Host "Toy gallery (threshold $ToyMaxDiffPercent%):"
            & python $compare $reference $candidate $ToyMaxDiffPercent @toyFiles
            if ($LASTEXITCODE -ne 0) { $failed = 1 }
            $checked += $toyFiles.Count
        }

        Write-Host ""
        if ($failed -eq 0) {
            Write-Host ("All {0} captures match." -f $checked)
        }
        exit $failed
    }
    finally {
        Remove-Item $candidate -Recurse -Force -ErrorAction SilentlyContinue
    }
}
finally {
    Pop-Location
}
