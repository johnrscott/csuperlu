//! Functions to create matrices in super-node format
//!

use crate::{c::free::c_destroy_super_node_matrix, c::super_matrix::CSuperMatrix};
use crate::c::value_type::ValueType;
use csuperlu_sys::SCformat;

/// Super-node matrix
///
#[derive(Debug)]
pub struct SuperNodeMatrix<P: ValueType<P>> {
    super_matrix: CSuperMatrix,
    marker: std::marker::PhantomData<P>,
}

impl<P: ValueType<P>> SuperNodeMatrix<P> {
    /// Create a super-node matrix from a SuperMatrix structure
    ///
    pub fn from_super_matrix(super_matrix: CSuperMatrix) -> Self {
        Self {
            super_matrix,
            marker: std::marker::PhantomData,
        }
    }
    pub fn values(&mut self) -> &[P] {
        unsafe {
            let c_scformat = self.super_matrix.store::<SCformat>();
            std::slice::from_raw_parts(c_scformat.nzval as *mut P, c_scformat.nnz as usize)
        }
    }
    pub fn super_matrix<'a>(&'a self) -> &'a CSuperMatrix {
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
