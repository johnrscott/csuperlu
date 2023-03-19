//! Interface to the simple driver routine
//!

use csuperlu_sys::{sgssv, superlu_options_t, SuperMatrix, dgssv, cgssv, zgssv};

use super::{options::SimpleDriverOptions, comp_col::{CompColMat, create_comp_col_mat::CreateCompColMat}, dense::{DenseMat, create_dense_mat::CreateDenseMat}, error::Error, stat::SuperluStat, super_matrix::CSuperMatrix};

/// Solution from the simple driver
pub struct SimpleSolution<P: SimpleDriver> {
    pub x: DenseMat<P>,
    pub perm_c: Vec<i32>,
    pub perm_r: Vec<i32>,
    pub l: CSuperMatrix,
    pub u: CSuperMatrix,    
}

/// Enum of errors that can arise during the solution
pub enum SimpleError {
    /// The factorisatio completed successfully, but A
    /// was singular so no solution was returned
    SingularFact {
	singular_column: usize,
	perm_c: Vec<i32>,
	perm_r: Vec<i32>,
	l: CSuperMatrix,
	u: CSuperMatrix,	
    },
    /// An out-of-memory error or another (unknown) error
    /// occured.
    Err(Error),
}

/// Find the return type from a *gssv routine
///
/// The success or failure of the *gssv routines is
/// indicated by the info argument, which is expected
/// to be non-negative. 0 indicates success (factorisation
/// and solution have been found). 0 < info < num_cols_a
/// means that A was singular, then U is exactly singular,
/// due to U(i,i) == 0 (here, the matrix diagonal is indexed
/// from 1). If info > num_cols_a, then a memory failure
/// occured. In that case, (info - num_cols_a) is the number
/// of bytes allocated when the failure occured. TODO check
/// the superlu source code that this is not out-by-one.
fn simple_result_from_vectors<P: SimpleDriver>(
    info: i32,
    num_cols_a: usize,
    x: DenseMat<P>,
    perm_c: Vec<i32>,
    perm_r: Vec<i32>,
    l: CSuperMatrix,
    u: CSuperMatrix,
) -> Result<SimpleSolution<P>, SimpleError> {
    if info < 0 {
	// Check for invalid (negative) info
	Err(SimpleError::Err(Error::UnknownError))
    } else if info == 0 {
	// Success -- system solved
	Ok(SimpleSolution {
	    x,
	    perm_c,
	    perm_r,
	    l,
	    u,
	})
    } else if info as usize <= num_cols_a {
	// A is singular, factorisation successful
	Err(SimpleError::SingularFact {
	    singular_column: info as usize - 1,
	    perm_c,
	    perm_r,
	    l,
	    u,
	})
    } else {
	// Failed due to singular A -- factorisation complete
	let mem_alloc_at_failure = info as usize - num_cols_a;
	Err(SimpleError::Err(Error::OutOfMemory { mem_alloc_at_failure  }))
    }
}

/// Make the permutation vectors for the simple driver. Pass
/// the size of the matrix (square, num_rows or num_cols),
/// the (optional) column permutation, and the options. If
/// the column permutation is already specified, the options
/// are modified to make SuperLU use the user columns
fn make_simple_perms(
    size: usize,
    perm_c: Option<Vec<i32>>,
    mut options: SimpleDriverOptions,
) -> (Vec<i32>, Vec<i32>, SimpleDriverOptions) {
    let perm_c = match perm_c {
	Some(perm) => {
	    options.set_user_column_perm();
	    perm
	},
	None => {
	    let mut perm = Vec::<i32>::with_capacity(size);
	    unsafe { perm.set_len(size); }
	    perm
	},
    };

    let mut perm_r = Vec::<i32>::with_capacity(size);
    unsafe { perm_r.set_len(size); }

    (perm_c, perm_r, options)
}

/// Trait implementing the solution function (simple_driver)
pub trait SimpleDriver: Sized + CreateCompColMat + CreateDenseMat {
    /// Solve a sparse linear system using the simple driver
    ///
    /// Maybe this doesn't need to be unsafe? Although it may
    /// depend on the options (for example, if perm_c or perm_r
    /// contain content).
    ///
    /// This function makes the assumption that dgssv etc. do not
    /// modify the options argument, or the input matrix a.
    /// TODO: check these assumptions.
    ///
    /// # Errors
    ///
    /// Can catch incorrect dimensions in a, b, perm_c and perm_r.
    /// Can also probably catch incorrect matrices a and b (consider
    /// doing this in the other functions too where applicable).
    ///
    /// # Safety
    ///
    /// The matrix a must be a compressed-column matrix (TODO
    /// implement the compressed-row matrix version). The matrix
    /// b must be a dense matrix. The matrices l and u must be
    /// allocated structures (SuperMatrix::alloc).
    ///
    unsafe fn simple_driver(
        options: SimpleDriverOptions,
        a: &CompColMat<Self>,
        perm_c: Option<Vec<i32>>,
        b: DenseMat<Self>,
        stat: &mut SuperluStat,
    ) -> Result<SimpleSolution<Self>, SimpleError>;
}

