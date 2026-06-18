#!/usr/bin/env python3
"""
PixelForge parity test runner skeleton.

Compares pixelforge-core output against golden images produced by Aseprite CLI.
Requires: Aseprite installed (optional for --list), pixelforge-core WASM build (future).

Usage:
  python run_parity.py --list
  python run_parity.py --target wasm
  python run_parity.py --target ndk --device 1029P002505S070810
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass, asdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent
FIXTURES = ROOT / "fixtures"
GOLDEN = ROOT / "golden"
OUTPUT = ROOT / "output"


@dataclass
class ParityCase:
    id: str
    fixture: str
    operation: str
    description: str


# Initial cases — expand to 50+ as fixtures are added
CASES: list[ParityCase] = [
    ParityCase("RT-001", "blank_16x16.aseprite", "open_save", "Round-trip blank indexed sprite"),
    ParityCase("RT-002", "tags_walk_cycle.aseprite", "open_save", "Frame tags preserved"),
    ParityCase("RT-003", "tilemap_manual.aseprite", "open_save", "Tilemap layer indices"),
    ParityCase("RT-004", "linked_cels.aseprite", "open_save", "Linked cels across frames"),
    ParityCase("RT-005", "slices_ninepatch.aseprite", "open_save", "Slice + 9-patch metadata"),
    ParityCase("OP-001", "pencil_stroke.aseprite", "pencil_line", "Pencil pixel-perfect line"),
    ParityCase("OP-002", "bucket_fill.aseprite", "paint_bucket", "Paint bucket tolerance"),
    ParityCase("OP-003", "rot_sprite.aseprite", "rotate_45", "RotSprite rotation"),
    ParityCase("EX-001", "anim_4frame.aseprite", "export_sheet", "Sprite sheet + JSON"),
]


def list_cases() -> None:
    for c in CASES:
        print(f"{c.id}\t{c.fixture}\t{c.operation}\t{c.description}")


def run(target: str, device: str | None) -> int:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    FIXTURES.mkdir(parents=True, exist_ok=True)
    GOLDEN.mkdir(parents=True, exist_ok=True)

    results = []
    for case in CASES:
        fixture_path = FIXTURES / case.fixture
        status = "skipped"
        detail = "fixture not present — add .aseprite files from Aseprite export"

        if fixture_path.exists():
            # TODO: invoke pixelforge-core (wasm/ndk) and diff against golden
            status = "pending_impl"
            detail = f"core runner not wired for target={target}"

        results.append({**asdict(case), "status": status, "detail": detail, "target": target})

    report = {
        "target": target,
        "device": device,
        "passed": sum(1 for r in results if r["status"] == "pass"),
        "failed": sum(1 for r in results if r["status"] == "fail"),
        "skipped": sum(1 for r in results if r["status"] == "skipped"),
        "pending": sum(1 for r in results if r["status"] == "pending_impl"),
        "cases": results,
    }
    report_path = OUTPUT / "report.json"
    report_path.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(f"Wrote {report_path}")
    print(f"skipped={report['skipped']} pending={report['pending']}")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description="PixelForge Aseprite parity harness")
    parser.add_argument("--list", action="store_true", help="List parity cases")
    parser.add_argument("--target", choices=["wasm", "ndk"], default="wasm")
    parser.add_argument("--device", help="ADB serial for NDK tests")
    args = parser.parse_args()

    if args.list:
        list_cases()
        return 0
    return run(args.target, args.device)


if __name__ == "__main__":
    sys.exit(main())
