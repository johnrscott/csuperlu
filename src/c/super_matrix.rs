use csuperlu_sys::{SuperMatrix, Stype_t_SLU_DN, Dtype_t_SLU_S, Mtype_t_SLU_GE};


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
    pub fn alloc() -> Self {
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
}
