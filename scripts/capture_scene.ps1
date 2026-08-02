<#
.SYNOPSIS
    Captures the whole-store screenshot scenes (gameplay, repair_bench).

.DESCRIPTION
    The shared ..\macroquad-toolkit\scripts\capture_ui.ps1 runs the *debug*
    exe, which is unusable here: the store stocks ~4500 toys, and an unoptimised
    build spends minutes per capture spawning and drawing them. Runs at 150 and
    12 frames were both killed before writing a PNG.

    This wrapper does the same job against the release build instead. Use
    ..\macroquad-toolkit\scripts\capture_ui.ps1 for games small enough that the
    debug build is fine, and scripts\capture_toys.ps1 for the one-toy gallery
    scene (fast enough in debug).

.EXAMPLE
    ./scripts/capture_scene.ps1                          # every scene
    ./scripts/capture_scene.ps1 -Scenes repair_bench     # just one
    ./scripts/capture_scene.ps1 -SkipBuild               # reuse the last build
#>
param(
    [string[]]$Scenes = @("gameplay", "repair_bench"),
    [int]$Frames = 30,
    [string]$OutputDir = "docs\verification",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
Push-Location $gameDir
try {
    if (-not $SkipBuild) {
        cargo build --release
        if ($LASTEXITCODE -ne 0) { throw "cargo build --release failed" }
    }

    $metadata = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json
    $exe = Join-Path $metadata.target_directory "release\toybox_after_hours.exe"
    if (-not (Test-Path $exe)) { throw "exe not found: $exe (build first?)" }

    New-Item -ItemType Directory -Force $OutputDir | Out-Null

    $failed = @()
    foreach ($scene in $Scenes) {
        $out = Join-Path (Resolve-Path $OutputDir) "ui_$scene.png"
        if (Test-Path $out) { Remove-Item $out -Confirm:$false }

        $env:TOYBOX_CAPTURE_PATH = $out
        $env:TOYBOX_CAPTURE_SCENE = $scene
        $env:TOYBOX_CAPTURE_FRAMES = "$Frames"
        & $exe | Out-Null

        # A solid-black 1280x720 PNG is ~19 KB; a real shop render is far
        # larger. The floor catches black/blank captures early.
        if ((Test-Path $out) -and (Get-Item $out).Length -ge 40000) {
            Write-Host ("  ok    {0}  ({1:N0} bytes)" -f $scene, (Get-Item $out).Length)
        } else {
            Write-Warning "FAILED $scene (missing or suspiciously small PNG)"
            $failed += $scene
        }
    }

    foreach ($name in "TOYBOX_CAPTURE_PATH", "TOYBOX_CAPTURE_SCENE", "TOYBOX_CAPTURE_FRAMES") {
        Remove-Item "env:$name" -ErrorAction SilentlyContinue
    }

    Write-Host ""
    Write-Host ("Captured {0}/{1} scenes -> {2}" -f ($Scenes.Count - $failed.Count), $Scenes.Count, $OutputDir)
    if ($failed.Count -gt 0) {
        Write-Warning ("Failed: " + ($failed -join ", "))
        exit 1
    }
}
finally {
    Pop-Location
}
