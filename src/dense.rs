//! Functions to create dense matrices.
//!

use crate::c::free::c_destroy_dense_matrix;
use crate::c::super_matrix::CSuperMatrix;
use crate::c::value_type::ValueType;
use csuperlu_sys::DNformat;

pub struct DenseMatrix<P: ValueType<P>> {
    super_matrix: CSuperMatrix,
    marker: std::marker::PhantomData<P>,
}

impl<P: ValueType<P>> DenseMatrix<P> {
    /// Specify a dense matrix from an input vector.
    ///
    /// Use this function to make a dense SuperMatrix. The vector
    /// which stores the values in the matrix is passed in as a
    /// mutable reference, because this storage is overwritten by
    /// the solver when the dense matrix is used as the right-hand
    /// side matrix.
    ///
    pub fn from_vectors(num_rows: usize, num_columns: usize, mut x: Vec<P>) -> Self {
        let super_matrix =
            P::c_create_dense_matrix(num_rows, num_columns, &mut x)
                .expect("Failed to create dense matrix -- replace with error handling");
        std::mem::forget(x);
        Self {
            super_matrix,
            marker: std::marker::PhantomData,
        }
    }

    /// Create a DenseMatrix from a SuperMatrix
    ///
    /// # Safety
    ///
    /// This function is unsafe because the SuperMatrix
    /// argument must be a valid initialised dense matrix.
    /// If it is not, the DenseMatrix may not be valid, and
    /// may lead to undefined behaviour in subsequent parts
    /// of the program.
    ///
    pub unsafe fn from_super_matrix(super_matrix: CSuperMatrix) -> Self {
        Self {
            super_matrix,
            marker: std::marker::PhantomData,
        }
    }

    /// Obtain the underlying SuperMatrix from this DenseMatrix
    ///
    ///
    /// # Safety
    ///
    /// The function is unsafe because the resulting object that
    /// is returned will not have its resources freed when it goes
    /// out of scope. It is necessary to ensure that the SuperMatrix
    /// is wrapped back in a DenseMatrix, or its resources are freed
    /// manually (c_destroy_dense_matrix).
    ///
    pub unsafe fn into_super_matrix(self) -> CSuperMatrix {
        // TODO check this really carefully. The idea is to get
        // the super matrix all the way out of the function without
        // copying it at any time, or calling the drop for Dense
        // Matrix (which would deallocate it).
        let super_matrix = std::ptr::read(&self.super_matrix);
        std::mem::forget(self);
        super_matrix
    }

    /// Get the number of rows in the dense matrix
    pub fn num_rows(&self) -> usize {
        self.super_matrix.num_rows()
    }

    /// Get the number of columns in the dense matrix
    pub fn num_columns(&self) -> usize {
        self.super_matrix.num_columns()
    }

    pub fn column_major_values(&mut self) -> &[P] {
        unsafe {
            let c_dnformat = self.super_matrix.store::<DNformat>();
            let size = self.num_rows() * self.num_columns();
            std::slice::from_raw_parts(c_dnformat.nzval as *mut P, size as usize)
        }
    }
    pub fn super_matrix<'a>(&'a self) -> &'a CSuperMatrix {
        &self.super_matrix
    }
    
    pub fn print(&self, what: &str) {
        unsafe {
            P::c_print_dense_matrix(what, &self.super_matrix);
        }
    }
}

impl<P: ValueType<P>> Drop for DenseMatrix<P> {
    fn drop(&mut self) {
	unsafe {
            c_destroy_dense_matrix(&mut self.super_matrix);
	}
    }
}
