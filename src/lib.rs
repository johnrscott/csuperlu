//! csuperlu is a Rust interface to SuperLU, a C library for solving sparse
//! linear systems. Currently, only the sequential solver is supported, but
//! the intent is to gradually extend the library to support all the features
//! of the underlying C library.
//!
//! The SuperLU User Guide is
//! <https://portal.nersc.gov/project/sparse/superlu/superlu_ug.pdf>, and
//! provides an overview of what SuperLU can do.
//!
//! SuperLU solves sparse systems of linear equations of the form $$AX = B,$$
//! where $A$ is a sparse $n\times n$ matrix, $B$ is a dense
//! $n \times n_\text{rhs}$ matrix of right-hand sides, and $X$ is the matrix
//! of unknowns (the same size as $B$).
//!
//!
//! The (C) function reference for SuperLU is provided  
//! <https://portal.nersc.gov/project/sparse/superlu/superlu_code_html/index.html>
//!

//#![warn(missing_docs)]
pub mod c;
pub mod comp_col;
pub mod dense;
pub mod super_node;
pub mod simple_driver;
pub mod super_matrix;
