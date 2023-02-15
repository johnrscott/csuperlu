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

//#![warn(missing_docs)]
pub mod c;
pub mod comp_col;
pub mod dense;
pub mod simple_driver;
pub mod super_matrix;
pub mod super_node;
pub mod lu_decomp;

#[cfg(test)]
mod tests {

    fn distance(v1: &[f64], v2: Vec<f64>) -> f64 {
	let mut val = 0.0;
	for n in 0..v2.len() {
	    val += (v1[n] - v2[n]) * (v1[n] - v2[n]);
	}
	val
    }
    
    use crate::c::options::{colperm_t, superlu_options_t};
    use crate::c::stat::{c_StatPrint, SuperLUStat_t};
    use crate::c::super_matrix::Mtype_t;
    use crate::comp_col::CompColMatrix;
    use crate::dense::DenseMatrix;
    use crate::simple_driver::{simple_driver, Solution};
    use crate::super_matrix::SuperMatrix;

    #[test]
    fn comp_col_matrix_values() {
	// Matrix dimensions
	let m: i32 = 5;
	let n: i32 = 5;

	// Number of non-zeros
	let nnz: i32 = 12;

	// Matrix elements
	let s: f64 = 19.0;
	let u: f64 = 21.0;
	let p: f64 = 16.0;
	let e: f64 = 5.0;
	let r: f64 = 18.0;
	let l: f64 = 12.0;

	// Vector of doubles of length nnz
	let a = vec![s, l, l, u, l, l, u, p, u, e, u, r];

	// Vector of ints of length nnz
	let asub = vec![0, 1, 4, 1, 2, 4, 0, 2, 0, 3, 3, 4];

	// Vector of ints of length n+1
	let xa = vec![0, 3, 6, 8, 10, 12];

	// Make the left-hand side matrix
	let mut a = CompColMatrix::from_vectors(m, n, nnz, a, asub, xa, Mtype_t::SLU_GE);

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
	
    }
    
    #[test]
    fn user_guide_example() {
	// Matrix dimensions
	let m: i32 = 5;
	let n: i32 = 5;

	// Number of non-zeros
	let nnz: i32 = 12;

	// Matrix elements
	let s: f64 = 19.0;
	let u: f64 = 21.0;
	let p: f64 = 16.0;
	let e: f64 = 5.0;
	let r: f64 = 18.0;
	let l: f64 = 12.0;

	// Vector of doubles of length nnz
	let a = vec![s, l, l, u, l, l, u, p, u, e, u, r];

	// Vector of ints of length nnz
	let asub = vec![0, 1, 4, 1, 2, 4, 0, 2, 0, 3, 3, 4];

	// Vector of ints of length n+1
	let xa = vec![0, 3, 6, 8, 10, 12];

	// Make the left-hand side matrix
	let mut a = CompColMatrix::from_vectors(m, n, nnz, a, asub, xa, Mtype_t::SLU_GE);

	// Make the RHS vector
	let nrhs = 1;
	let rhs = vec![1.0; m as usize];
	let b = DenseMatrix::from_vectors(m, nrhs, rhs, Mtype_t::SLU_GE);

	let mut options = superlu_options_t::new();
	options.ColPerm = colperm_t::NATURAL;

	let mut perm_r = Vec::<i32>::with_capacity(m as usize);
	let mut perm_c = Vec::<i32>::with_capacity(n as usize);

	let stat = SuperLUStat_t::new();
	let Solution {
            mut X,
            lu: _,
            stat: _,
            info: _,
	} = simple_driver(options, &mut a, &mut perm_c, &mut perm_r, b, stat);

	let x_vals = X.values();

	// True solution
	let x_true =  vec![-0.031249999999999976, 0.06547619047619045,
			   0.013392857142857161, 0.06249999999999996,
			   0.03273809523809525];
	assert_eq!(distance(x_vals, x_true) < 1e-8, true);
    }
}
