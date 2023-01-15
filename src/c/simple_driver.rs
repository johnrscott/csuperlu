use crate::c::options::superlu_options_t;
use crate::c::stat::SuperLUStat_t;
use crate::c::utils::c_SuperMatrix;
use libc;

#[link(name = "superlu")]
extern "C" {
    fn dgssv(
        options: *mut superlu_options_t,
        A: *mut c_SuperMatrix,
        perm_c: *mut libc::c_int,
        perm_r: *mut libc::c_int,
        L: *mut c_SuperMatrix,
        U: *mut c_SuperMatrix,
        B: *mut c_SuperMatrix,
        stat: *mut SuperLUStat_t,
        info: *mut libc::c_int,
    );
}

#[allow(non_snake_case)]
pub fn c_dgssv(
    options: *mut superlu_options_t,
    A: *mut c_SuperMatrix,
    perm_c: *mut libc::c_int,
    perm_r: *mut libc::c_int,
    L: *mut c_SuperMatrix,
    U: *mut c_SuperMatrix,
    B: *mut c_SuperMatrix,
    stat: *mut SuperLUStat_t,
    info: *mut libc::c_int,
) {
    unsafe {
        dgssv(options, A, perm_c, perm_r, L, U, B, stat, info);
    }
}
