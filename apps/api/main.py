"""PixelForge API — health, projects scaffold, AI proxy to MAZAYA-STUDIO."""

from __future__ import annotations

import os
import time
import urllib.error
import urllib.request
from typing import Any

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI(title="PixelForge API", version="0.1.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=os.getenv("CORS_ORIGINS", "*").split(","),
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


def _fetch_json(url: str, timeout: float = 5.0) -> tuple[bool, str, dict[str, Any] | None]:
    try:
        req = urllib.request.Request(url)
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            body = resp.read().decode()
            try:
                return True, "ok", json_loads(body)
            except ValueError:
                return True, "ok", {"raw": body[:200]}
    except urllib.error.HTTPError as e:
        return False, f"http {e.code}", None
    except urllib.error.URLError as e:
        return False, str(e.reason), None


def json_loads(s: str) -> dict[str, Any]:
    import json

    data = json.loads(s)
    return data if isinstance(data, dict) else {"data": data}


@app.get("/health")
def health() -> dict[str, Any]:
    return {"status": "ok", "service": "pixelforge-api", "ts": int(time.time())}


@app.get("/health/ai")
def health_ai() -> dict[str, Any]:
    ollama = os.getenv("OLLAMA_BASE_URL", "http://127.0.0.1:11434").rstrip("/")
    comfy = os.getenv("COMFYUI_BASE_URL", "http://127.0.0.1:8188").rstrip("/")

    ollama_ok, ollama_msg, ollama_data = _fetch_json(f"{ollama}/api/tags")
    comfy_ok, comfy_msg, comfy_data = _fetch_json(f"{comfy}/system_stats")

    online = ollama_ok and comfy_ok
    return {
        "status": "ok" if online else "degraded",
        "ai_available": online,
        "ollama": {"ok": ollama_ok, "detail": ollama_msg, "models": ollama_data},
        "comfyui": {"ok": comfy_ok, "detail": comfy_msg, "stats": comfy_data},
        "ts": int(time.time()),
    }


@app.get("/v1/projects")
def list_projects() -> dict[str, Any]:
    return {"projects": [], "note": "scaffold — implement in Phase 0"}
