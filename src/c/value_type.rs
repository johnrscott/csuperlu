//! Contains a trait with SuperLU functions that depend on precision
//!
//! Contains a trait for supported numerical value types in the
//! C SuperLU library. The supported values types are float (f32),
//! double (f64), complex float (num::Complex<f32>), and complex
//! double (num::Complex<f64>).
//!

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
    non_zero_vals: &Vec<T>,
    row_indices: &Vec<i32>,
    col_offsets: &Vec<i32>,
) -> Result<(), Error> {
    if col_offsets.len() == 0 {
        return Err(Error::CompColError);
    }
    if non_zero_vals.len() != row_indices.len() {
        return Err(Error::CompColError);
    }
    let num_non_zeros = *col_offsets.last().unwrap();
    if row_indices.len() != num_non_zeros.try_into().unwrap() {
        return Err(Error::CompColError);
    }
    Ok(())
}

/// Check necessary conditions for creating a dense matrix
///
/// # Error
///
/// If the length of col_offsets is not at least 1,
/// an error is returned. If the lengths of row_indices and
/// non_zeros_indices do not match, an error is returned.
///
fn check_dense_conditions<T>(
    num_rows: usize,
    num_cols: usize,
    column_major_values: &Vec<T>,
) -> Result<(), Error> {
    if column_major_values.len() != num_rows * num_cols {
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

/// Solution from the simple driver
pub struct CSimpleSolution {
    x: CSuperMatrix,
    perm_c: Vec<i32>,
    perm_r: Vec<i32>,
    l: CSuperMatrix,
    u: CSuperMatrix,    
}

pub enum CSimpleError {
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
fn simple_result_from_vectors(
    info: i32,
    num_cols_a: usize,
    x: CSuperMatrix,
    perm_c: Vec<i32>,
    perm_r: Vec<i32>,
    l: CSuperMatrix,
    u: CSuperMatrix,
) -> Result<CSimpleSolution, CSimpleError> {
    if info < 0 {
	// Check for invalid (negative) info
	Err(CSimpleError::Err(Error::UnknownError))
    } else if info == 0 {
	// Success -- system solved
	Ok(CSimpleSolution {
	    x,
	    perm_c,
	    perm_r,
	    l,
	    u,
	})
    } else if info as usize <= num_cols_a {
	// A is singular, factorisation successful
	Err(CSimpleError::SingularFact {
	    singular_column: info as usize - 1,
	    perm_c,
	    perm_r,
	    l,
	    u,
	})
    } else {
	// Failed due to singular A -- factorisation complete
	let mem_alloc_at_failure = info as usize - num_cols_a;
	Err(CSimpleError::Err(Error::OutOfMemory { mem_alloc_at_failure  }))
    }
}

// Valid numerical value types for the C SuperLU library
///
pub trait ValueType: Num + Copy + FromStr + std::fmt::Debug {
    
    /// Create a compressed-column matrix from raw vectors
    ///
    /// # Errors
    ///
    /// If the length of col_offsets is not equal to num_cols \+ 1
    /// then an error variant is returned. If the lengths of
    /// non_zero_vals and row_indices are not the same, an error is
    /// returned. The last element of col_offsets must be equal to the
    /// length of non_zero_vals, else error is returned. Other ways to
    /// pass invalid arguments are described in the safety section below.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the
    /// vectors passed to the function (the non-zero values,
    /// row indices, and columns pointers) must be a valid representation
    /// of a sparse matrix in compressed-column format. For example,
    /// no numbers in the row_indices or col_offsets can be out of range
    /// (all values in col_offsets must be valid indexes into row_indices,
    /// apart from col_offsets\[last\]; and all values in row_indices must
    /// be < num_rows).
    ///
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_vals: &Vec<Self>,
        row_indices: &Vec<i32>,
        col_offsets: &Vec<i32>,
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
    /// num_rows * num_cols, an error variant is returned.
    ///    
    fn c_create_dense_matrix(
        num_rows: usize,
        num_cols: usize,
        column_major_values: &Vec<Self>,
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
    ) -> Result<CSimpleSolution, CSimpleError>;
}

impl ValueType for f32 {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_vals: &Vec<f32>,
        row_indices: &Vec<i32>,
        col_offsets: &Vec<i32>,
    ) -> Result<CSuperMatrix, Error> {
        check_comp_col_conditions(non_zero_vals, row_indices, col_offsets)?;
        let a = CSuperMatrix::alloc();
        sCreate_CompCol_Matrix(
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (col_offsets.len() - 1) as i32,
            non_zero_vals.len() as i32,
            non_zero_vals.as_ptr() as *mut Self,
            row_indices.as_ptr() as *mut i32,
            col_offsets.as_ptr() as *mut i32,
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
        num_cols: usize,
        column_major_values: &Vec<f32>,
    ) -> Result<CSuperMatrix, Error> {
        check_dense_conditions(num_rows, num_cols, column_major_values)?;
        unsafe {
            let x = CSuperMatrix::alloc();
            sCreate_Dense_Matrix(
                x.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
                num_rows as i32,
                num_cols as i32,
                column_major_values.as_ptr() as *mut Self,
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
    ) -> Result<CSimpleSolution, CSimpleError> {
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

impl ValueType for f64 {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_vals: &Vec<f64>,
        row_indices: &Vec<i32>,
        col_offsets: &Vec<i32>,
    ) -> Result<CSuperMatrix, Error> {
        check_comp_col_conditions(non_zero_vals, row_indices, col_offsets)?;
        let a = CSuperMatrix::alloc();
        dCreate_CompCol_Matrix(
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (col_offsets.len() - 1) as i32,
            non_zero_vals.len() as i32,
            non_zero_vals.as_ptr() as *mut Self,
            row_indices.as_ptr() as *mut i32,
            col_offsets.as_ptr() as *mut i32,
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
        num_cols: usize,
        column_major_values: &Vec<f64>,
    ) -> Result<CSuperMatrix, Error> {
        check_dense_conditions(num_rows, num_cols, column_major_values)?;
        unsafe {
            let x = CSuperMatrix::alloc();
            dCreate_Dense_Matrix(
                x.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
                num_rows as i32,
                num_cols as i32,
                column_major_values.as_ptr() as *mut Self,
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
    ) -> Result<CSimpleSolution, CSimpleError> {

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

impl ValueType for num::Complex<f32> {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_vals: &Vec<num::Complex<f32>>,
        row_indices: &Vec<i32>,
        col_offsets: &Vec<i32>,
    ) -> Result<CSuperMatrix, Error> {
        check_comp_col_conditions(non_zero_vals, row_indices, col_offsets)?;
        let a = CSuperMatrix::alloc();
        cCreate_CompCol_Matrix(
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (col_offsets.len() - 1) as i32,
            non_zero_vals.len() as i32,
	    non_zero_vals.as_ptr() as *mut complex,
            row_indices.as_ptr() as *mut i32,
            col_offsets.as_ptr() as *mut i32,
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
        num_cols: usize,
        column_major_values: &Vec<num::Complex<f32>>,
    ) -> Result<CSuperMatrix, Error> {
        check_dense_conditions(num_rows, num_cols, column_major_values)?;
        unsafe {
            let x = CSuperMatrix::alloc();
            cCreate_Dense_Matrix(
                x.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
                num_rows as i32,
                num_cols as i32,
                column_major_values.as_ptr() as *mut complex,
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
    ) -> Result<CSimpleSolution, CSimpleError> {
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

impl ValueType for num::Complex<f64> {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_vals: &Vec<num::Complex<f64>>,
        row_indices: &Vec<i32>,
        col_offsets: &Vec<i32>,
    ) -> Result<CSuperMatrix, Error> {
        check_comp_col_conditions(non_zero_vals, row_indices, col_offsets)?;
        let a = CSuperMatrix::alloc();
        zCreate_CompCol_Matrix(
            a.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (col_offsets.len() - 1) as i32,
            non_zero_vals.len() as i32,
	    non_zero_vals.as_ptr() as *mut doublecomplex,
            row_indices.as_ptr() as *mut i32,
            col_offsets.as_ptr() as *mut i32,
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
        num_cols: usize,
        column_major_values: &Vec<num::Complex<f64>>,
    ) -> Result<CSuperMatrix, Error> {
        check_dense_conditions(num_rows, num_cols, column_major_values)?;
        unsafe {
            let x = CSuperMatrix::alloc();
            zCreate_Dense_Matrix(
                x.super_matrix() as *const SuperMatrix as *mut SuperMatrix,
                num_rows as i32,
                num_cols as i32,
                column_major_values.as_ptr() as *mut doublecomplex,
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
    ) -> Result<CSimpleSolution, CSimpleError> {
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
