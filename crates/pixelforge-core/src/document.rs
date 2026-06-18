use serde::{Deserialize, Serialize};

/// In-memory sprite document mirroring Aseprite's model (subset for bootstrap).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteDocument {
    pub width: u32,
    pub height: u32,
    pub color_mode: ColorMode,
    pub frames: Vec<Frame>,
    pub layers: Vec<Layer>,
    pub tags: Vec<FrameTag>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ColorMode {
    Rgb,
    Grayscale,
    Indexed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub duration_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub opacity: u8,
    pub kind: LayerKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerKind {
    Normal,
    Group,
    Tilemap,
    Reference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameTag {
    pub name: String,
    pub from_frame: u32,
    pub to_frame: u32,
    pub direction: TagDirection,
    pub repeat: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TagDirection {
    Forward,
    Reverse,
    PingPong,
}

impl SpriteDocument {
    pub fn new_blank(width: u32, height: u32, color_mode: ColorMode) -> Self {
        Self {
            width,
            height,
            color_mode,
            frames: vec![Frame { duration_ms: 100 }],
            layers: vec![Layer {
                name: "Layer 1".into(),
                visible: true,
                opacity: 255,
                kind: LayerKind::Normal,
            }],
            tags: vec![],
        }
    }
}
