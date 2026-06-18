use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TilesetRef {
    pub name: String,
    pub tile_width: u32,
    pub tile_height: u32,
    pub external_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TilemapGrid {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<u32>,
}

impl TilemapGrid {
    pub fn new(width: u32, height: u32) -> Self {
        let count = (width as usize).saturating_mul(height as usize);
        Self {
            width,
            height,
            tiles: vec![0; count],
        }
    }
}
