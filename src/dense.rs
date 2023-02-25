//! Functions to create dense matrices.
//!

use crate::free::c_destroy_dense_matrix;
use crate::super_matrix::SuperMatrix;
use crate::value_type::ValueType;
use csuperlu_sys::super_matrix::c_DNformat;
use csuperlu_sys::super_matrix::{c_SuperMatrix, Mtype_t};

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
    pub fn from_vectors(num_rows: usize, num_columns: usize, mut x: Vec<P>) -> Self {
        let c_super_matrix =
            P::c_create_dense_matrix(num_rows, num_columns, &mut x, Mtype_t::SLU_GE)
                .expect("Failed to create dense matrix -- replace with error handling");
        std::mem::forget(x);
        Self {
            c_super_matrix,
            marker: std::marker::PhantomData,
        }
    }

    /// Create a DenseMatrix from a c_SuperMatrix
    ///
    /// # Safety
    ///
    /// This function is unsafe because the c_SuperMatrix
    /// argument must be a valid initialised dense matrix.
    /// If it is not, the DenseMatrix may not be valid, and
    /// may lead to undefined behaviour in subsequent parts
    /// of the program.
    ///
    pub unsafe fn from_super_matrix(c_super_matrix: c_SuperMatrix) -> Self {
        Self {
            c_super_matrix,
            marker: std::marker::PhantomData,
        }
    }

    /// Obtain the underlying c_SuperMatrix from this DenseMatrix
    ///
    ///
    /// # Safety
    ///
    /// The function is unsafe because the resulting object that
    /// is returned will not have its resources freed when it goes
    /// out of scope. It is necessary to ensure that the c_SuperMatrix
    /// is wrapped back in a DenseMatrix, or its resources are freed
    /// manually (c_destroy_dense_matrix).
    ///
    pub unsafe fn into_super_matrix(self) -> c_SuperMatrix {
        // TODO check this really carefully. The idea is to get
        // the super matrix all the way out of the function without
        // copying it at any time, or calling the drop for Dense
        // Matrix (which would deallocate it).
        let c_super_matrix = std::ptr::read(&self.c_super_matrix);
        std::mem::forget(self);
        c_super_matrix
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
        unsafe {
            P::c_print_dense_matrix(what, &self.c_super_matrix);
        }
    }
}

impl<P: ValueType<P>> Drop for DenseMatrix<P> {
    fn drop(&mut self) {
        unsafe {
            c_destroy_dense_matrix(&mut self.c_super_matrix);
        }
    }
}
