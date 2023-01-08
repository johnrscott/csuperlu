//! Performance statistics functions
//!
//! SuperLU records performance statistics such as the number
//! of floating-point operations and the execution time of the
//! solvers. This module contains a wrapper around the
//! SuperLUStat_t object in the C library. All the functions
//! related to this structure are exposed here, except the
//! memory related functions (alloc and free) which are wrapped
//! up in new and drop.

use std::mem::MaybeUninit;
    
#[allow(non_camel_case_types)]
pub type flops_t = libc::c_float;

/// Performance statistics struct
///
/// This structure is documented in section 1.3.3 of them
/// SuperLU manual.
///
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

    // Create a new SuperLUStat_t. Memory is handled automatically
    // -- there is no need to run alloc and free functions manually
    // (as in the C library)
    pub fn new() -> Self {
	unsafe {
	    let mut stat = MaybeUninit::<SuperLUStat_t>::uninit();
	    c_StatInit(stat.as_mut_ptr());
	    stat.assume_init()
	}
    }
}

impl Drop for SuperLUStat_t {
    fn drop(&mut self) {
	c_StatFree(self);
    }
}
