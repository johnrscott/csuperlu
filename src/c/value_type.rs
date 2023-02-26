//! Contains a trait for supported numerical value types in the
//! C SuperLU library. The supported values types are float (f32),
//! double (f64), complex float (num::Complex<f32>), and complex
//! double (num::Complex<f64>).

use std::{ffi::CString, str::FromStr, mem::MaybeUninit};

use num::Num;

use csuperlu_sys::{
    cCreate_CompCol_Matrix, cPrint_CompCol_Matrix, dCreate_CompCol_Matrix,
    dPrint_CompCol_Matrix, sCreate_CompCol_Matrix, sPrint_CompCol_Matrix,
    zCreate_CompCol_Matrix, zPrint_CompCol_Matrix,
    cCreate_Dense_Matrix, cPrint_Dense_Matrix, dCreate_Dense_Matrix, dPrint_Dense_Matrix,
    sCreate_Dense_Matrix, sPrint_Dense_Matrix, zCreate_Dense_Matrix, zPrint_Dense_Matrix,
    superlu_options_t, cgssv, dgssv, sgssv, zgssv, SuperMatrix,
    cPrint_SuperNode_Matrix, dPrint_SuperNode_Matrix, sPrint_SuperNode_Matrix,
    zPrint_SuperNode_Matrix, Stype_t_SLU_NC, Dtype_t_SLU_S, Mtype_t, complex, doublecomplex, Dtype_t_SLU_D, Dtype_t_SLU_Z, Dtype_t_SLU_C,
};

