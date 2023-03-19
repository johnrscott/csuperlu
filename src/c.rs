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
//! 1. Create the matrix $A$ as a [CompColMat](comp_col::CompColMat).
//! 2. Create the right-hand side $B$ as a [DenseMat](dense::DenseMat).
//! 3. Solve the system using the [simple_driver](simple_driver::SimpleDriver::simple_driver)
//! function.
//!
//! The matrices are created using two helper structs [CompColRaw](comp_col::CompColRaw)
//! and [DenseRaw](dense::DenseRaw) that contain the rust vectors comprising each matrix.
//!
//! The solution is stored in a [SimpleSolution](simple_driver::SimpleSolution)
//! struct, which contains the the solution matrix $X$, the column and row permutation
//! matrices that were used, and the $LU$-decomposition. The Vec stored inside
//! the solution $X$ can be accessed by converting back to the DenseRaw type.
//!
//! Currently the wrapper to access the $LU$ factors is not implemented.
//! Direct access to the contents of these matrices is possible using the
//! [CSuperMatrix::store](super_matrix::CSuperMatrix::store) function, which
//! returns a pointer to the raw data in the matrix.
//!
//!


pub mod simple_driver;
pub mod value_type;
mod free;
pub mod options;
pub mod stat;
pub mod super_matrix;
pub mod comp_col;
pub mod dense;
pub mod error;
