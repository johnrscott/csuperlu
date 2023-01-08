//! This is the stat module
//!

use std::mem::MaybeUninit;

#[allow(non_camel_case_types)]
pub type flops_t = libc::c_float;

#[repr(C)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct SuperLUStat_t {
    panel_histo: *mut libc::c_int,
    utime: *mut libc::c_double,
    ops: *mut flops_t,
    TinyPivots: libc::c_int,
    RefineSteps: libc::c_int,
    expansions: libc::c_int,
}

#[link(name = "superlu")]
extern {
    fn StatInit(stat: *mut SuperLUStat_t);
    fn StatFree(stat: *mut SuperLUStat_t);
    fn StatPrint(stat: *mut SuperLUStat_t);	
}

#[allow(non_snake_case)]
pub fn c_StatInit(stat: *mut SuperLUStat_t) {
    unsafe {
	StatInit(stat);
    }
}

#[allow(non_snake_case)]
pub fn c_StatFree(stat: *mut SuperLUStat_t) {
    unsafe {
	StatFree(stat);
    }
}

#[allow(non_snake_case)]
pub fn c_StatPrint(stat: *mut SuperLUStat_t) {
    unsafe {
	StatPrint(stat);
    }
}

impl SuperLUStat_t {
    pub fn new() -> Self {
	unsafe {
	    let mut stat = MaybeUninit::<SuperLUStat_t>::uninit();
	    c_StatInit(stat.as_mut_ptr());
	    stat.assume_init()
	}
    }
}
