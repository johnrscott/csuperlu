//! Functions to create matrices in super-node format
//!

use crate::c::super_matrix::{c_SCformat, c_SuperMatrix};
use crate::c::super_node::{c_Destroy_SuperNode_Matrix, CSuperNodeMatrix};
use crate::super_matrix::SuperMatrix;

/// Super-node matrix
///
pub struct SuperNodeMatrix<P: CSuperNodeMatrix<P>> {
    c_super_matrix: c_SuperMatrix,
    marker: std::marker::PhantomData<P>,
}

impl<P: CSuperNodeMatrix<P>> SuperNodeMatrix<P> {
    /// Create a super-node matrix from a c_SuperMatrix structure
    ///
    pub fn from_super_matrix(c_super_matrix: c_SuperMatrix) -> Self {
        Self {
            c_super_matrix,
            marker: std::marker::PhantomData,
        }
    }
    pub fn values(&mut self) -> &[P] {
        unsafe {
            let c_scformat = &mut *(self.c_super_matrix.Store as *mut c_SCformat);
	    std::slice::from_raw_parts(c_scformat.nzval as *mut P, c_scformat.nnz as usize)
        }
    }
}

impl<P: CSuperNodeMatrix<P>> SuperMatrix for SuperNodeMatrix<P> {
    fn super_matrix<'a>(&'a mut self) -> &'a mut c_SuperMatrix {
        &mut self.c_super_matrix
    }
    fn print(&mut self, what: &str) {
        let c_str = std::ffi::CString::new(what).unwrap();
        P::c_print_super_node_matrix(c_str.as_ptr() as *mut libc::c_char, self.super_matrix());
    }
}

impl<P: CSuperNodeMatrix<P>> Drop for SuperNodeMatrix<P> {
    fn drop(&mut self) {
        // Note that the input vectors are also freed by this line
        c_Destroy_SuperNode_Matrix(&mut self.c_super_matrix);
    }
}
