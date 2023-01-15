//! Solve sparse linear systems using the simple driver
//!

use crate::c::options::superlu_options_t;
use crate::c::stat::SuperLUStat_t;
use crate::c::utils::c_SuperMatrix;
use std::mem::MaybeUninit;
use crate::super_matrix::SuperMatrix;

use crate::c::simple_driver::c_dgssv;

#[allow(non_snake_case)]
pub struct DgssvSolution {
    pub X: c_SuperMatrix,
    pub L: c_SuperMatrix,
    pub U: c_SuperMatrix,
    pub stat: SuperLUStat_t,
    pub info: i32,
}

/// Solve a sparse linear system AX = B.
///
/// The inputs to the function are the matrix A, the rhs matrix B,
/// and the permutation vectors. The outputs are the solution X
/// (which uses the same storage as B), the L and U matrices of
/// the LU decomposition, and
///
///
#[allow(non_snake_case)]
pub fn dgssv(
    mut options: superlu_options_t,
    A: &mut impl SuperMatrix,
    mut perm_c: Vec<i32>,
    mut perm_r: Vec<i32>,
    mut B: c_SuperMatrix,
    mut stat: SuperLUStat_t,
) -> DgssvSolution {
    let mut info = 0;
    unsafe {
        let mut L = MaybeUninit::<c_SuperMatrix>::uninit();
        let mut U = MaybeUninit::<c_SuperMatrix>::uninit();

        c_dgssv(
            &mut options,
            A,
            perm_c.as_mut_ptr(),
            perm_r.as_mut_ptr(),
            L.as_mut_ptr(),
            U.as_mut_ptr(),
            &mut B,
            &mut stat,
            &mut info,
        );
        DgssvSolution {
            X: B,
            L: L.assume_init(),
            U: U.assume_init(),
            stat,
            info,
        }
    }
}
