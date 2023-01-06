use libc;
use crate::superlu::utils::{Stype_t, Dtype_t, Mtype_t, SuperMatrix};

#[link(name = "superlu")]
extern {
    fn dCreate_CompCol_Matrix(A: *mut SuperMatrix,
			      m: libc::c_int,
			      n: libc::c_int,
			      nnz: libc::c_int,
			      nzval: *mut libc::c_double,
			      rowind: *mut libc::c_int,
			      colptr: *mut libc::c_int,
			      stype: Stype_t,
			      dtype: Dtype_t,
			      mtype: Mtype_t);
    fn Destroy_CompCol_Matrix(A: *mut SuperMatrix);
    fn dPrint_CompCol_Matrix(what: *mut libc::c_char, A: *mut SuperMatrix);
}

pub fn c_dCreate_CompCol_Matrix(A: *mut SuperMatrix,
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

pub fn c_Destroy_CompCol_Matrix(A: *mut SuperMatrix) {
    unsafe {
	Destroy_CompCol_Matrix(A);
    }
}

pub fn c_dPrint_CompCol_Matrix(what: *mut libc::c_char,
			       A: *mut SuperMatrix) {
    unsafe {
	dPrint_CompCol_Matrix(what, A);
    }
}
