use crate::c::super_matrix::{c_SuperMatrix, Dtype_t, Mtype_t, Stype_t};
use libc;

#[link(name = "superlu")]
extern "C" {
    fn dCreate_Dense_Matrix(
        X: *mut c_SuperMatrix,
        m: libc::c_int,
        n: libc::c_int,
        x: *mut libc::c_double,
        ldx: libc::c_int,
        stype: Stype_t,
        dtype: Dtype_t,
        mtype: Mtype_t,
    );
    fn dPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn Destroy_SuperMatrix_Store(A: *mut c_SuperMatrix);
}

#[allow(non_snake_case)]
pub fn c_dCreate_Dense_Matrix(
    X: *mut c_SuperMatrix,
    m: libc::c_int,
    n: libc::c_int,
    x: *mut libc::c_double,
    ldx: libc::c_int,
    stype: Stype_t,
    dtype: Dtype_t,
    mtype: Mtype_t,
) {
    unsafe {
        dCreate_Dense_Matrix(X, m, n, x, ldx, stype, dtype, mtype);
    }
}

#[allow(non_snake_case)]
pub fn c_dPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix) {
    unsafe {
        dPrint_Dense_Matrix(what, A);
    }
}

// This will deallocate only the data structure allocated by
// the Create_*_Matrix routine (leaving the input vectors to
// be freed by the caller).
#[allow(non_snake_case)]
pub fn c_Destroy_SuperMatrix_Store(A: *mut c_SuperMatrix) {
    unsafe {
        Destroy_SuperMatrix_Store(A);
    }
}
