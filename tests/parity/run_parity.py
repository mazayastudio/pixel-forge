#!/usr/bin/env python3
"""
PixelForge parity test runner.

Loads cases from tests/parity/fixtures/manifest.json.
Invokes Rust binaries for ASE round-trip / read-smoke on fixtures.

Usage:
  python run_parity.py --list-cases
  python run_parity.py --fixture-count
  python run_parity.py --generate-fixtures
  python run_parity.py --generate-goldens
  python run_parity.py --target wasm
  python run_parity.py --summary
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
MANIFEST = FIXTURES / "manifest.json"
DEFAULT_MATRIX = ROOT / "docs" / "specs" / "aseprite-parity-matrix.json"
MIN_FIXTURES = 50
CARGO = Path(os.environ.get("CARGO", Path.home() / ".cargo" / "bin" / "cargo.exe"))

BOOTSTRAP_STATUS: dict[str, dict[str, str]] = {
    "IO-01": {"web": "in_progress", "tablet": "in_progress"},
    "AI-08": {"web": "in_progress", "tablet": "in_progress"},
}


@dataclass
class ParityCase:
    id: str
    file: str
    category: str
    source: str
    matrix_ids: list[str]
    operation: str


def load_manifest() -> list[ParityCase]:
    if not MANIFEST.exists():
        raise FileNotFoundError(f"manifest not found: {MANIFEST}")
    data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    cases = []
    for entry in data.get("fixtures", []):
        cases.append(
            ParityCase(
                id=entry["id"],
                file=entry["file"],
                category=entry.get("category", ""),
                source=entry.get("source", "unknown"),
                matrix_ids=entry.get("matrixIds", []),
                operation=entry.get("operation", "open_save"),
            )
        )
    return cases


def list_cases() -> None:
    cases = load_manifest()
    print(f"{'ID':<10}\t{'FILE':<32}\t{'SRC':<12}\t{'OP':<12}\tEXISTS\tMATRIX")
    print("-" * 100)
    for c in cases:
        exists = (FIXTURES / c.file).exists()
        mids = ",".join(c.matrix_ids[:2])
        if len(c.matrix_ids) > 2:
            mids += "..."
        print(f"{c.id:<10}\t{c.file:<32}\t{c.source:<12}\t{c.operation:<12}\t{exists}\t{mids}")


def fixture_count_gate() -> int:
    cases = load_manifest()
    on_disk = sum(1 for c in cases if (FIXTURES / c.file).exists())
    ase_files = len(list(FIXTURES.glob("*.aseprite")))
    goldens = len(list(GOLDEN.glob("*.png")))
    print(f"manifest entries={len(cases)} fixtures_on_disk={on_disk} aseprite_files={ase_files} goldens={goldens}")
    if on_disk < MIN_FIXTURES:
        print(f"FAIL: need >= {MIN_FIXTURES} manifest fixtures on disk, have {on_disk}", file=sys.stderr)
        return 1
    if goldens < MIN_FIXTURES:
        print(f"WARN: goldens {goldens} < {MIN_FIXTURES} (run scripts/generate-goldens.ps1)")
    print(f"PASS: fixture-count gate ({on_disk} >= {MIN_FIXTURES})")
    return 0


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


def generate_goldens() -> int:
    script = ROOT / "scripts" / "generate-goldens.ps1"
    if not script.exists():
        print(f"Missing {script}", file=sys.stderr)
        return 1
    proc = subprocess.run(
        ["powershell", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", str(script)],
        cwd=ROOT,
    )
    return proc.returncode


def run_roundtrip(fixture_path: Path, read_only: bool = False) -> tuple[str, str]:
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
    ]
    if read_only:
        cmd.append("--read-only")
    cmd.append(str(fixture_path))
    proc = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True)
    out = (proc.stdout or proc.stderr or "").strip()
    if proc.returncode == 0:
        if read_only:
            return "read_ok", out
        return "pass", out
    return "fail", out or "roundtrip failed"


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
            for _, value in (("web", web), ("tablet", tablet)):
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
            "p0_pass_both": sum(
                1 for r in rows if r["priority"] == "P0" and r["web"] == "pass" and r["tablet"] == "pass"
            ),
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


def run_case(case: ParityCase, target: str) -> tuple[str, str]:
    fixture_path = FIXTURES / case.file
    if not fixture_path.exists():
        return "skipped", "fixture not present"

    if case.operation == "open_save":
        if case.source == "rust":
            return run_roundtrip(fixture_path, read_only=False)
        return "pending_impl", "full round-trip for aseprite-lua deferred to Step 4"

    if case.operation == "read_smoke":
        return run_roundtrip(fixture_path, read_only=True)

    if case.operation == "golden_only":
        golden = GOLDEN / f"{fixture_path.stem}.png"
        if golden.exists():
            return "golden_ok", f"golden present: {golden.name}"
        return "skipped", "golden PNG missing"

    return "pending_impl", f"operation {case.operation} not wired for target={target}"


def run(target: str, device: str | None) -> int:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    FIXTURES.mkdir(parents=True, exist_ok=True)
    GOLDEN.mkdir(parents=True, exist_ok=True)

    if not (FIXTURES / "blank_16x16.aseprite").exists():
        generate_fixtures()

    cases = load_manifest()
    results = []
    for case in cases:
        status, detail = run_case(case, target)
        results.append({**asdict(case), "status": status, "detail": detail, "target": target})

    report = {
        "target": target,
        "device": device,
        "passed": sum(1 for r in results if r["status"] in ("pass", "read_ok", "golden_ok")),
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
    parser.add_argument("--list-cases", action="store_true", help="List manifest parity cases")
    parser.add_argument("--list", action="store_true", help="Alias for --list-cases")
    parser.add_argument("--fixture-count", action="store_true", help="Verify >=50 fixtures on disk")
    parser.add_argument("--summary", action="store_true", help="Print parity matrix summary only")
    parser.add_argument("--generate-fixtures", action="store_true", help="Write rust bootstrap fixtures")
    parser.add_argument("--generate-goldens", action="store_true", help="Export golden PNGs via Aseprite")
    parser.add_argument("--matrix", type=Path, help="Path to aseprite-parity-matrix.json")
    parser.add_argument("--target", choices=["wasm", "ndk"], default="wasm")
    parser.add_argument("--device", help="ADB serial for NDK tests")
    args = parser.parse_args()

    if args.list_cases or args.list:
        list_cases()
        return 0
    if args.fixture_count:
        return fixture_count_gate()
    if args.generate_fixtures:
        return generate_fixtures()
    if args.generate_goldens:
        return generate_goldens()
    if args.summary:
        if not DEFAULT_MATRIX.exists():
            print(f"Matrix not found: {DEFAULT_MATRIX}", file=sys.stderr)
            return 1
        return matrix_report(DEFAULT_MATRIX, args.target, args.device)
    if args.matrix:
        return matrix_report(args.matrix, args.target, args.device)
    if DEFAULT_MATRIX.exists():
        matrix_report(DEFAULT_MATRIX, args.target, args.device)
    return run(args.target, args.device)


if __name__ == "__main__":
    sys.exit(main())
