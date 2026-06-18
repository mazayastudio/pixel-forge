use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CelId {
    pub layer: usize,
    pub frame: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CelLink {
    /// Cel owns its image via `image_id`.
    None,
    /// Shares pixel data with another cel (DOC-14).
    Linked {
        source: CelId,
        /// When true, x/y follow the source cel.
        share_position: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Cel {
    pub layer: usize,
    pub frame: usize,
    pub x: i32,
    pub y: i32,
    pub opacity: u8,
    pub z_index: i16,
    pub image_id: usize,
    pub link: CelLink,
}

impl Cel {
    pub fn id(&self) -> CelId {
        CelId {
            layer: self.layer,
            frame: self.frame,
        }
    }
}
