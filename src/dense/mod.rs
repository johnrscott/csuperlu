//! Functions to create dense matrices.
//!

use std::mem::MaybeUninit;
use crate::c::utils::{
    SuperMatrix,
    Stype_t,
    Mtype_t,
    Dtype_t,
};
use crate::c::dense::c_dCreate_Dense_Matrix;

/// Specify a compressed column matrix from input vectors
///
/// Use this function to make a SuperMatrix in compressed column
/// format, from the vector of values, row indices, and column
/// offsets. Compressed column format is documented in Section
/// 2.3 of the SuperLU manual.
///
#[allow(non_snake_case)]
pub fn dCreate_Dense_Matrix(m: i32,
			    n: i32,
			    x: &mut Vec<f64>,
			    ldx: i32,
			    stype: Stype_t,
			    dtype: Dtype_t,
			    mtype: Mtype_t) -> SuperMatrix {
    unsafe {
	let mut A = MaybeUninit::<SuperMatrix>::uninit();
	c_dCreate_Dense_Matrix(A.as_mut_ptr(), m, n, x.as_mut_ptr(), ldx,
			       stype, dtype, mtype);	
	A.assume_init()
    }
}
