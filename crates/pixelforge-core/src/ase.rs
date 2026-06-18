//! ASE (.aseprite) read/write — implements Aseprite ASE spec (v1.3.17.x baseline).

use crate::document::{Cel, ColorMode, Layer, Palette, PixelGrid, SpriteDocument};
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
    doc.set_frame_count(frames.max(1) as usize);

    Ok(doc)
}

/// Write a minimal indexed blank sprite from `SpriteDocument` cel/palette data.
pub fn write_ase(doc: &SpriteDocument) -> Result<Vec<u8>, CoreError> {
    doc.validate()?;
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
    for frame_idx in 0..frame_count as usize {
        body.extend(write_frame(doc, frame_idx, w, h)?);
    }

    let mut out = Vec::new();
    out.extend(write_header_placeholder(
        frame_count,
        w,
        h,
        doc.pixel_aspect.0,
        doc.pixel_aspect.1,
    ));
    out.extend(body);
    patch_file_size(&mut out);
    Ok(out)
}

fn write_header_placeholder(
    frames: u16,
    width: u16,
    height: u16,
    pixel_w: u8,
    pixel_h: u8,
) -> Vec<u8> {
    let mut header = vec![0u8; 128];
    header[4..6].copy_from_slice(&ASE_MAGIC.to_le_bytes()[..2]);
    header[6..8].copy_from_slice(&frames.to_le_bytes());
    header[8..10].copy_from_slice(&width.to_le_bytes());
    header[10..12].copy_from_slice(&height.to_le_bytes());
    header[12..14].copy_from_slice(&8u16.to_le_bytes());
    header[14..18].copy_from_slice(&1u32.to_le_bytes());
    header[18..20].copy_from_slice(&100u16.to_le_bytes());
    header[28] = 0;
    header[32..34].copy_from_slice(&0u16.to_le_bytes());
    header[34] = pixel_w;
    header[35] = pixel_h;
    header
}

fn patch_file_size(buf: &mut [u8]) {
    let size = (buf.len() as u32).to_le_bytes();
    buf[0..4].copy_from_slice(&size);
}

fn write_frame(
    doc: &SpriteDocument,
    frame_idx: usize,
    width: u16,
    height: u16,
) -> Result<Vec<u8>, CoreError> {
    let palette = write_palette_chunk(&doc.palette)?;
    let mut chunks: Vec<Vec<u8>> = vec![palette];

    for (layer_idx, layer) in doc.layers.iter().enumerate() {
        chunks.push(write_layer_chunk(layer)?);
        if let Some(cel) = doc.cel(layer_idx, frame_idx) {
            let pixels = doc
                .resolved_pixels(cel)
                .ok_or_else(|| CoreError::InvalidAse("cel missing pixel data".into()))?;
            chunks.push(write_cel_chunk(
                layer_idx as u16,
                cel,
                width,
                height,
                pixels,
            )?);
        }
    }

    let duration = doc
        .frames
        .get(frame_idx)
        .map(|f| f.duration_ms)
        .unwrap_or(100) as u16;
    let chunk_bytes: usize = chunks.iter().map(|c| c.len()).sum();
    let frame_size = (16 + chunk_bytes) as u32;

    let mut frame = Vec::new();
    frame.extend(frame_size.to_le_bytes());
    frame.extend(FRAME_MAGIC.to_le_bytes());
    frame.extend(0xFFFFu16.to_le_bytes());
    frame.extend(duration.to_le_bytes());
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

fn write_palette_chunk(palette: &Palette) -> Result<Vec<u8>, CoreError> {
    let active = palette.len().max(2).min(Palette::MAX_COLORS);
    let last = (active - 1) as u32;

    let mut data = Vec::new();
    data.extend((active as u32).to_le_bytes());
    data.extend(0u32.to_le_bytes());
    data.extend(last.to_le_bytes());
    data.extend([0u8; 8]);

    for i in 0..active {
        let color = palette.colors.get(i).copied().unwrap_or_default();
        data.extend(0u16.to_le_bytes());
        data.extend([color.r, color.g, color.b, color.a]);
    }

    Ok(write_chunk(CHUNK_PALETTE, &data))
}

fn write_layer_chunk(layer: &Layer) -> Result<Vec<u8>, CoreError> {
    let mut flags: u16 = 0;
    if layer.visible {
        flags |= 0x0001;
    }
    if layer.is_background {
        flags |= 0x0008;
    }

    let layer_type: u16 = match &layer.kind {
        crate::document::LayerKind::Normal => 0,
        crate::document::LayerKind::Group { .. } => 1,
        crate::document::LayerKind::Tilemap { .. } => 2,
        crate::document::LayerKind::Reference { .. } => 3,
    };

    let mut data = Vec::new();
    data.extend(flags.to_le_bytes());
    data.extend(layer_type.to_le_bytes());
    data.extend(0u16.to_le_bytes());
    data.extend(0u16.to_le_bytes());
    data.extend(0u16.to_le_bytes());
    data.extend((layer.blend_mode as u16).to_le_bytes());
    data.push(layer.opacity);
    data.extend([0u8; 3]);
    data.extend(write_string(&layer.name));
    Ok(write_chunk(CHUNK_LAYER, &data))
}

fn write_cel_chunk(
    layer_index: u16,
    cel: &Cel,
    doc_width: u16,
    doc_height: u16,
    pixels: &PixelGrid,
) -> Result<Vec<u8>, CoreError> {
    let (w, h, raw) = match pixels {
        PixelGrid::Indexed {
            width,
            height,
            indices,
        } => {
            let w = *width as u16;
            let h = *height as u16;
            if w != doc_width || h != doc_height {
                return Err(CoreError::InvalidAse("cel size mismatch with sprite".into()));
            }
            (w, h, indices.clone())
        }
        _ => {
            return Err(CoreError::InvalidAse(
                "indexed write requires indexed cel pixels".into(),
            ));
        }
    };

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&raw).map_err(CoreError::Io)?;
    let compressed = encoder.finish().map_err(CoreError::Io)?;

    let mut data = Vec::new();
    data.extend(layer_index.to_le_bytes());
    data.extend((cel.x as i16).to_le_bytes());
    data.extend((cel.y as i16).to_le_bytes());
    data.push(cel.opacity);
    data.extend(2u16.to_le_bytes());
    data.extend(cel.z_index.to_le_bytes());
    data.extend([0u8; 5]);
    data.extend(w.to_le_bytes());
    data.extend(h.to_le_bytes());
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

    #[test]
    fn round_trip_multi_frame_indexed() {
        let mut doc = SpriteDocument::new_blank(8, 8, ColorMode::Indexed);
        doc.set_frame_count(4);
        let bytes = write_ase(&doc).expect("write");
        let parsed = read_ase(&bytes).expect("read");
        assert_eq!(parsed.frames.len(), 4);
    }
}
