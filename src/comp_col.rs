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

use crate::c::comp_col::c_Destroy_CompCol_Matrix;
use crate::c::comp_col::CCompColMatrix;
use crate::c::super_matrix::{c_NCformat, c_SuperMatrix, Mtype_t};
use crate::harwell_boeing::HarwellBoeingMatrix;
use crate::super_matrix::SuperMatrix;
use std::mem::MaybeUninit;
use std::fs;
use std::ops::Mul;
use std::process;

/// Compressed-column matrix
///
///
pub struct CompColMatrix<P: CCompColMatrix<P>> {
    c_super_matrix: c_SuperMatrix,
    marker: std::marker::PhantomData<P>,
}

impl<P: CCompColMatrix<P>> CompColMatrix<P> {
    /// Create a compressed-column matrix from a c_SuperMatrix structure
    ///
    pub fn from_super_matrix(c_super_matrix: c_SuperMatrix) -> Self {
        Self {
            c_super_matrix,
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
	let num_columns = matrix.num_columns();

	// Vector of doubles of length nnz
	let (column_offsets,
	     row_indices,
	     non_zero_values) = matrix.to_vectors();
	
	// Make the left-hand side matrix
	Self::from_vectors(num_rows,
			   num_columns,
			   non_zero_values.len(),
			   non_zero_values, row_indices, column_offsets)
    }

    /// Specify a compressed column matrix from input vectors.
    ///
    /// Use this function to make a c_SuperMatrix in compressed column
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
        num_columns: usize,
        num_non_zeros: usize,
        mut non_zero_values: Vec<P>,
        mut row_indices: Vec<i32>,
        mut column_offsets: Vec<i32>,
    ) -> Self {
        let c_super_matrix = unsafe {
            let mut c_super_matrix = MaybeUninit::<c_SuperMatrix>::uninit();
            P::c_create_comp_col_matrix(
                &mut c_super_matrix,
                num_rows as i32,
                num_columns as i32,
                num_non_zeros as i32,
                &mut non_zero_values,
                &mut row_indices,
                &mut column_offsets,
                Mtype_t::SLU_GE,
            );
            // The freeing of the input vectors is handed over
            // to the C library functions (see drop)
            std::mem::forget(non_zero_values);
            std::mem::forget(row_indices);
            std::mem::forget(column_offsets);
            c_super_matrix.assume_init()
        };
        Self {
            c_super_matrix,
            marker: std::marker::PhantomData,
        }
    }

    pub fn value(&mut self, row: usize, col: usize) -> P {
	let c_super_matrix = self.super_matrix();
	assert!(row < c_super_matrix.nrow as usize,
		"Row index out of range");
	assert!(col < c_super_matrix.ncol as usize,
		"Column index out of range");
	let col_start = self.column_offsets()[col] as usize;
	let col_end = self.column_offsets()[col+1] as usize;
	let row_indices = &self.row_indices()[col_start..col_end];
	match row_indices.binary_search(&(row as i32)) {
	    Ok(row_index) => self.non_zero_values()[col_start + row_index],
	    Err(_) => P::zero(),
	}
    }

    pub fn num_rows(&self) -> usize {
        self.c_super_matrix.nrow as usize
    }

    pub fn num_columns(&self) -> usize {
        self.c_super_matrix.ncol as usize
    }

    pub fn non_zero_values(&self) -> &[P] {
        unsafe {
            let c_ncformat = &mut *(self.c_super_matrix.Store as *mut c_NCformat);
            std::slice::from_raw_parts(c_ncformat.nzval as *mut P, c_ncformat.nnz as usize) 
        }
    }
    pub fn column_offsets(&self) -> &[i32] {
        unsafe {
            let c_ncformat = &mut *(self.c_super_matrix.Store as *mut c_NCformat);
            std::slice::from_raw_parts(c_ncformat.colptr as *mut i32, self.c_super_matrix.ncol as usize + 1) 
        }
    }
    pub fn row_indices(&self) -> &[i32] {
        unsafe {
            let c_ncformat = &mut *(self.c_super_matrix.Store as *mut c_NCformat);
            std::slice::from_raw_parts(c_ncformat.rowind as *mut i32, c_ncformat.nnz as usize) 
        }
    }

}

impl<'a, P: CCompColMatrix<P>> Mul<&Vec<P>> for &'a mut CompColMatrix<P> {
    
    type Output = Vec<P>;
    
    /// Naive matrix multiplication which loops over all
    /// each full row of the sparse matrix and adds up the
    /// results.
    fn mul(self, x: &Vec<P>) -> Vec<P> {
	assert!(self.num_columns() == x.len(),
		"Cannot multiply; incompatible dimensions");
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

impl<P: CCompColMatrix<P>> SuperMatrix for CompColMatrix<P> {
    fn super_matrix<'a>(&'a self) -> &'a c_SuperMatrix {
        &self.c_super_matrix
    }
    fn print(&self, what: &str) {
        let c_str = std::ffi::CString::new(what).unwrap();
        P::c_print_comp_col_matrix(
	    c_str.as_ptr() as *mut libc::c_char,
	    &self.c_super_matrix as *const c_SuperMatrix
		as *mut c_SuperMatrix);
    }
}

impl<P: CCompColMatrix<P>> Drop for CompColMatrix<P> {
    fn drop(&mut self) {
        // Note that the input vectors are also freed by this line
        c_Destroy_CompCol_Matrix(&mut self.c_super_matrix);
    }
}
