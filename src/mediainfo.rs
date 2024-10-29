//! This module contains function that relies on third party tool.
//!
//! MediaInfo official website: [mediaarea.net/en/MediaInfo](https://mediaarea.net/en/MediaInfo)
//!
//! # Installing mediainfo
//!
//! The mediainfo tool is required to use the [`extract_metadata`] function. Follow the
//! instructions below to install mediainfo on your system.
//!
//! ## Windows
//!
//! To install mediainfo on Windows, you can use the winget package manager:
//! ```powershell
//! winget install MediaArea.MediaInfo
//! ```
//!
//! ## macOS
//! For macOS users, mediainfo can be installed using Homebrew:
//! ```bash
//! brew install mediainfo
//! ```
//!
//! ## Linux
//!
//! On most Linux distributions, mediainfo can be installed via the package manager.
//!
//! ## Verifying Installation
//!
//! After installation, verify that mediainfo is available by running:
//!
//! ```shell
//! mediainfo --version
//! ```
//! If correctly installed, this should display the version of mediainfo.

mod helper;
pub use helper::extract_metadata;
