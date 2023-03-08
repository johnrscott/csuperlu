use csuperlu_sys::{SuperMatrix, Stype_t_SLU_DN, Dtype_t_SLU_S, Mtype_t_SLU_GE};

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
    pub fn num_columns(&self) -> usize {
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
