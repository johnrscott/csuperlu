//! Functions to create matrices in super-node format
//!

use crate::c::super_matrix::{c_SuperMatrix, Mtype_t, c_NCformat};
use crate::c::comp_col::c_Destroy_CompCol_Matrix;
use crate::super_matrix::SuperMatrix;
use std::mem::MaybeUninit;

/// Super-node matrix
///
pub struct SuperNodeMatrix<P> {
    c_super_matrix: c_SuperMatrix,
    marker: std::marker::PhantomData<P>,
}

impl<P: CCreateCompColMatrix<P>> SuperNodeMatrix<P> {
    /// Create a super-node matrix from a c_SuperMatrix structure
    ///
    pub fn from_super_matrix(c_super_matrix: c_SuperMatrix) -> Self {
	Self {
            c_super_matrix,
	    marker: std::marker::PhantomData,
        }
    }
    pub fn values(&mut self) -> &mut Vec<P> {
	unsafe {
	    let c_scformat = &mut *(self.c_super_matrix.Store as *mut c_SCformat);
	    &mut *(c_scformat.nzval as *mut Vec<P>)
	}
    }
}

impl<P> SuperMatrix for SuperNodeMatrix<P> {
    fn super_matrix<'a>(&'a mut self) -> &'a mut c_SuperMatrix {
        &mut self.c_super_matrix
    }
    fn print(&mut self, what: &str) {
	let c_str = std::ffi::CString::new(what).unwrap();
	P::c_print_super_node_matrix(c_str.as_ptr() as *mut libc::c_char,
				   self.super_matrix());
    }
}

impl<P: CCreateCompColMatrix<P>> Drop for CompColMatrix<P> {
    fn drop(&mut self) {
	// Note that the input vectors are also freed by this line
        c_Destroy_CompCol_Matrix(&mut self.c_super_matrix);
    }
}

