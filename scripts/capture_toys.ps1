<#
.SYNOPSIS
    Captures a 4-view gallery screenshot (front/right/back/left) of every
    procedural toy model.

.DESCRIPTION
    Drives the TOYBOX_CAPTURE_* screenshot harness once per toy identity.
    The toy list is derived from src\toys\*.rs (module names double as toy
    slugs), so new toys are picked up automatically. PNGs land in
    docs\verification\toys\<toy>.png.

.EXAMPLE
    ./scripts/capture_toys.ps1                      # all toys
    ./scripts/capture_toys.ps1 -Toys bear,duck      # just these two
    ./scripts/capture_toys.ps1 -SkipBuild           # reuse the last build
#>
param(
    [string[]]$Toys = @(),
    [int]$Frames = 5,
    [string]$OutputDir = "docs\verification\toys",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
Push-Location $gameDir
try {
    if (-not $SkipBuild) {
        cargo build
        if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }
    }

    $metadata = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json
    $exe = Join-Path $metadata.target_directory "debug\toybox_after_hours.exe"
    if (-not (Test-Path $exe)) { throw "exe not found: $exe (build first?)" }

    if ($Toys.Count -eq 0) {
        $shared = @("library", "repair_parts")
        $Toys = Get-ChildItem "src\toys\*.rs" |
            ForEach-Object { $_.BaseName } |
            Where-Object { $shared -notcontains $_ } |
            Sort-Object
    }

    New-Item -ItemType Directory -Force $OutputDir | Out-Null

    $failed = @()
    foreach ($toy in $Toys) {
        $out = Join-Path (Resolve-Path $OutputDir) "$toy.png"
        if (Test-Path $out) { Remove-Item $out -Confirm:$false }

        $env:TOYBOX_CAPTURE_PATH = $out
        $env:TOYBOX_CAPTURE_SCENE = "toy_gallery"
        $env:TOYBOX_CAPTURE_TOY = $toy
        $env:TOYBOX_CAPTURE_FRAMES = "$Frames"
        & $exe | Out-Null

        # A solid-black 1280x720 PNG is ~19 KB; a real 4-view render is far
        # larger. The floor catches black/blank captures early.
        if ((Test-Path $out) -and (Get-Item $out).Length -ge 25000) {
            Write-Host ("  ok    {0}  ({1:N0} bytes)" -f $toy, (Get-Item $out).Length)
        } else {
            Write-Warning "FAILED $toy (missing or suspiciously small PNG)"
            $failed += $toy
        }
    }

    foreach ($name in "TOYBOX_CAPTURE_PATH", "TOYBOX_CAPTURE_SCENE", "TOYBOX_CAPTURE_TOY", "TOYBOX_CAPTURE_FRAMES") {
        Remove-Item "env:$name" -ErrorAction SilentlyContinue
    }

    Write-Host ""
    Write-Host ("Captured {0}/{1} toys -> {2}" -f ($Toys.Count - $failed.Count), $Toys.Count, $OutputDir)
    if ($failed.Count -gt 0) {
        Write-Warning ("Failed: " + ($failed -join ", "))
        exit 1
    }
}
finally {
    Pop-Location
}