impl SimpleDriver for f32 {
    unsafe fn simple_driver(
        options: SimpleDriverOptions,
        a: &CompColMat<Self>,
        perm_c: Option<Vec<i32>>,
        b: DenseMat<Self>,
        stat: &mut SuperluStat,
    ) -> Result<SimpleSolution<Self>, SimpleError> {
	let mut info = 0i32;
	let l = CSuperMatrix::alloc();
        let u = CSuperMatrix::alloc();
	let (mut perm_c, mut perm_r, options)
	    = make_simple_perms(a.num_cols(), perm_c, options);
		
        sgssv(
            options.get_options() as *const superlu_options_t as *mut superlu_options_t,
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            perm_c.as_mut_ptr(),
            perm_r.as_mut_ptr(),
            l.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            u.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            b.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            stat.get_stat(),
            &mut info,
        );

	simple_result_from_vectors(info, a.num_cols(), b, perm_c, perm_r, l, u,)
    }
}

impl SimpleDriver for f64 {
    unsafe fn simple_driver(
        options: SimpleDriverOptions,
        a: &CompColMat<Self>,
        perm_c: Option<Vec<i32>>,
        b: DenseMat<Self>,
        stat: &mut SuperluStat,
    ) -> Result<SimpleSolution<Self>, SimpleError> {

	let mut info = 0i32;
	let l = CSuperMatrix::alloc();
        let u = CSuperMatrix::alloc();
	let (mut perm_c, mut perm_r, options)
	    = make_simple_perms(a.num_cols(), perm_c, options);

        dgssv(
            options.get_options() as *const superlu_options_t as *mut superlu_options_t,
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            perm_c.as_mut_ptr(),
            perm_r.as_mut_ptr(),
            l.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            u.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            b.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            stat.get_stat(),
            &mut info,
        );

	simple_result_from_vectors(info, a.num_cols(), b, perm_c, perm_r, l, u,)
    }
}

impl SimpleDriver for num::Complex<f32> {
    unsafe fn simple_driver(
        options: SimpleDriverOptions,
        a: &CompColMat<Self>,
        perm_c: Option<Vec<i32>>,
        b: DenseMat<Self>,
        stat: &mut SuperluStat,
    ) -> Result<SimpleSolution<Self>, SimpleError> {
	let mut info = 0i32;
	let l = CSuperMatrix::alloc();
        let u = CSuperMatrix::alloc();
	let (mut perm_c, mut perm_r, options)
	    = make_simple_perms(a.num_cols(), perm_c, options);

	cgssv(
            options.get_options() as *const superlu_options_t as *mut superlu_options_t,
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            perm_c.as_mut_ptr(),
            perm_r.as_mut_ptr(),
            l.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            u.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            b.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            stat.get_stat(),
            &mut info,
        );

	simple_result_from_vectors(info, a.num_cols(), b, perm_c, perm_r, l, u,)
    }
}

impl SimpleDriver for num::Complex<f64> {
    unsafe fn simple_driver(
        options: SimpleDriverOptions,
        a: &CompColMat<Self>,
        perm_c: Option<Vec<i32>>,
        b: DenseMat<Self>,
        stat: &mut SuperluStat,
    ) -> Result<SimpleSolution<Self>, SimpleError> {
	let mut info = 0i32;
	let l = CSuperMatrix::alloc();
        let u = CSuperMatrix::alloc();
	let (mut perm_c, mut perm_r, options)
	    = make_simple_perms(a.num_cols(), perm_c, options);

	zgssv(
            options.get_options() as *const superlu_options_t as *mut superlu_options_t,
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            perm_c.as_mut_ptr(),
            perm_r.as_mut_ptr(),
            l.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            u.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            b.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            stat.get_stat(),
            &mut info,
        );

	simple_result_from_vectors(info, a.num_cols(), b, perm_c, perm_r, l, u,)
    }
}
