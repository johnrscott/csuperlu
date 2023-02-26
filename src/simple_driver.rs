//! Solve sparse linear systems using the simple driver
//!

use crate::comp_col::CompColMatrix;
use crate::dense::DenseMatrix;
use crate::options::CSuperluOptions;
use crate::value_type::ValueType;
use csuperlu_sys::{colperm_t, colperm_t_MY_PERMC};
use csuperlu_sys::SuperLUStat_t;
use csuperlu_sys::SuperMatrix as c_SuperMatrix;

use crate::lu_decomp::LUDecomp;
use crate::super_matrix::SuperMatrix;
use crate::super_node::SuperNodeMatrix;

use std::{error::Error, fmt};

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

pub struct SimpleSolution<P: ValueType<P>> {
    pub a: CompColMatrix<P>,
    pub x: DenseMatrix<P>,
    pub lu: LUDecomp<P>,
    pub column_perm: ColumnPerm,
    pub row_perm: RowPerm,
}

#[derive(Debug)]
pub struct ColumnPerm {
    column_perm: Vec<i32>,
}

// impl<'a> ColumnPerm {
//     fn get_perm(&'a mut self) -> &'a mut Vec<i32> {
// 	&mut self.column_perm
//     }
// }

impl ColumnPerm {
    /// Unsafe because content of Vec is not checked
    /// (elements need to be unique)
    pub unsafe fn from_raw(column_perm: Vec<i32>) -> Self {
	Self {
	    column_perm,
	}
    }
}

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

pub struct SimpleSystem<P: ValueType<P>> {
    /// The (sparse) matrix A in AX = B
    pub a: CompColMatrix<P>,
    /// The right-hand side(s) matrix B in AX = B 
    pub b: DenseMatrix<P>,
}

pub struct SamePattern<P: ValueType<P>> {
    pub a: CompColMatrix<P>,
    pub b: DenseMatrix<P>,
    pub column_perm: ColumnPerm,
}

impl<P: ValueType<P>> SamePattern<P> {
    pub fn solve(
	self,
	stat: &mut SuperLUStat_t,
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
	options.ColPerm = colperm_t_MY_PERMC;
	
	let mut row_perm = Vec::<i32>::with_capacity(a.num_rows());

	let mut info = 0;
	unsafe {
            let mut l = c_SuperMatrix::alloc();
            let mut u = c_SuperMatrix::alloc();

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
	stat: &mut SuperLUStat_t,
	column_perm_policy: ColumnPermPolicy,
    ) -> Result<SimpleSolution<P>, SolverError> {

	let SimpleSystem {a, b} = self;

	// TODO: Check for invalid dimensions

	let mut options = superlu_options_t::new();
	match column_perm_policy {
	    ColumnPermPolicy::Natural => options.ColPerm = colperm_t::NATURAL,
	    ColumnPermPolicy::MmdAtA => options.ColPerm = colperm_t::MMD_ATA,
	    ColumnPermPolicy::MmdAtPlusA => options.ColPerm = colperm_t::MMD_AT_PLUS_A,
	    ColumnPermPolicy::ColAMD => options.ColPerm = colperm_t::COLAMD,
	}

	let mut column_perm = Vec::<i32>::with_capacity(a.num_columns());
	let mut row_perm = Vec::<i32>::with_capacity(a.num_rows());
	unsafe {
	    column_perm.set_len(a.num_columns());
	    row_perm.set_len(a.num_rows());
	}

	let mut info = 0;
	unsafe {
            let mut l = c_SuperMatrix::alloc();
            let mut u = c_SuperMatrix::alloc();

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


/*
/// Solve a sparse linear system AX = B.
///
/// The inputs to the function are the matrix A, the rhs matrix B,
/// and the permutation vectors. The outputs are the solution X
/// (which uses the same storage as B), the L and U matrices of
/// the LU decomposition.
///
/// The matrix A must be in column-major compressed-column format.
/// (see Section 2.3 in the SuperLU manual.) If a row-major matrix
/// is passed for A (CompRowMatrix), then the routine will decompose
/// A^T. Make sure to convert the CompRowMatrix to a CompColumnMatrix
/// if you want to solve A.
///
/// # Safety?
///
/// If perm_c and perm_r are input arguments (depends on the options),
/// then either a check is needed for validity (too costly), or this
/// function must be marked as unsafe. 
///
pub fn simple_driver<P: ValueType<P>>(
    mut options: superlu_options_t,
    a: &CompColMatrix<P>,
    perm_c: &mut Vec<i32>,
    perm_r: &mut Vec<i32>,
    b: DenseMatrix<P>,
    stat: &mut SuperLUStat_t,
) -> Result<SimpleSolution<P>, SolverError> {
    let mut info = 0;
    unsafe {
        let mut l = c_SuperMatrix::alloc();
        let mut u = c_SuperMatrix::alloc();

        let mut b_super_matrix = b.into_super_matrix();

        P::c_simple_driver(
            &mut options,
            a.super_matrix(),
            perm_c,
            perm_r,
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

        if info != 0 {
            Err(SolverError { info })
        } else {
            Ok(SimpleSolution { x, lu })
        }
    }
}
*/
