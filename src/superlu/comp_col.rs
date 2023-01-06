use libc;
use std::mem::MaybeUninit;
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

pub struct CompColMatrix {
    pub super_matrix: SuperMatrix,
}

impl CompColMatrix {
    pub fn new(m: i32, n: i32, nnz: i32, mut a: Vec<f64>,
	   mut asub: Vec<i32>, mut xa: Vec<i32>) -> Self {
	let super_matrix = unsafe {
	    let mut super_matrix = MaybeUninit::<SuperMatrix>::uninit();
	    dCreate_CompCol_Matrix(super_matrix.as_mut_ptr(),
				   m, n, nnz, a.as_mut_ptr(),
				   asub.as_mut_ptr(), xa.as_mut_ptr(),
				   Stype_t::SLU_NC, Dtype_t::SLU_D,
				   Mtype_t::SLU_GE);
	    std::mem::forget(a);
	    std::mem::forget(asub);
	    std::mem::forget(xa);
	    super_matrix.assume_init()
	};
	Self {
	    super_matrix,
	}
    }
    pub fn from_super_matrix(super_matrix: SuperMatrix) -> Self {
    	Self {
    	    super_matrix
    	}
    }
    pub fn print(&mut self, label: &str) {
	let c = std::ffi::CString::new(label).unwrap();
	unsafe {
	    dPrint_CompCol_Matrix(c.as_ptr() as *mut libc::c_char,
				  &mut self.super_matrix);
	}
    }
}

impl Drop for CompColMatrix {
    fn drop(&mut self) {
	unsafe {
	    Destroy_CompCol_Matrix(&mut self.super_matrix);
	}
    }
}
