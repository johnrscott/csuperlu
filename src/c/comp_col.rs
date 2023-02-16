use std::mem::MaybeUninit;

use crate::c::super_matrix::{c_SuperMatrix, Dtype_t, Mtype_t, Stype_t};
use libc;
use num::FromPrimitive;

#[link(name = "superlu")]
extern "C" {
    fn dCreate_CompCol_Matrix(
        A: *mut c_SuperMatrix,
        m: libc::c_int,
        n: libc::c_int,
        nnz: libc::c_int,
        nzval: *mut libc::c_double,
        rowind: *mut libc::c_int,
        colptr: *mut libc::c_int,
        stype: Stype_t,
        dtype: Dtype_t,
        mtype: Mtype_t,
    );
    fn sCreate_CompCol_Matrix(
        A: *mut c_SuperMatrix,
        m: libc::c_int,
        n: libc::c_int,
        nnz: libc::c_int,
        nzval: *mut libc::c_float,
        rowind: *mut libc::c_int,
        colptr: *mut libc::c_int,
        stype: Stype_t,
        dtype: Dtype_t,
        mtype: Mtype_t,
    );
    fn Destroy_CompCol_Matrix(A: *mut c_SuperMatrix);
    fn dPrint_CompCol_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn sPrint_CompCol_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
}

/// Create compressed column matrix of particular precision
///
/// Trait for access to low level C functions from SuperLU, which
/// dispatches correctly based on the desired precision (and picks
/// the right value for the Dtype argument). This trait is necessary
/// because the function names for different precisions are
/// different.
///
/// The assumption of this trait is that it is necessary to pass the
/// correct value of Dtype corresponding to the prefix in front of
/// the function (d, s, c, z). This would make the Dtype argument redundant,
/// but I don't understand what the purpose of it is otherwise; the
/// SuperLU functions do not allocate their own memory for vectors,
/// so they cannot perform a precision conversion
/// (one hypothesis for the Dtype argument), and it seems to lead to
/// seg faults if the "wrong" Dtype is passed.
///
/// For a similar reason, the Stype parameter is also omitted, because this
/// is always SLU_NC for this function (as stated in the doxygen docs).
///
/// Rename this to something like CompColUtils
pub trait CCompColMatrix<P>: Clone + FromPrimitive {
    fn c_create_comp_col_matrix(
        a: &mut MaybeUninit<c_SuperMatrix>,
        m: i32,
        n: i32,
        nnz: i32,
        nzval: &mut Vec<P>,
        rowind: &mut Vec<i32>,
        colptr: &mut Vec<i32>,
        mtype: Mtype_t,
    );
    fn c_print_comp_col_matrix(what: *mut libc::c_char, a: *mut c_SuperMatrix);
}

impl CCompColMatrix<f64> for f64 {
    fn c_create_comp_col_matrix(
        a: &mut MaybeUninit<c_SuperMatrix>,
        m: i32,
        n: i32,
        nnz: i32,
        nzval: &mut Vec<f64>,
        rowind: &mut Vec<i32>,
        colptr: &mut Vec<i32>,
        mtype: Mtype_t,
    ) {
        unsafe {
            dCreate_CompCol_Matrix(
                a.as_mut_ptr(),
                m,
                n,
                nnz,
                nzval.as_mut_ptr(),
                rowind.as_mut_ptr(),
                colptr.as_mut_ptr(),
                Stype_t::SLU_NC,
                Dtype_t::SLU_D,
                mtype,
            );
        }
    }

    fn c_print_comp_col_matrix(what: *mut libc::c_char, a: *mut c_SuperMatrix) {
        unsafe {
            dPrint_CompCol_Matrix(what, a);
        }
    }
}

impl CCompColMatrix<f32> for f32 {
    fn c_create_comp_col_matrix(
        a: &mut MaybeUninit<c_SuperMatrix>,
        m: i32,
        n: i32,
        nnz: i32,
        nzval: &mut Vec<f32>,
        rowind: &mut Vec<i32>,
        colptr: &mut Vec<i32>,
        mtype: Mtype_t,
    ) {
        unsafe {
            sCreate_CompCol_Matrix(
                a.as_mut_ptr(),
                m,
                n,
                nnz,
                nzval.as_mut_ptr(),
                rowind.as_mut_ptr(),
                colptr.as_mut_ptr(),
                Stype_t::SLU_NC,
                Dtype_t::SLU_S,
                mtype,
            );
        }
    }

    fn c_print_comp_col_matrix(what: *mut libc::c_char, a: *mut c_SuperMatrix) {
        unsafe {
            sPrint_CompCol_Matrix(what, a);
        }
    }
}

/// This will attempt to deallocate the three input vectors used to
/// create the comp_col matrix.
#[allow(non_snake_case)]
pub fn c_Destroy_CompCol_Matrix(A: *mut c_SuperMatrix) {
    unsafe {
        Destroy_CompCol_Matrix(A);
    }
}
