//! Functions to create matrices in compressed column format.
//!
//! Compressed-column matrices are very important in SuperLU, because
//! the main solver routines assume that the imput matrix $A$ is in
//! column-major format.
//!
//! A compressed-column matrix stored stores a sparse matrix in
//! column-major order, but only stores the non-zero elements in
//! each column. In order to identify which elements in the column
//! are non-zero, a vector of integers is maintained which stores
//! the row indices of the elements in the column. Arrays like this
//! are stored one after the other, one for each column in the matrix.
//! Since each column may be a different length, a third vector of
//! integers is maintained showing where each new column starts.

use crate::free::c_destroy_comp_col_matrix;
use crate::harwell_boeing::HarwellBoeingMatrix;
use crate::super_matrix::CSuperMatrix;
use csuperlu_sys::{Mtype_t_SLU_GE, NCformat};
use crate::value_type::ValueType;
use std::fs;
use std::ops::Mul;
use std::process;

/// Compressed-column matrix
///
///
pub struct CompColMatrix<P: ValueType<P>> {
    super_matrix: CSuperMatrix,
    marker: std::marker::PhantomData<P>,
}

impl<P: ValueType<P>> CompColMatrix<P> {
    /// Create a compressed-column matrix from a SuperMatrix structure
    ///
    pub fn from_super_matrix(super_matrix: CSuperMatrix) -> Self {
        Self {
            super_matrix,
            marker: std::marker::PhantomData,
        }
    }

    /// Create a compressed-column matrix from a file
    /// stored in Harwell-Boeing format. The function will
    /// attempt to parse the non-zero values in the precision
    /// P
    pub fn from_harwell_boeing(file_path: String) -> Self {
        let file = fs::File::open(&file_path).unwrap_or_else(|err| {
            println!("Problem opening file '{file_path}': {err}");
            process::exit(1);
        });

        let matrix = HarwellBoeingMatrix::<P>::from_file(file);

        // Matrix dimensions
        let num_rows = matrix.num_rows();

        // Vector of doubles of length nnz
        let (column_offsets, row_indices, non_zero_values) = matrix.to_vectors();

        // Make the left-hand side matrix
        Self::from_vectors(num_rows, non_zero_values, row_indices, column_offsets)
    }

    /// Specify a compressed column matrix from input vectors.
    ///
    /// Use this function to make a SuperMatrix in compressed column
    /// format, from the vector of values, row indices, and column
    /// offsets. Compressed column format is documented in Section
    /// 2.3 of the SuperLU manual.
    ///
    /// Need to check what Mtype_t is used for. The table in Section 2.3
    /// shows SLU_GE for A, but SLU_TRLU for L and U; however, does the
    /// user of the library ever need to pick a different value? If not,
    /// the argument can be removed.
    ///
    pub fn from_vectors(
        num_rows: usize,
        mut non_zero_values: Vec<P>,
        mut row_indices: Vec<i32>,
        mut column_offsets: Vec<i32>,
    ) -> Self {
        let super_matrix = unsafe {
            let super_matrix = P::c_create_comp_col_matrix(
                num_rows,
                &mut non_zero_values,
                &mut row_indices,
                &mut column_offsets,
                Mtype_t_SLU_GE,
            )
            .expect("Error creating comp col -- replace with error handling");
            // The freeing of the input vectors is handed over
            // to the C library functions (see drop)
            std::mem::forget(non_zero_values);
            std::mem::forget(row_indices);
            std::mem::forget(column_offsets);
            super_matrix
        };
        Self {
            super_matrix,
            marker: std::marker::PhantomData,
        }
    }

    pub fn value(&mut self, row: usize, col: usize) -> P {
        let super_matrix = self.super_matrix();
        assert!(row < super_matrix.num_rows(), "Row index out of range");
        assert!(
            col < super_matrix.num_columns(),
            "Column index out of range"
        );
        let col_start = self.column_offsets()[col] as usize;
        let col_end = self.column_offsets()[col + 1] as usize;
        let row_indices = &self.row_indices()[col_start..col_end];
        match row_indices.binary_search(&(row as i32)) {
            Ok(row_index) => self.non_zero_values()[col_start + row_index],
            Err(_) => P::zero(),
        }
    }

    /// Get the number of rows in the sparse matrix
    pub fn num_rows(&self) -> usize {
        self.super_matrix.num_rows()
    }

    /// Get the number of columns in the sparse matrix
    pub fn num_columns(&self) -> usize {
        self.super_matrix.num_columns()
    }

    /// Get the read-only non-zero values in the compressed-column format
    pub fn non_zero_values(&self) -> &[P] {
        unsafe {
            let c_ncformat = self.super_matrix.store::<NCformat>();
            std::slice::from_raw_parts(c_ncformat.nzval as *mut P, c_ncformat.nnz as usize)
        }
    }

    /// Get the read-only column offsets in the compressed-column format
    pub fn column_offsets(&self) -> &[i32] {
        unsafe {
            let c_ncformat = self.super_matrix.store::<NCformat>();
            std::slice::from_raw_parts(
                c_ncformat.colptr as *mut i32,
                self.super_matrix.num_columns() + 1,
            )
        }
    }

    /// Get the read-only row indices in the compressed-column format
    pub fn row_indices(&self) -> &[i32] {
        unsafe {
            let c_ncformat = self.super_matrix.store::<NCformat>();
            std::slice::from_raw_parts(c_ncformat.rowind as *mut i32, c_ncformat.nnz as usize)
        }
    }

    /// Get the underlying CSuperMatrix
    pub fn super_matrix<'a>(&'a self) -> &'a CSuperMatrix {
        &self.super_matrix
    }

    /// Call the SuperLU C library print function for this type
    pub fn print(&self, what: &str) {
        unsafe {
            P::c_print_comp_col_matrix(what, &self.super_matrix);
        }
    }
}

impl<'a, P: ValueType<P>> Mul<&Vec<P>> for &'a mut CompColMatrix<P> {
    type Output = Vec<P>;

    /// Naive matrix multiplication which loops over all
    /// each full row of the sparse matrix and adds up the
    /// results.
    fn mul(self, x: &Vec<P>) -> Vec<P> {
        assert!(
            self.num_columns() == x.len(),
            "Cannot multiply; incompatible dimensions"
        );
        let mut b = Vec::<P>::new();
        for row in 0..self.num_rows() {
            let mut value = P::zero();
            for column in 0..self.num_columns() {
                value = value + (self.value(row, column) * x[row]);
            }
            b.push(value);
        }
        b
    }
}

impl<P: ValueType<P>> Drop for CompColMatrix<P> {
    fn drop(&mut self) {
        unsafe {
	    c_destroy_comp_col_matrix(&mut self.super_matrix);
	}
    }
}
