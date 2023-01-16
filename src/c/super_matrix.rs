//! Structs and functions for interfacing to the SuperMatrix
//! C struct. The SuperMatrix struct is responsible for defining
//! the matrix types in SuperLU.

#[repr(C)]
#[allow(non_camel_case_types)]

/// The matrix numerical type and floating-point precision
pub enum Dtype_t {
    /// Single-precision real
    SLU_S,
    /// Double-precision real
    SLU_D,
    /// Single-precision complex
    SLU_C,
    /// Double-precision complex
    SLU_Z,
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
#[allow(non_camel_case_types)]
pub struct c_SuperMatrix {
    Stype: Stype_t,
    Dtype: Dtype_t,
    Mtype: Mtype_t,
    nrow: libc::c_int,
    ncol: libc::c_int,
    Store: *mut libc::c_void,
}
