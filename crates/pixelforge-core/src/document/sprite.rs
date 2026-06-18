use serde::{Deserialize, Serialize};

use crate::error::CoreError;

use super::cel::{Cel, CelId, CelLink};
use super::layer::{Layer, LayerKind, ReferenceSource};
use super::pixels::{ColorMode, ColorProfile, Palette, PixelGrid};
use super::slice::{Slice, SliceBounds};
use super::tileset::{TilemapGrid, TilesetRef};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Frame {
    pub duration_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FrameTag {
    pub name: String,
    pub from_frame: u32,
    pub to_frame: u32,
    pub direction: TagDirection,
    pub repeat: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TagDirection {
    Forward,
    Reverse,
    PingPong,
}

/// In-memory sprite document mirroring Aseprite semantics (DOC-01…14).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpriteDocument {
    pub width: u32,
    pub height: u32,
    pub color_mode: ColorMode,
    pub pixel_aspect: (u8, u8),
    pub color_profile: ColorProfile,
    pub user_data: Vec<u8>,
    pub palette: Palette,
    pub frames: Vec<Frame>,
    pub layers: Vec<Layer>,
    pub images: Vec<PixelGrid>,
    pub cels: Vec<Cel>,
    pub tags: Vec<FrameTag>,
    pub slices: Vec<Slice>,
    pub tilesets: Vec<TilesetRef>,
}

impl SpriteDocument {
    pub fn new_blank(width: u32, height: u32, color_mode: ColorMode) -> Self {
        let palette = match color_mode {
            ColorMode::Indexed => Palette::indexed_default(),
            _ => Palette { colors: vec![] },
        };
        let image = PixelGrid::new_empty(width, height, color_mode);
        let mut doc = Self {
            width,
            height,
            color_mode,
            pixel_aspect: (1, 1),
            color_profile: ColorProfile::Srgb,
            user_data: Vec::new(),
            palette,
            frames: vec![Frame { duration_ms: 100 }],
            layers: vec![Layer::background("Background")],
            images: vec![image],
            cels: Vec::new(),
            tags: Vec::new(),
            slices: Vec::new(),
            tilesets: Vec::new(),
        };
        doc.cels.push(Cel {
            layer: 0,
            frame: 0,
            x: 0,
            y: 0,
            opacity: 255,
            z_index: 0,
            image_id: 0,
            link: CelLink::None,
        });
        doc
    }

    pub fn cel(&self, layer: usize, frame: usize) -> Option<&Cel> {
        self.cels
            .iter()
            .find(|c| c.layer == layer && c.frame == frame)
    }

    pub fn cel_mut(&mut self, layer: usize, frame: usize) -> Option<&mut Cel> {
        self.cels
            .iter_mut()
            .find(|c| c.layer == layer && c.frame == frame)
    }

    pub fn image(&self, image_id: usize) -> Option<&PixelGrid> {
        self.images.get(image_id)
    }

    pub fn resolved_pixels(&self, cel: &Cel) -> Option<&PixelGrid> {
        let image_id = self.resolved_image_id(cel)?;
        self.images.get(image_id)
    }

    fn resolved_image_id(&self, cel: &Cel) -> Option<usize> {
        match &cel.link {
            CelLink::None => Some(cel.image_id),
            CelLink::Linked { source, .. } => self
                .cel(source.layer, source.frame)
                .map(|s| self.resolved_image_id(s).unwrap_or(s.image_id)),
        }
    }

    pub fn add_frame(&mut self) -> usize {
        let frame = self.frames.len();
        self.frames.push(Frame { duration_ms: 100 });
        let prev = frame.saturating_sub(1);
        for (layer_idx, layer) in self.layers.iter().enumerate() {
            if !layer.accepts_cels() {
                continue;
            }
            if let Some(prev_cel) = self.cel(layer_idx, prev).cloned() {
                let image_id = self.images.len();
                if let Some(px) = self.resolved_pixels(&prev_cel) {
                    self.images.push(px.clone());
                } else {
                    self.images
                        .push(PixelGrid::new_empty(self.width, self.height, self.color_mode));
                }
                self.cels.push(Cel {
                    layer: layer_idx,
                    frame,
                    x: prev_cel.x,
                    y: prev_cel.y,
                    opacity: prev_cel.opacity,
                    z_index: prev_cel.z_index,
                    image_id,
                    link: CelLink::None,
                });
            }
        }
        frame
    }

    pub fn set_frame_count(&mut self, count: usize) {
        while self.frames.len() < count {
            self.add_frame();
        }
        if self.frames.len() > count {
            self.frames.truncate(count);
            self.cels.retain(|c| c.frame < count);
        }
    }

    pub fn add_layer(&mut self, layer: Layer) -> usize {
        let idx = self.layers.len();
        self.layers.push(layer);
        for frame in 0..self.frames.len() {
            let image_id = self.images.len();
            self.images
                .push(PixelGrid::new_empty(self.width, self.height, self.color_mode));
            self.cels.push(Cel {
                layer: idx,
                frame,
                x: 0,
                y: 0,
                opacity: 255,
                z_index: 0,
                image_id,
                link: CelLink::None,
            });
        }
        idx
    }

    pub fn add_group(&mut self, name: impl Into<String>, collapsed: bool) -> usize {
        self.add_layer(Layer::group(name, collapsed))
    }

    pub fn add_reference_layer(&mut self, name: impl Into<String>, source: ReferenceSource) -> usize {
        let layer = Layer {
            name: name.into(),
            visible: true,
            opacity: 255,
            blend_mode: super::layer::BlendMode::Normal,
            parent: None,
            kind: LayerKind::Reference { source },
            is_background: false,
            is_editable: false,
        };
        let idx = self.layers.len();
        self.layers.push(layer);
        idx
    }

    pub fn add_tilemap_layer(
        &mut self,
        name: impl Into<String>,
        tileset_id: usize,
        grid: TilemapGrid,
    ) -> usize {
        let layer = Layer {
            name: name.into(),
            visible: true,
            opacity: 255,
            blend_mode: super::layer::BlendMode::Normal,
            parent: None,
            kind: LayerKind::Tilemap {
                tileset_id,
                grid,
            },
            is_background: false,
            is_editable: true,
        };
        let idx = self.layers.len();
        self.layers.push(layer);
        idx
    }

    pub fn move_layer(&mut self, from: usize, to: usize) -> Result<(), CoreError> {
        if from >= self.layers.len() || to >= self.layers.len() {
            return Err(CoreError::Validation("layer index out of range".into()));
        }
        if self.layers[from].is_background || to == 0 {
            return Err(CoreError::Validation(
                "background layer cannot be moved".into(),
            ));
        }
        let layer = self.layers.remove(from);
        self.layers.insert(to, layer);
        Ok(())
    }

    pub fn link_cels(&mut self, from: CelId, to: CelId, share_position: bool) -> Result<(), CoreError> {
        let source = self
            .cel(from.layer, from.frame)
            .ok_or_else(|| CoreError::Validation("source cel missing".into()))?
            .clone();
        let target = self
            .cel_mut(to.layer, to.frame)
            .ok_or_else(|| CoreError::Validation("target cel missing".into()))?;
        target.image_id = source.image_id;
        target.link = CelLink::Linked {
            source: from,
            share_position,
        };
        if share_position {
            target.x = source.x;
            target.y = source.y;
        }
        Ok(())
    }

    pub fn unlink_cel(&mut self, layer: usize, frame: usize) -> Result<(), CoreError> {
        let cel = self
            .cel(layer, frame)
            .ok_or_else(|| CoreError::Validation("cel missing".into()))?
            .clone();
        if matches!(cel.link, CelLink::None) {
            return Ok(());
        }
        let pixels = self
            .resolved_pixels(&cel)
            .cloned()
            .unwrap_or_else(|| PixelGrid::new_empty(self.width, self.height, self.color_mode));
        let image_id = self.images.len();
        self.images.push(pixels);
        let target = self.cel_mut(layer, frame).expect("cel exists");
        target.image_id = image_id;
        target.link = CelLink::None;
        Ok(())
    }

    pub fn add_tag(&mut self, tag: FrameTag) {
        self.tags.push(tag);
    }

    pub fn add_slice(&mut self, name: impl Into<String>, bounds: SliceBounds) {
        self.slices.push(Slice {
            name: name.into(),
            bounds,
            pivot: None,
        });
    }

    pub fn attach_tileset(&mut self, tileset: TilesetRef) -> usize {
        let id = self.tilesets.len();
        self.tilesets.push(tileset);
        id
    }

    pub fn validate(&self) -> Result<(), CoreError> {
        if self.width == 0 || self.height == 0 {
            return Err(CoreError::Validation("canvas dimensions must be > 0".into()));
        }
        if self.width > u16::MAX as u32 || self.height > u16::MAX as u32 {
            return Err(CoreError::Validation("canvas dimensions exceed ASE limit".into()));
        }
        if self.frames.is_empty() {
            return Err(CoreError::Validation("sprite must have at least one frame".into()));
        }
        if self.layers.is_empty() {
            return Err(CoreError::Validation("sprite must have at least one layer".into()));
        }

        let bg_count = self.layers.iter().filter(|l| l.is_background).count();
        if bg_count != 1 {
            return Err(CoreError::Validation(format!(
                "expected exactly one background layer, found {bg_count}"
            )));
        }
        if !self.layers[0].is_background {
            return Err(CoreError::Validation(
                "background layer must be at stack index 0".into(),
            ));
        }
        if self.layers[0].opacity != 255 {
            return Err(CoreError::Validation(
                "background layer must be fully opaque".into(),
            ));
        }

        if self.color_mode == ColorMode::Indexed && self.palette.len() > Palette::MAX_COLORS {
            return Err(CoreError::Validation("indexed palette exceeds 256 colors".into()));
        }

        for layer in &self.layers {
            if let Some(parent) = layer.parent {
                if parent >= self.layers.len() {
                    return Err(CoreError::Validation("invalid layer parent index".into()));
                }
            }
        }

        for cel in &self.cels {
            if cel.layer >= self.layers.len() || cel.frame >= self.frames.len() {
                return Err(CoreError::Validation("cel references invalid layer or frame".into()));
            }
            if cel.image_id >= self.images.len() {
                return Err(CoreError::Validation("cel references invalid image".into()));
            }
            let layer = &self.layers[cel.layer];
            if !layer.accepts_cels() {
                return Err(CoreError::Validation(format!(
                    "layer '{}' does not accept cels",
                    layer.name
                )));
            }
        }

        for ts in &self.tilesets {
            if ts.tile_width == 0 || ts.tile_height == 0 {
                return Err(CoreError::Validation("tileset tile size must be > 0".into()));
            }
        }

        Ok(())
    }

    /// Cels for a frame sorted by z-index (DOC-11).
    pub fn cels_in_frame(&self, frame: usize) -> Vec<&Cel> {
        let mut list: Vec<&Cel> = self.cels.iter().filter(|c| c.frame == frame).collect();
        list.sort_by_key(|c| c.z_index);
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::layer::BlendMode;

    #[test]
    fn blank_creation_indexed() {
        let doc = SpriteDocument::new_blank(16, 16, ColorMode::Indexed);
        doc.validate().expect("valid");
        assert_eq!(doc.width, 16);
        assert_eq!(doc.color_mode, ColorMode::Indexed);
        assert_eq!(doc.pixel_aspect, (1, 1));
        assert_eq!(doc.color_profile, ColorProfile::Srgb);
        assert!(doc.layers[0].is_background);
        assert!(doc.cel(0, 0).is_some());
        assert_eq!(doc.palette.get(1), Some(super::super::pixels::Rgba::WHITE));
    }

    #[test]
    fn blank_creation_rgb_and_gray() {
        let rgb = SpriteDocument::new_blank(8, 8, ColorMode::Rgb);
        assert_eq!(rgb.images[0].mode(), ColorMode::Rgb);
        let gray = SpriteDocument::new_blank(8, 8, ColorMode::Grayscale);
        assert_eq!(gray.images[0].mode(), ColorMode::Grayscale);
    }

    #[test]
    fn cel_grid_xy_and_z_index() {
        let mut doc = SpriteDocument::new_blank(16, 16, ColorMode::Indexed);
        doc.add_layer(Layer::normal("Layer 2"));
        let cel = doc.cel_mut(1, 0).unwrap();
        cel.x = 4;
        cel.y = -2;
        cel.z_index = 3;
        let ordered = doc.cels_in_frame(0);
        assert_eq!(ordered.last().unwrap().z_index, 3);
    }

    #[test]
    fn background_rules() {
        let mut doc = SpriteDocument::new_blank(8, 8, ColorMode::Indexed);
        let err = doc.move_layer(0, 1).unwrap_err();
        assert!(matches!(err, CoreError::Validation(_)));

        let mut bad = doc.clone();
        bad.layers.push(Layer::background("Bg2"));
        assert!(bad.validate().is_err());
    }

    #[test]
    fn layer_group_collapse() {
        let mut doc = SpriteDocument::new_blank(16, 16, ColorMode::Indexed);
        let gid = doc.add_group("Group", true);
        assert!(matches!(
            doc.layers[gid].kind,
            LayerKind::Group { collapsed: true }
        ));
    }

    #[test]
    fn linked_cels_share_pixels() {
        let mut doc = SpriteDocument::new_blank(8, 8, ColorMode::Indexed);
        doc.set_frame_count(2);
        doc.link_cels(CelId { layer: 0, frame: 0 }, CelId { layer: 0, frame: 1 }, false)
            .unwrap();
        let img0 = doc.cel(0, 0).unwrap().image_id;
        let img1 = doc.cel(0, 1).unwrap().image_id;
        assert_eq!(img0, img1);

        doc.unlink_cel(0, 1).unwrap();
        let img1_after = doc.cel(0, 1).unwrap().image_id;
        assert_ne!(img0, img1_after);
    }

    #[test]
    fn blend_and_opacity_serde() {
        let mut doc = SpriteDocument::new_blank(8, 8, ColorMode::Rgb);
        doc.layers[0].blend_mode = BlendMode::Multiply;
        doc.cel_mut(0, 0).unwrap().opacity = 128;
        let json = serde_json::to_string(&doc).unwrap();
        let back: SpriteDocument = serde_json::from_str(&json).unwrap();
        assert_eq!(back.layers[0].blend_mode, BlendMode::Multiply);
        assert_eq!(back.cel(0, 0).unwrap().opacity, 128);
    }

    #[test]
    fn metadata_user_data() {
        let mut doc = SpriteDocument::new_blank(8, 8, ColorMode::Indexed);
        doc.user_data = b"pixelforge-test".to_vec();
        doc.pixel_aspect = (2, 1);
        doc.validate().unwrap();
        assert_eq!(doc.user_data, b"pixelforge-test");
        assert_eq!(doc.pixel_aspect, (2, 1));
    }

    #[test]
    fn tilemap_and_reference_layers() {
        let mut doc = SpriteDocument::new_blank(32, 32, ColorMode::Indexed);
        let ts = doc.attach_tileset(TilesetRef {
            name: "tiles".into(),
            tile_width: 16,
            tile_height: 16,
            external_path: None,
        });
        doc.add_tilemap_layer("Map", ts, TilemapGrid::new(4, 4));
        doc.add_reference_layer(
            "Ref",
            ReferenceSource::FilePath {
                path: "ref.aseprite".into(),
            },
        );
        doc.validate().unwrap();
    }

    #[test]
    fn json_snapshot_round_trip() {
        let mut doc = SpriteDocument::new_blank(16, 16, ColorMode::Indexed);
        doc.add_layer(Layer::normal("Foreground"));
        doc.add_tag(FrameTag {
            name: "walk".into(),
            from_frame: 0,
            to_frame: 3,
            direction: TagDirection::Forward,
            repeat: 0,
        });
        doc.add_slice(
            "panel",
            SliceBounds {
                x: 2,
                y: 2,
                width: 12,
                height: 12,
            },
        );
        let json = serde_json::to_string_pretty(&doc).unwrap();
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/document/blank_indexed_16.json");
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::fs::write(&path, &json).ok();
        let back: SpriteDocument = serde_json::from_str(&json).unwrap();
        assert_eq!(doc, back);
    }

    #[test]
    fn set_frame_count() {
        let mut doc = SpriteDocument::new_blank(8, 8, ColorMode::Indexed);
        doc.set_frame_count(4);
        assert_eq!(doc.frames.len(), 4);
        assert!(doc.cel(0, 3).is_some());
    }
}
