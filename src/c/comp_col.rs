use crate::c::super_matrix::{c_SuperMatrix, Dtype_t, Mtype_t, Stype_t};
use libc;

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
}

pub trait CreateCompColMatrix<P> {
    fn create_comp_col_matrix(
	A: &mut c_SuperMatrix,
        m: i32,
        n: i32,
        nnz: i32,
        nzval: &mut Vec<P>,
        rowind: &mut Vec<i32>,
        colptr: &mut Vec<i32>,
        stype: Stype_t,
        mtype: Mtype_t
    );
}

impl CreateCompColMatrix<f64> for f64 {
    fn create_comp_col_matrix(
	a: &mut c_SuperMatrix,
        m: i32,
        n: i32,
        nnz: i32,
        nzval: &mut Vec<f64>,
        rowind: &mut Vec<i32>,
        colptr: &mut Vec<i32>,
        stype: Stype_t,
        mtype: Mtype_t
    ) {
	unsafe {
            dCreate_CompCol_Matrix(a, m, n, nnz,
				   nzval.as_mut_ptr(),
				   rowind.as_mut_ptr(),
				   colptr.as_mut_ptr(),
				   stype, Dtype_t::SLU_D, mtype);
	}	
    }
}

impl CreateCompColMatrix<f32> for f32 {
    fn create_comp_col_matrix(
	a: &mut c_SuperMatrix,
        m: i32,
        n: i32,
        nnz: i32,
        nzval: &mut Vec<f32>,
        rowind: &mut Vec<i32>,
        colptr: &mut Vec<i32>,
        stype: Stype_t,
        mtype: Mtype_t
    ) {
	unsafe {
            sCreate_CompCol_Matrix(a, m, n, nnz,
				   nzval.as_mut_ptr(),
				   rowind.as_mut_ptr(),
				   colptr.as_mut_ptr(),
				   stype, Dtype_t::SLU_S, mtype);
	}	
    }
}

/// This will attempt to deallocate the three input matrices used to
/// create the comp_col matrix.
#[allow(non_snake_case)]
pub fn c_Destroy_CompCol_Matrix(A: *mut c_SuperMatrix) {
    unsafe {
        Destroy_CompCol_Matrix(A);
    }
}

#[allow(non_snake_case)]
pub fn c_dPrint_CompCol_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix) {
    unsafe {
        dPrint_CompCol_Matrix(what, A);
    }
}
