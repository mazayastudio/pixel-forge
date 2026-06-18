//! Round-trip or read-smoke a fixture through pixelforge-core ASE codec.

use pixelforge_core::ase::{read_ase, write_ase};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let read_only = args.first().map(|a| a == "--read-only").unwrap_or(false);
    if read_only {
        args.remove(0);
    }

    let path = args
        .first()
        .map(PathBuf::from)
        .ok_or("usage: roundtrip [--read-only] <file.aseprite>")?;

    let bytes = fs::read(&path)?;
    let doc = read_ase(&bytes)?;

    if read_only {
        println!(
            "READ_OK: {} {}x{} {} frames ({} bytes)",
            path.display(),
            doc.width,
            doc.height,
            doc.frames.len(),
            bytes.len()
        );
        return Ok(());
    }

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
