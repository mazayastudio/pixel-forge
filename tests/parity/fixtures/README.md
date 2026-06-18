# Parity test fixtures

Place `.aseprite` test files here. Generate goldens with desktop Aseprite v1.3.17.x on MAZAYA-STUDIO.

| File | Purpose |
|------|---------|
| `blank_16x16.aseprite` | Minimal indexed sprite |
| `tags_walk_cycle.aseprite` | Tags with ping-pong + repeat |
| `tilemap_manual.aseprite` | Manual tilemap layer |
| `linked_cels.aseprite` | Linked cel animation |
| `slices_ninepatch.aseprite` | 9-slice regions |

Run: `python run_parity.py --list`
