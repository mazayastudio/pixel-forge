//! Round-trip a fixture through pixelforge-core ASE codec (parity harness helper).

use pixelforge_core::ase::{read_ase, write_ase};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: roundtrip <file.aseprite>")?;

    let bytes = fs::read(&path)?;
    let doc = read_ase(&bytes)?;
    let out = write_ase(&doc)?;
    let reparsed = read_ase(&out)?;

    if doc.width != reparsed.width || doc.height != reparsed.height {
        eprintln!(
            "FAIL: dimension mismatch {}x{} -> {}x{}",
            doc.width, doc.height, reparsed.width, reparsed.height
        );
        std::process::exit(1);
    }

    println!(
        "PASS: {} {}x{} ({} -> {} bytes)",
        path.display(),
        doc.width,
        doc.height,
        bytes.len(),
        out.len()
    );
    Ok(())
}
