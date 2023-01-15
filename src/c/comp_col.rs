use libc;
use crate::c::utils::{Stype_t, Dtype_t, Mtype_t, c_SuperMatrix};

#[link(name = "superlu")]
extern {
    fn dCreate_CompCol_Matrix(A: *mut c_SuperMatrix,
			      m: libc::c_int,
			      n: libc::c_int,
			      nnz: libc::c_int,
			      nzval: *mut libc::c_double,
			      rowind: *mut libc::c_int,
			      colptr: *mut libc::c_int,
			      stype: Stype_t,
			      dtype: Dtype_t,
			      mtype: Mtype_t);
    fn Destroy_CompCol_Matrix(A: *mut c_SuperMatrix);
    fn dPrint_CompCol_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
}

#[allow(non_snake_case)]
pub fn c_dCreate_CompCol_Matrix(A: *mut c_SuperMatrix,
			    m: libc::c_int,
			    n: libc::c_int,
			    nnz: libc::c_int,
			    nzval: *mut libc::c_double,
			    rowind: *mut libc::c_int,
			    colptr: *mut libc::c_int,
			    stype: Stype_t,
			    dtype: Dtype_t,
			    mtype: Mtype_t) {
    unsafe {
	dCreate_CompCol_Matrix(A, m, n, nnz, nzval, rowind, colptr,
			       stype, dtype, mtype);
    }
}

#[allow(non_snake_case)]
pub fn c_Destroy_CompCol_Matrix(A: *mut c_SuperMatrix) {
    unsafe {
	Destroy_CompCol_Matrix(A);
    }
}

#[allow(non_snake_case)]
pub fn c_dPrint_CompCol_Matrix(what: *mut libc::c_char,
			       A: *mut c_SuperMatrix) {
    unsafe {
	dPrint_CompCol_Matrix(what, A);
    }
}
