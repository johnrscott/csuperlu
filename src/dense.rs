//! Functions to create dense matrices.
//!

use crate::free::c_destroy_dense_matrix;
use crate::value_type::ValueType;
use csuperlu_sys::DNformat;
use csuperlu_sys::Mtype_t_SLU_GE;
use csuperlu_sys::SuperMatrix;

pub struct DenseMatrix<P: ValueType<P>> {
    super_matrix: SuperMatrix,
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
            P::c_create_dense_matrix(num_rows, num_columns, &mut x, Mtype_t_SLU_GE)
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
    pub unsafe fn from_super_matrix(super_matrix: SuperMatrix) -> Self {
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
    pub unsafe fn into_super_matrix(self) -> SuperMatrix {
        // TODO check this really carefully. The idea is to get
        // the super matrix all the way out of the function without
        // copying it at any time, or calling the drop for Dense
        // Matrix (which would deallocate it).
        let super_matrix = std::ptr::read(&self.super_matrix);
        std::mem::forget(self);
        super_matrix
    }

    pub fn num_rows(&self) -> usize {
        self.super_matrix.nrow as usize
    }

    pub fn num_columns(&self) -> usize {
        self.super_matrix.ncol as usize
    }

    pub fn column_major_values(&mut self) -> &[P] {
        unsafe {
            let c_dnformat = &mut *(self.super_matrix.Store as *mut DNformat);
            let size = self.super_matrix.nrow * self.super_matrix.ncol;
            std::slice::from_raw_parts(c_dnformat.nzval as *mut P, size as usize)
        }
    }
    pub fn super_matrix<'a>(&'a self) -> &'a SuperMatrix {
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
