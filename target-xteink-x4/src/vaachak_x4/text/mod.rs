//! Shared text and font primitives for Vaachak X4 apps.
//!
//! This module is intentionally small and allocation-free. It provides the
//! contracts needed by reader, home, settings, and sleep-screen apps before a
//! real shaped glyph atlas renderer is wired in.

pub mod font_asset_reader;
pub mod font_assets;
pub mod font_catalog;
pub mod glyph_bitmap_renderer;
pub mod glyph_cache;
pub mod glyph_run;
pub mod layout;
pub mod script;
pub mod text_run;

pub use font_catalog::{FontDescriptor, font_for_script};
pub use glyph_run::shape_placeholder_run;
pub use layout::TextLayoutStyle;
pub use script::{ScriptClass, dominant_script};
