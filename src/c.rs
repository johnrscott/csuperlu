//! C-to-Rust interface for SuperLU
//!
//! This module contains a low-level interface to the csuperlu_sys
//! library. The purpose is to expose the functions in csuperlu_sys
//! in a form with native rust types, using a rust naming convention
//! for names. Functions which depend on precision (many of them) are
//! defined inside the ValueType trait. Most functions are unsafe.
//!
//! To use the simple driver to solve a linear system, follow these
//! steps:
//!
//! 1. Create the matrix $A$ as a CompColMat (see comp_col).
//! 2. Create the right-hand side $B$ as a DenseMat (see dense).
//! 3. Solve the system using the c_simple_driver function (simple_driver)
//!
//! The matrices are created using two helper structs (named *Raw)
//! that contain the rust vectors comprising each matrix. Memory
//! management is handled automatically. Results can be obtained out
//! of the matrix types by converting back to *Raw.


pub mod simple_driver;
pub mod value_type;
mod free;
pub mod options;
pub mod stat;
mod super_matrix;
pub mod comp_col;
pub mod dense;
pub mod error;
