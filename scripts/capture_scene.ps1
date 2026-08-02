<#
.SYNOPSIS
    Captures the whole-store screenshot scenes (gameplay, repair_bench,
    mid_run, tool_shop).

.DESCRIPTION
    A thin wrapper over the shared ..\macroquad-toolkit\scripts\capture_ui.ps1,
    passing -Release and this game's scene list.

    The release build is not optional here: the store stocks ~4500 loose toys,
    and an unoptimised build spends minutes per capture spawning and drawing
    them — debug runs at 150 and 12 frames were both killed before writing a
    PNG. Release captures finish in seconds.

    Use scripts\capture_toys.ps1 for the one-toy gallery scene, which is small
    enough that the debug build is fine.

.EXAMPLE
    ./scripts/capture_scene.ps1                          # every scene
    ./scripts/capture_scene.ps1 -Scenes repair_bench     # just one
    ./scripts/capture_scene.ps1 -SkipBuild               # reuse the last build
#>
param(
    [string[]]$Scenes = @("gameplay", "repair_bench", "mid_run", "tool_shop"),
    [int]$Frames = 30,
    [string]$OutputDir = "docs\verification",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$shared = Join-Path (Split-Path -Parent $gameDir) "macroquad-toolkit\scripts\capture_ui.ps1"
if (-not (Test-Path -LiteralPath $shared)) { throw "Shared capture script not found: $shared" }

# -Prefix is not optional: the shared script derives it from the package name
# (toybox_after_hours -> TOYBOX_AFTER_HOURS), but the game reads TOYBOX_*.
# Without it the capture vars never match, `CaptureConfig::from_env` returns
# None, and the exe launches as a normal interactive game that never exits.
& $shared -GameDir $gameDir -Prefix "TOYBOX" -Scenes $Scenes -Frames $Frames `
    -OutputDir $OutputDir -Release -SkipBuild:$SkipBuild
