//! Generate parity test fixtures (.aseprite) from manifest.json (rust-sourced entries).

use pixelforge_core::ase::{read_ase, write_ase};
use pixelforge_core::document::{ColorMode, Frame, SpriteDocument};
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct Manifest {
    fixtures: Vec<ManifestEntry>,
}

#[derive(Debug, Deserialize)]
struct ManifestEntry {
    id: String,
    file: String,
    source: String,
    #[serde(default)]
    frames: Option<u32>,
    #[serde(default)]
    width: Option<u32>,
    #[serde(default)]
    height: Option<u32>,
}

fn parse_size_from_file(name: &str) -> Option<(u32, u32, u32)> {
    // blank_16x16.aseprite, blank_16x16_2f.aseprite, blank_8x8_4f.aseprite
    let stem = name.strip_suffix(".aseprite")?;
    let parts: Vec<&str> = stem.split('_').collect();
    if parts.first() != Some(&"blank") {
        return None;
    }
    let mut size_part = parts.get(1)?;
    let mut frames = 1u32;
    if let Some(last) = parts.last() {
        if let Some(n) = last.strip_suffix('f') {
            if parts.len() >= 3 {
                frames = n.parse().ok()?;
                size_part = parts.get(1)?;
            }
        }
    }
    let wh: Vec<&str> = size_part.split('x').collect();
    if wh.len() != 2 {
        return None;
    }
    let w: u32 = wh[0].parse().ok()?;
    let h: u32 = wh[1].parse().ok()?;
    Some((w, h, frames))
}

fn doc_for_entry(entry: &ManifestEntry) -> Option<SpriteDocument> {
    if let (Some(w), Some(h)) = (entry.width, entry.height) {
        let frames = entry.frames.unwrap_or(1);
        let mut doc = SpriteDocument::new_blank(w, h, ColorMode::Indexed);
        doc.frames = (0..frames)
            .map(|_| Frame { duration_ms: 100 })
            .collect();
        return Some(doc);
    }
    let (w, h, frames) = parse_size_from_file(&entry.file)?;
    let mut doc = SpriteDocument::new_blank(w, h, ColorMode::Indexed);
    doc.frames = (0..frames)
        .map(|_| Frame { duration_ms: 100 })
        .collect();
    Some(doc)
}

fn load_manifest(path: &Path) -> Result<Manifest, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixtures_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("tests/parity/fixtures"));

    let manifest_path = fixtures_dir.join("manifest.json");
    let manifest = load_manifest(&manifest_path)?;

    fs::create_dir_all(&fixtures_dir)?;

    let mut wrote = 0u32;
    for entry in &manifest.fixtures {
        if entry.source != "rust" {
            continue;
        }
        let doc = doc_for_entry(entry)
            .ok_or_else(|| format!("cannot derive sprite for rust entry {}", entry.id))?;
        let bytes = write_ase(&doc)?;
        let path = fixtures_dir.join(&entry.file);
        fs::write(&path, &bytes)?;
        let round = read_ase(&bytes)?;
        if round.width != doc.width || round.height != doc.height {
            return Err(format!("round-trip size mismatch for {}", entry.id).into());
        }
        println!("wrote {} -> {} ({} bytes)", entry.id, path.display(), bytes.len());
        wrote += 1;
    }

    println!("generate_fixtures: {} rust fixtures in {}", wrote, fixtures_dir.display());
    Ok(())
}
