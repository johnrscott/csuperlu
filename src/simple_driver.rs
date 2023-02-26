//! Solve sparse linear systems using the simple driver
//!

use crate::comp_col::CompColMatrix;
use crate::dense::DenseMatrix;
use crate::c::options::{CSuperluOptions, ColumnPermPolicy};
use crate::c::stat::CSuperluStat;
use crate::c::super_matrix::CSuperMatrix;
use crate::c::value_type::ValueType;

use crate::lu_decomp::LUDecomp;
use crate::super_node::SuperNodeMatrix;

use std::{error::Error, fmt};

/// Errors from the solver
#[derive(Debug)]
pub struct SolverError {
    /// info != 0 indicates solver error. See e.g. dgssv documentation
    /// for the meaning of info.
    info: i32,
}

impl Error for SolverError {}

impl fmt::Display for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to solve the linear system; info = {}", self.info)
    }
}

/// Contains the solution corresponding to a SimpleSystem
pub struct SimpleSolution<P: ValueType<P>> {
    pub a: CompColMatrix<P>,
    pub x: DenseMatrix<P>,
    pub lu: LUDecomp<P>,
    pub column_perm: ColumnPerm,
    pub row_perm: RowPerm,
}

/// Stores a column permutation vector
#[derive(Debug)]
pub struct ColumnPerm {
    column_perm: Vec<i32>,
}

impl ColumnPerm {
    /// Unsafe because content of Vec is not checked
    /// (elements need to be unique)
    pub unsafe fn from_raw(column_perm: Vec<i32>) -> Self {
	Self {
	    column_perm,
	}
    }
}

/// Stores a row permutation vector
pub struct RowPerm {
    row_perm: Vec<i32>,
}

impl RowPerm {
    /// Unsafe because content of Vec is not checked
    /// (elements need to be unique)
    pub unsafe fn from_raw(row_perm: Vec<i32>) -> Self {
	Self {
	    row_perm,
	}
    }
}

/// Defines a simple sparse linear system $AX = B$
pub struct SimpleSystem<P: ValueType<P>> {
    /// The (sparse) matrix $A$
    pub a: CompColMatrix<P>,
    /// The right-hand side(s) matrix $B$
    pub b: DenseMatrix<P>,
}

/// Defines a sparse linear system $AX = B$ and a predefined
/// column permutation for use during the solution
pub struct SamePattern<P: ValueType<P>> {
    /// The (sparse) matrix $A$
    pub a: CompColMatrix<P>,
    /// The right-hand side(s) matrix $B$
    pub b: DenseMatrix<P>,
    /// The column permutation to use for the solution
    pub column_perm: ColumnPerm,
}

impl<P: ValueType<P>> SamePattern<P> {
    pub fn solve(
	self,
	stat: &mut CSuperluStat,
    ) -> Result<SimpleSolution<P>, SolverError> {

	let SamePattern {
	    a,
	    b,
	    column_perm: ColumnPerm {
		mut column_perm,
	    }
	} = self;

	// TODO: Check for invalid dimensions

	let mut options = CSuperluOptions::new();

	// Use the same column permutation. In the dgssv
	// (simple driver) source code, the options.Fact
	// value must be set to DOFACT (it is not allowed
	// to use SamePattern). The use of a user supplied
	// column permutation is controlled by MY_PERMC,
	// which, if specified, means that get_perm_c
	// (computing the column permutation) is not called
	// (line 192-193, dgssv.c).
	options.set_user_column_perm();
	
	let mut row_perm = Vec::<i32>::with_capacity(a.num_rows());

	let mut info = 0;
	unsafe {
	    let mut l = CSuperMatrix::alloc();
            let mut u = CSuperMatrix::alloc();

            let mut b_super_matrix = b.into_super_matrix();

            P::c_simple_driver(
		&mut options,
		&mut a.super_matrix(),
		&mut column_perm,
		&mut row_perm,
		&mut l,
		&mut u,
		&mut b_super_matrix,
		stat,
		&mut info,
            );
            let l = SuperNodeMatrix::from_super_matrix(l);
            let u = CompColMatrix::from_super_matrix(u);
            let lu = LUDecomp::from_matrices(l, u);
            let x = DenseMatrix::<P>::from_super_matrix(b_super_matrix);
	    let column_perm = ColumnPerm::from_raw(column_perm);
	    let row_perm = RowPerm::from_raw(row_perm);
	    
            if info != 0 {
		Err(SolverError { info })
            } else {
		Ok(SimpleSolution { a, x, lu, column_perm, row_perm })
            }
	}
    }
}

impl<P: ValueType<P>> SimpleSystem<P> {

    /// Solve a simple linear system AX = B with default solver
    /// options
    ///
    /// The function calls the simple driver as described in the
    /// SuperLU manual. In the simple driver, column permutations
    /// are chosen to increase sparsity in the L and U factors,
    /// and row permutations are chosen to increase numerical
    /// stability. No equilibration is performed (D_r = D_c = I).
    ///
    /// Column permutations are chosen according to a policy. The
    /// available policies are documented in the SuperLU manual
    /// Section 1.3.5.
    ///
    pub fn solve(
	self,
	stat: &mut CSuperluStat,
	column_perm_policy: ColumnPermPolicy,
    ) -> Result<SimpleSolution<P>, SolverError> {

	let SimpleSystem {a, b} = self;

	// TODO: Check for invalid dimensions

	let mut options = CSuperluOptions::new();
	options.set_column_perm_policy(column_perm_policy);
	
	let mut column_perm = Vec::<i32>::with_capacity(a.num_columns());
	let mut row_perm = Vec::<i32>::with_capacity(a.num_rows());
	unsafe {
	    column_perm.set_len(a.num_columns());
	    row_perm.set_len(a.num_rows());
	}

	b.print("B");
	
	let mut info = 0;
	unsafe {
	    let mut l = CSuperMatrix::alloc();
            let mut u = CSuperMatrix::alloc();
	    
            let mut b_super_matrix = b.into_super_matrix();

            P::c_simple_driver(
		&mut options,
		&mut a.super_matrix(),
		&mut column_perm,
		&mut row_perm,
		&mut l,
		&mut u,
		&mut b_super_matrix,
		stat,
		&mut info,
            );
            let l = SuperNodeMatrix::from_super_matrix(l);
            let u = CompColMatrix::from_super_matrix(u);
            let lu = LUDecomp::from_matrices(l, u);
            let x = DenseMatrix::<P>::from_super_matrix(b_super_matrix);
	    let column_perm = ColumnPerm::from_raw(column_perm);
	    let row_perm = RowPerm::from_raw(row_perm);
	    
            if info != 0 {
		Err(SolverError { info })
            } else {
		Ok(SimpleSolution { a, x, lu, column_perm, row_perm })
            }
	}
    }
}
