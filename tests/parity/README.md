# Parity test harness

Validates **Aseprite v1.3.17.x** behavioral parity for `pixelforge-core` on WASM and Android NDK.

## Quick start

```powershell
# List cases
python tests/parity/run_parity.py --list

# Bootstrap blank fixture + run round-trip tests
python tests/parity/run_parity.py --target wasm

# Regenerate fixtures only
python tests/parity/run_parity.py --generate-fixtures
```

## Layout

| Path | Purpose |
|------|---------|
| `fixtures/` | Input `.aseprite` files |
| `golden/` | PNG exports from desktop Aseprite CLI |
| `output/report.json` | Latest run summary (gitignored) |
| `run_parity.py` | Harness entrypoint |

## Case status (bootstrap)

- **RT-001** (`blank_16x16.aseprite`) — auto-generated; round-trip **passes**
- **RT-002 … EX-001** — export from Aseprite v1.3.17.x on MAZAYA-STUDIO; pending golden + op wiring

## Android NDK

```powershell
python tests/parity/run_parity.py --target ndk --device 1029P002505S070810
```

QA device: **Advan Tab Sketsa 3** (`1029P002505S070810`).
