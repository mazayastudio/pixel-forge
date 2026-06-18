//! Generate parity test fixtures (.aseprite) for the harness.

use pixelforge_core::ase::{read_ase, write_ase};
use pixelforge_core::document::{ColorMode, SpriteDocument};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("tests/parity/fixtures"));

    fs::create_dir_all(&root)?;

    let fixtures: Vec<(&str, SpriteDocument)> = vec![(
        "blank_16x16.aseprite",
        SpriteDocument::new_blank(16, 16, ColorMode::Indexed),
    )];

    for (name, doc) in fixtures {
        let bytes = write_ase(&doc)?;
        let path = root.join(name);
        fs::write(&path, &bytes)?;
        let round = read_ase(&bytes)?;
        assert_eq!(round.width, doc.width);
        assert_eq!(round.height, doc.height);
        println!("wrote {} ({} bytes)", path.display(), bytes.len());
    }

    Ok(())
}
