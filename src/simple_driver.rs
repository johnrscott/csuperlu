//! Solve sparse linear systems using the simple driver
//!

use crate::comp_col::CompColMatrix;
use crate::dense::DenseMatrix;
use crate::value_type::ValueType;
use csuperlu_sys::options::superlu_options_t;
use csuperlu_sys::stat::SuperLUStat_t;
use csuperlu_sys::super_matrix::c_SuperMatrix;

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
    pub x: DenseMatrix<P>,
    pub lu: LUDecomp<P>,
    pub column_perm: ColumnPerm,
    pub row_perm: RowPerm,
}

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


pub struct SimpleSystem<P: ValueType<P>> {
    /// The (sparse) matrix A in AX = B
    pub a: CompColMatrix<P>,
    /// The right-hand side(s) matrix B in AX = B 
    pub b: DenseMatrix<P>,
}

impl<P: ValueType<P>> SimpleSystem<P> {

    /// Solve a simple linear system AX = B with default solver
    /// options
    ///
    ///
    pub fn solve(
	self,
	stat: &mut SuperLUStat_t,
    ) -> Result<SimpleSolution<P>, SolverError> {

	let SimpleSystem {a, b} = self;

	// Check for invalid dimensions
	let mut options = superlu_options_t::new();
	let mut column_perm = Vec::<i32>::with_capacity(a.num_columns());
	let mut row_perm = Vec::<i32>::with_capacity(a.num_rows());
	
	let mut info = 0;
	unsafe {
            let mut l = c_SuperMatrix::alloc();
            let mut u = c_SuperMatrix::alloc();

            let mut b_super_matrix = b.into_super_matrix();

            P::c_simple_driver(
		&mut options,
		&a.super_matrix(),
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
		Ok(SimpleSolution { x, lu, column_perm, row_perm })
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
