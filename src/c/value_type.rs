//! Contains a trait with SuperLU functions that depend on precision
//!
//! Contains a trait for supported numerical value types in the
//! C SuperLU library. The supported values types are float (f32),
//! double (f64), complex float (num::Complex<f32>), and complex
//! double (num::Complex<f64>).
//!
//! To change:
//! - Consider removing the c_ prefix from these functions (and from
//!   related functions.

use std::{ffi::CString, str::FromStr};

use num::Num;

use csuperlu_sys::{
    cCreate_CompCol_Matrix, cPrint_CompCol_Matrix, dCreate_CompCol_Matrix,
    dPrint_CompCol_Matrix, sCreate_CompCol_Matrix, sPrint_CompCol_Matrix,
    zCreate_CompCol_Matrix, zPrint_CompCol_Matrix,
    cCreate_Dense_Matrix, cPrint_Dense_Matrix, dCreate_Dense_Matrix, dPrint_Dense_Matrix,
    sCreate_Dense_Matrix, sPrint_Dense_Matrix, zCreate_Dense_Matrix, zPrint_Dense_Matrix,
    superlu_options_t, cgssv, dgssv, sgssv, zgssv, SuperMatrix,
    cPrint_SuperNode_Matrix, dPrint_SuperNode_Matrix, sPrint_SuperNode_Matrix,
    zPrint_SuperNode_Matrix, Stype_t_SLU_NC, Dtype_t_SLU_S, complex, doublecomplex,
    Dtype_t_SLU_D, Dtype_t_SLU_Z, Dtype_t_SLU_C, Stype_t_SLU_DN, Mtype_t_SLU_GE,
};

use crate::{c::stat::CSuperluStat, c::super_matrix::CSuperMatrix};

use std::fmt;

#[derive(Debug)]
pub enum Error {
    CompColError,
    DenseMatrixError,
    OutOfMemory { mem_alloc_at_failure: usize },
    UnknownError,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Self::UnknownError => write!(f, "An unknown error occured"),
	    Self::CompColError => write!(f, "An error occured creating a compressed column matrix"),
	    Self::DenseMatrixError => write!(f, "An error occured creating a dense matrix"),
	    Self::OutOfMemory { mem_alloc_at_failure } =>
		write!(f, "Simple driver ran out of memory ({mem_alloc_at_failure} B allocated at failure)"),
	}
    }
}

use super::options::SimpleDriverOptions;

/// Check necessary conditions for creating a compressed
/// column matrix
///
/// # Errors
///
/// As described in documentation for c_create_comp_col_matrix.
///
fn check_comp_col_conditions<T>(
    non_zero_values: &mut Vec<T>,
    row_indices: &mut Vec<i32>,
    column_offsets: &mut Vec<i32>,
) -> Result<(), Error> {
    if column_offsets.len() == 0 {
        return Err(Error::CompColError);
    }
    if non_zero_values.len() != row_indices.len() {
        return Err(Error::CompColError);
    }
    let num_non_zeros = *column_offsets.last().unwrap();
    if row_indices.len() != num_non_zeros.try_into().unwrap() {
        return Err(Error::CompColError);
    }
    Ok(())
}

/// Check necessary conditions for creating a dense matrix
///
/// # Error
///
/// If the length of column_offsets is not at least 1,
/// an error is returned. If the lengths of row_indices and
/// non_zeros_indices do not match, an error is returned.
///
fn check_dense_conditions<T>(
    num_rows: usize,
    num_columns: usize,
    column_major_values: &mut Vec<T>,
) -> Result<(), Error> {
    if column_major_values.len() != num_rows * num_columns {
        return Err(Error::DenseMatrixError);
    }
    Ok(())
}

