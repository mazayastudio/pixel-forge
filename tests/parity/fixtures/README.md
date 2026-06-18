# Parity test fixtures

**Manifest:** [`manifest.json`](manifest.json) — 50+ fixture definitions (single source of truth).

## Generate all fixtures (Step 1)

```powershell
.\scripts\bootstrap-parity-fixtures.ps1
```

Or step by step:

```powershell
cargo run -p pixelforge-core --bin generate_fixtures -- tests/parity/fixtures
.\scripts\generate-aseprite-fixtures.ps1
.\scripts\generate-goldens.ps1
python tests/parity/run_parity.py --fixture-count
```

## Sources

| Source | Count | Description |
|--------|------:|-------------|
| `rust` | 10 | Indexed blank sprites (sizes + multi-frame) |
| `aseprite-lua` | 40 | Complex fixtures via Steam Aseprite CLI |

## Verify

```powershell
python tests/parity/run_parity.py --list-cases
python tests/parity/run_parity.py --fixture-count
```

See [ASEPRITE_BASELINE.md](../ASEPRITE_BASELINE.md) for Aseprite v1.3.17.x requirements.
