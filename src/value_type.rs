//! Contains a trait for supported numerical value types in the
//! C SuperLU library. The supported values types are float (f32),
//! double (f64), complex float (num::Complex<f32>), and complex
//! double (num::Complex<f64>). 

use std::{str::FromStr, ffi::CString};

use num::Num;

use csuperlu_sys::{super_matrix::{c_SuperMatrix, Mtype_t, Stype_t, Dtype_t}, options::superlu_options_t, stat::SuperLUStat_t, comp_col::{sCreate_CompCol_Matrix, sPrint_CompCol_Matrix, dCreate_CompCol_Matrix, dPrint_CompCol_Matrix, cCreate_CompCol_Matrix, cPrint_CompCol_Matrix, zCreate_CompCol_Matrix, zPrint_CompCol_Matrix}, dense::{sCreate_Dense_Matrix, sPrint_Dense_Matrix, dCreate_Dense_Matrix, dPrint_Dense_Matrix, cCreate_Dense_Matrix, cPrint_Dense_Matrix, zCreate_Dense_Matrix, zPrint_Dense_Matrix}, super_node::{sPrint_SuperNode_Matrix, dPrint_SuperNode_Matrix, cPrint_SuperNode_Matrix, zPrint_SuperNode_Matrix}, simple_driver::{sgssv, dgssv, cgssv, zgssv}};

use crate::Error;

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
	return Err(Error{});
    }
    if non_zero_values.len() != row_indices.len() {
	return Err(Error{});
    }
    let num_non_zeros = *column_offsets.last().unwrap();
    if row_indices.len() != num_non_zeros.try_into().unwrap() {
	return Err(Error{});
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
	return Err(Error{});
    }
    Ok(())
}

/// Convert a rust string reference to a C string
fn c_string(string: &str) -> CString {
    std::ffi::CString::new(string).unwrap()
}

/// Valid numerical value types for the C SuperLU library
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
    /// apart from column_offsets[last]; and all values in row_indices must
    /// be < num_rows).
    ///
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<P>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> Result<c_SuperMatrix, Error>;

    /// Print a compressed-column matrix (using the print
    /// from the SuperLU library)
    ///
    /// The function makes the assumption that the C library does
    /// not modify the arguments.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the matrix (c_SuperMatrix)
    /// passed as the argument must have been created using the
    /// c_create_comp_col_matrix function. Using other c_SuperMatrix
    /// items may result in undefined behaviour.
    ///
    unsafe fn c_print_comp_col_matrix(what: &str, a: &c_SuperMatrix);

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
        mtype: Mtype_t,
    ) -> Result<c_SuperMatrix, Error>;

    /// Print a dense matrix (using the print
    /// from the SuperLU library)
    ///
    /// The function makes the assumption that the C library does
    /// not modify the arguments.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the matrix (c_SuperMatrix)
    /// passed as the argument must have been created using the
    /// c_create_dense_matrix function. Using other c_SuperMatrix
    /// items may result in undefined behaviour.
    ///
    unsafe fn c_print_dense_matrix(what: &str, a: &c_SuperMatrix);

    /// Print a super-nodal matrix (using the print
    /// from the SuperLU library)
    ///
    /// The function makes the assumption that the C library does
    /// not modify the arguments.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the matrix (c_SuperMatrix)
    /// passed as the argument must be a super-nodal matrix (i.e.
    /// the L returned by the solver). Using other c_SuperMatrix
    /// items may result in undefined behaviour.
    ///
    unsafe fn c_print_super_node_matrix(what: &str, a: &c_SuperMatrix);

    
    /// Solve a sparse linear system using the simple driver
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
    /// allocated structures (c_SuperMatrix::alloc).
    ///
    unsafe fn c_simple_driver(
	options: &mut superlu_options_t,
	a: &c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: &mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    );
}

impl ValueType<f32> for f32 {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<f32>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> Result<c_SuperMatrix, Error> {
	check_comp_col_conditions(
	    non_zero_values,
	    row_indices,
	    column_offsets)?;
	let mut a = c_SuperMatrix::alloc();
        sCreate_CompCol_Matrix(
            &mut a as *mut c_SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (column_offsets.len() - 1) as i32,
            non_zero_values.len() as i32,
            non_zero_values.as_mut_ptr(),
            row_indices.as_mut_ptr(),
            column_offsets.as_mut_ptr(),
            Stype_t::SLU_NC,
            Dtype_t::SLU_S,
            mtype,
        );
	Ok(a)
    }

