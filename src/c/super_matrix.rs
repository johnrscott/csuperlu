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

#[link(name = "superlu")]
extern "C" {
    fn Destroy_SuperMatrix_Store(A: *mut c_SuperMatrix);
}


// This will deallocate only the data structure allocated by
// the Create_*_Matrix routine (leaving the input vectors to
// be freed by the caller).
#[allow(non_snake_case)]
pub fn c_Destroy_SuperMatrix_Store(A: *mut c_SuperMatrix) {
    unsafe {
        Destroy_SuperMatrix_Store(A);
    }
}


/// The matrix numerical type and floating-point precision.
#[repr(C)]
#[allow(non_camel_case_types)]
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

/// The SuperMatrix structure, which stores all types of matrices
/// in SuperLU.
#[repr(C)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct c_SuperMatrix {
    /// The storage format for the matrix data (determines the
    /// type of Store).
    pub Stype: Stype_t,
    /// Specifies the precision
    pub Dtype: Dtype_t,
    /// Any mathematical properties of the matrix
    pub Mtype: Mtype_t,
    /// Number of rows
    pub nrow: libc::c_int,
    /// Number of columns
    pub ncol: libc::c_int,
    /// The data structure storing the values in the matrix. The
    /// format depends on the Stype.
    pub Store: *mut libc::c_void,
}

/// The C structure for the compressed-column format.
#[repr(C)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct c_NCformat {
    /// Total number of non-zeroes in the matrix
    pub nnz: libc::c_int,
    /// Array of non-zero values, column-major order
    pub nzval: *mut libc::c_void,
    /// Array containing the row indices of the non-zeroes
    pub rowind: *mut libc::c_int,
    /// Array of indices showing where each new column starts in rowind
    pub colptr: *mut libc::c_int,
}

/// The C structure for the compressed-row format.
#[repr(C)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct c_NRformat {
    /// Total number of non-zeroes in the matrix
    pub nnz: libc::c_int,
    /// Array of non-zero values, column-major order
    pub nzval: *mut libc::c_void,
    /// Array containing the column indices of the non-zeroes
    pub colind: *mut libc::c_int,
    /// Array of indices showing where each new row starts in colind
    pub rowptr: *mut libc::c_int,
}

/// The C structure for the (compressed-column) super-node format.
#[repr(C)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct c_SCformat {
    /// Total number of non-zeroes in the matrix
    pub nnz: libc::c_int,
    /// Index of the last super node
    pub nsuper: libc::c_int,
    /// Array of non-zero values, column-major order
    pub nzval: *mut libc::c_void,
    /// Array of indices showing where each new column starts
    /// in nzval. These are the 
    pub nzval_colptr: *mut libc::c_int,
    /// Array of compressed row indices of rectangular super nodes
    pub rowind: *mut libc::c_int,
    /// Location in rowind which starts each new column
    pub rowind_colptr: *mut libc::c_int,
    /// Map column number to super node index
    pub col_to_sup: *mut libc::c_int,
    /// Map super node index to the first column in the super node
    pub sup_to_col: *mut libc::c_int,
}


/// The C structure for the dense matrix format, which is
/// column major format.
#[repr(C)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct c_DNformat {
    /// The leading dimension of the array, equal to the
    /// number of rows
    pub lda: libc::c_int,
    /// Array of non-zero values, column-major order, of
    /// length lda * ncol ( = nrow * ncol)
    pub nzval: *mut libc::c_void,
}
