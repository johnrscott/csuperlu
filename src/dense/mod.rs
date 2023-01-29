//! Functions to create dense matrices.
//!

use crate::c::dense::c_Destroy_SuperMatrix_Store;
use crate::c::dense::CCreateDenseMatrix;
use crate::c::super_matrix::{c_SuperMatrix, Mtype_t};
use crate::super_matrix::SuperMatrix;
use std::mem::MaybeUninit;

pub struct DenseMatrix<P: CCreateDenseMatrix<P>> {
    pub x: Vec<P>, 
    c_super_matrix: c_SuperMatrix,
}

impl<P: CCreateDenseMatrix<P>> DenseMatrix<P> {
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
        mut x: Vec<P>,
        ldx: i32,
        mtype: Mtype_t,
    ) -> Self {
        let c_super_matrix = unsafe {
            let mut c_super_matrix = MaybeUninit::<c_SuperMatrix>::uninit();
            P::c_create_dense_matrix(
                &mut c_super_matrix,
                m,
                n,
                &mut x,
                ldx,
                mtype,
            );
            c_super_matrix.assume_init()
        };
        Self {
	    x,
	    c_super_matrix
	}
    }
}

impl<P: CCreateDenseMatrix<P>> SuperMatrix for DenseMatrix<P> {
    fn super_matrix<'a>(&'a mut self) -> &'a mut c_SuperMatrix {
        &mut self.c_super_matrix
    }
    fn print(&mut self, what: &str) {
	let c_str = std::ffi::CString::new(what).unwrap();
	P::c_print_dense_matrix(c_str.as_ptr() as *mut libc::c_char,
				self.super_matrix());
    }
}

impl<P: CCreateDenseMatrix<P>> Drop for DenseMatrix<P> {
    fn drop(&mut self) {
	// Note that the input vectors are not freed by this line
        c_Destroy_SuperMatrix_Store(&mut self.c_super_matrix);
    }
}
