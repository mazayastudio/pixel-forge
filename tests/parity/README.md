# Parity test harness

Validates **Aseprite v1.3.17.x** behavioral parity for `pixelforge-core` on WASM and Android NDK.

## Quick start

```powershell
# Bootstrap 50+ fixtures + goldens (MAZAYA-STUDIO with Steam Aseprite)
.\scripts\bootstrap-parity-fixtures.ps1

# List manifest cases
python tests/parity/run_parity.py --list-cases

# Fixture count gate (CI + local)
python tests/parity/run_parity.py --fixture-count

# Run round-trip / read-smoke tests
python tests/parity/run_parity.py --target wasm
```

## Layout

| Path | Purpose |
|------|---------|
| `fixtures/manifest.json` | Single source of truth for 50+ cases |
| `fixtures/*.aseprite` | Input sprites (Rust + Aseprite Lua) |
| `golden/*.png` | PNG exports from desktop Aseprite CLI |
| `output/report.json` | Latest run summary (gitignored) |
| `run_parity.py` | Harness entrypoint |

## Case status (Step 1)

- **55 fixtures** in manifest — 10 `rust` (indexed round-trip), 45 `aseprite-lua` (read smoke until Steps 3–4)
- **Rust `open_save`** — round-trip passes via `roundtrip` binary
- **Lua fixtures** — `read_smoke` only (`roundtrip --read-only`) until codec matures
- **`golden_only`** — EX-* export fixtures; golden PNG present, no round-trip yet

Regenerate goldens on Windows only:

```powershell
.\scripts\generate-goldens.ps1
# or
python tests/parity/run_parity.py --generate-goldens
```

## Android NDK

```powershell
python tests/parity/run_parity.py --target ndk --device 1029P002505S070810
```

QA device: **Advan Tab Sketsa 3** (`1029P002505S070810`).
