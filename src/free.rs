//! Functions for freeing memory allocated to superlu structures
//!

use csuperlu_sys::{comp_col::Destroy_CompCol_Matrix, super_matrix::c_SuperMatrix, dense::Destroy_Dense_Matrix};

pub unsafe fn c_destroy_comp_col_matrix(a: &mut c_SuperMatrix) {
    Destroy_CompCol_Matrix (a);
}

pub unsafe fn c_destroy_dense_matrix(a: &mut c_SuperMatrix) {
    Destroy_Dense_Matrix (a);
}
