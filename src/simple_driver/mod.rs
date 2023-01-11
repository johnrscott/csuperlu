//! Solve sparse linear systems using the simple driver
//!

use std::mem::MaybeUninit;
use crate::c::utils::SuperMatrix;
use crate::c::options::superlu_options_t;
use crate::c::stat::SuperLUStat_t;

use crate::c::simple_driver::c_dgssv;

/// Solve a sparse linear system AX = B.
///
///
/// TODO: Need to think through ownership for this function -- e.g.
/// B should be consumed, because the storage for B is reused as
/// the solution X.
///
#[allow(non_snake_case)]
pub fn dgssv(mut options: superlu_options_t,
	     A: &mut SuperMatrix,
	     mut perm_c: Vec<i32>,
	     mut perm_r: Vec<i32>,
	     mut B: SuperMatrix,
	     stat: &mut SuperLUStat_t)
	     -> (SuperMatrix, SuperMatrix, SuperMatrix, i32) {
    let mut info = 0;
    unsafe {
    	let mut L = MaybeUninit::<SuperMatrix>::uninit();
    	let mut U = MaybeUninit::<SuperMatrix>::uninit();
	
    	c_dgssv(&mut options, A, perm_c.as_mut_ptr(),
    		perm_r.as_mut_ptr(),
    		L.as_mut_ptr(), U.as_mut_ptr(),
    		&mut B, stat, &mut info);
    	(
	    B,
    	    L.assume_init(),
    	    U.assume_init(),
    	    info
    	)
    }
}
