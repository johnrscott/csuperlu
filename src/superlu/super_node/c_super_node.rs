use libc;
use crate::superlu::utils::SuperMatrix;

#[link(name = "superlu")]
extern {
    fn dPrint_SuperNode_Matrix(what: *mut libc::c_char, A: *mut SuperMatrix);
    fn Destroy_SuperNode_Matrix(A: *mut SuperMatrix);    
}

pub fn c_dPrint_SuperNode_Matrix(what: *mut libc::c_char, A: *mut SuperMatrix) {
    unsafe {
	dPrint_SuperNode_Matrix(what, A);
    }
}

pub fn c_Destroy_SuperNode_Matrix(A: *mut SuperMatrix) {
    unsafe {
	Destroy_SuperNode_Matrix(A);
    }
}
