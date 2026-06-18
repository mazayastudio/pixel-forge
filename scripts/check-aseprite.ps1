#Requires -Version 5.1
<#
.SYNOPSIS
  Resolve Aseprite CLI path and verify v1.3.17.x baseline.

.EXIT
  0 if Aseprite found; 1 if not found.
#>
param(
    [switch]$Quiet
)

$ErrorActionPreference = "Stop"

function Write-Info($msg) { if (-not $Quiet) { Write-Host $msg } }
function Write-Warn($msg) { if (-not $Quiet) { Write-Warning $msg } }

function Test-AsepriteExe([string]$Path) {
    return $Path -and (Test-Path -LiteralPath $Path)
}

function Get-AsepriteVersion([string]$Exe) {
    $flags = @("-version", "--version", "-v")
    foreach ($flag in $flags) {
        try {
            $out = & $Exe $flag 2>&1 | Out-String
            if ($out.Trim()) { return $out.Trim() }
        } catch {
            continue
        }
    }
    return $null
}

$candidates = @()

if ($env:ASEPRITE_CLI) {
    $candidates += $env:ASEPRITE_CLI
}

$candidates += @(
    "${env:ProgramFiles}\Aseprite\Aseprite.exe",
    "${env:ProgramFiles(x86)}\Aseprite\Aseprite.exe",
    "$env:LOCALAPPDATA\Programs\Aseprite\Aseprite.exe"
)

$steamRoot = "C:\Program Files (x86)\Steam\steamapps\common"
if (Test-Path $steamRoot) {
    Get-ChildItem -Path $steamRoot -Filter "Aseprite.exe" -Recurse -ErrorAction SilentlyContinue |
        ForEach-Object { $candidates += $_.FullName }
}

$aseprite = $null
foreach ($path in $candidates | Select-Object -Unique) {
    if (Test-AsepriteExe $path) {
        $aseprite = $path
        break
    }
}

if (-not $aseprite) {
    $cmd = Get-Command aseprite -ErrorAction SilentlyContinue
    if ($cmd) { $aseprite = $cmd.Source }
}

if (-not $aseprite) {
    Write-Warn "Aseprite CLI not found. Set ASEPRITE_CLI or install Aseprite v1.3.17.x."
    Write-Info "See tests/parity/ASEPRITE_BASELINE.md"
    exit 1
}

$version = Get-AsepriteVersion $aseprite
Write-Info "Aseprite: $aseprite"
if ($version) {
    Write-Info "Version: $version"
    if ($version -notmatch "1\.3\.17") {
        Write-Warn "Expected Aseprite v1.3.17.x for parity goldens (found: $version)"
    }
} else {
    Write-Warn "Could not read Aseprite version string."
}

exit 0
