mod c_super_node;

use crate::superlu::super_node::c_super_node::{
    c_dPrint_SuperNode_Matrix,
    c_Destroy_SuperNode_Matrix,
};
use crate::superlu::utils::SuperMatrix;

#[allow(non_snake_case)]
pub fn dPrint_SuperNode_Matrix(what: *mut libc::c_char, A: *mut SuperMatrix) {
    c_dPrint_SuperNode_Matrix(what, A);
}

#[allow(non_snake_case)]
pub fn Destroy_SuperNode_Matrix(A: *mut SuperMatrix) {
    c_Destroy_SuperNode_Matrix(A);
}
