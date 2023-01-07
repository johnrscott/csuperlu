use crate::superlu::utils::{
    superlu_options_t,
    SuperLUStat_t,
};

#[link(name = "superlu")]
extern {
    fn set_default_options(options: *mut superlu_options_t);
    fn StatInit(stat: *mut SuperLUStat_t);
    fn StatFree(stat: *mut SuperLUStat_t);
}

pub fn c_set_default_options(options: *mut superlu_options_t) {
    unsafe {
	set_default_options(options);
    }
}

pub fn c_StatInit(stat: *mut SuperLUStat_t) {
    unsafe {
	StatInit(stat);
    }
}


pub fn c_StatFree(stat: *mut SuperLUStat_t) {
    unsafe {
	StatFree(stat);
    }
}

