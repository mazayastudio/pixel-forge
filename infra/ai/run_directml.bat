@echo off
REM PixelForge — ComfyUI DirectML launcher for MAZAYA-STUDIO (RX 5600 XT, 4GB VRAM)
REM Place ComfyUI portable in infra/ai/ComfyUI/ (gitignored) or set COMFYUI_ROOT

setlocal
if not defined COMFYUI_ROOT set COMFYUI_ROOT=%~dp0ComfyUI
if not exist "%COMFYUI_ROOT%\main.py" (
  echo ComfyUI not found at %COMFYUI_ROOT%
  echo Download: https://github.com/comfyanonymous/ComfyUI/releases
  exit /b 1
)

cd /d "%COMFYUI_ROOT%"
python main.py ^
  --directml ^
  --lowvram ^
  --force-fp16 ^
  --use-split-cross-attention ^
  --listen 127.0.0.1 ^
  --port 8188

endlocal
