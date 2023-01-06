use libc;

use crate::superlu::utils::{Stype_t, Dtype_t, Mtype_t, SuperMatrix};


#[link(name = "superlu")]
extern {
    fn dCreate_Dense_Matrix(X: *mut SuperMatrix,
			    m: libc::c_int,
			    n: libc::c_int,
			    x: *mut libc::c_double,
			    ldx: libc::c_int,
			    stype: Stype_t,
			    dtype: Dtype_t,
			    mtype: Mtype_t);
    fn dPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut SuperMatrix);
    fn Destroy_SuperMatrix_Store(A: *mut SuperMatrix);
}

pub fn c_dCreate_Dense_Matrix(X: *mut SuperMatrix,
			      m: libc::c_int,
			      n: libc::c_int,
			      x: *mut libc::c_double,
			      ldx: libc::c_int,
			      stype: Stype_t,
			      dtype: Dtype_t,
			      mtype: Mtype_t) {
    unsafe {
	dCreate_Dense_Matrix(X, m, n, x, ldx, stype, dtype, mtype);
    }
}

pub fn c_dPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut SuperMatrix) {
    unsafe {
	dPrint_Dense_Matrix(what, A);	
    }
}

pub fn c_Destroy_SuperMatrix_Store(A: *mut SuperMatrix) {
    unsafe {
	Destroy_SuperMatrix_Store(A);	
    }
}
