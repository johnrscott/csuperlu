use crate::c::super_matrix::{c_SuperMatrix, Dtype_t, Mtype_t, Stype_t};
use libc;
use std::mem::MaybeUninit;

#[link(name = "superlu")]
extern "C" {
    fn sCreate_Dense_Matrix(
        X: *mut c_SuperMatrix,
        m: libc::c_int,
        n: libc::c_int,
        x: *mut libc::c_float,
        ldx: libc::c_int,
        stype: Stype_t,
        dtype: Dtype_t,
        mtype: Mtype_t,
    );
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
    fn cCreate_Dense_Matrix(
        X: *mut c_SuperMatrix,
        m: libc::c_int,
        n: libc::c_int,
        x: *mut libc::c_float,
        ldx: libc::c_int,
        stype: Stype_t,
        dtype: Dtype_t,
        mtype: Mtype_t,
    );
    fn zCreate_Dense_Matrix(
        X: *mut c_SuperMatrix,
        m: libc::c_int,
        n: libc::c_int,
        x: *mut libc::c_double,
        ldx: libc::c_int,
        stype: Stype_t,
        dtype: Dtype_t,
        mtype: Mtype_t,
    );
    fn sPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn dPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn cPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn zPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn Destroy_Dense_Matrix(A: *mut c_SuperMatrix);
}

/// Create dense matrix of particular precision
///
/// Trait for access to low level C functions from SuperLU, which
/// dispatches correctly based on the desired precision (and picks
/// the right value for the Dtype argument).
///
pub trait CCreateDenseMatrix<P> {
    fn c_create_dense_matrix(
        x: &mut MaybeUninit<c_SuperMatrix>,
        m: i32,
        n: i32,
        values: &mut Vec<P>,
        ldx: i32,
        mtype: Mtype_t,
    );
    fn c_print_dense_matrix(what: *mut libc::c_char, a: *mut c_SuperMatrix);
}



//// This will attempt to deallocate the three input vectors used to
//// create the comp_col matrix.
// #[allow(non_snake_case)]
// pub fn c_Destroy_Dense_Matrix(A: *mut c_SuperMatrix) {
//     unsafe {
//         Destroy_Dense_Matrix(A);
//     }
// }