    unsafe fn c_print_comp_col_matrix(what: &str, a: &c_SuperMatrix) {
        sPrint_CompCol_Matrix(c_string(what).as_ptr() as *mut libc::c_char,
			      a as *const c_SuperMatrix as *mut c_SuperMatrix);
    }
    
    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<f32>,
        mtype: Mtype_t,
    ) -> Result<c_SuperMatrix, Error> {
	check_dense_conditions(
            num_rows,
            num_columns,
            column_major_values)?;
	unsafe {
	    let mut x = c_SuperMatrix::alloc();
            sCreate_Dense_Matrix(
		&mut x as *mut c_SuperMatrix,
		num_rows as i32,
		num_columns as i32,
		column_major_values.as_mut_ptr(),
		num_rows as i32,
		Stype_t::SLU_DN,
		Dtype_t::SLU_S,
		mtype,
            );
	    Ok(x)
	}
    }

    unsafe fn c_print_dense_matrix(what: &str, a: &c_SuperMatrix) {
	sPrint_Dense_Matrix(c_string(what).as_ptr() as *mut libc::c_char,
			    a as *const c_SuperMatrix as *mut c_SuperMatrix);
    }
    
    unsafe fn c_print_super_node_matrix(what: &str, a: &c_SuperMatrix) {
        sPrint_SuperNode_Matrix(c_string(what).as_ptr() as *mut libc::c_char,
				a as *const c_SuperMatrix as *mut c_SuperMatrix);
	
    }
    
    unsafe fn c_simple_driver(
	options: &mut superlu_options_t,
	a: &c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: &mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    ) {
        sgssv(options,
	      a as *const c_SuperMatrix as *mut c_SuperMatrix,
	      perm_c.as_mut_ptr(),
	      perm_r.as_mut_ptr(),
	      l,
	      u,
	      b,
	      stat,
	      info);
    }
}

impl ValueType<f64> for f64 {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<f64>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> Result<c_SuperMatrix, Error> {
	check_comp_col_conditions(
	    non_zero_values,
	    row_indices,
	    column_offsets)?;
	let mut a = c_SuperMatrix::alloc();
        dCreate_CompCol_Matrix(
            &mut a as *mut c_SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (column_offsets.len() - 1) as i32,
            non_zero_values.len() as i32,
            non_zero_values.as_mut_ptr(),
            row_indices.as_mut_ptr(),
            column_offsets.as_mut_ptr(),
            Stype_t::SLU_NC,
            Dtype_t::SLU_D,
            mtype,
        );
	Ok(a)
    }

    unsafe fn c_print_comp_col_matrix(what: &str, a: &c_SuperMatrix) {
        dPrint_CompCol_Matrix(c_string(what).as_ptr() as *mut libc::c_char,
			      a as *const c_SuperMatrix as *mut c_SuperMatrix);

    }
    
    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<f64>,
        mtype: Mtype_t,
    ) -> Result<c_SuperMatrix, Error> {
	check_dense_conditions(
            num_rows,
            num_columns,
            column_major_values)?;
	unsafe {
	    let mut x = c_SuperMatrix::alloc();
            dCreate_Dense_Matrix(
		&mut x as *mut c_SuperMatrix,
		num_rows as i32,
		num_columns as i32,
		column_major_values.as_mut_ptr(),
		num_rows as i32,
		Stype_t::SLU_DN,
		Dtype_t::SLU_D,
		mtype,
            );
	    Ok(x)
	}
    }

    unsafe fn c_print_dense_matrix(what: &str, a: &c_SuperMatrix) {
	dPrint_Dense_Matrix(c_string(what).as_ptr() as *mut libc::c_char,
			    a as *const c_SuperMatrix as *mut c_SuperMatrix);
    }

    unsafe fn c_print_super_node_matrix(what: &str, a: &c_SuperMatrix) {
	dPrint_SuperNode_Matrix(c_string(what).as_ptr() as *mut libc::c_char,
				a as *const c_SuperMatrix as *mut c_SuperMatrix);
    }
    
    unsafe fn c_simple_driver(
	options: &mut superlu_options_t,
	a: &c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: &mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    ) {
        dgssv(options,
	      a as *const c_SuperMatrix as *mut c_SuperMatrix,
	      perm_c.as_mut_ptr(),
	      perm_r.as_mut_ptr(),
	      l,
	      u,
	      b,
	      stat,
	      info);
    }

}

impl ValueType<num::Complex<f32>> for num::Complex<f32> {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<num::Complex<f32>>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> Result<c_SuperMatrix, Error> {
	check_comp_col_conditions(
	    non_zero_values,
	    row_indices,
	    column_offsets)?;
	let mut a = c_SuperMatrix::alloc();
        cCreate_CompCol_Matrix(
            &mut a as *mut c_SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (column_offsets.len() - 1) as i32,
            non_zero_values.len() as i32,
            non_zero_values.as_mut_ptr() as *mut libc::c_float,
            row_indices.as_mut_ptr(),
            column_offsets.as_mut_ptr(),
            Stype_t::SLU_NC,
            Dtype_t::SLU_C,
            mtype,
        );
	Ok(a)
    }

