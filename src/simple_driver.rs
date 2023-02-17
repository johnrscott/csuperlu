//! Solve sparse linear systems using the simple driver
//!

use crate::c::options::superlu_options_t;
use crate::c::stat::SuperLUStat_t;
use crate::c::super_matrix::c_SuperMatrix;
use crate::comp_col::CompColMatrix;
use crate::dense::DenseMatrix;

use crate::lu_decomp::LUDecomp;
use crate::super_matrix::SuperMatrix;
use crate::super_node::SuperNodeMatrix;

use crate::c::simple_driver::CSimpleDriver;

#[allow(non_snake_case)]
pub struct SimpleSolution<P: CSimpleDriver<P>> {
    pub x: DenseMatrix<P>,
    pub lu: LUDecomp<P>,
    pub info: i32,
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
pub fn simple_driver<P: CSimpleDriver<P>>(
    mut options: superlu_options_t,
    A: &mut CompColMatrix<P>,
    perm_c: &mut Vec<i32>,
    perm_r: &mut Vec<i32>,
    mut B: DenseMatrix<P>,
    stat: &mut SuperLUStat_t,
) -> SimpleSolution<P> {
    let mut info = 0;
    unsafe {
        let mut L = c_SuperMatrix::alloc();
        let mut U = c_SuperMatrix::alloc();

        P::c_simple_driver(
            &mut options,
            A.super_matrix(),
            perm_c,
            perm_r,
            &mut L,
            &mut U,
            B.super_matrix(),
            stat,
            &mut info,
        );
        let l = SuperNodeMatrix::from_super_matrix(L);
        let u = CompColMatrix::from_super_matrix(U);
        let lu = LUDecomp::from_matrices(l, u);
        SimpleSolution {
            x: B,
            lu,
	    info,
        }
    }
}