/// Convert a rust string reference to a C string
fn c_string(string: &str) -> CString {
    std::ffi::CString::new(string).unwrap()
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

/// The items returned by the c_simple_driver functions
///
pub enum CSimpleResult {
    /// The solution completed successfully (A was not
    /// singular, and no memory errors occured)
    Solution {
	x: CSuperMatrix,
	perm_c: Vec<i32>,
	perm_r: Vec<i32>,
	l: CSuperMatrix,
	u: CSuperMatrix,
    },
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

impl CSimpleResult {

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
    fn from_vectors(
	info: i32,
	num_cols_a: usize,
	x: CSuperMatrix,
	perm_c: Vec<i32>,
	perm_r: Vec<i32>,
	l: CSuperMatrix,
	u: CSuperMatrix,
    ) -> Self {
	if info < 0 {
	    // Check for invalid (negative) info
	    Self::Err(Error::UnknownError)
	} else if info == 0 {
	    // Success -- system solved
	    Self::Solution {
		x,
		perm_c,
		perm_r,
		l,
		u,
	    }
	} else if info as usize <= num_cols_a {
	    // A is singular, factorisation successful
	    Self::SingularFact {
		singular_column: info as usize - 1,
		perm_c,
		perm_r,
		l,
		u,
	    }
	} else {
	    // Failed due to singular A -- factorisation complete
	    let mem_alloc_at_failure = info as usize - num_cols_a;
	    Self::Err(Error::OutOfMemory { mem_alloc_at_failure  })
	}
    }
}

// Valid numerical value types for the C SuperLU library
///
pub trait ValueType<P>: Num + Copy + FromStr + std::fmt::Debug {
    
    /// Create a compressed-column matrix from raw vectors
    ///
    /// # Errors
    ///
    /// If the length of column_offsets is not equal to num_columns \+ 1
    /// then an error variant is returned. If the lengths of
    /// non_zero_values and row_indices are not the same, an error is
    /// returned. The last element of column_offsets must be equal to the
    /// length of non_zero_values, else error is returned. Other ways to
    /// pass invalid arguments are described in the safety section below.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the
    /// vectors passed to the function (the non-zero values,
    /// row indices, and columns pointers) must be a valid representation
    /// of a sparse matrix in compressed-column format. For example,
    /// no numbers in the row_indices or column_offsets can be out of range
    /// (all values in column_offsets must be valid indexes into row_indices,
    /// apart from column_offsets\[last\]; and all values in row_indices must
    /// be < num_rows).
    ///
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<P>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
    ) -> Result<CSuperMatrix, Error>;

    /// Print a compressed-column matrix (using the print
    /// from the SuperLU library)
    ///
    /// The function makes the assumption that the C library does
    /// not modify the arguments.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the matrix (SuperMatrix)
    /// passed as the argument must have been created using the
    /// c_create_comp_col_matrix function. Using other SuperMatrix
    /// items may result in undefined behaviour.
    ///
    unsafe fn c_print_comp_col_matrix(what: &str, a: &CSuperMatrix);

    /// Create a dense matrix from a raw vector
    ///
    /// # Errors
    ///
    /// If the length of column_major_values is not equal to
    /// num_rows * num_columns, an error variant is returned.
    ///    
    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<P>,
    ) -> Result<CSuperMatrix, Error>;

    /// Print a dense matrix (using the print
    /// from the SuperLU library)
    ///
    /// The function makes the assumption that the C library does
    /// not modify the arguments.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the matrix (SuperMatrix)
    /// passed as the argument must have been created using the
    /// c_create_dense_matrix function. Using other SuperMatrix
    /// items may result in undefined behaviour.
    ///
    unsafe fn c_print_dense_matrix(what: &str, a: &CSuperMatrix);

    /// Print a super-nodal matrix (using the print
    /// from the SuperLU library)
    ///
    /// The function makes the assumption that the C library does
    /// not modify the arguments.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the matrix (SuperMatrix)
    /// passed as the argument must be a super-nodal matrix (i.e.
    /// the L returned by the solver). Using other SuperMatrix
    /// items may result in undefined behaviour.
    ///
    unsafe fn c_print_super_node_matrix(what: &str, a: &CSuperMatrix);

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
    unsafe fn c_simple_driver(
        options: SimpleDriverOptions,
        a: &CSuperMatrix,
        perm_c: Option<Vec<i32>>,
        b: CSuperMatrix,
        stat: &mut CSuperluStat,
    ) -> CSimpleResult;
}

