use itertools::Itertools;
use std::fmt;
use std::collections::HashMap;

use crate::c::value_type::ValueType;

#[derive(Debug)]
pub struct SparseMatrix<P: ValueType<P>> {
    nrows: usize,
    ncols: usize,
    values: HashMap<(usize, usize), P>,
}

impl<P: ValueType<P>> SparseMatrix<P> {
    /// Create an empty sparse matrix with zero rows and columns
    pub fn empty() -> Self {
        Self {
            nrows: 0,
            ncols: 0,
            values: HashMap::new(),
        }
    }

    /// Create an empty sparse matrix of the given size
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Self {
            nrows,
            ncols,
            values: HashMap::new(),
        }	
    }

    /// Input a triplet into the sparse matrix, checking the row and column against the matrix size
    pub fn set(&mut self, row: usize, col: usize, val: P) {
	if row >= self.nrows || col >= self.ncols {
	    panic!("Triplet index ({}, {}) out of range for matrix size {}x{}",
		   row, col, self.nrows, self.ncols);
	}

	if val == P::zero() {
	    if self.values.contains_key(&(row, col)) {
		self.values.remove(&(row, col));
	    }
	}
	else {
            self.values.insert((row, col), val);
	}
    }

    /// Input a triplet into the sparse matrix, allowing the matrix to automatically
    /// resize to fit the new element
    pub fn set_with_resize(&mut self, row: usize, col: usize, val: P) {
	if val == P::zero() {
	    if self.values.contains_key(&(row, col)) {
		self.values.remove(&(row, col));
	    }
	}
	else {
            self.values.insert((row, col), val);
	    if row >= self.nrows {
		self.nrows = row + 1;
	    }
	    if col >= self.ncols {
		self.ncols = col + 1;
	    }
	}
    }
}

impl<P: ValueType<P>> fmt::Display for SparseMatrix<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	writeln!(f, "{} x {} matrix, {} non-zero values", self.nrows, self.ncols, self.values.len())?;
	let sorted_keys = self.values.keys()
	    .sorted_unstable_by_key(|a| (a.1, a.0)); 
	for key in sorted_keys {
	    writeln!(f, "({}, {}) = {:?}", key.0, key.1, self.values[key])?;
	}
	Ok(())
    }
}


#[cfg(test)]
mod tests;
