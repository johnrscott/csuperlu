//! Functions to create dense matrices.
//!

use crate::c::dense::c_dCreate_Dense_Matrix;
use crate::c::utils::{c_SuperMatrix, Dtype_t, Mtype_t, Stype_t};
use std::mem::MaybeUninit;

/// Specify a dense matrix from an input vector.
///
/// Use this function to make a dense c_SuperMatrix. The vector
/// which stores the values in the matrix is passed in as a
/// mutable reference, because this storage is overwritten by
/// the solver when the dense matrix is used as the right-hand
/// side matrix.
///
/// TODO: check that the ldx parameter is used to specify
/// column- major or row-major order.
///
#[allow(non_snake_case)]
pub fn dCreate_Dense_Matrix(
    m: i32,
    n: i32,
    x: &mut Vec<f64>,
    ldx: i32,
    stype: Stype_t,
    dtype: Dtype_t,
    mtype: Mtype_t,
) -> c_SuperMatrix {
    unsafe {
        let mut A = MaybeUninit::<c_SuperMatrix>::uninit();
        c_dCreate_Dense_Matrix(
            A.as_mut_ptr(),
            m,
            n,
            x.as_mut_ptr(),
            ldx,
            stype,
            dtype,
            mtype,
        );
        A.assume_init()
    }
}
