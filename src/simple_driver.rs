//! Solve sparse linear systems using the simple driver
//!
//! This module contains sparse solvers based on the simple
//! driver in SuperLU (sgssv, dgssv, cgssv and zgssv). The
//! simple driver allow access to a subset of the solver
//! options, but not all of them. More advanced options are
//! available using the expert driver.
//!
//! The simple driver performs the following steps to solve a
//! linear system $AX = B$, outlined in the SuperLU manual:
//!
//! 1. Permute the columns of $A$ to increase sparsity, using
//! a column-permutation $P_c$ which is either user-defined, or
//! is computed using a user-defined algorithm.
//! 2. Factor $A$ as $P_rAP_c = LU$, using Gaussian elimitation,
//! where $P_r$ is the row permutation obtained by using partial
//! pivoting.
//! 3. Solve the equation $AX = B$ using the $LU$-decomposition
//! in step 2.

use crate::comp_col::CompColMatrix;
use crate::dense::DenseMatrix;
use crate::c::options::{ColumnPermPolicy, SimpleDriverOptions};
use crate::c::stat::CSuperluStat;
use crate::c::value_type::{ValueType, CSimpleResult, Error};

use crate::lu_decomp::LUDecomp;
use crate::super_node::SuperNodeMatrix;

/// Contains the solution corresponding to a SimpleSystem
// pub enum SimpleResult<P: ValueType<P>> {
//     /// The solution was computed without any errors
//     Solution {
// 	a: CompColMatrix<P>,
// 	x: DenseMatrix<P>,
// 	lu: LUDecomp<P>,
// 	column_perm: ColumnPerm,
// 	row_perm: RowPerm,
//     },
//     /// The $LU$-factorisation was computed, but the
//     /// $A$ is singular (the factor $U$ contains a 0 at
//     /// index singular_col), and the solution was not
//     /// computed
//     SingularFactorisation {
// 	a: CompColMatrix<P>,
// 	singular_column: usize,
// 	lu: LUDecomp<P>,
// 	column_perm: ColumnPerm,
// 	row_perm: RowPerm,
//     },
//     /// A different kind of error occured
//     Err(Error)
// }

#[derive(Debug)]
pub enum SimpleError<P: ValueType<P>> {
    /// The $LU$-factorisation was computed, but the
    /// $A$ is singular (the factor $U$ contains a 0 at
    /// index singular_col), and the solution was not
    /// computed
    Singular {
	a: CompColMatrix<P>,
	singular_column: usize,
	lu: LUDecomp<P>,
	column_perm: ColumnPerm,
	row_perm: RowPerm,
    },
    /// A different kind of error occured
    Other(Error)
}

/// The solution was computed without any errors
pub struct SimpleSolution<P: ValueType<P>> {
    pub a: CompColMatrix<P>,
    pub x: DenseMatrix<P>,
    pub lu: LUDecomp<P>,
    pub column_perm: ColumnPerm,
    pub row_perm: RowPerm,
}

/// This function turns the result type from c_simple_driver into
/// whatever we want to serve up to users of the solve function
unsafe fn from_c_result<P: ValueType<P>>(
    a: CompColMatrix<P>,
    result: CSimpleResult
) -> Result<SimpleSolution<P>, SimpleError<P>> {
    match result {
	CSimpleResult::Solution {
	    x,
	    perm_c,
	    perm_r,
	    l,
	    u
	} => {
	    let l = SuperNodeMatrix::from_super_matrix(l);
	    let u = CompColMatrix::from_super_matrix(u);
	    let lu = LUDecomp::from_matrices(l, u);
	    let x = DenseMatrix::<P>::from_super_matrix(x);
	    let column_perm = ColumnPerm::from_raw(perm_c);
	    let row_perm = RowPerm::from_raw(perm_r);
	    Ok(SimpleSolution { a, x, lu, column_perm, row_perm })
	},
	CSimpleResult::SingularFact {
	    singular_column,
	    perm_c,
	    perm_r,
	    l,
	    u
	} => {
	    let l = SuperNodeMatrix::from_super_matrix(l);
	    let u = CompColMatrix::from_super_matrix(u);
	    let lu = LUDecomp::from_matrices(l, u);
	    let column_perm = ColumnPerm::from_raw(perm_c);
	    let row_perm = RowPerm::from_raw(perm_r);
	    Err(SimpleError::Singular { a, lu, singular_column, column_perm, row_perm })
	},
	CSimpleResult::Err(err) => Err(SimpleError::Other(err)),
    }
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
#[derive(Debug)]
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
/// column permutation for use during the solution.
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
    ) -> Result<SimpleSolution<P>, SimpleError<P>> {

	let SamePattern {
	    a,
	    b,
	    column_perm: ColumnPerm {
		column_perm,
	    }
	} = self;

	// TODO: Check for invalid dimensions

	let options = SimpleDriverOptions::new();

	unsafe {
            let b_super_matrix = b.into_super_matrix();

	    let result = P::c_simple_driver(
		options,
		&mut a.super_matrix(),
		Some(column_perm),
		b_super_matrix,
		stat,
            );
	    
	    from_c_result::<P>(a, result)
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
    ) -> Result<SimpleSolution<P>, SimpleError<P>> {

	let SimpleSystem {a, b} = self;

	// TODO: Check for invalid dimensions

	let mut options = SimpleDriverOptions::new();
	options.set_superlu_column_perm(column_perm_policy);
	
	unsafe {
            let b_super_matrix = b.into_super_matrix();

	    let result = P::c_simple_driver(
		options,
		&mut a.super_matrix(),
		None,
		b_super_matrix,
		stat,
            );

	    from_c_result::<P>(a, result)
	}
    }
}
