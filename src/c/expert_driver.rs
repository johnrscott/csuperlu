use crate::c::options::superlu_options_t;
use crate::c::stat::SuperLUStat_t;
use crate::c::utils::c_SuperMatrix;
use libc;

#[repr(C)]
#[allow(non_snake_case)]
pub struct mem_usage_t {
    for_lu: libc::c_float,
    total_needed: libc::c_float,
}

#[link(name = "superlu")]
extern "C" {
    fn dgssvx(
        options: *mut superlu_options_t,
        A: *mut c_SuperMatrix,
        perm_c: *mut libc::c_int,
        perm_r: *mut libc::c_int,
        etree: *mut libc::c_int,
        equed: *mut libc::c_char,
        R: libc::c_double,
        C: libc::c_double,
        L: *mut c_SuperMatrix,
        U: *mut c_SuperMatrix,
        work: *mut libc::c_void,
        lwork: libc::c_int,
        B: *mut c_SuperMatrix,
        X: *mut c_SuperMatrix,
        recip_pivot_growth: *mut libc::c_double,
        rcond: *mut libc::c_double,
        ferr: *mut libc::c_double,
        berr: *mut libc::c_double,
        mem_usage: *mut mem_usage_t,
        stat: *mut SuperLUStat_t,
        info: *mut libc::c_int,
    );
}

#[allow(non_snake_case)]
pub fn c_dgssvx(
    options: *mut superlu_options_t,
    A: *mut c_SuperMatrix,
    perm_c: *mut libc::c_int,
    perm_r: *mut libc::c_int,
    etree: *mut libc::c_int,
    equed: *mut libc::c_char,
    R: libc::c_double,
    C: libc::c_double,
    L: *mut c_SuperMatrix,
    U: *mut c_SuperMatrix,
    work: *mut libc::c_void,
    lwork: libc::c_int,
    B: *mut c_SuperMatrix,
    X: *mut c_SuperMatrix,
    recip_pivot_growth: *mut libc::c_double,
    rcond: *mut libc::c_double,
    ferr: *mut libc::c_double,
    berr: *mut libc::c_double,
    mem_usage: *mut mem_usage_t,
    stat: *mut SuperLUStat_t,
    info: *mut libc::c_int,
) {
    unsafe {
        dgssvx(
            options,
            A,
            perm_c,
            perm_r,
            etree,
            equed,
            R,
            C,
            L,
            U,
            work,
            lwork,
            B,
            X,
            recip_pivot_growth,
            rcond,
            ferr,
            berr,
            mem_usage,
            stat,
            info,
        );
    }
}
