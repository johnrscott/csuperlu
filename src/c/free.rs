//! Functions for freeing memory allocated to superlu structures
//!

use csuperlu_sys::{comp_col::Destroy_CompCol_Matrix, super_matrix::c_SuperMatrix, dense::Destroy_Dense_Matrix, super_node::Destroy_SuperNode_Matrix};

/// Deallocate a c_SuperMatrix structure and store
///
/// This includes deallocating the vectors inside the matrix store.
pub unsafe fn c_destroy_comp_col_matrix(a: &mut c_SuperMatrix) {
    Destroy_CompCol_Matrix (a);
}

/// Deallocate a c_SuperMatrix structure and store
///
/// This includes deallocating the vectors inside the matrix store.
pub unsafe fn c_destroy_dense_matrix(a: &mut c_SuperMatrix) {
    Destroy_Dense_Matrix (a);
}

/// Deallocate a c_SuperMatrix structure and store
///
/// This includes deallocating the vectors inside the matrix store.
pub unsafe fn c_destroy_super_node_matrix(a: &mut c_SuperMatrix) {
    Destroy_SuperNode_Matrix (a);
}
