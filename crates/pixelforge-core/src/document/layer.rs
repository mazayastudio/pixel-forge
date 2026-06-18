use serde::{Deserialize, Serialize};

use super::tileset::TilemapGrid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum BlendMode {
    #[default]
    Normal = 0,
    Multiply = 1,
    Screen = 2,
    Overlay = 3,
    Darken = 4,
    Lighten = 5,
    ColorDodge = 6,
    ColorBurn = 7,
    HardLight = 8,
    SoftLight = 9,
    Difference = 10,
    Exclusion = 11,
    Hue = 12,
    Saturation = 13,
    Color = 14,
    Luminosity = 15,
    Addition = 16,
    Subtract = 17,
    Divide = 18,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceSource {
    FilePath { path: String },
    Clipboard,
    Embedded { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LayerKind {
    Normal,
    Group { collapsed: bool },
    Tilemap {
        tileset_id: usize,
        grid: TilemapGrid,
    },
    Reference { source: ReferenceSource },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub opacity: u8,
    pub blend_mode: BlendMode,
    pub parent: Option<usize>,
    pub kind: LayerKind,
    pub is_background: bool,
    pub is_editable: bool,
}

impl Layer {
    pub fn background(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visible: true,
            opacity: 255,
            blend_mode: BlendMode::Normal,
            parent: None,
            kind: LayerKind::Normal,
            is_background: true,
            is_editable: true,
        }
    }

    pub fn normal(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visible: true,
            opacity: 255,
            blend_mode: BlendMode::Normal,
            parent: None,
            kind: LayerKind::Normal,
            is_background: false,
            is_editable: true,
        }
    }

    pub fn group(name: impl Into<String>, collapsed: bool) -> Self {
        Self {
            name: name.into(),
            visible: true,
            opacity: 255,
            blend_mode: BlendMode::Normal,
            parent: None,
            kind: LayerKind::Group { collapsed },
            is_background: false,
            is_editable: true,
        }
    }

    pub fn accepts_cels(&self) -> bool {
        matches!(self.kind, LayerKind::Normal)
    }
}
