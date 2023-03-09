//! C-to-Rust interface for SuperLU
//!
//! This module contains a low-level interface to the csuperlu_sys
//! library. The purpose is to expose the functions in csuperlu_sys
//! in a form with native rust types, using a rust naming convention
//! for names. Functions which depend on precision (many of them) are
//! defined inside the ValueType trait. Most functions are unsafe.
//!

pub mod value_type;
pub mod free;
pub mod options;
pub mod stat;
pub mod super_matrix;

#[cfg(test)]
mod tests;
