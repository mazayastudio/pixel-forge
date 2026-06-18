# Generation pipeline — pixel-specific AI

**Status:** Documented + infra scaffold  
**Host:** MAZAYA-STUDIO

## Stack

| Role | Component |
|------|-----------|
| LLM | Ollama `qwen2.5:7b-instruct` |
| Image | ComfyUI + DirectML + `pixel_dream v1.0` (SD 1.5) |
| Post-process | Nearest downscale → palette quantize → despeckle → ASE layer |
| Offline fast path | `pixelforge-core::postprocess` (no diffusion) |

## Modes

| Mode | Pipeline | PC required |
|------|----------|-------------|
| Text → pixel | Ollama spec → ComfyUI txt2img → post-process | Yes |
| JPG/PNG → pixel | Preprocess → optional Ollama tags → img2img denoise 0.35–0.55 | Yes |
| Sketch → pixel | Active cel as img2img source | Yes |
| Inpaint | Masked img2img | Yes |
| Fast quantize | WASM/NDK CPU only | No |

## User controls (img convert)

- Target size: 16–128px or custom
- Palette: project / presets / auto-extract
- Max colors: 4–256
- Denoise, dither, outline, background removal

## Files

- `infra/ai/run_directml.bat` — GPU launcher
- `infra/ai/workflows/txt2img_pixel_dream.json`
- `infra/ai/workflows/img2img_photo_to_pixel.json`
- `infra/ai/benchmark.py` — 512×512 &lt;60s target
- `infra/ai/.env.example`

## Rejected for v1

- SDXL / Pixel Art XL (8GB+ VRAM)
- FLUX and large diffusion models

## Optional v1.5

- LCM LoRA on SD 1.5 for faster steps
- GGUF-quantized checkpoints if VRAM headroom improves
