use crate::c::super_matrix::c_SuperMatrix;
use libc;

#[link(name = "superlu")]
extern "C" {
    fn sPrint_SuperNode_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn dPrint_SuperNode_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn cPrint_SuperNode_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn zPrint_SuperNode_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn Destroy_SuperNode_Matrix(A: *mut c_SuperMatrix);
}


pub trait CSuperNodeMatrix<P> {
    fn c_print_super_node_matrix(what: *mut libc::c_char, a: *mut c_SuperMatrix);
}

impl CSuperNodeMatrix<f32> for f32 {
    fn c_print_super_node_matrix(what: *mut libc::c_char, a: *mut c_SuperMatrix) {
        unsafe {
            sPrint_SuperNode_Matrix(what, a);
        }
    }
}

impl CSuperNodeMatrix<f64> for f64 {
    fn c_print_super_node_matrix(what: *mut libc::c_char, a: *mut c_SuperMatrix) {
        unsafe {
            dPrint_SuperNode_Matrix(what, a);
        }
    }
}

impl CSuperNodeMatrix<num::Complex<f32>> for num::Complex<f32> {
    fn c_print_super_node_matrix(what: *mut libc::c_char, a: *mut c_SuperMatrix) {
        unsafe {
            cPrint_SuperNode_Matrix(what, a);
        }
    }
}

impl CSuperNodeMatrix<num::Complex<f64>> for num::Complex<f64> {
    fn c_print_super_node_matrix(what: *mut libc::c_char, a: *mut c_SuperMatrix) {
        unsafe {
            zPrint_SuperNode_Matrix(what, a);
        }
    }
}

#[allow(non_snake_case)]
pub fn c_Destroy_SuperNode_Matrix(A: *mut c_SuperMatrix) {
    unsafe {
        Destroy_SuperNode_Matrix(A);
    }
}
