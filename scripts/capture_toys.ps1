<#
.SYNOPSIS
    Captures a 4-view gallery screenshot (front/right/back/top) of every
    procedural toy model.

.DESCRIPTION
    Drives every toy identity through one capture process/window.
    The toy list is derived from the draw dispatch in src\toys.rs, so new toys
    are picked up automatically once they can be drawn. PNGs land in
    docs\verification\<toy>.png.

    The game window stays hidden by default. Pass -Visible to watch the gallery
    render while diagnosing a capture.

.EXAMPLE
    ./scripts/capture_toys.ps1                      # all toys
    ./scripts/capture_toys.ps1 -Toys bear,duck      # just these two
    ./scripts/capture_toys.ps1 -SkipBuild           # reuse the last build
    ./scripts/capture_toys.ps1 -Toys bear -Visible  # watch one capture
#>
param(
    [string[]]$Toys = @(),
    [int]$Frames = 5,
    [string]$OutputDir = "docs\verification",
    [switch]$SkipBuild,
    [switch]$Visible
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$captureEnvNames = @(
    "TOYBOX_CAPTURE_MANIFEST",
    "TOYBOX_CAPTURE_FRAMES",
    "TOYBOX_HEADLESS"
)
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
        # The draw dispatch in src\toys.rs is the source of truth for what is a
        # toy: "ToyIdentity::Bear => bear::draw(...)". Listing src\toys\*.rs and
        # subtracting a hardcoded set of helpers went stale the moment one was
        # added — part_accents.rs was swept as a toy, the harness rendered a
        # bear captioned UNKNOWN TOY, and this script reported it as "ok".
        # Deriving from the dispatch means a new helper is excluded for free and
        # a new toy is picked up the moment it can actually be drawn.
        $dispatch = Get-Content "src\toys.rs" -Raw
        $Toys = [regex]::Matches($dispatch, 'ToyIdentity::\w+\s*=>\s*(\w+)::draw') |
            ForEach-Object { $_.Groups[1].Value } |
            Sort-Object -Unique
        if ($Toys.Count -eq 0) {
            throw "no toy modules found in the src\toys.rs draw dispatch"
        }
    }

    New-Item -ItemType Directory -Force $OutputDir | Out-Null

    $manifest = Join-Path (Resolve-Path $OutputDir) ".capture_manifest_$PID.tsv"
    $rows = @()
    foreach ($toy in $Toys) {
        $out = Join-Path (Resolve-Path $OutputDir) "$toy.png"
        if (Test-Path $out) { Remove-Item $out -Confirm:$false }
        $rows += "toy_gallery:$toy`t$out"
    }
    Set-Content -LiteralPath $manifest -Value $rows -Encoding utf8

    $env:TOYBOX_CAPTURE_MANIFEST = $manifest
    $env:TOYBOX_CAPTURE_FRAMES = "$Frames"
    $env:TOYBOX_HEADLESS = $(if ($Visible) { "0" } else { "1" })
    $startArgs = @{ FilePath = $exe; PassThru = $true }
    if (-not $Visible) { $startArgs.WindowStyle = "Hidden" }
    $proc = Start-Process @startArgs
    Write-Host ("Capturing {0} toys in one process (PID {1})..." -f $Toys.Count, $proc.Id)
    $proc.WaitForExit()
    if ($proc.ExitCode -ne 0) { throw "capture process exited with code $($proc.ExitCode)" }

    $failed = @()
    foreach ($toy in $Toys) {
        $out = Join-Path (Resolve-Path $OutputDir) "$toy.png"
        # A solid-black 1280x720 PNG is ~19 KB; a real 4-view render is far
        # larger. The floor catches black/blank captures early.
        if ((Test-Path $out) -and (Get-Item $out).Length -ge 25000) {
            Write-Host ("  ok    {0}  ({1:N0} bytes)" -f $toy, (Get-Item $out).Length)
        } else {
            Write-Warning "FAILED $toy (missing or suspiciously small PNG)"
            $failed += $toy
        }
    }

    Write-Host ""
    Write-Host ("Captured {0}/{1} toys -> {2}" -f ($Toys.Count - $failed.Count), $Toys.Count, $OutputDir)
    if ($failed.Count -gt 0) {
        Write-Warning ("Failed: " + ($failed -join ", "))
        exit 1
    }
}
finally {
    if ($manifest) { Remove-Item -LiteralPath $manifest -Force -ErrorAction SilentlyContinue }
    foreach ($name in $captureEnvNames) {
        Remove-Item "env:$name" -ErrorAction SilentlyContinue
    }
    Pop-Location
}
