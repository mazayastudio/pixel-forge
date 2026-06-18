#Requires -Version 5.1
<#
.SYNOPSIS
  Resolve Aseprite executable path to $env:ASEPRITE_CLI (does not exit).
#>
param([switch]$Quiet)

function Write-Info($msg) { if (-not $Quiet) { Write-Host $msg } }

$candidates = @()
if ($env:ASEPRITE_CLI) { $candidates += $env:ASEPRITE_CLI }
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
foreach ($path in $candidates | Select-Object -Unique) {
    if ($path -and (Test-Path -LiteralPath $path)) {
        $env:ASEPRITE_CLI = $path
        Write-Info "Aseprite: $path"
        return $path
    }
}
$cmd = Get-Command aseprite -ErrorAction SilentlyContinue
if ($cmd) {
    $env:ASEPRITE_CLI = $cmd.Source
    Write-Info "Aseprite: $($cmd.Source)"
    return $cmd.Source
}
return $null
