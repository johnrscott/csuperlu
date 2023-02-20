use crate::csuperlu_sys::options::superlu_options_t;
use crate::csuperlu_sys::stat::SuperLUStat_t;
use crate::csuperlu_sys::super_matrix::c_SuperMatrix;
use libc;

// use super::comp_col::CCompColMatrix;
// use super::dense::CCreateDenseMatrix;
// use super::super_node::CSuperNodeMatrix;

#[link(name = "superlu")]
extern "C" {
    pub fn sgssv(
        options: *mut superlu_options_t,
        A: *mut c_SuperMatrix,
        perm_c: *mut libc::c_int,
        perm_r: *mut libc::c_int,
        L: *mut c_SuperMatrix,
        U: *mut c_SuperMatrix,
        B: *mut c_SuperMatrix,
        stat: *mut SuperLUStat_t,
        info: *mut libc::c_int,
    );
    pub fn dgssv(
        options: *mut superlu_options_t,
        A: *mut c_SuperMatrix,
        perm_c: *mut libc::c_int,
        perm_r: *mut libc::c_int,
        L: *mut c_SuperMatrix,
        U: *mut c_SuperMatrix,
        B: *mut c_SuperMatrix,
        stat: *mut SuperLUStat_t,
        info: *mut libc::c_int,
    );
    pub fn cgssv(
        options: *mut superlu_options_t,
        A: *mut c_SuperMatrix,
        perm_c: *mut libc::c_int,
        perm_r: *mut libc::c_int,
        L: *mut c_SuperMatrix,
        U: *mut c_SuperMatrix,
        B: *mut c_SuperMatrix,
        stat: *mut SuperLUStat_t,
        info: *mut libc::c_int,
    );
    pub fn zgssv(
        options: *mut superlu_options_t,
        A: *mut c_SuperMatrix,
        perm_c: *mut libc::c_int,
        perm_r: *mut libc::c_int,
        L: *mut c_SuperMatrix,
        U: *mut c_SuperMatrix,
        B: *mut c_SuperMatrix,
        stat: *mut SuperLUStat_t,
        info: *mut libc::c_int,
    );
}

/*
pub trait CSimpleDriver<P>: CCompColMatrix<P> + CCreateDenseMatrix<P> + CSuperNodeMatrix<P> {
    fn c_simple_driver(
	options: &mut superlu_options_t,
	a: *mut c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: *mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    );
}

impl CSimpleDriver<f32> for f32 {
    fn c_simple_driver(
	options: &mut superlu_options_t,
	a: *mut c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: *mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    ) {
	unsafe {
            sgssv(options, a, perm_c.as_mut_ptr(), perm_r.as_mut_ptr(),
		  l, u, b, stat, info);
	}	
    }
}

impl CSimpleDriver<f64> for f64 {
    fn c_simple_driver(
	options: &mut superlu_options_t,
	a: *mut c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: *mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    ) {
	unsafe {
            dgssv(options, a, perm_c.as_mut_ptr(), perm_r.as_mut_ptr(),
		  l, u, b, stat, info);
	}	
    }
}

impl CSimpleDriver<num::Complex<f32>> for num::Complex<f32> {
    fn c_simple_driver(
	options: &mut superlu_options_t,
	a: *mut c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: *mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    ) {
	unsafe {
            cgssv(options, a, perm_c.as_mut_ptr(), perm_r.as_mut_ptr(),
		  l, u, b, stat, info);
	}	
    }
}

impl CSimpleDriver<num::Complex<f64>> for num::Complex<f64> {
    fn c_simple_driver(
	options: &mut superlu_options_t,
	a: *mut c_SuperMatrix,
	perm_c: &mut Vec<i32>,
	perm_r: &mut Vec<i32>,
	l: &mut c_SuperMatrix,
	u: &mut c_SuperMatrix,
	b: *mut c_SuperMatrix,
	stat: &mut SuperLUStat_t,
	info: &mut i32,
    ) {
	unsafe {
            zgssv(options, a, perm_c.as_mut_ptr(), perm_r.as_mut_ptr(),
		  l, u, b, stat, info);
	}	
    }
}
*/
