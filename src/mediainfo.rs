//! This module contains function that relies on third party tool.
//!
#![doc = include_str!("../mediainfo.md")]

mod helper;
pub use helper::extract_metadata;
