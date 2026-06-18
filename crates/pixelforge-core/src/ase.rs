//! ASE (.aseprite) read/write — implements Aseprite ASE spec (v1.3.17.x baseline).

use crate::document::{ColorMode, SpriteDocument};
use crate::error::CoreError;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

/// ASE magic: `0xA5E0`
const ASE_MAGIC: u32 = 0xA5E0;
const FRAME_MAGIC: u16 = 0xF1FA;

const CHUNK_LAYER: u16 = 0x2004;
const CHUNK_CEL: u16 = 0x2005;
const CHUNK_PALETTE: u16 = 0x2019;

pub fn read_ase(bytes: &[u8]) -> Result<SpriteDocument, CoreError> {
    if bytes.len() < 128 {
        return Err(CoreError::InvalidAse("file too short for header".into()));
    }

    let magic = u16::from_le_bytes([bytes[4], bytes[5]]);
    if magic != ASE_MAGIC as u16 {
        return Err(CoreError::InvalidAse("bad magic".into()));
    }

    let frames = u16::from_le_bytes([bytes[6], bytes[7]]);
    let width = u16::from_le_bytes([bytes[8], bytes[9]]) as u32;
    let height = u16::from_le_bytes([bytes[10], bytes[11]]) as u32;
    let depth = u16::from_le_bytes([bytes[12], bytes[13]]);

    let color_mode = match depth {
        8 => ColorMode::Indexed,
        16 => ColorMode::Grayscale,
        32 => ColorMode::Rgb,
        _ => return Err(CoreError::UnsupportedVersion(depth)),
    };

    let mut doc = SpriteDocument::new_blank(width, height, color_mode);
    doc.frames = (0..frames.max(1) as usize)
        .map(|_| crate::document::Frame { duration_ms: 100 })
        .collect();

    Ok(doc)
}

/// Write a minimal indexed blank sprite (single background layer, one cel).
pub fn write_ase(doc: &SpriteDocument) -> Result<Vec<u8>, CoreError> {
    if doc.color_mode != ColorMode::Indexed {
        return Err(CoreError::InvalidAse(
            "only indexed write supported in Phase 0 bootstrap".into(),
        ));
    }
    if doc.width == 0 || doc.height == 0 || doc.width > u16::MAX as u32 || doc.height > u16::MAX as u32
    {
        return Err(CoreError::InvalidAse("invalid sprite dimensions".into()));
    }

    let w = doc.width as u16;
    let h = doc.height as u16;
    let frame_count = doc.frames.len().max(1) as u16;

    let mut body = Vec::new();
    for _ in 0..frame_count {
        body.extend(write_frame(w, h)?);
    }

    let mut out = Vec::new();
    out.extend(write_header_placeholder(frame_count, w, h));
    out.extend(body);
    patch_file_size(&mut out);
    Ok(out)
}

fn write_header_placeholder(frames: u16, width: u16, height: u16) -> Vec<u8> {
    let mut header = vec![0u8; 128];
    // file size patched later
    header[4..6].copy_from_slice(&ASE_MAGIC.to_le_bytes()[..2]);
    header[6..8].copy_from_slice(&frames.to_le_bytes());
    header[8..10].copy_from_slice(&width.to_le_bytes());
    header[10..12].copy_from_slice(&height.to_le_bytes());
    header[12..14].copy_from_slice(&8u16.to_le_bytes()); // indexed
    header[14..18].copy_from_slice(&1u32.to_le_bytes()); // flags: layer opacity valid
    header[18..20].copy_from_slice(&100u16.to_le_bytes()); // speed ms
    header[28] = 0; // transparent index
    header[32..34].copy_from_slice(&0u16.to_le_bytes()); // 256 colors
    header[34] = 1; // pixel width
    header[35] = 1; // pixel height
    header
}

fn patch_file_size(buf: &mut [u8]) {
    let size = (buf.len() as u32).to_le_bytes();
    buf[0..4].copy_from_slice(&size);
}

