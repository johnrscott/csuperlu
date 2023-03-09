use csuperlu_sys::{SuperMatrix, Stype_t_SLU_DN, Dtype_t_SLU_S, Mtype_t_SLU_GE};

use super::{value_type::{ValueType, Error}, free::c_destroy_super_matrix_store};

/// A SuperLU compressed-column matrix in column-major format
///
/// This is ultimately a wrapper around a SuperMatrix struct
/// (in the C library), containing a a SCformat store referring
/// to vectors allocated in rust. When this struct is dropped,
/// rust will deallocate the vectors (non-zero values, row indices
/// and column offsets), and the SuperLU library will free the
/// SuperMatrix struct. You should not need to worry about memory
/// when using this struct, apart from ensuring that the safety
/// contract of the from_vectors function is fulfilled.
pub struct CCompColMat<P: ValueType> {
    non_zero_vals: Vec<P>,
    row_indices: Vec<i32>,
    col_offsets: Vec<i32>,
    super_matrix: CSuperMatrix,
}

impl<P: ValueType> CCompColMat<P> {

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
    pub unsafe fn from_vectors(
        num_rows: usize,
	non_zero_vals: Vec<P>,
	row_indices: Vec<i32>,
	col_offsets: Vec<i32>,
    ) -> Result<Self, Error> {
	let super_matrix = P::c_create_comp_col_matrix(
	    num_rows,
	    &non_zero_vals,
	    &row_indices,
	    &col_offsets,
	)?;
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

    
}

impl<P: ValueType> Drop for CCompColMat<P> {
    fn drop(&mut self) {
	unsafe {
	    c_destroy_super_matrix_store(&mut self.super_matrix);
	}
    }
}

#[derive(Debug)]
pub struct CSuperMatrix {
    super_matrix: SuperMatrix,
}

impl CSuperMatrix {
    /// Allocate an empty SuperMatrix structure.
    ///
    /// The values of the fields are meangingless. The only 
    /// purpose of this function is to safely allocate a
    /// SuperMatrix for passing into (e.g.) dgssv as L and
    /// U. It would be better not to initialise at all -- however,
    /// at least this method is not undefined behaviour (hopefully).
    ///
    /// # Safety
    ///
    /// You will get an object which is not a valid CSuperMatrix. Only
    /// certain functions (e.g. c_create_dense_matrix, dgssv ) can create valid
    /// CSuperMatrix structs. You must pass the object created here to
    /// dgssv as the L and U parameters in order to have them initialised
    /// properly.
    ///
    pub unsafe fn alloc() -> Self {
	let super_matrix = SuperMatrix {
	    Stype: Stype_t_SLU_DN,
	    Dtype: Dtype_t_SLU_S,
	    Mtype: Mtype_t_SLU_GE,
	    nrow: 0,
	    ncol: 0,
	    Store: std::ptr::null_mut(),
	};
	Self {
	    super_matrix,
	}
    }

    /// Get the number of rows in the matrix
    pub fn num_rows(&self) -> usize {
	self.super_matrix.nrow as usize
    }

    /// Get the number of columns in the matrix
    pub fn num_cols(&self) -> usize {
	self.super_matrix.ncol as usize
    }

    /// Get a reference to the underlying SuperMatrix
    ///
    pub fn super_matrix(&self) -> &SuperMatrix {
	&self.super_matrix
    }
    
    /// Get the SuperMatrix store
    ///
    /// # Safety 
    ///
    /// This function is unsafe because you must use the
    /// correct type T for the type of matrix you want to access
    pub unsafe fn store<T>(&self) -> &T {
	&*(self.super_matrix.Store as *const T)
    }
    
}
