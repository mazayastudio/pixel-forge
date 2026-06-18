#!/usr/bin/env python3
"""
Benchmark ComfyUI pixel_dream txt2img on MAZAYA-STUDIO.
Target: 512x512 < 60s on RX 5600 XT DirectML.

Usage:
  python benchmark.py
  python benchmark.py --comfy-url http://127.0.0.1:8188
"""

from __future__ import annotations

import argparse
import json
import time
import urllib.error
import urllib.request
from pathlib import Path

WORKFLOWS = Path(__file__).parent / "workflows"


def load_workflow(name: str) -> dict:
    path = WORKFLOWS / name
    if not path.exists():
        raise FileNotFoundError(f"Missing workflow: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def queue_prompt(base_url: str, workflow: dict) -> str:
    payload = json.dumps({"prompt": workflow}).encode("utf-8")
    req = urllib.request.Request(
        f"{base_url.rstrip('/')}/prompt",
        data=payload,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=30) as resp:
        data = json.loads(resp.read().decode())
    return data.get("prompt_id", "")


def wait_history(base_url: str, prompt_id: str, timeout_s: float = 300) -> bool:
    deadline = time.time() + timeout_s
    while time.time() < deadline:
        try:
            req = urllib.request.Request(f"{base_url.rstrip('/')}/history/{prompt_id}")
            with urllib.request.urlopen(req, timeout=10) as resp:
                hist = json.loads(resp.read().decode())
            if prompt_id in hist:
                return True
        except urllib.error.HTTPError as e:
            if e.code != 404:
                raise
        time.sleep(1)
    return False


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--comfy-url", default="http://127.0.0.1:8188")
    parser.add_argument("--workflow", default="txt2img_pixel_dream.json")
    args = parser.parse_args()

    try:
        workflow = load_workflow(args.workflow)
    except FileNotFoundError as e:
        print(f"SKIP: {e}")
        return 0

    print(f"ComfyUI: {args.comfy_url}")
    print(f"Workflow: {args.workflow}")
    t0 = time.perf_counter()
    try:
        prompt_id = queue_prompt(args.comfy_url, workflow)
        ok = wait_history(args.comfy_url, prompt_id)
    except urllib.error.URLError as e:
        print(f"FAIL: ComfyUI unreachable — {e}")
        print("Start: infra/ai/run_directml.bat")
        return 1

    elapsed = time.perf_counter() - t0
    status = "PASS" if ok and elapsed < 60 else ("SLOW" if ok else "FAIL")
    print(f"prompt_id={prompt_id} elapsed={elapsed:.1f}s status={status}")
    if ok and elapsed >= 60:
        print("WARN: exceeds 60s GPU target")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
