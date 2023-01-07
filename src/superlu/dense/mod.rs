mod c_dense;

use crate::superlu::dense::c_dense::{
    c_dCreate_Dense_Matrix,
    c_dPrint_Dense_Matrix,
    c_Destroy_SuperMatrix_Store,
};
use crate::superlu::utils::{Stype_t, Dtype_t, Mtype_t, SuperMatrix};

#[allow(non_snake_case)]
pub fn dCreate_Dense_Matrix(X: *mut SuperMatrix,
			      m: libc::c_int,
			      n: libc::c_int,
			      x: *mut libc::c_double,
			      ldx: libc::c_int,
			      stype: Stype_t,
			      dtype: Dtype_t,
			      mtype: Mtype_t) {
    c_dCreate_Dense_Matrix(X, m, n, x, ldx, stype, dtype, mtype);
}

#[allow(non_snake_case)]
pub fn dPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut SuperMatrix) {
    c_dPrint_Dense_Matrix(what, A);	
}

#[allow(non_snake_case)]
pub fn Destroy_SuperMatrix_Store(A: *mut SuperMatrix) {
    c_Destroy_SuperMatrix_Store(A);	
}



