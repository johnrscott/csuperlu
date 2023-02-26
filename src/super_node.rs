//! Functions to create matrices in super-node format
//!

use crate::free::c_destroy_super_node_matrix;
use crate::value_type::ValueType;
use csuperlu_sys::{SCformat, SuperMatrix};

/// Super-node matrix
///
pub struct SuperNodeMatrix<P: ValueType<P>> {
    super_matrix: SuperMatrix,
    marker: std::marker::PhantomData<P>,
}

impl<P: ValueType<P>> SuperNodeMatrix<P> {
    /// Create a super-node matrix from a SuperMatrix structure
    ///
    pub fn from_super_matrix(super_matrix: SuperMatrix) -> Self {
        Self {
            super_matrix,
            marker: std::marker::PhantomData,
        }
    }
    pub fn values(&mut self) -> &[P] {
        unsafe {
            let c_scformat = &mut *(self.super_matrix.Store as *mut SCformat);
            std::slice::from_raw_parts(c_scformat.nzval as *mut P, c_scformat.nnz as usize)
        }
    }
    pub fn super_matrix<'a>(&'a self) -> &'a SuperMatrix {
        &self.super_matrix
    }
    pub fn print(&self, what: &str) {
        unsafe {
            P::c_print_super_node_matrix(what, &self.super_matrix);
        }
    }
}

impl<P: ValueType<P>> Drop for SuperNodeMatrix<P> {
    fn drop(&mut self) {
	unsafe {
            c_destroy_super_node_matrix(&mut self.super_matrix);
	}
    }
}
