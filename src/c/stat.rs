//! Performance statistics functions.
//!
//! SuperLU records performance statistics such as the number
//! of floating-point operations and the execution time of the
//! solvers. This module contains a wrapper around the
//! SuperLUStat_t object in the C library. All the functions
//! related to this structure are exposed here, except the
//! memory related functions (alloc and free) which are wrapped
//! up in new and drop.

use std::mem::MaybeUninit;

use csuperlu_sys::{StatInit, StatFree, SuperLUStat_t, StatPrint};

pub struct CSuperluStat {
    stat: SuperLUStat_t,
}

impl CSuperluStat {
    /// Create a new SuperLU stats struct
    ///
    pub fn new() -> Self {
        let stat = unsafe {
            let mut stat = MaybeUninit::<SuperLUStat_t>::uninit();
            StatInit(stat.as_mut_ptr());
            stat.assume_init()
        };
	Self {
	    stat,
	}
    }

    /// Get the underlying SuperLUStat_t struct
    ///
    /// This function is intended for use in the driver wrapper
    /// routines for getting raw access to the stat struct.
    pub fn get_stat(&mut self) -> &mut SuperLUStat_t {
	&mut self.stat
    }
    
    /// Print a stats struct (using the C library print function)
    ///
    /// This function makes the assumption that the C library does not
    /// modify the stats object while printing it.
    ///
    pub fn print(&self) {
	unsafe {
	    StatPrint(&self.stat as *const SuperLUStat_t as *mut SuperLUStat_t);
	}
    }
    
}

impl Drop for CSuperluStat {
    fn drop(&mut self) {
	unsafe {
            StatFree(&mut self.stat as *mut SuperLUStat_t);
	}
    }
}
