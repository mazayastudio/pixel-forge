#Requires -Version 5.1
<#
.SYNOPSIS
  Bootstrap PixelForge dev environment (Step 0 verification gate).

.DESCRIPTION
  Installs Rust stable targets (wasm32, aarch64-linux-android), builds pixelforge-core
  natively and for WASM, runs tests, and optionally checks Aseprite / NDK paths.
#>
param(
    [switch]$SkipOptionalChecks
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $RepoRoot

function Invoke-Step([string]$Label, [scriptblock]$Action) {
    Write-Host "`n==> $Label" -ForegroundColor Cyan
    & $Action
    if ($LASTEXITCODE -and $LASTEXITCODE -ne 0) {
        throw "Step failed ($LASTEXITCODE): $Label"
    }
}

function Ensure-Rustup() {
    $rustup = Get-Command rustup -ErrorAction SilentlyContinue
    if (-not $rustup) {
        Write-Host "rustup not found. Install with:" -ForegroundColor Yellow
        Write-Host "  winget install Rustlang.Rustup"
        Write-Host "Then restart the terminal and re-run this script."
        exit 1
    }
}

Ensure-Rustup

Invoke-Step "Install stable toolchain (rust-toolchain.toml)" {
    rustup toolchain install stable
    rustup default stable
}

Invoke-Step "Add cross-compile targets" {
    rustup target add wasm32-unknown-unknown aarch64-linux-android
}

Invoke-Step "rustup show" { rustup show }

Invoke-Step "Verify wasm32 target installed" {
    $targets = @(rustup target list --installed)
    $hasWasm = $targets | Where-Object { $_ -match '^wasm32-unknown-unknown' }
    if (-not $hasWasm) {
        throw "wasm32-unknown-unknown not in installed targets"
    }
    $targets
}

Invoke-Step "cargo build -p pixelforge-core" {
    cargo build -p pixelforge-core
}

Invoke-Step "cargo test -p pixelforge-core" {
    cargo test -p pixelforge-core
}

Invoke-Step "cargo build -p pixelforge-core --target wasm32-unknown-unknown" {
    cargo build -p pixelforge-core --target wasm32-unknown-unknown
}

Invoke-Step "parity matrix summary" {
    python tests/parity/run_parity.py --summary
}

if (-not $SkipOptionalChecks) {
    Write-Host "`n==> Optional: Android NDK" -ForegroundColor DarkCyan
    if ($env:ANDROID_NDK_HOME) {
        Write-Host "ANDROID_NDK_HOME=$env:ANDROID_NDK_HOME"
        Write-Host "Copy .cargo/config.toml.example to .cargo/config.toml and set linker paths (Step 28)."
    } else {
        Write-Warning "ANDROID_NDK_HOME not set. NDK linker setup deferred to Step 28."
    }

    Write-Host "`n==> Optional: Aseprite baseline" -ForegroundColor DarkCyan
    $checkScript = Join-Path $PSScriptRoot "check-aseprite.ps1"
    if (Test-Path $checkScript) {
        & $checkScript
        if ($LASTEXITCODE -ne 0) {
            Write-Warning "Aseprite not detected - required for Step 1 golden generation."
        }
    }
}

Write-Host "`nStep 0 verification gate: PASS" -ForegroundColor Green
