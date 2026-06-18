//! ASE (.aseprite) read/write — implements github.com/aseprite/aseprite ASE spec.

use crate::document::SpriteDocument;
use crate::error::CoreError;

/// ASE magic: `0xA5E0`
const ASE_MAGIC: u32 = 0xA5E0;

pub fn read_ase(bytes: &[u8]) -> Result<SpriteDocument, CoreError> {
    if bytes.len() < 6 {
        return Err(CoreError::InvalidAse("file too short".into()));
    }
    let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    if magic != ASE_MAGIC {
        return Err(CoreError::InvalidAse("bad magic".into()));
    }
    let _frames = u16::from_le_bytes([bytes[4], bytes[5]]);
    // Full parser: Phase 0 implementation
    Ok(SpriteDocument::new_blank(16, 16, crate::document::ColorMode::Indexed))
}

pub fn write_ase(doc: &SpriteDocument) -> Result<Vec<u8>, CoreError> {
    let _ = doc;
    Err(CoreError::InvalidAse("ASE writer not yet implemented".into()))
}
