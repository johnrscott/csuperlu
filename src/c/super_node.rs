use libc;
use crate::c::utils::SuperMatrix;

#[link(name = "superlu")]
extern {
    fn dPrint_SuperNode_Matrix(what: *mut libc::c_char, A: *mut SuperMatrix);
    fn Destroy_SuperNode_Matrix(A: *mut SuperMatrix);    
}

#[allow(non_snake_case)]
pub fn c_dPrint_SuperNode_Matrix(what: *mut libc::c_char, A: *mut SuperMatrix) {
    unsafe {
	dPrint_SuperNode_Matrix(what, A);
    }
}

#[allow(non_snake_case)]
pub fn c_Destroy_SuperNode_Matrix(A: *mut SuperMatrix) {
    unsafe {
	Destroy_SuperNode_Matrix(A);
    }
}
