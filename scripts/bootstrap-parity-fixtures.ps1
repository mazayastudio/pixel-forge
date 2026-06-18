#Requires -Version 5.1
<#
.SYNOPSIS
  Step 1 bootstrap: generate all parity fixtures and goldens.
#>
$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $RepoRoot

Write-Host "==> Aseprite check" -ForegroundColor Cyan
$ase = & (Join-Path $PSScriptRoot "Resolve-Aseprite.ps1")
if (-not $ase) { throw "Aseprite not found (Steam install or ASEPRITE_CLI required)" }

Write-Host "`n==> Rust fixtures" -ForegroundColor Cyan
cargo run -p pixelforge-core --bin generate_fixtures -- tests/parity/fixtures
if ($LASTEXITCODE -ne 0) { throw "generate_fixtures failed" }

Write-Host "`n==> Aseprite Lua fixtures" -ForegroundColor Cyan
& (Join-Path $PSScriptRoot "generate-aseprite-fixtures.ps1")

Write-Host "`n==> Golden PNGs" -ForegroundColor Cyan
& (Join-Path $PSScriptRoot "generate-goldens.ps1")

Write-Host "`n==> Parity harness" -ForegroundColor Cyan
python tests/parity/run_parity.py --list-cases
python tests/parity/run_parity.py --fixture-count
if ($LASTEXITCODE -ne 0) { throw "fixture-count gate failed" }

python tests/parity/run_parity.py --target wasm
Write-Host "`nStep 1 verification gate: PASS" -ForegroundColor Green
