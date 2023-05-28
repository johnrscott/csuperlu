//! SuperLU-format compressed-column matrix
//!
//! Matrices are stored in column-major compressed-column format.

use std::mem;

use csuperlu_sys::SuperMatrix;

use self::create_comp_col_mat::CreateCompColMat;

use super::{error::Error, free::destroy_super_matrix_store, super_matrix::CSuperMatrix};

pub mod create_comp_col_mat;

/// The rust vectors comprising the matrix
pub struct CompColRaw<P: CreateCompColMat> {
    pub num_rows: usize,
    pub non_zero_vals: Vec<P>,
    pub row_indices: Vec<i32>,
    pub col_offsets: Vec<i32>,
}

/// A SuperLU compressed-column matrix in column-major format
///
/// This is ultimately a wrapper around a SuperMatrix struct
/// (in the C library), containing a a SCformat store referring
/// to vectors allocated in rust. When this struct is dropped,
/// rust will deallocate the vectors (non-zero values, row indices
/// and column offsets), and the SuperLU library will free the
/// SuperMatrix struct. You should not need to worry about memory
/// when using this struct, apart from ensuring that the safety
/// contract of the from_raw function is fulfilled.
pub struct CompColMat<P: CreateCompColMat> {
    non_zero_vals: Vec<P>,
    row_indices: Vec<i32>,
    col_offsets: Vec<i32>,
    super_matrix: CSuperMatrix,
}

impl<P: CreateCompColMat> CompColMat<P> {
    /// Create a new compressed column matrix from raw vectors
    ///
    /// # Errors
    ///
    /// If the length of col_offsets is not equal to num_cols \+ 1
    /// then an error variant is returned. If the lengths of
    /// non_zero_vals and row_indices are not the same, an error is
    /// returned. The last element of col_offsets must be equal to the
    /// length of non_zero_vals, else error is returned. Other ways to
    /// pass invalid arguments are described in the safety section below.
    ///
    /// # Safety
    ///
    /// No checks are performed to ensure that the input vectors
    /// format a valid compressed column matrix, apart from basic
    /// checks on the lenths of the vectors. You must ensure the
    /// following conditions are met:
    ///
    /// * All values in row indices must be within the range for
    /// the matrix height (0 <= row < num_rows).
    /// * All row indices must be in ascending order (TODO check
    /// if this is a requirement))
    /// * All values in column offsets must be within range for
    /// the matrix width (0 <= col < len())
    ///
    /// If the input vectors are invalid, undefined behaviour may
    /// result in the SuperLU routines.
    ///
    pub unsafe fn from_raw(raw: CompColRaw<P>) -> Result<Self, Error> {
        let CompColRaw {
            num_rows,
            non_zero_vals,
            row_indices,
            col_offsets,
        } = raw;

        let super_matrix =
            P::create_comp_col_matrix(num_rows, &non_zero_vals, &row_indices, &col_offsets)?;
        Ok(Self {
            non_zero_vals,
            row_indices,
            col_offsets,
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

    /// Get the underlying vectors from the object.
    ///
    /// No copies are made; you get the vectors that were
    /// inside the  CompColMat object by move. The arguments
    /// in the returned tuple are the same as the from_raw
    /// function: (num_rows, non_zero_vals, row_indices,
    /// col_offsets)
    ///
    ///
    pub fn to_raw(mut self) -> CompColRaw<P> {
        // You can't just move the vectors out of the matrix because
        // of the drop trait. Instead, get raw pointers to the vectors
        // and then reconstruct the Vecs to "trick" the compiler, then
        // call the free manually and forget the matrix struct.

        // These lines are fine because the arguments to from_raw_parts
        // came from a Vec
        let non_zero_vals = unsafe {
            Vec::from_raw_parts(
                self.non_zero_vals.as_mut_ptr(),
                self.non_zero_vals.len(),
                self.non_zero_vals.capacity(),
            )
        };
        let row_indices = unsafe {
            Vec::from_raw_parts(
                self.row_indices.as_mut_ptr(),
                self.row_indices.len(),
                self.row_indices.capacity(),
            )
        };
        let col_offsets = unsafe {
            Vec::from_raw_parts(
                self.col_offsets.as_mut_ptr(),
                self.col_offsets.len(),
                self.col_offsets.capacity(),
            )
        };

        // Also get the number of rows before dropping
        let num_rows = self.num_rows();

        // Call the destructor (to avoid the need for drop)
        unsafe {
            destroy_super_matrix_store(&mut self.super_matrix);
        };

        // Treat self as deallocated already
        mem::forget(self);

        CompColRaw {
            num_rows,
            non_zero_vals,
            row_indices,
            col_offsets,
        }
    }

    pub fn super_matrix(&self) -> &SuperMatrix {
        &self.super_matrix.super_matrix()
    }
}

impl<P: CreateCompColMat> Drop for CompColMat<P> {
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
    // Make a simple compressed column matrix
    let num_rows = 2;
    let non_zero_vals = vec![1.0, 2.0];
    let row_indices = vec![1, 2];
    let col_offsets = vec![0, 1, 2];

    let raw = CompColRaw {
        num_rows,
        non_zero_vals,
        row_indices,
        col_offsets,
    };

    // Create the matrix wrapper
    let _a = unsafe { CompColMat::from_raw(raw).expect("Failed to create matrix") };
}

/// Replace this with better tests covering creating
/// matrices and getting values
#[test]
fn test_comp_col_matrix() {
    // Make a simple compressed column matrix
    let num_rows = 2;
    let non_zero_vals = vec![1.0, 2.0];
    let row_indices = vec![1, 2];
    let col_offsets = vec![0, 1, 2];

    let raw = CompColRaw {
        num_rows,
        non_zero_vals,
        row_indices,
        col_offsets,
    };

    // Create the matrix wrapper
    let a = unsafe { CompColMat::from_raw(raw).expect("Failed to create matrix") };

    // Check the size
    assert_eq!(a.num_cols(), 2);
    assert_eq!(a.num_rows(), 2);

    // Check the values

    // Get the vectors back out
    let CompColRaw {
        num_rows,
        non_zero_vals,
        row_indices,
        col_offsets,
    } = a.to_raw();

    // Check the vectors are all correct
    assert_eq!(num_rows, 2);
    assert_eq!(non_zero_vals, vec![1.0, 2.0]);
    assert_eq!(row_indices, vec![1, 2]);
    assert_eq!(col_offsets, vec![0, 1, 2]);
}
