mod c_dgssv;

use crate::superlu::utils::{SuperLUStat_t, superlu_options_t, SuperMatrix};
use c_dgssv::c_dgssv;

/// Solve the sparse system of equations AX = B
///
/// This is the simple driver routine that solves
/// AX = B by overwriting the input B with the solution
/// matrix X. As part of the solution, the function
/// performs an LU decomposition.
///
/// The arguments are passed directly to the underlying
/// C library.
#[allow(non_snake_case)]
pub fn dgssv(options: *mut superlu_options_t,
	     A: *mut SuperMatrix,
	     perm_c: *mut libc::c_int,
	     perm_r: *mut libc::c_int,
	     L: *mut SuperMatrix,
	     U: *mut SuperMatrix,
	     B: *mut SuperMatrix,
	     stat: *mut SuperLUStat_t,
	     info: *mut libc::c_int) {
    c_dgssv(options, A, perm_c, perm_r, L, U, B, stat, info);
}