use crate::{Error, options::CSuperluOptions, stat::CSuperluStat};

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
        return Err(Error {});
    }
    if non_zero_values.len() != row_indices.len() {
        return Err(Error {});
    }
    let num_non_zeros = *column_offsets.last().unwrap();
    if row_indices.len() != num_non_zeros.try_into().unwrap() {
        return Err(Error {});
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
        return Err(Error {});
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
    /// apart from column_offsets\[last\]; and all values in row_indices must
    /// be < num_rows).
    ///
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<P>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> Result<SuperMatrix, Error>;

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
    unsafe fn c_print_comp_col_matrix(what: &str, a: &SuperMatrix);

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
    ) -> Result<SuperMatrix, Error>;

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
    unsafe fn c_print_dense_matrix(what: &str, a: &SuperMatrix);

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
    unsafe fn c_print_super_node_matrix(what: &str, a: &SuperMatrix);

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
        options: &CSuperluOptions,
        a: &SuperMatrix,
        perm_c: &mut Vec<i32>,
        perm_r: &mut Vec<i32>,
        l: &mut SuperMatrix,
        u: &mut SuperMatrix,
        b: &mut SuperMatrix,
        stat: &mut CSuperluStat,
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
    ) -> Result<SuperMatrix, Error> {
        check_comp_col_conditions(non_zero_values, row_indices, column_offsets)?;
        let mut a = MaybeUninit::<SuperMatrix>::uninit().assume_init();
        sCreate_CompCol_Matrix(
            &mut a as *mut SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (column_offsets.len() - 1) as i32,
            non_zero_values.len() as i32,
            non_zero_values.as_mut_ptr(),
            row_indices.as_mut_ptr(),
            column_offsets.as_mut_ptr(),
            Stype_t_SLU_NC,
            Dtype_t_SLU_S,
            mtype,
        );
        Ok(a)
    }

    unsafe fn c_print_comp_col_matrix(what: &str, a: &SuperMatrix) {
        sPrint_CompCol_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<f32>,
        mtype: Mtype_t,
    ) -> Result<SuperMatrix, Error> {
        check_dense_conditions(num_rows, num_columns, column_major_values)?;
        unsafe {
	    let mut x = MaybeUninit::<SuperMatrix>::uninit().assume_init();
            sCreate_Dense_Matrix(
                &mut x as *mut SuperMatrix,
                num_rows as i32,
                num_columns as i32,
                column_major_values.as_mut_ptr(),
                num_rows as i32,
                Stype_t_SLU_NC,
                Dtype_t_SLU_S,
                mtype,
            );
            Ok(x)
        }
    }

    unsafe fn c_print_dense_matrix(what: &str, a: &SuperMatrix) {
        sPrint_Dense_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_print_super_node_matrix(what: &str, a: &SuperMatrix) {
        sPrint_SuperNode_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_simple_driver(
        options: &CSuperluOptions,
        a: &SuperMatrix,
        perm_c: &mut Vec<i32>,
        perm_r: &mut Vec<i32>,
        l: &mut SuperMatrix,
        u: &mut SuperMatrix,
        b: &mut SuperMatrix,
        stat: &mut CSuperluStat,
        info: &mut i32,
    ) {
        sgssv(
            options.get_options() as *const superlu_options_t as *mut superlu_options_t,
            a as *const SuperMatrix as *mut SuperMatrix,
            perm_c.as_mut_ptr(),
            perm_r.as_mut_ptr(),
            l,
            u,
            b,
            stat.get_stat(),
            info,
        );
    }
}

impl ValueType<f64> for f64 {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<f64>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> Result<SuperMatrix, Error> {
        check_comp_col_conditions(non_zero_values, row_indices, column_offsets)?;
        let mut a = MaybeUninit::<SuperMatrix>::uninit().assume_init();
        dCreate_CompCol_Matrix(
            &mut a as *mut SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (column_offsets.len() - 1) as i32,
            non_zero_values.len() as i32,
            non_zero_values.as_mut_ptr(),
            row_indices.as_mut_ptr(),
            column_offsets.as_mut_ptr(),
            Stype_t_SLU_NC,
            Dtype_t_SLU_D,
            mtype,
        );
        Ok(a)
    }

    unsafe fn c_print_comp_col_matrix(what: &str, a: &SuperMatrix) {
        dPrint_CompCol_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<f64>,
        mtype: Mtype_t,
    ) -> Result<SuperMatrix, Error> {
        check_dense_conditions(num_rows, num_columns, column_major_values)?;
        unsafe {
	    let mut x = MaybeUninit::<SuperMatrix>::uninit().assume_init();
            dCreate_Dense_Matrix(
                &mut x as *mut SuperMatrix,
                num_rows as i32,
                num_columns as i32,
                column_major_values.as_mut_ptr(),
                num_rows as i32,
                Stype_t_SLU_NC,
                Dtype_t_SLU_D,
                mtype,
            );
            Ok(x)
        }
    }

    unsafe fn c_print_dense_matrix(what: &str, a: &SuperMatrix) {
        dPrint_Dense_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_print_super_node_matrix(what: &str, a: &SuperMatrix) {
        dPrint_SuperNode_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_simple_driver(
        options: &CSuperluOptions,
        a: &SuperMatrix,
        perm_c: &mut Vec<i32>,
        perm_r: &mut Vec<i32>,
        l: &mut SuperMatrix,
        u: &mut SuperMatrix,
        b: &mut SuperMatrix,
        stat: &mut CSuperluStat,
        info: &mut i32,
    ) {
        dgssv(
            options.get_options() as *const superlu_options_t as *mut superlu_options_t,
            a as *const SuperMatrix as *mut SuperMatrix,
            perm_c.as_mut_ptr(),
            perm_r.as_mut_ptr(),
            l,
            u,
            b,
            stat.get_stat(),
            info,
        );
    }
}

impl ValueType<num::Complex<f32>> for num::Complex<f32> {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<num::Complex<f32>>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> Result<SuperMatrix, Error> {
        check_comp_col_conditions(non_zero_values, row_indices, column_offsets)?;
        let mut a = MaybeUninit::<SuperMatrix>::uninit().assume_init();
        cCreate_CompCol_Matrix(
            &mut a as *mut SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (column_offsets.len() - 1) as i32,
            non_zero_values.len() as i32,
            non_zero_values.as_mut_ptr() as *mut complex,
            row_indices.as_mut_ptr(),
            column_offsets.as_mut_ptr(),
            Stype_t_SLU_NC,
            Dtype_t_SLU_C,
            mtype,
        );
        Ok(a)
    }

    unsafe fn c_print_comp_col_matrix(what: &str, a: &SuperMatrix) {
        cPrint_CompCol_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<num::Complex<f32>>,
        mtype: Mtype_t,
    ) -> Result<SuperMatrix, Error> {
        check_dense_conditions(num_rows, num_columns, column_major_values)?;
        unsafe {
            let mut x = MaybeUninit::<SuperMatrix>::uninit().assume_init();
            cCreate_Dense_Matrix(
                &mut x as *mut SuperMatrix,
                num_rows as i32,
                num_columns as i32,
                column_major_values.as_mut_ptr() as *mut complex,
                num_rows as i32,
                Stype_t_SLU_NC,
                Dtype_t_SLU_C,
                mtype,
            );
            Ok(x)
        }
    }

    unsafe fn c_print_dense_matrix(what: &str, a: &SuperMatrix) {
        cPrint_Dense_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a as *const SuperMatrix as *mut SuperMatrix,
        );
    }
    unsafe fn c_print_super_node_matrix(what: &str, a: &SuperMatrix) {
        cPrint_SuperNode_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a as *const SuperMatrix as *mut SuperMatrix,
        );
    }
    unsafe fn c_simple_driver(
        options: &CSuperluOptions,
        a: &SuperMatrix,
        perm_c: &mut Vec<i32>,
        perm_r: &mut Vec<i32>,
        l: &mut SuperMatrix,
        u: &mut SuperMatrix,
        b: &mut SuperMatrix,
        stat: &mut CSuperluStat,
        info: &mut i32,
    ) {
        cgssv(
            options.get_options() as *const superlu_options_t as *mut superlu_options_t,
            a as *const SuperMatrix as *mut SuperMatrix,
            perm_c.as_mut_ptr(),
            perm_r.as_mut_ptr(),
            l,
            u,
            b,
            stat.get_stat(),
            info,
        );
    }
}

impl ValueType<num::Complex<f64>> for num::Complex<f64> {
    unsafe fn c_create_comp_col_matrix(
        num_rows: usize,
        non_zero_values: &mut Vec<num::Complex<f64>>,
        row_indices: &mut Vec<i32>,
        column_offsets: &mut Vec<i32>,
        mtype: Mtype_t,
    ) -> Result<SuperMatrix, Error> {
        check_comp_col_conditions(non_zero_values, row_indices, column_offsets)?;
        let mut a = MaybeUninit::<SuperMatrix>::uninit().assume_init();
        zCreate_CompCol_Matrix(
            &mut a as *mut SuperMatrix,
            i32::try_from(num_rows).unwrap(),
            (column_offsets.len() - 1) as i32,
            non_zero_values.len() as i32,
            non_zero_values.as_mut_ptr() as *mut doublecomplex,
            row_indices.as_mut_ptr(),
            column_offsets.as_mut_ptr(),
            Stype_t_SLU_NC,
            Dtype_t_SLU_Z,
            mtype,
        );
        Ok(a)
    }

    unsafe fn c_print_comp_col_matrix(what: &str, a: &SuperMatrix) {
        zPrint_CompCol_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    fn c_create_dense_matrix(
        num_rows: usize,
        num_columns: usize,
        column_major_values: &mut Vec<num::Complex<f64>>,
        mtype: Mtype_t,
    ) -> Result<SuperMatrix, Error> {
        check_dense_conditions(num_rows, num_columns, column_major_values)?;
        unsafe {
            let mut x = MaybeUninit::<SuperMatrix>::uninit().assume_init();
	    zCreate_Dense_Matrix(
                &mut x as *mut SuperMatrix,
                num_rows as i32,
                num_columns as i32,
                column_major_values.as_mut_ptr() as *mut doublecomplex,
                num_rows as i32,
                Stype_t_SLU_NC,
                Dtype_t_SLU_Z,
                mtype,
            );
            Ok(x)
        }
    }
    unsafe fn c_print_dense_matrix(what: &str, a: &SuperMatrix) {
        zPrint_Dense_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_print_super_node_matrix(what: &str, a: &SuperMatrix) {
        zPrint_SuperNode_Matrix(
            c_string(what).as_ptr() as *mut libc::c_char,
            a as *const SuperMatrix as *mut SuperMatrix,
        );
    }

    unsafe fn c_simple_driver(
        options: &CSuperluOptions,
        a: &SuperMatrix,
        perm_c: &mut Vec<i32>,
        perm_r: &mut Vec<i32>,
        l: &mut SuperMatrix,
        u: &mut SuperMatrix,
        b: &mut SuperMatrix,
        stat: &mut CSuperluStat,
        info: &mut i32,
    ) {
        zgssv(
            options.get_options() as *const superlu_options_t as *mut superlu_options_t,
            a as *const SuperMatrix as *mut SuperMatrix,
            perm_c.as_mut_ptr(),
            perm_r.as_mut_ptr(),
            l,
            u,
            b,
            stat.get_stat(),
            info,
        );
    }
}
