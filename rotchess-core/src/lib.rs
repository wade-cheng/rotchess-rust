//! Core library code for headless rotchess logic.
//!
//! This module provides functionality for a [`RotchessEmulator`] to modify
//! and record state over time as it is fed information in the form of
//! [events](`emulator::Event`). External code should read the board's state and
//! draw it for the user. This crate is not responsible for any drawing of state.
//! It does, however, provide functions that help the drawing program understand
//! what to draw.
//!
//! # Important definitions
//!
//! Ok well, coords and angles are almost definitely all outdated and wrong now, haha.
//!
//! - coordinate system: all coordinates are standard euclidean coordinates.
//!   That is, x and y increase going right and up. This may differ from code
//!   that draws the rotchess pieces, where the coordinate system may increase
//!   y when going down.
//! - angles: follows standard math convention. Unless otherwise specified,
//!   they're measured in radians, 0 at the positive x-axis, increasing anticlockwise.
//! - rotchess-unit: an eighth of the side length of the board.

pub mod emulator;
pub mod floating_drift;
pub mod piece;
pub mod turn;

pub use emulator::RotchessEmulator;
