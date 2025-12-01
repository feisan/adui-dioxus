//! Foundation module providing shared types and utilities for component styling.
//!
//! This module contains:
//! - Semantic classNames/styles system (aligned with Ant Design 6.0)
//! - Variant system for form controls
//! - Common responsive breakpoints

mod semantic;
mod variant;

pub use semantic::*;
pub use variant::*;
