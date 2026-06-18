# ComfyUI setup — MAZAYA-STUDIO

Install **pixel_dream v1.0** + DirectML ComfyUI for local image generation.

## Prerequisites

- Windows on MAZAYA-STUDIO
- AMD RX 5600 XT (4 GB VRAM)
- Python 3.10+ (bundled with ComfyUI portable)
- [pixel_dream v1.0](https://civitai.com/models/1010709/pixeldream) checkpoint (~3.2 GB)

## Install steps

1. Download [ComfyUI Windows portable](https://github.com/comfyanonymous/ComfyUI/releases) to `infra/ai/ComfyUI/` (gitignored).

2. Install DirectML support per ComfyUI AMD docs (portable build often includes it).

3. Copy checkpoint:
   ```
   ComfyUI/models/checkpoints/pixel_dream_v1.0.safetensors
   ```

4. Copy workflow templates:
   - `workflows/txt2img_pixel_dream.json`
   - `workflows/img2img_photo_to_pixel.json`

5. Launch:
   ```bat
   infra\ai\run_directml.bat
   ```

6. Verify API:
   ```powershell
   Invoke-RestMethod http://127.0.0.1:8188/system_stats
   ```

7. Benchmark:
   ```powershell
   python infra/ai/benchmark.py
   ```

## Tailscale exposure

Backend on homelab proxies to `http://100.89.170.66:8188` — bind ComfyUI to localhost only; expose via reverse proxy or Tailscale serve on the studio PC.

## CPU fallback

If DirectML OOMs, retry with:
```bat
python main.py --cpu --listen 127.0.0.1 --port 8188
```

Target: 512×512 generation &lt; 60s on GPU; &lt; 180s on CPU.

## Models directory (gitignored)

```
infra/ai/ComfyUI/
  models/checkpoints/pixel_dream_v1.0.safetensors
```
