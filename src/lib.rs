#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs
)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::too_many_lines)]

//! Procedural generation in bevy

mod util;

/// Noise configuration
pub mod noise;
/// Map and texture generation
pub mod noise_map;
/// Planet generation
pub mod planet;
/// Terrain  generation
pub mod terrain;
