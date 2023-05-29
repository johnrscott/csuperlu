//! SuperLU-format dense matrix
//!
//! Matrices are stored in column-major format. Dense matrices
//! are used for the right-hand side matrix and the solution matrix.

use csuperlu_sys::SuperMatrix;
use std::mem;

use self::create_dense_mat::CreateDenseMat;

use super::{error::Error, free::destroy_super_matrix_store, super_matrix::CSuperMatrix};

pub mod create_dense_mat;

/// The rust vectors comprising the matrix
pub struct DenseRaw<P: CreateDenseMat> {
    pub num_rows: usize,
    pub num_cols: usize,
    pub col_maj_vals: Vec<P>,
}

/// Dense matrix
#[derive(Debug)]
pub struct DenseMat<P: CreateDenseMat> {
    col_maj_vals: Vec<P>,
    super_matrix: CSuperMatrix,
}

impl<P: CreateDenseMat> DenseMat<P> {
    /// Make a dense matrix from raw components
    ///
    /// # Errors
    ///
    /// An error is returned if the num_rows and num_cols are
    /// not compatible with the col_maj_vals.
    ///
    /// # Safety
    ///
    /// Unlike the compressed-column matrix, this function is
    /// safe. This is because it is not possible to provide an
    /// invalid DenseRaw that is not caught by the error
    /// checking.
    pub fn from_raw(raw: DenseRaw<P>) -> Result<Self, Error> {
        let DenseRaw {
            num_rows,
            num_cols,
            col_maj_vals,
        } = raw;

        let super_matrix = P::create_dense_matrix(num_rows, num_cols, &col_maj_vals)?;

        Ok(Self {
            col_maj_vals,
            super_matrix,
        })
    }

    /// Get the number of rows in the matrix
    pub fn num_rows(&self) -> usize {
        self.super_matrix.num_rows()
    }

    /// Get the number of columns in the matrix
    pub fn num_cols(&self) -> usize {
        self.super_matrix.num_cols()
    }

    pub fn to_raw(mut self) -> DenseRaw<P> {
        let col_maj_vals = unsafe {
            Vec::from_raw_parts(
                self.col_maj_vals.as_mut_ptr(),
                self.col_maj_vals.len(),
                self.col_maj_vals.capacity(),
            )
        };

        // Also get the number of rows before dropping
        let num_rows = self.num_rows();
        let num_cols = self.num_cols();

        // Call the destructor (to avoid the need for drop)
        unsafe {
            destroy_super_matrix_store(&mut self.super_matrix);
        };

        // Treat self as deallocated already
        mem::forget(self);

        DenseRaw {
            num_rows,
            num_cols,
            col_maj_vals,
        }
    }

    pub fn super_matrix(&self) -> &SuperMatrix {
        &self.super_matrix.super_matrix()
    }
}

impl<P: CreateDenseMat> Drop for DenseMat<P> {
    fn drop(&mut self) {
        unsafe {
            destroy_super_matrix_store(&mut self.super_matrix);
        }
    }
}

/// This test checks that dropping a matrix as it
/// leaves scope does not cause memory leaks
#[test]
fn test_drop_leaks() {
    // Make a simple dense matrix
    let num_rows = 3;
    let num_cols = 3;
    // Note the matrix isn't what it looks like!
    let col_maj_vals = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

    let raw = DenseRaw {
        num_rows,
        num_cols,
        col_maj_vals,
    };

    // Create the matrix wrapper
    let _a = DenseMat::from_raw(raw).expect("Failed to create matrix");
}

#[test]
fn test_dense_matrix() {
    // Make a simple dense matrix
    let num_rows = 3;
    let num_cols = 3;
    // Note the matrix isn't what it looks like!
    let col_maj_vals = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

    let raw = DenseRaw {
        num_rows,
        num_cols,
        col_maj_vals,
    };

    // Create the matrix wrapper
    let a = DenseMat::from_raw(raw).expect("Failed to create matrix");

    // Check the size
    assert_eq!(a.num_cols(), 3);
    assert_eq!(a.num_rows(), 3);

    // Check the values

    // Get the vectors back out
    let DenseRaw {
        num_rows,
        num_cols,
        col_maj_vals,
    } = a.to_raw();

    // Check the vectors are all correct
    assert_eq!(num_rows, 3);
    assert_eq!(num_cols, 3);
    assert_eq!(
        col_maj_vals,
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
    );
}
