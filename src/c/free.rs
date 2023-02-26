//! Functions for freeing memory allocated to superlu structures
//!

use csuperlu_sys::{Destroy_CompCol_Matrix,
		   Destroy_Dense_Matrix, Destroy_SuperNode_Matrix, SuperMatrix};

use crate::c::super_matrix::CSuperMatrix;

/// Deallocate a c_SuperMatrix structure and store
///
/// This includes deallocating the vectors inside the matrix store.
pub unsafe fn c_destroy_comp_col_matrix(a: &mut CSuperMatrix) {
    Destroy_CompCol_Matrix(a.super_matrix() as *const SuperMatrix as *mut SuperMatrix);
}

/// Deallocate a c_SuperMatrix structure and store
///
/// This includes deallocating the vectors inside the matrix store.
pub unsafe fn c_destroy_dense_matrix(a: &mut CSuperMatrix) {
    Destroy_Dense_Matrix(a.super_matrix() as *const SuperMatrix as *mut SuperMatrix);
}

/// Deallocate a c_SuperMatrix structure and store
///
/// This includes deallocating the vectors inside the matrix store.
pub unsafe fn c_destroy_super_node_matrix(a: &mut CSuperMatrix) {
    Destroy_SuperNode_Matrix(a.super_matrix() as *const SuperMatrix as *mut SuperMatrix);
}
