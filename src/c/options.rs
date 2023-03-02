//! Options argument
//!
//! The options argument is used to specify how the solver
//! should work. It is documented in section 2.4 of the
//! SuperLU manual.
//!

use std::mem::MaybeUninit;

use csuperlu_sys::{superlu_options_t, set_default_options, colperm_t_NATURAL, colperm_t_MMD_ATA, colperm_t_MMD_AT_PLUS_A, colperm_t_COLAMD, colperm_t_MY_PERMC, rowperm_t_MY_PERMR, yes_no_t_YES, yes_no_t_NO};

/// Options for the simple driver routines
///
/// The simple driver is a basic solver wrapper that
/// exposes a certain amount of solving functionality
/// in the C SuperLU library. The simple driver solves
/// $AX = B$ and returns $B$ and the associated $LU$
/// decomposition. The system is solved from scratch,
/// without reusing information from previous solutions
/// (apart from potentially column permutations).
///
/// Advanced features such as matrix equilibration and
/// reuse of the $LU$ factorisation requires the expert driver.
///
/// The following options are available in the simple
/// driver:
///
/// * Choosing how SuperLU will compute the column
/// permutation (which algorithm to use, or whether
/// to use a user-specified permutation).
/// * What type of partial pivoting to perform: a
/// spectrum ranging from traditional partial pivoting
/// to no pivoting (always using the diagonal element as
/// the pivot).
///
/// A special option is available for matrices which are
/// diagonally dominant (have large magnitude elements on the
/// diagonal).
///
pub struct SimpleDriverOptions {
    options: CSuperluOptions,
    diagonally_dominant: bool,
}

impl SimpleDriverOptions {

    /// Create a new options object with default settings.
    /// This will perform the $LU$ decomposition of $A$,
    /// with column permutations calculated using the
    /// column approximate minimum degree algorithm and
    /// row permutations obtained by traditional partial
    /// pivoting. $A$ is not assumed to be diagonally
    /// dominant.
    pub fn new() -> Self {
	Self {
	    options: CSuperluOptions::new(),
	    diagonally_dominant: false,
	}
    }
    
    /// Instruct SuperLU to calculate the column permutation
    ///
    /// If this function is called, SuperLU will calculate the
    /// column permutation based on the matrix $A$, according to
    /// the algorithm specified. The input column permutation
    /// will be overwritten
    ///
    /// # Panic
    ///
    /// Do not call this function if set_diagonally_dominant(true, u)
    /// has been called; that implicitly sets the policy to
    /// $(A^T + A)$-based column permutations, which would be overwritten
    /// here
    pub fn set_superlu_column_perm(&mut self, policy: ColumnPermPolicy) {
	if self.diagonally_dominant {
	    panic!("Invalid attempt to set permutation policy for diagonally dominant mode");
	}
	self.options.set_column_perm_policy(policy);
    }

    /// Instruct SuperLU to use a user-specified column permutation
    ///
    /// If this function is called, SuperLU will use the column
    /// permutation passed as an argument to the simple driver.
    /// SuperLU may still modify the column permutation as part
    /// of the solution. This is because SuperLU computes the
    /// column permutation in two steps:
    ///
    /// 1. Using an algorithm based on the structure of $A$ to
    /// computate an initial permutation
    /// 2. Updating the permutation in a postordering step based
    /// on the elimination tree, which depends on the values of
    /// $A$.
    ///
    /// Providing a user-supplied column permutation means SuperLU
    /// whill skip step 1, but not step 2. The updated column
    /// permutation will overwrite the user-supplied one. TODO
    /// double-check all this is right.
    ///
    pub fn set_user_column_perm(&mut self) {
	self.options.set_user_column_perm();
    }
    
