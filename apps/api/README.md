# PixelForge API — scaffold

REST API: projects, assets, sync, AI job proxy to MAZAYA-STUDIO.

## Run locally

```powershell
cd apps/api
python -m venv .venv
.\.venv\Scripts\Activate.ps1
pip install -r requirements.txt
$env:OLLAMA_BASE_URL = "http://100.89.170.66:11434"
$env:COMFYUI_BASE_URL = "http://100.89.170.66:8188"
uvicorn main:app --reload --port 3000
```

## Endpoints

| Route | Purpose |
|-------|---------|
| `GET /health` | API liveness |
| `GET /health/ai` | Ollama + ComfyUI reachability |
| `GET /v1/projects` | Project list (stub) |

Env: copy `infra/ai/.env.example` to repo `.env`.
