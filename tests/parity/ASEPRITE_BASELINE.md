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

Export a fixture to PNG for `tests/parity/golden/`:

```powershell
$ase = $env:ASEPRITE_CLI
if (-not $ase) { $ase = "C:\Program Files\Aseprite\Aseprite.exe" }

& $ase -b tests/parity/fixtures/blank_16x16.aseprite `
  --save-as tests/parity/golden/blank_16x16.png
```

Batch mode flags used in parity harness (Step 1+):

```text
aseprite -b input.aseprite --save-as output.png
aseprite -b input.aseprite --sheet sheet.png --data sheet.json
```

---

## Version check

`check-aseprite.ps1` warns if the version string does not contain `1.3.17`.

Track v1.3.18 betas as optional smoke tests only — **P0 goldens stay on 1.3.17.x**.

---

## MAZAYA-STUDIO

Install Aseprite v1.3.17.x on the studio PC before Step 1 (fixture pipeline). Steam or direct installer both work; pin the build in your notes when exporting goldens.
