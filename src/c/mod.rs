//! The C module contains low-level interface to the SuperLU library.
//!
//! All functions that interface directly with SuperLU have the same names
//! as the corresponding SuperLU functions, with an additional c_ prepended
//! to the name. 
//!

pub mod utils;
pub mod stat;
pub mod options;
pub mod comp_col;
pub mod dense;
pub mod simple_driver;
pub mod expert_driver;
pub mod super_node;
