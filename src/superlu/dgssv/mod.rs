mod c_dgssv;

use crate::superlu::utils::{SuperLUStat_t, superlu_options_t, SuperMatrix};
use c_dgssv::c_dgssv;

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
