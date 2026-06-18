use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ColorMode {
    Rgb,
    Grayscale,
    Indexed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ColorProfile {
    #[default]
    Srgb,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub const TRANSPARENT: Self = Self { r: 0, g: 0, b: 0, a: 0 };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
}

/// Up to 256 entries for indexed sprites (DOC-02).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Palette {
    pub colors: Vec<Rgba>,
}

impl Default for Palette {
    fn default() -> Self {
        Self::indexed_default()
    }
}

impl Palette {
    pub const MAX_COLORS: usize = 256;

    pub fn indexed_default() -> Self {
        let mut colors = vec![Rgba::TRANSPARENT; Self::MAX_COLORS];
        colors[1] = Rgba::WHITE;
        Self { colors }
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    pub fn get(&self, index: u8) -> Option<Rgba> {
        self.colors.get(index as usize).copied()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum PixelGrid {
    Indexed {
        width: u32,
        height: u32,
        indices: Vec<u8>,
    },
    Grayscale {
        width: u32,
        height: u32,
        values: Vec<u8>,
    },
    Rgb {
        width: u32,
        height: u32,
        rgba: Vec<u8>,
    },
}

impl PixelGrid {
    pub fn new_empty(width: u32, height: u32, mode: ColorMode) -> Self {
        let count = (width as usize).saturating_mul(height as usize);
        match mode {
            ColorMode::Indexed => Self::Indexed {
                width,
                height,
                indices: vec![0; count],
            },
            ColorMode::Grayscale => Self::Grayscale {
                width,
                height,
                values: vec![0; count],
            },
            ColorMode::Rgb => Self::Rgb {
                width,
                height,
                rgba: vec![0; count * 4],
            },
        }
    }

    pub fn width(&self) -> u32 {
        match self {
            Self::Indexed { width, .. } | Self::Grayscale { width, .. } | Self::Rgb { width, .. } => {
                *width
            }
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Self::Indexed { height, .. }
            | Self::Grayscale { height, .. }
            | Self::Rgb { height, .. } => *height,
        }
    }

    pub fn mode(&self) -> ColorMode {
        match self {
            Self::Indexed { .. } => ColorMode::Indexed,
            Self::Grayscale { .. } => ColorMode::Grayscale,
            Self::Rgb { .. } => ColorMode::Rgb,
        }
    }

    pub fn indexed_indices(&self) -> Option<&[u8]> {
        match self {
            Self::Indexed { indices, .. } => Some(indices),
            _ => None,
        }
    }
}
