# Aseprite baseline — parity golden generation

**Required version:** [Aseprite v1.3.17.x](https://www.aseprite.org/release-notes/) stable

PixelForge parity tests compare output against goldens produced by desktop Aseprite. Using a different version invalidates pixel diffs.

---

## Detect CLI

```powershell
.\scripts\check-aseprite.ps1
```

Override path:

```powershell
$env:ASEPRITE_CLI = "C:\Program Files\Aseprite\Aseprite.exe"
.\scripts\check-aseprite.ps1
```

### Windows search order

1. `$env:ASEPRITE_CLI`
2. `%ProgramFiles%\Aseprite\Aseprite.exe`
3. `%ProgramFiles(x86)%\Aseprite\Aseprite.exe`
4. `%LOCALAPPDATA%\Programs\Aseprite\Aseprite.exe`
5. Steam: `steamapps\common\**\Aseprite.exe`
6. `aseprite` on `PATH`

---

## Golden generation

One-command bootstrap (fixtures + goldens + harness gate):

```powershell
.\scripts\bootstrap-parity-fixtures.ps1
```

Export a fixture to PNG for `tests/parity/golden/`:

```powershell
$ase = $env:ASEPRITE_CLI
if (-not $ase) { & .\scripts\Resolve-Aseprite.ps1 }

& $ase -b tests/parity/fixtures/blank_16x16.aseprite `
  --frame-range 0,0 `
  --save-as tests/parity/golden/blank_16x16.png
```

Batch all manifest entries:

```powershell
.\scripts\generate-goldens.ps1
```

Batch mode flags used in parity harness:

```text
aseprite -b input.aseprite --frame-range 0,0 --save-as output.png
aseprite -b input.aseprite --sheet sheet.png --data sheet.json
```

---

## Version check

`check-aseprite.ps1` warns if the version string does not contain `1.3.17`.

Track v1.3.18 betas as optional smoke tests only — **P0 goldens stay on 1.3.17.x**.

---

## MAZAYA-STUDIO

**Pinned build:** Aseprite **v1.3.17.2-x64** via Steam (`C:\Program Files (x86)\Steam\steamapps\common\Aseprite\Aseprite.exe`).

Goldens in `tests/parity/golden/` were generated with this build on 2026-06-18.
