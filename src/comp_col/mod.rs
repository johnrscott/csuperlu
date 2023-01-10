use std::mem::MaybeUninit;
use crate::c::utils::{
    SuperMatrix,
    Stype_t,
    Mtype_t,
    Dtype_t,
};
use crate::c::comp_col::c_dCreate_CompCol_Matrix;

#[allow(non_snake_case)]
pub fn dCreate_CompCol_Matrix(m: libc::c_int,
			      n: libc::c_int,
			      nnz: libc::c_int,
			      mut nzval: Vec<f64>,
			      mut rowind: Vec<i32>,
			      mut colptr: Vec<i32>,
			      stype: Stype_t,
			      dtype: Dtype_t,
			      mtype: Mtype_t) -> SuperMatrix {
    unsafe {
	let mut A = MaybeUninit::<SuperMatrix>::uninit();
	c_dCreate_CompCol_Matrix(A.as_mut_ptr(), m, n, nnz,
				 nzval.as_mut_ptr(), rowind.as_mut_ptr(),
				 colptr.as_mut_ptr(), stype, dtype, mtype);

	// When the CompCol matrix is created, the vectors a, asub and xa are
	// considered to be owned by the matrix. This means that the matrix free
	// function also frees these vectors. In order to avoid rust also freeing
	// them, forget them here.
	std::mem::forget(nzval);
	std::mem::forget(rowind);
	std::mem::forget(colptr);
	A.assume_init()
    }
}
