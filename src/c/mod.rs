//! The C module contains the low-level interface to the SuperLU library.
//!
//! All functions that interface directly with SuperLU have the same names
//! as the corresponding SuperLU functions, with an additional c_ prepended
//! to the name.
//!

pub mod comp_col;
pub mod dense;
pub mod expert_driver;
pub mod options;
pub mod simple_driver;
pub mod stat;
pub mod super_node;
pub mod super_matrix;
