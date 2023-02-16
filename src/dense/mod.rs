//! Functions to create dense matrices.
//!

use crate::c::dense::c_Destroy_Dense_Matrix;
use crate::c::dense::CDenseMatrix;
use crate::c::super_matrix::c_DNformat;
use crate::c::super_matrix::{c_SuperMatrix, Mtype_t};
use crate::super_matrix::SuperMatrix;
use std::mem::MaybeUninit;

pub struct DenseMatrix<P: CDenseMatrix> {
    c_super_matrix: c_SuperMatrix,
    marker: std::marker::PhantomData<P>,
}

impl<P: CDenseMatrix> DenseMatrix<P> {
    /// Specify a dense matrix from an input vector.
    ///
    /// Use this function to make a dense c_SuperMatrix. The vector
    /// which stores the values in the matrix is passed in as a
    /// mutable reference, because this storage is overwritten by
    /// the solver when the dense matrix is used as the right-hand
    /// side matrix.
    ///
    pub fn from_vectors(m: i32, n: i32, mut x: Vec<P>, mtype: Mtype_t) -> Self {
        let c_super_matrix = unsafe {
            let mut c_super_matrix = MaybeUninit::<c_SuperMatrix>::uninit();
            P::c_create_dense_matrix(&mut c_super_matrix, m, n, &mut x, m, mtype);
            c_super_matrix.assume_init()
        };
        std::mem::forget(x);
        Self {
            c_super_matrix,
            marker: std::marker::PhantomData,
        }
    }
    // pub fn values(&mut self) -> &mut Vec<P> {
    //     unsafe {
    //         let c_dnformat = &mut *(self.c_super_matrix.Store as *mut c_DNformat);
    //         &mut *(c_dnformat.nzval as *mut Vec<P>)
    //     }
    // }
    pub fn values(&mut self) -> &[P] {
        unsafe {
            let c_dnformat = &mut *(self.c_super_matrix.Store as *mut c_DNformat);
	    let size = self.c_super_matrix.nrow * self.c_super_matrix.ncol;
            std::slice::from_raw_parts(c_dnformat.nzval as *mut P, size as usize) 
        }
    }

}

impl<P: CDenseMatrix> SuperMatrix for DenseMatrix<P> {
    fn super_matrix<'a>(&'a mut self) -> &'a mut c_SuperMatrix {
        &mut self.c_super_matrix
    }
    fn print(&mut self, what: &str) {
        let c_str = std::ffi::CString::new(what).unwrap();
        P::c_print_dense_matrix(c_str.as_ptr() as *mut libc::c_char, self.super_matrix());
    }
}

impl<P: CDenseMatrix> Drop for DenseMatrix<P> {
    fn drop(&mut self) {
        // Note that the input vectors are not freed by this line
        c_Destroy_Dense_Matrix(&mut self.c_super_matrix);
    }
}
