use csuperlu_sys::{zCreate_Dense_Matrix, SuperMatrix, doublecomplex, Stype_t_SLU_DN, Dtype_t_SLU_Z, Mtype_t_SLU_GE, Dtype_t_SLU_C, complex, cCreate_Dense_Matrix, dCreate_Dense_Matrix, Dtype_t_SLU_D, Dtype_t_SLU_S, sCreate_Dense_Matrix};

use crate::c::{error::Error, super_matrix::CSuperMatrix};

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


pub trait CCreateDenseMat: Sized {
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
}

impl CCreateDenseMat for f32 {
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
}

impl CCreateDenseMat for f64 {
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
}

impl CCreateDenseMat for num::Complex<f32> {
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
}

impl CCreateDenseMat for num::Complex<f64> {
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
}

