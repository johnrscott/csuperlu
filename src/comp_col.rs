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
use crate::super_matrix::SuperMatrix;
use std::mem::MaybeUninit;
use std::fs;
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
    /// stored in Harwell-Boeing format. 
    pub fn from_harwell_boeing(file_path: String) -> Self {

	let file = fs::File::open(&file_path).unwrap_or_else(|err| {
            println!("Problem opening file '{file_path}': {err}");
            process::exit(1);
	});	    

	// Matrix dimensions
	let m: i32 = 5;
	let n: i32 = 5;

	// Number of non-zeros
	let nnz: i32 = 12;

	// Vector of doubles of length nnz
	let a = vec![P::from_f32(1.0).unwrap(); nnz as usize];

	// Vector of ints of length nnz
	let asub = vec![0, 1, 4, 1, 2, 4, 0, 2, 0, 3, 3, 4];

	// Vector of ints of length n+1
	let xa = vec![0, 3, 6, 8, 10, 12];

	// Make the left-hand side matrix
	Self::from_vectors(m, n, nnz, a, asub, xa)
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
        m: i32,
        n: i32,
        nnz: i32,
        mut nzval: Vec<P>,
        mut rowind: Vec<i32>,
        mut colptr: Vec<i32>,
    ) -> Self {
        let c_super_matrix = unsafe {
            let mut c_super_matrix = MaybeUninit::<c_SuperMatrix>::uninit();
            P::c_create_comp_col_matrix(
                &mut c_super_matrix,
                m,
                n,
                nnz,
                &mut nzval,
                &mut rowind,
                &mut colptr,
                Mtype_t::SLU_GE,
            );
            // The freeing of the input vectors is handed over
            // to the C library functions (see drop)
            std::mem::forget(nzval);
            std::mem::forget(rowind);
            std::mem::forget(colptr);
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
	let col_start = self.column_pointers()[col] as usize;
	let col_end = self.column_pointers()[col+1] as usize;
	let row_indices = &self.row_indices()[col_start..col_end];
	match row_indices.binary_search(&(row as i32)) {
	    Ok(row_index) =>
		self.nonzero_values()[col_start + row_index].clone(),
	    Err(_) => P::from_f32(0.0).unwrap()
	}
    }

    
    pub fn nonzero_values(&mut self) -> &[P] {
        unsafe {
            let c_ncformat = &mut *(self.c_super_matrix.Store as *mut c_NCformat);
            std::slice::from_raw_parts(c_ncformat.nzval as *mut P, c_ncformat.nnz as usize) 
        }
    }
    pub fn column_pointers(&mut self) -> &[i32] {
        unsafe {
            let c_ncformat = &mut *(self.c_super_matrix.Store as *mut c_NCformat);
            std::slice::from_raw_parts(c_ncformat.colptr as *mut i32, self.c_super_matrix.ncol as usize + 1) 
        }
    }
    pub fn row_indices(&mut self) -> &[i32] {
        unsafe {
            let c_ncformat = &mut *(self.c_super_matrix.Store as *mut c_NCformat);
            std::slice::from_raw_parts(c_ncformat.rowind as *mut i32, c_ncformat.nnz as usize) 
        }
    }

}

impl<P: CCompColMatrix<P>> SuperMatrix for CompColMatrix<P> {
    fn super_matrix<'a>(&'a mut self) -> &'a mut c_SuperMatrix {
        &mut self.c_super_matrix
    }
    fn print(&mut self, what: &str) {
        let c_str = std::ffi::CString::new(what).unwrap();
        P::c_print_comp_col_matrix(c_str.as_ptr() as *mut libc::c_char, self.super_matrix());
    }
}

impl<P: CCompColMatrix<P>> Drop for CompColMatrix<P> {
    fn drop(&mut self) {
        // Note that the input vectors are also freed by this line
        c_Destroy_CompCol_Matrix(&mut self.c_super_matrix);
    }
}
