//! PixelForge core — Aseprite v1.3.17.x parity engine.
//!
//! Compiled to WASM (web) and native (Android NDK). One codebase, one parity suite.

pub mod ase;
pub mod document;
pub mod error;
pub mod postprocess;

pub use document::{
    BlendMode, Cel, CelId, CelLink, ColorMode, ColorProfile, Frame, FrameTag, Layer, LayerKind,
    Palette, PixelGrid, ReferenceSource, Rgba, Slice, SliceBounds, SpriteDocument, TagDirection,
    TilemapGrid, TilesetRef,
};
pub use error::CoreError;
