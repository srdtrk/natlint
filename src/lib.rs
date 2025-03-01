//! # Natlint
//! A linter for solidity natspecs.

// TODO: Use README.md as the crate documentation when we have a readme
//#![doc = include_str!("../README.md")]
#![deny(clippy::nursery, clippy::pedantic, warnings, missing_docs)]

pub mod cli;
pub mod files;
pub mod parser;
pub mod rules;
