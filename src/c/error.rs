//! Errors that can occur when solving the system

use std::{ffi, fmt};

#[derive(Debug)]
pub enum Error {
    CompColError,
    DenseMatrixError,
    OutOfMemory { mem_alloc_at_failure: usize },
    UnknownError,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Self::UnknownError => write!(f, "An unknown error occured"),
	    Self::CompColError => write!(f, "An error occured creating a compressed column matrix"),
	    Self::DenseMatrixError => write!(f, "An error occured creating a dense matrix"),
	    Self::OutOfMemory { mem_alloc_at_failure } =>
		write!(f, "Simple driver ran out of memory ({mem_alloc_at_failure} B allocated at failure)"),
	}
    }
}

/// Convert a rust string reference to a C string
fn c_string(string: &str) -> ffi::CString {
    ffi::CString::new(string).unwrap()
}
