mod c_comp_col;

use crate::superlu::utils::{SuperMatrix, Stype_t, Dtype_t, Mtype_t};
use crate::superlu::comp_col::c_comp_col::{
    c_dCreate_CompCol_Matrix,
    c_Destroy_CompCol_Matrix,
    c_dPrint_CompCol_Matrix,    
};

#[allow(non_snake_case)]
pub fn dCreate_CompCol_Matrix(A: *mut SuperMatrix,
			      m: libc::c_int,
			      n: libc::c_int,
			      nnz: libc::c_int,
			      nzval: *mut libc::c_double,
			      rowind: *mut libc::c_int,
			      colptr: *mut libc::c_int,
			      stype: Stype_t,
			      dtype: Dtype_t,
			      mtype: Mtype_t) {
    c_dCreate_CompCol_Matrix(A, m, n, nnz, nzval, rowind, colptr,
			     stype, dtype, mtype);
}

#[allow(non_snake_case)]
pub fn Destroy_CompCol_Matrix(A: *mut SuperMatrix) {
    c_Destroy_CompCol_Matrix(A);
}

#[allow(non_snake_case)]
pub fn dPrint_CompCol_Matrix(what: *mut libc::c_char,
			     A: *mut SuperMatrix) {
    c_dPrint_CompCol_Matrix(what, A);
}
