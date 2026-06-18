//! CPU post-process pipeline for AI output and fast quantize (offline path).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostProcessOptions {
    pub target_width: u32,
    pub target_height: u32,
    pub max_colors: u8,
    pub dither: bool,
    pub despeckle: bool,
    pub outline: OutlineMode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OutlineMode {
    #[default]
    None,
    Black1px,
    ProjectStyle,
}

/// Nearest-neighbor downscale → palette quantize → optional despeckle.
pub fn process_rgba(
    rgba: &[u8],
    width: u32,
    height: u32,
    opts: &PostProcessOptions,
) -> Vec<u8> {
    let _ = (rgba, width, height, opts);
    // Phase 1: port from plan pipeline
    vec![]
}
