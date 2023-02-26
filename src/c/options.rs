//! Options argument
//!
//! The options argument is used to specify how the solver
//! should work. It is documented in section 2.4 of the
//! SuperLU manual.
//!

use std::mem::MaybeUninit;

use csuperlu_sys::{superlu_options_t, set_default_options};

/// Wrapper for the SuperLU C library superlu_options_t. 
///
/// The superlu_options_t enum controls the behaviour of the
/// simple driver and expert drivers. This struct provides
/// a wrapper that enforces consistency of options.
pub struct CSuperluOptions {
    options: superlu_options_t
}

impl CSuperluOptions {
    pub fn new() -> Self {
        let options = unsafe {
            let mut options = MaybeUninit::<superlu_options_t>::uninit();
            set_default_options(options.as_mut_ptr());
            options.assume_init()
        };
	Self {
	    options,
	}
    }
}
