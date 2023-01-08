// Data type
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum Dtype_t {
    SLU_S,
    SLU_D,
    SLU_C,
    SLU_Z
}

// Specifies some mathematical properties
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum Mtype_t {
    SLU_GE,
    SLU_TRLU,
    SLU_TRUU,
    SLU_TRL,
    SLU_TRU,
    SLU_SYL,
    SLU_SYU,
    SLU_HEL,
    SLU_HEU,
}

// Storage type
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum Stype_t {
    SLU_NC,
    SLU_NCP,
    SLU_NR,
    SLU_SC,
    SLU_SCP,
    SLU_SR,
    SLU_DN,
    SLU_NR_loc,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct SuperMatrix {
    Stype: Stype_t,
    Dtype: Dtype_t,
    Mtype: Mtype_t,
    nrow: libc::c_int,
    ncol: libc::c_int,
    Store: *mut libc::c_void,
}