impl ValueType<f32> for f32 {
 
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<f32>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
    ) -> Result<CSuperMatrix, Error> {
        check_comp_col_conditions(non_zero_values, row_indices, column_offsets)?;
        let a = CSuperMatrix::alloc();
        sCreate_CompCol_Matrix(
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (column_offsets.len() - 1) as i32,
            non_zero_values.len() as i32,
            non_zero_values.as_mut_ptr(),
            row_indices.as_mut_ptr(),
            column_offsets.as_mut_ptr(),
            Stype_t_SLU_NC,
            Dtype_t_SLU_S,
            Mtype_t_SLU_GE,
        );
        Ok(a)
    }

    unsafe fn c_print_comp_col_matrix(what: &str, a: &CSuperMatrix) {
        sPrint_CompCol_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<f32>,
    ) -> Result<CSuperMatrix, Error> {
        check_dense_conditions(num_rows, num_columns, column_major_values)?;
        unsafe {
            let x = CSuperMatrix::alloc();
            sCreate_Dense_Matrix(
                x.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
                num_rows as i32,
                num_columns as i32,
                column_major_values.as_mut_ptr(),
                num_rows as i32,
                Stype_t_SLU_DN,
                Dtype_t_SLU_S,
                Mtype_t_SLU_GE,
            );
            Ok(x)
        }
    }

    unsafe fn c_print_dense_matrix(what: &str, a: &CSuperMatrix) {
        sPrint_Dense_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a.super_matrix()  as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_print_super_node_matrix(what: &str, a: &CSuperMatrix) {
        sPrint_SuperNode_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_simple_driver(
        options: SimpleDriverOptions,
        a: &CSuperMatrix,
        perm_c: Option<Vec<i32>>,
        b: CSuperMatrix,
        stat: &mut CSuperluStat,
    ) -> CSimpleResult {
	let mut info = 0i32;
	let l = CSuperMatrix::alloc();
        let u = CSuperMatrix::alloc();
	let (mut perm_c, mut perm_r, options)
	    = make_simple_perms(a.num_columns(), perm_c, options);
		
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

	CSimpleResult::from_vectors(info, a.num_columns(), b, perm_c, perm_r, l, u,)
    }
}

impl ValueType<f64> for f64 {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<f64>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
    ) -> Result<CSuperMatrix, Error> {
        check_comp_col_conditions(non_zero_values, row_indices, column_offsets)?;
        let a = CSuperMatrix::alloc();
        dCreate_CompCol_Matrix(
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (column_offsets.len() - 1) as i32,
            non_zero_values.len() as i32,
            non_zero_values.as_mut_ptr(),
            row_indices.as_mut_ptr(),
            column_offsets.as_mut_ptr(),
            Stype_t_SLU_NC,
            Dtype_t_SLU_D,
            Mtype_t_SLU_GE,
        );
        Ok(a)
    }

    unsafe fn c_print_comp_col_matrix(what: &str, a: &CSuperMatrix) {
        dPrint_CompCol_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<f64>,
    ) -> Result<CSuperMatrix, Error> {
        check_dense_conditions(num_rows, num_columns, column_major_values)?;
        unsafe {
            let x = CSuperMatrix::alloc();
            dCreate_Dense_Matrix(
                x.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
                num_rows as i32,
                num_columns as i32,
                column_major_values.as_mut_ptr(),
                num_rows as i32,
                Stype_t_SLU_DN,
                Dtype_t_SLU_D,
		Mtype_t_SLU_GE,
            );
            Ok(x)
        }
    }

    unsafe fn c_print_dense_matrix(what: &str, a: &CSuperMatrix) {
        dPrint_Dense_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a.super_matrix()  as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_print_super_node_matrix(what: &str, a: &CSuperMatrix) {
        dPrint_SuperNode_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_simple_driver(
        options: SimpleDriverOptions,
        a: &CSuperMatrix,
        perm_c: Option<Vec<i32>>,
        b: CSuperMatrix,
        stat: &mut CSuperluStat,
    ) -> CSimpleResult {

	let mut info = 0i32;
	let l = CSuperMatrix::alloc();
        let u = CSuperMatrix::alloc();
	let (mut perm_c, mut perm_r, options)
	    = make_simple_perms(a.num_columns(), perm_c, options);

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

	CSimpleResult::from_vectors(info, a.num_columns(), b, perm_c, perm_r, l, u,)
    }
}

impl ValueType<num::Complex<f32>> for num::Complex<f32> {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<num::Complex<f32>>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
    ) -> Result<CSuperMatrix, Error> {
        check_comp_col_conditions(non_zero_values, row_indices, column_offsets)?;
        let a = CSuperMatrix::alloc();
        cCreate_CompCol_Matrix(
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (column_offsets.len() - 1) as i32,
            non_zero_values.len() as i32,
            non_zero_values.as_mut_ptr() as *mut complex,
            row_indices.as_mut_ptr(),
            column_offsets.as_mut_ptr(),
            Stype_t_SLU_NC,
            Dtype_t_SLU_C,
            Mtype_t_SLU_GE,
        );
        Ok(a)
    }

    unsafe fn c_print_comp_col_matrix(what: &str, a: &CSuperMatrix) {
        cPrint_CompCol_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<num::Complex<f32>>,
    ) -> Result<CSuperMatrix, Error> {
        check_dense_conditions(num_rows, num_columns, column_major_values)?;
        unsafe {
            let x = CSuperMatrix::alloc();
            cCreate_Dense_Matrix(
                x.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
                num_rows as i32,
                num_columns as i32,
                column_major_values.as_mut_ptr() as *mut complex,
                num_rows as i32,
                Stype_t_SLU_DN,
                Dtype_t_SLU_C,
                Mtype_t_SLU_GE,
            );
            Ok(x)
        }
    }

    unsafe fn c_print_dense_matrix(what: &str, a: &CSuperMatrix) {
        cPrint_Dense_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a.super_matrix()  as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_print_super_node_matrix(what: &str, a: &CSuperMatrix) {
        cPrint_SuperNode_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_simple_driver(
        options: SimpleDriverOptions,
        a: &CSuperMatrix,
        perm_c: Option<Vec<i32>>,
        b: CSuperMatrix,
        stat: &mut CSuperluStat,
    ) -> CSimpleResult {
	let mut info = 0i32;
	let l = CSuperMatrix::alloc();
        let u = CSuperMatrix::alloc();
	let (mut perm_c, mut perm_r, options)
	    = make_simple_perms(a.num_columns(), perm_c, options);

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

	CSimpleResult::from_vectors(info, a.num_columns(), b, perm_c, perm_r, l, u,)
    }
}

impl ValueType<num::Complex<f64>> for num::Complex<f64> {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<num::Complex<f64>>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
    ) -> Result<CSuperMatrix, Error> {
        check_comp_col_conditions(non_zero_values, row_indices, column_offsets)?;
        let a = CSuperMatrix::alloc();
        zCreate_CompCol_Matrix(
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (column_offsets.len() - 1) as i32,
            non_zero_values.len() as i32,
            non_zero_values.as_mut_ptr() as *mut doublecomplex,
            row_indices.as_mut_ptr(),
            column_offsets.as_mut_ptr(),
            Stype_t_SLU_NC,
            Dtype_t_SLU_Z,
            Mtype_t_SLU_GE,
        );
        Ok(a)
    }

    unsafe fn c_print_comp_col_matrix(what: &str, a: &CSuperMatrix) {
        zPrint_CompCol_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<num::Complex<f64>>,
    ) -> Result<CSuperMatrix, Error> {
        check_dense_conditions(num_rows, num_columns, column_major_values)?;
        unsafe {
            let x = CSuperMatrix::alloc();
            zCreate_Dense_Matrix(
                x.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
                num_rows as i32,
                num_columns as i32,
                column_major_values.as_mut_ptr() as *mut doublecomplex,
                num_rows as i32,
                Stype_t_SLU_DN,
                Dtype_t_SLU_Z,
                Mtype_t_SLU_GE,
            );
            Ok(x)
        }
    }

    unsafe fn c_print_dense_matrix(what: &str, a: &CSuperMatrix) {
        zPrint_Dense_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a.super_matrix()  as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_print_super_node_matrix(what: &str, a: &CSuperMatrix) {
        zPrint_SuperNode_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_simple_driver(
        options: SimpleDriverOptions,
        a: &CSuperMatrix,
        perm_c: Option<Vec<i32>>,
        b: CSuperMatrix,
        stat: &mut CSuperluStat,
    ) -> CSimpleResult {
	let mut info = 0i32;
	let l = CSuperMatrix::alloc();
        let u = CSuperMatrix::alloc();
	let (mut perm_c, mut perm_r, options)
	    = make_simple_perms(a.num_columns(), perm_c, options);

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

	CSimpleResult::from_vectors(info, a.num_columns(), b, perm_c, perm_r, l, u,)
    }
}
