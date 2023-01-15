use crate::c::utils::c_SuperMatrix;
use libc;

#[link(name = "superlu")]
extern "C" {
    fn dPrint_SuperNode_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn Destroy_SuperNode_Matrix(A: *mut c_SuperMatrix);
}

#[allow(non_snake_case)]
pub fn c_dPrint_SuperNode_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix) {
    unsafe {
        dPrint_SuperNode_Matrix(what, A);
    }
}

#[allow(non_snake_case)]
pub fn c_Destroy_SuperNode_Matrix(A: *mut c_SuperMatrix) {
    unsafe {
        Destroy_SuperNode_Matrix(A);
    }
}
