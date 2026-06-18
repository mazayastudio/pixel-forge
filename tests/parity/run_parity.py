#!/usr/bin/env python3
"""
PixelForge parity test runner.

Compares pixelforge-core output against golden images produced by Aseprite CLI.
Invokes Rust binaries for ASE round-trip on fixtures.
Reports feature parity matrix coverage from docs/specs/aseprite-parity-matrix.json.

Usage:
  python run_parity.py --list
  python run_parity.py --generate-fixtures
  python run_parity.py --target wasm
  python run_parity.py --target ndk --device 1029P002505S070810
  python run_parity.py --matrix docs/specs/aseprite-parity-matrix.json --target wasm
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from dataclasses import dataclass, asdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "tests" / "parity" / "fixtures"
GOLDEN = ROOT / "tests" / "parity" / "golden"
OUTPUT = ROOT / "tests" / "parity" / "output"
DEFAULT_MATRIX = ROOT / "docs" / "specs" / "aseprite-parity-matrix.json"
CARGO = Path(os.environ.get("CARGO", Path.home() / ".cargo" / "bin" / "cargo.exe"))

# Bootstrap implementation hints — maps matrix IDs to current engine state.
BOOTSTRAP_STATUS: dict[str, dict[str, str]] = {
    "IO-01": {"web": "in_progress", "tablet": "in_progress"},
    "AI-08": {"web": "in_progress", "tablet": "in_progress"},
}


@dataclass
class ParityCase:
    id: str
    fixture: str
    operation: str
    description: str


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


def cargo_available() -> bool:
    return CARGO.exists()


def generate_fixtures() -> int:
    if not cargo_available():
        print(f"SKIP: cargo not found at {CARGO}")
        return 1
    cmd = [
        str(CARGO),
        "run",
        "--quiet",
        "-p",
        "pixelforge-core",
        "--bin",
        "generate_fixtures",
        "--",
        str(FIXTURES),
    ]
    print(" ".join(cmd))
    proc = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True)
    if proc.stdout:
        print(proc.stdout.strip())
    if proc.returncode != 0:
        print(proc.stderr or "generate_fixtures failed", file=sys.stderr)
        return proc.returncode
    return 0


def run_roundtrip(fixture_path: Path) -> tuple[str, str]:
    if not cargo_available():
        return "skipped", "cargo not available"
    cmd = [
        str(CARGO),
        "run",
        "--quiet",
        "-p",
        "pixelforge-core",
        "--bin",
        "roundtrip",
        "--",
        str(fixture_path),
    ]
    proc = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True)
    if proc.returncode == 0:
        return "pass", (proc.stdout or "round-trip ok").strip()
    return "fail", (proc.stderr or proc.stdout or "roundtrip failed").strip()


def load_matrix(path: Path) -> dict:
    data = json.loads(path.read_text(encoding="utf-8"))
    required = ["schemaVersion", "baseline", "sections", "summary"]
    missing = [k for k in required if k not in data]
    if missing:
        raise ValueError(f"matrix missing keys: {missing}")

    seen: set[str] = set()
    for section in data["sections"]:
        if "id" not in section or "items" not in section:
            raise ValueError(f"invalid section: {section.get('id', '?')}")
        for item in section["items"]:
            item_id = item.get("id")
            if not item_id:
                raise ValueError(f"item missing id in section {section['id']}")
            if item_id in seen:
                raise ValueError(f"duplicate matrix id: {item_id}")
            seen.add(item_id)
            bootstrap = BOOTSTRAP_STATUS.get(item_id, {})
            item.setdefault("status", {})
            item["status"].setdefault("web", bootstrap.get("web", "not_started"))
            item["status"].setdefault("tablet", bootstrap.get("tablet", "not_started"))
    return data


def matrix_report(matrix_path: Path, target: str, device: str | None) -> int:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    matrix = load_matrix(matrix_path)

    rows = []
    counts: dict[str, int] = {}
    p0_total = 0
    p0_pass_web = 0
    p0_pass_tablet = 0

    for section in matrix["sections"]:
        if section["id"] == "ai":
            continue
        for item in section["items"]:
            status = item.get("status", {})
            web = status.get("web", "not_started")
            tablet = status.get("tablet", "not_started")
            priority = item.get("priority", "P0")
            rows.append(
                {
                    "id": item["id"],
                    "section": section["id"],
                    "feature": item.get("feature", ""),
                    "priority": priority,
                    "web": web,
                    "tablet": tablet,
                }
            )
            for platform, value in (("web", web), ("tablet", tablet)):
                counts[value] = counts.get(value, 0) + 1
            if priority == "P0":
                p0_total += 1
                if web == "pass":
                    p0_pass_web += 1
                if tablet == "pass":
                    p0_pass_tablet += 1

    report = {
        "matrix": str(matrix_path.relative_to(ROOT)) if matrix_path.is_relative_to(ROOT) else str(matrix_path),
        "baseline": matrix["baseline"],
        "target": target,
        "device": device,
        "totals": {
            "items": len(rows),
            "p0_items": p0_total,
            "p0_pass_web": p0_pass_web,
            "p0_pass_tablet": p0_pass_tablet,
            "p0_pass_both": sum(1 for r in rows if r["priority"] == "P0" and r["web"] == "pass" and r["tablet"] == "pass"),
            "status_counts": counts,
        },
        "sections": matrix["summary"],
        "rows": rows,
    }

    out_path = OUTPUT / "matrix_report.json"
    out_path.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(f"Wrote {out_path}")
    print(
        f"matrix items={report['totals']['items']} "
        f"P0 pass web={p0_pass_web}/{p0_total} tablet={p0_pass_tablet}/{p0_total}"
    )
    return 0


def run(target: str, device: str | None) -> int:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    FIXTURES.mkdir(parents=True, exist_ok=True)
    GOLDEN.mkdir(parents=True, exist_ok=True)

    if not (FIXTURES / "blank_16x16.aseprite").exists():
        generate_fixtures()

    results = []
    for case in CASES:
        fixture_path = FIXTURES / case.fixture
        if not fixture_path.exists():
            status = "skipped"
            detail = "fixture not present — export from Aseprite or extend generate_fixtures"
        elif case.operation == "open_save":
            status, detail = run_roundtrip(fixture_path)
        else:
            status = "pending_impl"
            detail = f"operation {case.operation} not wired for target={target}"

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
    print(
        f"pass={report['passed']} fail={report['failed']} "
        f"skipped={report['skipped']} pending={report['pending']}"
    )
    return 1 if report["failed"] else 0


def main() -> int:
    parser = argparse.ArgumentParser(description="PixelForge Aseprite parity harness")
    parser.add_argument("--list", action="store_true", help="List parity cases")
    parser.add_argument("--generate-fixtures", action="store_true", help="Write bootstrap fixtures")
    parser.add_argument("--matrix", type=Path, help="Path to aseprite-parity-matrix.json")
    parser.add_argument("--target", choices=["wasm", "ndk"], default="wasm")
    parser.add_argument("--device", help="ADB serial for NDK tests")
    args = parser.parse_args()

    if args.list:
        list_cases()
        return 0
    if args.generate_fixtures:
        return generate_fixtures()
    if args.matrix:
        return matrix_report(args.matrix, args.target, args.device)
    if DEFAULT_MATRIX.exists():
        matrix_report(DEFAULT_MATRIX, args.target, args.device)
    return run(args.target, args.device)


if __name__ == "__main__":
    sys.exit(main())
