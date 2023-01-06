use libc;
use std::mem::MaybeUninit;
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

pub struct DenseMatrix {
    pub super_matrix: SuperMatrix,
}

impl DenseMatrix {
    pub fn new(m: i32, n: i32, x: &mut Vec<f64>) -> Self {
	let super_matrix = unsafe {
	    let mut super_matrix = MaybeUninit::<SuperMatrix>::uninit();
	    dCreate_Dense_Matrix(super_matrix.as_mut_ptr(),
				 m, n, x.as_mut_ptr(),
				 m, Stype_t::SLU_DN, Dtype_t::SLU_D,
				 Mtype_t::SLU_GE);
	    super_matrix.assume_init()
	};
	// No need to forget x because Destroy_SuperMatrix_Store
	// does not drop x (it is on us to free that one)
	Self {
	    super_matrix,
	}
    }
    pub fn print(&mut self, label: &str) {
	let c = std::ffi::CString::new(label).unwrap();
	unsafe {
	    dPrint_Dense_Matrix(c.as_ptr() as *mut libc::c_char,
				&mut self.super_matrix);
	}
    }
}

impl Drop for DenseMatrix {
    fn drop(&mut self) {
	unsafe {
	    Destroy_SuperMatrix_Store(&mut self.super_matrix);
	}
    }
}
