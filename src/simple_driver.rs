//! Solve sparse linear systems using the simple driver
//!

use csuperlu_sys::options::superlu_options_t;
use crate::value_type::ValueType;
use csuperlu_sys::stat::SuperLUStat_t;
use csuperlu_sys::super_matrix::c_SuperMatrix;
use crate::comp_col::CompColMatrix;
use crate::dense::DenseMatrix;

use crate::lu_decomp::LUDecomp;
use crate::super_matrix::SuperMatrix;
use crate::super_node::SuperNodeMatrix;

use std::{error::Error, fmt};

#[derive(Debug)]
pub struct SolverError {
    /// info != 0 indicates solver error. See e.g. dgssv documentation
    /// for the meaning of info. 
    info: i32,
}

impl Error for SolverError {}

impl fmt::Display for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to solve the linear system; info = {}", self.info)
    }
}

#[allow(non_snake_case)]
pub struct SimpleSolution<P: ValueType<P>> {
    pub x: DenseMatrix<P>,
    pub lu: LUDecomp<P>,
}

/// Solve a sparse linear system AX = B.
///
/// The inputs to the function are the matrix A, the rhs matrix B,
/// and the permutation vectors. The outputs are the solution X
/// (which uses the same storage as B), the L and U matrices of
/// the LU decomposition.
///
/// The matrix A must be in column-major compressed-column format.
/// (see Section 2.3 in the SuperLU manual.) If a row-major matrix
/// is passed for A (CompRowMatrix), then the routine will decompose
/// A^T. Make sure to convert the CompRowMatrix to a CompColumnMatrix
/// if you want to solve A.
///
#[allow(non_snake_case)]
pub fn simple_driver<P: ValueType<P>>(
    mut options: superlu_options_t,
    A: &CompColMatrix<P>,
    perm_c: &mut Vec<i32>,
    perm_r: &mut Vec<i32>,
    B: DenseMatrix<P>,
    stat: &mut SuperLUStat_t,
) -> Result<SimpleSolution<P>, SolverError> {
    let mut info = 0;
    unsafe {
        let mut L = c_SuperMatrix::alloc();
        let mut U = c_SuperMatrix::alloc();

	let mut b_super_matrix = B.into_super_matrix();
	
        P::c_simple_driver(
            &mut options,
            A.super_matrix(),
            perm_c,
            perm_r,
            &mut L,
            &mut U,
            &mut b_super_matrix,
            stat,
            &mut info,
        );
        let l = SuperNodeMatrix::from_super_matrix(L);
        let u = CompColMatrix::from_super_matrix(U);
        let lu = LUDecomp::from_matrices(l, u);
	let x = DenseMatrix::<P>::from_super_matrix(b_super_matrix);
	
	if info != 0 {
	    Err(SolverError {
		info
	    })
	} else {
	    Ok(SimpleSolution {
		x,
		lu,
            })
	}
    }
}