    /// Factorise $AX = B$ under the assumption that $A$
    /// is diagonally dominant
    ///
    /// As described in Section 2.5.2 of the manual, this mode
    /// is used when the magnitude of the elements on the diagonal
    /// is large compared to the other elements. In SuperLU, this
    /// is called "symmetric mode", even though $A$ is not required to
    /// be symmetric.
    ///
    /// The diagonal pivot threshold $u$ is passed as part of this
    /// option, which should be small. Choosing a small threshold
    /// strongly biases SuperLU against choosing a different pivot
    /// element other than the diagonal (which is dominant). Choose
    /// a value such as $u = 0.1$ or $u = 0.01$. See the documentation
    /// for set_diagonal_pivot_threshold() for more information.
    ///
    /// This function also sets the column permutation policy to
    /// multiple minimum degree using $A^T + A$. It is an error
    /// attempt to set the policy manually after calling this function.
    ///
    /// It is an error to call set_diagonal_pivot_threshold() after
    /// calling this function, because that would overwrite this
    /// threshold.
    ///
    pub fn set_diagonally_dominant(&mut self, value: bool, u: f64) {
	self.options.set_symmetric_mode(value);
	if value {
	    self.set_diagonal_pivot_threshold(u);
	    self.set_superlu_column_perm(ColumnPermPolicy::MmdAtPlusA);
	}
	self.diagonally_dominant = value;
    } 

    /// Set the diagonal pivot threshold
    ///
    /// In the $LU$ factorisation step, SuperLU performs
    /// a modified partial pivoting algorithm. In traditional
    /// partial pivoting, the largest magnitude value in the
    /// current column is selected as the pivot element
    /// (defining the row that will be used to eliminate other
    /// non-zero values in this column). 
    /// 
    /// Instead of choosing the largest magnitude value, SuperLU
    /// computes a threshold $t \ge 0$ above which the diagonal
    /// element will be considered "large enough", and used as
    /// the pivot element, even if others are bigger. The threshold
    /// is calculated
    /// as $t = u |a_\text{max}|$, where is $a_\text{max}$ is the
    /// next largest magnitude element (below the diagonal element).
    /// If $|a_\text{diag}| > t$, the diagonal element is used as
    /// the pivot. Else, the (true) largest magnitude element is
    /// used as the pivot. This algorithm avoids row permutations
    /// in cases where they are not required for numerical stability.
    /// Choosing lower values for $u$ makes the algorithm less likely
    /// to pivot rows, at the possible cost of numerical stability.
    ///
    /// The value $u$ is called the diagonal pivot threshold.
    /// In the special case where $u = 0.0$ (identically), SuperLU
    /// will always use the diagonal pivot. If $u = 1.0$ (identically),
    /// SuperLU will perform traditional partial pivoting (selecting
    /// the maximum-magnitude element as the pivot).
    ///
    /// It is not possible to specify a user-defined row
    /// permutation in the simple driver; see the expert driver.
    ///
    /// # Panics
    ///
    /// This function may only be called if set_diagonally_dominant(true, u)
    /// has not already been called. The diagonal pivot threshold
    /// is set as part of that function, and should not be overwritten
    /// here. TODO this should be an error, or maybe some other
    /// design.
    ///
    pub fn set_diagonal_pivot_threshold(&mut self, u: f64) {
	if self.diagonally_dominant {
	    panic!("You cannot call set_diagonal_pivot_threshold after set_diagonally_dominant()")
	}
	self.options.set_diagonal_pivot_threshold(u);
    }
}

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

    pub fn set_diagonal_pivot_threshold(&mut self, u: f64) {
	self.options.DiagPivotThresh = u;
    }
    
    pub fn set_symmetric_mode(&mut self, value: bool) {
	if value {
	    self.options.SymmetricMode = yes_no_t_YES
	} else {
	    self.options.SymmetricMode = yes_no_t_NO
	}
    }
    
    /// Get the underlying superlu_options_t struct
    ///
    /// This function is intended for use in the driver wrapper
    /// routines for getting raw access to the options struct.
    pub fn get_options(&self) -> &superlu_options_t {
	&self.options
    }

    /// Set the algorithm to be used for computing column permutations
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
