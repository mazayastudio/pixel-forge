# Aseprite Round-Trip Specification — PixelForge

**Version:** 1.0  
**Status:** Engineering spec  
**Baseline format:** `.aseprite` per [ASE file spec](https://github.com/aseprite/aseprite/blob/main/docs/ase-file-specs.md)

External desktop Aseprite is **optional** — in-app editor targets 100% parity. This spec ensures **lossless interchange** when users export, edit in Aseprite, and re-import.

---

## 1. Goals

1. **Zero data loss** for layers, cels, palettes, tags, tilemaps, slices, and user data
2. **Preserve PixelForge metadata** in sidecar files when Aseprite ignores them
3. **Watch-folder sync** for "Open in Aseprite" workflow
4. **Server-side CLI** export identical to `aseprite -b` for CI/game pipelines

---

## 2. File bundle

Each logical asset is stored as:

```
projects/{projectId}/assets/{assetId}/
  sprite.aseprite          # Canonical sprite (Aseprite-native)
  meta.json                # PixelForge-only metadata (see §4)
  .pixelforge/
    generation/            # AI history (optional)
    versions/              # Version snapshots
```

**Rule:** `sprite.aseprite` must open in desktop Aseprite v1.3.17.x without warnings or missing features.

---

## 3. Round-trip workflow

```mermaid
sequenceDiagram
  participant PF as PixelForge
  participant Disk as Local_or_Cloud_Storage
  participant AS as Desktop_Aseprite

  PF->>Disk: Write sprite.aseprite + meta.json
  PF->>AS: Launch with file path (optional)
  AS->>Disk: User saves sprite.aseprite
  Note over Disk: Watch folder detects mtime change
  Disk->>PF: Import + merge meta.json
  PF->>PF: Bump version; resolve conflicts if needed
```

### 3.1 Export from PixelForge

| Step | Behavior |
|------|----------|
| Encode | Full ASE v1 spec: layers, groups, tilemaps, tags, slices, palette, frame durations |
| User data | Write Aseprite-compatible `USER_DATA` chunks where supported |
| Sidecar | Always write/update `meta.json` atomically (write temp + rename) |
| Reference layers | Embed or link per user preference (default: embed) |

### 3.2 Import from Aseprite

| Step | Behavior |
|------|----------|
| Validate | Parse ASE; reject with actionable error if corrupt |
| Diff | Compare layer count, frame count, tag set vs last known version |
| Merge meta | Reattach `meta.json` fields not in ASE (AI history, style tags) |
| Conflict | If PixelForge edited same asset while external edit pending → user merge UI |

### 3.3 Watch folder

| Setting | Default |
|---------|---------|
| Enabled | On when "Open in Aseprite" used |
| Poll interval | 2s local; 5s cloud sync folder |
| Debounce | 500ms after last write |
| Ignore | `*.tmp`, `~*`, `.pixelforge/` |

**Events:**

- `FILE_CHANGED` → queue re-import job
- `FILE_DELETED` → warn user; offer restore from version history
- `CONFLICT` → both copies diverged → show three-way preview

### 3.4 Open in Aseprite

| Platform | Behavior |
|----------|----------|
| Web | Download `.aseprite` + optional "watch" token; re-upload on save |
| Tablet | Export to shared folder / SAF; same watch semantics |
| Windows (MAZAYA-STUDIO) | Detect install path; `aseprite.exe "{path}"` |

**Detect paths (nice-to-have):**

- Windows: `%ProgramFiles%\Aseprite\Aseprite.exe`, Steam install
- macOS: `/Applications/Aseprite.app`
- Linux: `aseprite` on `PATH`

---

## 4. Sidecar `meta.json` schema (v1)

```json
{
  "schemaVersion": 1,
  "assetId": "uuid",
  "projectId": "uuid",
  "aseVersion": "1.3.17",
  "pixelforgeVersion": "0.1.0",
  "contentHash": "sha256-of-sprite-bytes",
  "version": 3,
  "tags": ["character", "ui"],
  "styleBibleRef": "project-style-v1",
  "generationHistory": [
    {
      "id": "gen-uuid",
      "mode": "img2img",
      "prompt": "...",
      "seed": 42,
      "model": "pixel_dream_v1",
      "timestamp": "2026-06-18T12:00:00Z"
    }
  ],
  "pivot": { "x": 16, "y": 32 },
  "collisionBox": { "x": 8, "y": 24, "w": 16, "h": 8 },
  "externalEdit": {
    "lastOpenedAt": "2026-06-18T12:05:00Z",
    "watching": true
  }
}
```

**Invariant:** Fields in `meta.json` never overwrite ASE pixel data on import — only augment.

---

## 5. Preserved ASE elements (checklist)

| Element | Required | Test fixture |
|---------|----------|--------------|
| Layer hierarchy + groups | Yes | `fixtures/groups.aseprite` |
| Indexed palette + color profiles | Yes | `fixtures/indexed.aseprite` |
| Frame tags (direction, repeat) | Yes | `fixtures/tags.aseprite` |
| Linked cels | Yes | `fixtures/linked_cels.aseprite` |
| Tilemap layers + external tileset | Yes | `fixtures/tilemap.aseprite` |
| Slices + 9-patch pivots | Yes | `fixtures/slices.aseprite` |
| Per-cel opacity (RGB) | Yes | `fixtures/cel_opacity.aseprite` |
| Global user data (v1.3) | Yes | `fixtures/user_data.aseprite` |

---

## 6. CLI batch export (server)

Backend exposes jobs equivalent to:

```bash
aseprite -b input.aseprite --sheet out.png --data out.json \
  --split-tags --list-layer-hierarchy
```

**API:** `POST /v1/assets/{id}/export` with `format`, `sheet-layout`, `split-tags`, etc.

Output byte-compared against golden files from real Aseprite CLI in CI (MAZAYA-STUDIO runner).

---

## 7. Failure modes

| Error | User message | Recovery |
|-------|--------------|----------|
| ASE newer than supported | "File uses Aseprite X features; update PixelForge" | Read-only preview if possible |
| Import hash mismatch | "File changed on disk" | Reload or keep local |
| Watch stale lock | "Waiting for Aseprite to close file" | Retry |
| Meta missing | Silent | Create fresh meta from ASE contents |

---

## 8. Test requirements

1. **Round-trip test:** PF → disk → Aseprite CLI save → PF import → pixel diff == 0
2. **Meta survival:** `generationHistory` unchanged after external edit
3. **Tag survival:** Tag names, colors, ping-pong, repeat count intact
4. **Tilemap survival:** Tile indices and external tileset paths valid

See `tests/parity/` for harness.

---

## Related documents

- [Parity matrix](./aseprite-parity-matrix.md)
- [V1 scope — sync](../decisions/v1-scope.md)
