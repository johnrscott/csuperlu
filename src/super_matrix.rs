//! SuperMatrix struct for defining matrices.
//!
//! Structs and functions for interfacing to the SuperMatrix
//! C struct. The SuperMatrix struct is responsible for defining
//! the matrix types in SuperLU.
//!

use csuperlu_sys::{SuperMatrix, Mtype_t_SLU_GE, Dtype_t_SLU_C, Stype_t_SLU_DN};

/// The SuperMatrix structure, which stores all types of matrices
/// in SuperLU.
pub struct CSuperMatrix {
    super_matrix: SuperMatrix,
}

impl CSuperMatrix {
    /// Create a new struct with default values for the elements. These
    /// values are well-defined, but meaningless, and this function only
    /// exists to create SuperMatrix structures for passing to SuperLU
    /// functions. Note in particular that the Store field is an invalid
    /// (null) pointer.
    pub unsafe fn alloc() -> Self {
	SuperMatrix {
	    Stype: Stype_t_SLU_DN,
	    Dtype: Dtype_t_SLU_C,
	    Mtype: Mtype_t_SLU_GE,
	    nrow: 0,
	    ncol: 0,
	    Store: 0 as *mut libc::c_void,
	}
    }
}
