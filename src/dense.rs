//! Functions to create dense matrices.
//!

use csuperlu_sys::dense::c_Destroy_Dense_Matrix;
use csuperlu_sys::super_matrix::c_DNformat;
use csuperlu_sys::super_matrix::{c_SuperMatrix, Mtype_t};
use crate::super_matrix::SuperMatrix;
use crate::value_type::ValueType;
use std::mem::MaybeUninit;

pub struct DenseMatrix<P: ValueType<P>> {
    c_super_matrix: c_SuperMatrix,
    marker: std::marker::PhantomData<P>,
}

impl<P: ValueType<P>> DenseMatrix<P> {
    /// Specify a dense matrix from an input vector.
    ///
    /// Use this function to make a dense c_SuperMatrix. The vector
    /// which stores the values in the matrix is passed in as a
    /// mutable reference, because this storage is overwritten by
    /// the solver when the dense matrix is used as the right-hand
    /// side matrix.
    ///
    pub fn from_vectors(num_rows: usize, num_columns: usize,
			mut x: Vec<P>) -> Self {
        let c_super_matrix = unsafe {
            P::c_create_dense_matrix(num_rows as i32,
				     num_columns as i32, &mut x, num_rows as i32,
				     Mtype_t::SLU_GE)
        };
        std::mem::forget(x);
        Self {
            c_super_matrix,
            marker: std::marker::PhantomData,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.c_super_matrix.nrow as usize
    }

    pub fn num_columns(&self) -> usize {
        self.c_super_matrix.ncol as usize
    }

    pub fn column_major_values(&mut self) -> &[P] {
        unsafe {
            let c_dnformat = &mut *(self.c_super_matrix.Store as *mut c_DNformat);
	    let size = self.c_super_matrix.nrow * self.c_super_matrix.ncol;
            std::slice::from_raw_parts(c_dnformat.nzval as *mut P, size as usize) 
        }
    }

}

impl<P: ValueType<P>> SuperMatrix for DenseMatrix<P> {
    fn super_matrix<'a>(&'a self) -> &'a c_SuperMatrix {
        &self.c_super_matrix
    }
    fn print(&self, what: &str) {
        let c_str = std::ffi::CString::new(what).unwrap();
	unsafe {
            P::c_print_dense_matrix(c_str.as_ptr(), &self.c_super_matrix);
	}
    }
}

impl<P: ValueType<P>> Drop for DenseMatrix<P> {
    fn drop(&mut self) {
        // Note that the input vectors are not freed by this line
        c_Destroy_Dense_Matrix(&mut self.c_super_matrix);
    }
}
