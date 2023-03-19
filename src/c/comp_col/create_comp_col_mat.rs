//! Low-level creation of compressed-column matrices

use csuperlu_sys::{sCreate_CompCol_Matrix, SuperMatrix, dCreate_CompCol_Matrix, Stype_t_SLU_NC, Dtype_t_SLU_S, Mtype_t_SLU_GE, Dtype_t_SLU_D, cCreate_CompCol_Matrix, complex, Dtype_t_SLU_C, zCreate_CompCol_Matrix, doublecomplex, Dtype_t_SLU_Z};

use crate::c::{error::Error, super_matrix::CSuperMatrix};

/// Check necessary conditions for creating a compressed
/// column matrix
///
/// # Errors
///
/// As described in documentation for create_comp_col_matrix.
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

pub trait CreateCompColMat: Sized {
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
    unsafe fn create_comp_col_matrix(
        num_rows: usize,
        non_zero_vals: &Vec<Self>,
        row_indices: &Vec<i32>,
        col_offsets: &Vec<i32>,
    ) -> Result<CSuperMatrix, Error>;
}

impl CreateCompColMat for f32 {
    unsafe fn create_comp_col_matrix(
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
}

impl CreateCompColMat for f64 {
    unsafe fn create_comp_col_matrix(
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
}

impl CreateCompColMat for num::Complex<f32> {
    unsafe fn create_comp_col_matrix(
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
}

impl CreateCompColMat for num::Complex<f64> {
    unsafe fn create_comp_col_matrix(
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
}
