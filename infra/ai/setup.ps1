# ComfyUI bootstrap — MAZAYA-STUDIO
# Run from repo root: powershell -File infra/ai/setup.ps1

param(
    [string]$InstallDir = "$PSScriptRoot/ComfyUI",
    [string]$CheckpointUrl = ""
)

$ErrorActionPreference = "Stop"

Write-Host "PixelForge ComfyUI setup -> $InstallDir"

if (-not (Test-Path $InstallDir)) {
    Write-Host "Download ComfyUI Windows portable from:"
    Write-Host "  https://github.com/comfyanonymous/ComfyUI/releases"
    Write-Host "Extract to: $InstallDir"
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

$models = Join-Path $InstallDir "models/checkpoints"
New-Item -ItemType Directory -Force -Path $models | Out-Null

$ckpt = Join-Path $models "pixel_dream_v1.0.safetensors"
if (-not (Test-Path $ckpt)) {
    Write-Host "Place pixel_dream v1.0 checkpoint at:"
    Write-Host "  $ckpt"
    Write-Host "Civitai: https://civitai.com/models/1010709/pixeldream"
}

Write-Host "Launch GPU server:"
Write-Host "  infra\ai\run_directml.bat"
Write-Host "Benchmark:"
Write-Host "  python infra/ai/benchmark.py"
