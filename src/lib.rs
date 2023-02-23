//! csuperlu is a Rust interface to SuperLU, a C library for solving sparse
//! linear systems. Currently, only the sequential solver is supported, but
//! the intent is to gradually extend the library to support all the features
//! of the underlying C library.
//!
//! The SuperLU User Guide is
//! [here](https://portal.nersc.gov/project/sparse/superlu/superlu_ug.pdf),
//! and provides an overview of what SuperLU can do. SuperLU solves sparse
//! systems of linear equations of the form $$AX = B,$$ where $A$ is a sparse
//! $n\times n$ matrix, $B$ is a dense $n \times n_\text{rhs}$ matrix of
//! right-hand sides, and $X$ is the matrix of unknowns (the same size as $B$).
//!
//! The (C) function reference for SuperLU is provided
//! [here](https://portal.nersc.gov/project/sparse/superlu/superlu_code_html/index.html). 
//! The functioning of SuperLU is described in detail in the paper *Demmel, James W.,
//! et al. "A supernodal approach to sparse partial pivoting." SIAM Journal on Matrix
//! Analysis and Applications 20.3 (1999): 720-755.*, available
//! [here](https://portal.nersc.gov/project/sparse/xiaoye-web/simax-29176.pdf).
//!
//! # Current status
//!
//! Substantial parts of the library and interface are currently incomplete,
//! so you may have difficuly using it for anything right now. This notice
//! will be removed when the library is in an initial usable state.
//!
//! # Development plans
//!
//! The library is currently under development, and the API is not stable
//! yet. The API will be stable at version 1.0.0.
//!
//! **This version is A frontend package for SuperLU. Currently, it
//! manually links to libsuperlu-dev on unix systems. This means you can use
//! this crate now, if you are happy to manually install superlu yourself. A
//! libsuperlu-sys package is planned that will expose the underlying
//! library in a portable way. This will not affect the API of this crate.**
//!
//! 


//#![warn(missing_docs)]
pub mod error;
pub mod comp_col;
pub mod dense;
pub mod simple_driver;
pub mod super_matrix;
pub mod super_node;
pub mod lu_decomp;
pub mod harwell_boeing;
pub mod utils;
pub mod value_type;

pub use error::Error;

#[cfg(test)]
mod tests {
    
    use csuperlu_sys::options::{colperm_t, superlu_options_t};
    use csuperlu_sys::stat::SuperLUStat_t;
    use crate::comp_col::CompColMatrix;
    use crate::dense::DenseMatrix;
    use crate::simple_driver::{simple_driver, SimpleSolution};
    use crate::utils::distance;
    
    #[test]
    fn comp_col_matrix_values() {

	// Matrix dimensions
	let num_rows = 5usize;
	
	// Matrix elements
	let s: f64 = 19.0;
	let u: f64 = 21.0;
	let p: f64 = 16.0;
	let e: f64 = 5.0;
	let r: f64 = 18.0;
	let l: f64 = 12.0;

	// Vector of doubles of length nnz
	let non_zero_values = vec![s, l, l, u, l, l, u, p, u, e, u, r];

	// Vector of ints of length nnz
	let row_indices = vec![0, 1, 4, 1, 2, 4, 0, 2, 0, 3, 3, 4];

	// Vector of ints of length num_columns + 1
	let column_offsets = vec![0, 3, 6, 8, 10, 12];
	
	// Make the left-hand side matrix
	let mut a = CompColMatrix::from_vectors(num_rows,
						non_zero_values,
						row_indices,
						column_offsets);

	// Check non-zero matrix values
	assert_eq!((a.value(0,0) - s).abs() < 1e-8, true);
	assert_eq!((a.value(0,2) - u).abs() < 1e-8, true);
	assert_eq!((a.value(0,3) - u).abs() < 1e-8, true);
	assert_eq!((a.value(1,0) - l).abs() < 1e-8, true);
	assert_eq!((a.value(1,1) - u).abs() < 1e-8, true);
	assert_eq!((a.value(2,1) - l).abs() < 1e-8, true);
	assert_eq!((a.value(2,2) - p).abs() < 1e-8, true);
	assert_eq!((a.value(3,3) - e).abs() < 1e-8, true);
	assert_eq!((a.value(3,4) - u).abs() < 1e-8, true);
	assert_eq!((a.value(4,0) - l).abs() < 1e-8, true);
	assert_eq!((a.value(4,1) - l).abs() < 1e-8, true);
	assert_eq!((a.value(4,4) - r).abs() < 1e-8, true);

	// Check (identically) zero matrix values
	assert_eq!(a.value(0,1), 0.0);
	assert_eq!(a.value(0,4), 0.0);
	assert_eq!(a.value(1,2), 0.0);
	assert_eq!(a.value(1,3), 0.0);
	assert_eq!(a.value(1,4), 0.0);
	assert_eq!(a.value(2,0), 0.0);
	assert_eq!(a.value(2,3), 0.0);
	assert_eq!(a.value(2,4), 0.0);
	assert_eq!(a.value(3,0), 0.0);
	assert_eq!(a.value(3,1), 0.0);
	assert_eq!(a.value(3,2), 0.0);
	assert_eq!(a.value(4,2), 0.0);
	assert_eq!(a.value(4,3), 0.0);
    }
    
    #[test]
    fn user_guide_example() {
	// Matrix dimensions
	let num_rows = 5usize;
	let num_columns = 5usize;

	// Matrix elements
	let s: f64 = 19.0;
	let u: f64 = 21.0;
	let p: f64 = 16.0;
	let e: f64 = 5.0;
	let r: f64 = 18.0;
	let l: f64 = 12.0;

	// Vector of doubles of length nnz
	let non_zero_values = vec![s, l, l, u, l, l, u, p, u, e, u, r];

	// Vector of ints of length nnz
	let row_indices = vec![0, 1, 4, 1, 2, 4, 0, 2, 0, 3, 3, 4];

	// Vector of ints of length num_columns + 1
	let column_offsets = vec![0, 3, 6, 8, 10, 12];

	// Make the left-hand side matrix
	let mut a = CompColMatrix::from_vectors(num_rows,
						non_zero_values,
						row_indices,
						column_offsets);

	// Make the RHS vector
	let nrhs = 1;
	let rhs = vec![1.0; num_rows];
	let b = DenseMatrix::from_vectors(num_rows, nrhs, rhs);

	let mut options = superlu_options_t::new();
	options.ColPerm = colperm_t::NATURAL;

	let mut perm_r = Vec::<i32>::with_capacity(num_rows);
	let mut perm_c = Vec::<i32>::with_capacity(num_columns);

	let mut stat = SuperLUStat_t::new();
	let SimpleSolution {
            mut x,
            lu: _,
	} = simple_driver(options, &mut a, &mut perm_c, &mut perm_r, b, &mut stat)
	    .expect("Failed to solve linear system");

	let x_vals = x.column_major_values();

	// True solution
	let x_true =  vec![-0.031249999999999976, 0.06547619047619045,
			   0.013392857142857161, 0.06249999999999996,
			   0.03273809523809525];
	assert_eq!(distance(x_vals, x_true) < 1e-8, true);
    }
}
