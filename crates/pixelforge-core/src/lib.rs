//! PixelForge core — Aseprite v1.3.17.x parity engine.
//!
//! Compiled to WASM (web) and native (Android NDK). One codebase, one parity suite.

pub mod ase;
pub mod document;
pub mod error;
pub mod postprocess;

pub use document::SpriteDocument;
pub use error::CoreError;
