# Editor architecture — locked

**Decision:** Shared **Rust** sprite engine matching Aseprite v1.3.17.x + ASE spec.

| Target | Delivery |
|--------|----------|
| Web | WASM + WebGL/Canvas |
| Android tablet | NDK/JNI native library |
| UI | Thin shells — React/Svelte (web), Compose/RN (tablet) |
| Parity | One test suite for WASM + NDK |

See `crates/pixelforge-core/`.
