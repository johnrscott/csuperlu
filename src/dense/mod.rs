//! Functions to create dense matrices.
//!

use crate::c::dense::c_Destroy_SuperMatrix_Store;
use crate::c::dense::{c_dCreate_Dense_Matrix, c_dPrint_Dense_Matrix};
use crate::c::super_matrix::{c_SuperMatrix, Dtype_t, Mtype_t, Stype_t};
use crate::super_matrix::SuperMatrix;
use std::mem::MaybeUninit;

pub struct DenseMatrix {
    c_super_matrix: c_SuperMatrix,
}

impl DenseMatrix {
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
    pub fn new(
        m: i32,
        n: i32,
        x: &mut Vec<f64>,
        ldx: i32,
        stype: Stype_t,
        dtype: Dtype_t,
        mtype: Mtype_t,
    ) -> Self {
        let c_super_matrix = unsafe {
            let mut c_super_matrix = MaybeUninit::<c_SuperMatrix>::uninit();
            c_dCreate_Dense_Matrix(
                c_super_matrix.as_mut_ptr(),
                m,
                n,
                x.as_mut_ptr(),
                ldx,
                stype,
                dtype,
                mtype,
            );
            c_super_matrix.assume_init()
        };

        Self { c_super_matrix }
    }
}

impl SuperMatrix for DenseMatrix {
    fn super_matrix<'a>(&'a mut self) -> &'a mut c_SuperMatrix {
        &mut self.c_super_matrix
    }
    fn print(&mut self, what: &str) {
	let c_str = std::ffi::CString::new(what).unwrap();
	c_dPrint_Dense_Matrix(c_str.as_ptr() as *mut libc::c_char,
				self.super_matrix());
    }
}

impl Drop for DenseMatrix {
    fn drop(&mut self) {
        c_Destroy_SuperMatrix_Store(&mut self.c_super_matrix);
    }
}
