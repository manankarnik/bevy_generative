#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs
)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::module_name_repetitions)]

//! Procedural generation in bevy

mod noise;

pub mod noise_map;