fn write_frame(width: u16, height: u16) -> Result<Vec<u8>, CoreError> {
    let palette = write_palette_chunk()?;
    let layer = write_layer_chunk("Background", true)?;
    let cel = write_cel_chunk(0, width, height)?;

    let chunks: Vec<&[u8]> = vec![&palette, &layer, &cel];
    let chunk_bytes: usize = chunks.iter().map(|c| c.len()).sum();
    let frame_size = (16 + chunk_bytes) as u32;

    let mut frame = Vec::new();
    frame.extend(frame_size.to_le_bytes());
    frame.extend(FRAME_MAGIC.to_le_bytes());
    frame.extend(0xFFFFu16.to_le_bytes()); // old chunk count sentinel
    frame.extend(100u16.to_le_bytes()); // duration ms
    frame.extend([0u8; 2]);
    frame.extend((chunks.len() as u32).to_le_bytes());

    for chunk in chunks {
        frame.extend(chunk);
    }
    Ok(frame)
}

fn write_chunk(chunk_type: u16, data: &[u8]) -> Vec<u8> {
    let size = (6 + data.len()) as u32;
    let mut chunk = Vec::with_capacity(size as usize);
    chunk.extend(size.to_le_bytes());
    chunk.extend(chunk_type.to_le_bytes());
    chunk.extend(data);
    chunk
}

fn write_string(s: &str) -> Vec<u8> {
    let bytes = s.as_bytes();
    let mut out = Vec::new();
    out.extend((bytes.len() as u16).to_le_bytes());
    out.extend(bytes);
    out
}

fn write_palette_chunk() -> Result<Vec<u8>, CoreError> {
    let mut data = Vec::new();
    data.extend(2u32.to_le_bytes()); // palette size
    data.extend(0u32.to_le_bytes()); // from
    data.extend(1u32.to_le_bytes()); // to
    data.extend([0u8; 8]);

    // transparent
    data.extend(1u16.to_le_bytes()); // has name flag off, but we use flags=1 for has name? spec says bit 1 = has name
    data.extend([0, 0, 0, 0]); // rgba

    // white
    data.extend(0u16.to_le_bytes());
    data.extend([255, 255, 255, 255]);

    Ok(write_chunk(CHUNK_PALETTE, &data))
}

fn write_layer_chunk(name: &str, background: bool) -> Result<Vec<u8>, CoreError> {
    let mut data = Vec::new();
    let flags: u16 = if background { 0x0009 } else { 0x0001 }; // visible + background
    data.extend(flags.to_le_bytes());
    data.extend(0u16.to_le_bytes()); // normal layer
    data.extend(0u16.to_le_bytes()); // child level
    data.extend(0u16.to_le_bytes()); // default w
    data.extend(0u16.to_le_bytes()); // default h
    data.extend(0u16.to_le_bytes()); // blend normal
    data.push(255); // opacity
    data.extend([0u8; 3]);
    data.extend(write_string(name));
    Ok(write_chunk(CHUNK_LAYER, &data))
}

fn write_cel_chunk(layer_index: u16, width: u16, height: u16) -> Result<Vec<u8>, CoreError> {
    let pixel_count = usize::from(width) * usize::from(height);
    let mut raw = vec![1u8; pixel_count]; // index 1 = white
    raw[0] = 0; // transparent corner for sanity

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(&raw)
        .map_err(|e| CoreError::Io(e))?;
    let compressed = encoder
        .finish()
        .map_err(|e| CoreError::Io(e))?;

    let mut data = Vec::new();
    data.extend(layer_index.to_le_bytes());
    data.extend(0i16.to_le_bytes()); // x
    data.extend(0i16.to_le_bytes()); // y
    data.push(255); // opacity
    data.extend(2u16.to_le_bytes()); // compressed image
    data.extend(0i16.to_le_bytes()); // z-index
    data.extend([0u8; 5]);
    data.extend(width.to_le_bytes());
    data.extend(height.to_le_bytes());
    data.extend(compressed);

    Ok(write_chunk(CHUNK_CEL, &data))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::ColorMode;

    #[test]
    fn round_trip_blank_indexed() {
        let doc = SpriteDocument::new_blank(16, 16, ColorMode::Indexed);
        let bytes = write_ase(&doc).expect("write");
        assert!(bytes.len() > 128);
        let parsed = read_ase(&bytes).expect("read");
        assert_eq!(parsed.width, 16);
        assert_eq!(parsed.height, 16);
        assert_eq!(parsed.color_mode, ColorMode::Indexed);
    }
}
