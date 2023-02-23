//! C-to-Rust interface for SuperLU
//!
//! This module contains a low-level interface to the csuperlu_sys
//! library. The purpose is to expose the functions in csuperlu_sys
//! in a form with native rust types, using a rust naming convention
//! for names. In addition, in certain cases, arguments are inferred
//! in order to make the functions easier to call. Functions which
//! depend on precision (most of them) are defined inside the ValueType
//! trait. Most functions are unsafe.
//!
//! Functions here are in one-to-one correspondence with superlu library
//! functions, but in a form that is easier to understand and call from
//! rust. If you want to do something with superlu functions directly,
//! you can do it from here.

pub mod value_type;
pub mod free;
