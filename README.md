# PixelForge

**Aseprite parity + local AI pixel art** — web and Android tablet.

## Overview

- **Editor:** Shared Rust core (`pixelforge-core`) → WASM (web) + NDK (Android)
- **AI:** Ollama + ComfyUI (`pixel_dream v1.0`) on MAZAYA-STUDIO — no cloud AI
- **Offline:** Full editor when PC is off; AI disabled with banner

## Repository layout

```
apps/
  web/          # Web UI shell (scaffold)
  android/      # Android tablet app (scaffold)
  api/          # Backend API (scaffold)
crates/
  pixelforge-core/   # Shared sprite engine
docs/
  PRD.md
  decisions/    # Locked v1 decisions
  specs/        # Parity matrix, round-trip
infra/
  ai/           # ComfyUI DirectML setup, workflows, benchmark
tests/
  parity/       # Aseprite parity harness
```

## Development setup

**Step 0** — bootstrap toolchain and run the verification gate on MAZAYA-STUDIO:

```powershell
.\scripts\setup-dev.ps1
```

This installs Rust stable, adds `wasm32-unknown-unknown` and `aarch64-linux-android` targets, builds native + WASM, and runs tests. Optional: Aseprite CLI check — see [tests/parity/ASEPRITE_BASELINE.md](tests/parity/ASEPRITE_BASELINE.md).

Android NDK linker config: copy [`.cargo/config.toml.example`](.cargo/config.toml.example) → `.cargo/config.toml` (Step 28).

CI: [`.github/workflows/ci.yml`](.github/workflows/ci.yml) — native + WASM build on every push to `main`.

## Quick start

### Rust core

```bash
cargo build -p pixelforge-core
cargo test -p pixelforge-core
```

### Parity tests

```bash
python tests/parity/run_parity.py --list
python tests/parity/run_parity.py --target wasm
```

### AI stack (MAZAYA-STUDIO)

See [infra/ai/README.md](infra/ai/README.md).

```bat
infra\ai\run_directml.bat
python infra\ai\benchmark.py
```

Copy `infra/ai/.env.example` to backend `.env`.

## QA device

**Advan Tab Sketsa 3** — Android 13, 1280×800, active stylus (`elan_pen`).

## Documentation

- [Build steps (step-by-step)](docs/BUILD-STEPS.md) — **start here to implement the project**
- [PRD](docs/PRD.md)
- [V1 scope](docs/decisions/v1-scope.md)
- [Module priority](docs/decisions/module-priority.md)
- [Aseprite parity matrix](docs/specs/aseprite-parity-matrix.md)
- [Round-trip spec](docs/specs/aseprite-roundtrip.md)

## License

TBD — Rust core intended MIT/Apache-2.0 dual license.
