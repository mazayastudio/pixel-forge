#Requires -Version 5.1
$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $RepoRoot

. (Join-Path $PSScriptRoot "Resolve-Aseprite.ps1") | Out-Null
$ase = & (Join-Path $PSScriptRoot "Resolve-Aseprite.ps1")
if (-not $ase) {
    Write-Error "Aseprite not found. Install via Steam or set ASEPRITE_CLI."
}

$fixturesDir = Join-Path $RepoRoot "tests\parity\fixtures"
$env:PIXELFORGE_FIXTURES_DIR = $fixturesDir
$script = Join-Path $RepoRoot "scripts\aseprite-fixtures\generate_all.lua"

Write-Host "Running Aseprite Lua fixture generator..."
Write-Host "  $ase -b --script $script"
$log = & $ase -b --script $script 2>&1
$log | ForEach-Object { Write-Host $_ }
if ($log -match "error|Error:") {
    throw "Aseprite Lua script reported errors (see output above)"
}
Write-Host "Aseprite fixtures generated in $fixturesDir"
