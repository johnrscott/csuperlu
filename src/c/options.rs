//! Options argument
//!
//! The options argument is used to specify how the solver
//! should work. It is documented in section 2.4 of the
//! SuperLU manual.
//!

use std::mem::MaybeUninit;

use csuperlu_sys::{superlu_options_t, set_default_options, colperm_t_NATURAL, colperm_t_MMD_ATA, colperm_t_MMD_AT_PLUS_A, colperm_t_COLAMD, colperm_t_MY_PERMC, rowperm_t_MY_PERMR};

/// SuperLU implements several policies for re-ordering the
/// columns of A before solving, when a specific ordering is
/// to passed to the solver. The orderings are described in
/// Section 1.3.5 of the SuperLU manual.
pub enum ColumnPermPolicy {
    /// Do not re-order columns (Pc = I)
    Natural,
    /// Multiple minimum degree applied to A^TA
    MmdAtA,
    /// Multiple minimum degree applied to A^T + A    
    MmdAtPlusA,
    /// Column approximate minimum degree. Designed particularly
    /// for unsymmetric matrices when partial pivoting is needed.
    /// It usually gives comparable orderings as MmdAtA, but
    /// is faster.
    ColAMD,
}

/// Wrapper for the SuperLU C library superlu_options_t. 
///
/// The superlu_options_t struct controls the behaviour of the
/// simple driver and expert drivers.
pub struct CSuperluOptions {
    options: superlu_options_t
}

impl CSuperluOptions {

    /// Create a new CSuperluOptions struct (containing superlu_options_t)
    ///
    /// The default options documented in Section 2.4 of the SuperLU manual:
    ///
    /// Fact = DOFACT /* factor from scratch */
    /// Equil = YES
    /// ColPerm = COLAMD
    /// Trans = NOTRANS
    /// IterRefine = NOREFINE
    /// DiagPivotThresh = 1.0 /* partial pivoting */
    /// SymmetricMode = NO
    /// PivotGrowth = NO;
    /// ConditionNumber = NO;
    /// PrintStat = YES
    ///
    pub fn new() -> Self {
        let options = unsafe {
            let mut options = MaybeUninit::<superlu_options_t>::uninit();
            set_default_options(options.as_mut_ptr());
            options.assume_init()
        };
	Self {
	    options,
	}
    }

    /// Get the underlying superlu_options_t struct
    ///
    /// This function is intended for use in the driver wrapper
    /// routines for getting raw access to the options struct.
    pub fn get_options(&self) -> &superlu_options_t {
	&self.options
    }

    /// Setting the algorithm to be used for computing column permutations
    ///
    pub fn set_column_perm_policy(&mut self, policy: ColumnPermPolicy) {
	match policy {
	    ColumnPermPolicy::Natural => self.options.ColPerm = colperm_t_NATURAL,
	    ColumnPermPolicy::MmdAtA => self.options.ColPerm = colperm_t_MMD_ATA,
	    ColumnPermPolicy::MmdAtPlusA => self.options.ColPerm = colperm_t_MMD_AT_PLUS_A,
	    ColumnPermPolicy::ColAMD => self.options.ColPerm = colperm_t_COLAMD,
	}
    }

    /// Set the column permutation option to use a user supplied vector
    ///
    pub fn set_user_column_perm(&mut self) {
	self.options.ColPerm = colperm_t_MY_PERMC;
    }

    /// Set the column permutation option to use a user supplied vector
    ///
    pub fn set_user_row_perm(&mut self) {
	self.options.RowPerm = rowperm_t_MY_PERMR;
    }

    
}
