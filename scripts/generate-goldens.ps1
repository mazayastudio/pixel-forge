#Requires -Version 5.1
$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $RepoRoot

$ase = & (Join-Path $PSScriptRoot "Resolve-Aseprite.ps1")
if (-not $ase) {
    Write-Error "Aseprite not found. Install via Steam or set ASEPRITE_CLI."
}

$fixturesDir = Join-Path $RepoRoot "tests\parity\fixtures"
$goldenDir = Join-Path $RepoRoot "tests\parity\golden"
$manifestPath = Join-Path $fixturesDir "manifest.json"
New-Item -ItemType Directory -Force -Path $goldenDir | Out-Null

$manifest = Get-Content $manifestPath -Raw | ConvertFrom-Json
$hashes = @()
$count = 0
$failed = @()

foreach ($entry in $manifest.fixtures) {
    $fixturePath = Join-Path $fixturesDir $entry.file
    if (-not (Test-Path $fixturePath)) {
        Write-Warning "SKIP golden (missing): $($entry.file)"
        continue
    }
    $base = [System.IO.Path]::GetFileNameWithoutExtension($entry.file)
    $outPng = Join-Path $goldenDir "$base.png"
    & $ase -b $fixturePath --frame-range 0,0 --save-as $outPng 2>&1 | Out-Null
    if (-not (Test-Path $outPng)) {
        $failed += $entry.file
        continue
    }
    $hash = (Get-FileHash $fixturePath -Algorithm SHA256).Hash
    $hashes += "$hash  $($entry.file)"
    $count++
    Write-Host "golden: $base.png"
}

$hashPath = Join-Path $goldenDir "manifest.sha256"
$hashes | Set-Content -Path $hashPath -Encoding utf8
Write-Host "Wrote $count goldens to $goldenDir"
Write-Host "Wrote $hashPath"

if ($failed.Count -gt 0) {
    throw "Golden export failed for: $($failed -join ', ')"
}
