//! SuperMatrix struct for defining matrices.
//!
//! Structs and functions for interfacing to the SuperMatrix
//! C struct. The SuperMatrix struct is responsible for defining
//! the matrix types in SuperLU.
//!
//! Note: be aware that the order of enums in this file is important.
//! The must match the C representation so that the underlying
//! C library interprets the enum integers correctly. Do not change
//! the order without checking what effect it might have.

#[repr(C)]
#[allow(non_camel_case_types)]

/// The matrix numerical type and floating-point precision.
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

/// A simple method to turn a precision into a Dtype_t.
/// This is a bit like metaprogramming tricks in C++.
/// TODO: is this the best way to do this? (map a type to
/// a value).
struct GetDtype<PhantomData>;

impl GetDtype<f64> {
    fn get() -> Dtype_t {
	Dtype_t::SLU_D
    }
}

impl GetDtype<f32> {
    fn get() -> Dtype_t {
	Dtype_t::SLU_S
    }
}

/// Specifies some mathematical properties of the matrix.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum Mtype_t {
    /// General matrix
    SLU_GE,
    /// Lower-triangular, unit diagonal
    SLU_TRLU,
    /// Upper-triangular, unit diagonal
    SLU_TRUU,
    /// Lower-triangular
    SLU_TRL,
    /// Upper-triangular
    SLU_TRU,
    /// Symmetric, store lower half
    SLU_SYL,
    /// Symmetric, store upper half
    SLU_SYU,
    /// Hermitian, store lower half
    SLU_HEL,
    /// Hermitian, store upper half
    SLU_HEU,
}

/// Specifies the manner of matrix storage in memory
///
/// Column-major storage is when elements in the same
/// column of the matrix are stored contiguously in memory, and
/// column arrays are placed one after the other. Row-major
/// storage places elements in the same row next to
/// each other instead.
///
/// A supernodal matrix is a sparse matrix that groups together
/// columns (or rows) with a similar layout of non-zero elements.
///
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum Stype_t {
    /// Not supernodel, column-major
    SLU_NC,
    /// Not supernodal, column-major, permuted by columns
    SLU_NCP,
    /// Not supernodal, row-major
    SLU_NR,
    /// Supernodal, column-major
    SLU_SC,
    /// Supernodal, column-major, permuted by columns
    SLU_SCP,
    /// Supernodal, row-major
    SLU_SR,
    /// Dense, column-major (Fortran-style)
    SLU_DN,
    /// Distributed compressed row format
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