    unsafe fn c_print_comp_col_matrix(what: &str, a: &c_SuperMatrix) {
	cPrint_CompCol_Matrix(c_string(what).as_ptr() as *mut libc::c_char,
			      a as *const c_SuperMatrix as *mut c_SuperMatrix);
    }

    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<num::Complex<f32>>,
        mtype: Mtype_t,
    ) -> Result<c_SuperMatrix, Error> {
	check_dense_conditions(
            num_rows,
            num_columns,
            column_major_values)?;
	unsafe {
	    let mut x = c_SuperMatrix::alloc();
            cCreate_Dense_Matrix(
		&mut x as *mut c_SuperMatrix,
		num_rows as i32,
		num_columns as i32,
		column_major_values.as_mut_ptr() as *mut libc::c_float,
		num_rows as i32,
		Stype_t::SLU_DN,
		Dtype_t::SLU_C,
		mtype,
            );
	    Ok(x)
	}
    }
    
    unsafe fn c_print_dense_matrix(what: &str, a: &c_SuperMatrix) {
	cPrint_Dense_Matrix(c_string(what).as_ptr() as *mut libc::c_char,
			    a as *const c_SuperMatrix as *mut c_SuperMatrix);
    }
    unsafe fn c_print_super_node_matrix(what: &str, a: &c_SuperMatrix) {
	cPrint_SuperNode_Matrix(c_string(what).as_ptr() as *mut libc::c_char,
				a as *const c_SuperMatrix as *mut c_SuperMatrix);
    }
    unsafe fn c_simple_driver(
	options: &mut superlu_options_t,
	a: &c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: &mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    ) {
        cgssv(options,
	      a as *const c_SuperMatrix as *mut c_SuperMatrix,
	      perm_c.as_mut_ptr(),
	      perm_r.as_mut_ptr(),
	      l,
	      u,
	      b,
	      stat,
	      info);
    }
}

impl ValueType<num::Complex<f64>> for num::Complex<f64> {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<num::Complex<f64>>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> Result<c_SuperMatrix, Error> {
	check_comp_col_conditions(
	    non_zero_values,
	    row_indices,
	    column_offsets)?;
	let mut a = c_SuperMatrix::alloc();
        zCreate_CompCol_Matrix(
            &mut a as *mut c_SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (column_offsets.len() - 1) as i32,
            non_zero_values.len() as i32,
            non_zero_values.as_mut_ptr() as *mut libc::c_double,
            row_indices.as_mut_ptr(),
            column_offsets.as_mut_ptr(),
            Stype_t::SLU_NC,
            Dtype_t::SLU_Z,
            mtype,
        );
	Ok(a)
    }

    unsafe fn c_print_comp_col_matrix(what: &str, a: &c_SuperMatrix) {
	zPrint_CompCol_Matrix(c_string(what).as_ptr() as *mut libc::c_char,
			      a as *const c_SuperMatrix as *mut c_SuperMatrix);
    }

    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<num::Complex<f64>>,
        mtype: Mtype_t,
    ) -> Result<c_SuperMatrix, Error> {
        check_dense_conditions(
            num_rows,
            num_columns,
            column_major_values)?;
	unsafe {
	    let mut x = c_SuperMatrix::alloc();
            zCreate_Dense_Matrix(
		&mut x as *mut c_SuperMatrix,
		num_rows as i32,
		num_columns as i32,
		column_major_values.as_mut_ptr() as *mut libc::c_double,
		num_rows as i32,
		Stype_t::SLU_DN,
		Dtype_t::SLU_Z,
		mtype,
            );
	    Ok(x)
	}
    }
    unsafe fn c_print_dense_matrix(what: &str, a: &c_SuperMatrix) {
	zPrint_Dense_Matrix(c_string(what).as_ptr() as *mut libc::c_char,
			    a as *const c_SuperMatrix as *mut c_SuperMatrix);
    }

    unsafe fn c_print_super_node_matrix(what: &str, a: &c_SuperMatrix) {
	zPrint_SuperNode_Matrix(c_string(what).as_ptr() as *mut libc::c_char,
				a as *const c_SuperMatrix as *mut c_SuperMatrix);
	
    }

    unsafe fn c_simple_driver(
	options: &mut superlu_options_t,
	a: &c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: &mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    ) {
        zgssv(options,
	      a as *const c_SuperMatrix as *mut c_SuperMatrix,
	      perm_c.as_mut_ptr(),
	      perm_r.as_mut_ptr(),
	      l,
	      u,
	      b,
	      stat,
	      info);
    }
}
