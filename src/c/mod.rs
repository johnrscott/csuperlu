//! The C module contains low-level interface to the SuperLU library.
//!
//! All functions that interface directly with SuperLU have the same names
//! as the corresponding SuperLU functions, with an additional c_ prepended
//! to the name. 
//!

pub mod utils;
pub mod comp_col;
pub mod dense;
pub mod dgssv;
pub mod super_node;
