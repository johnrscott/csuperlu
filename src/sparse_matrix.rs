//! Create and manipulate sparse matrices

use itertools::Itertools;
use std::fmt;
use std::collections::HashMap;

use crate::c::value_type::ValueType;

#[derive(Debug)]
pub struct SparseMatrix<P: ValueType> {
    nrows: usize,
    ncols: usize,
    values: HashMap<(usize, usize), P>,
}

impl<P: ValueType> SparseMatrix<P> {
    /// Create an empty sparse matrix with zero rows and columns.
    pub fn empty() -> Self {
        Self {
            nrows: 0,
            ncols: 0,
            values: HashMap::new(),
        }
    }

    /// Create an empty sparse matrix of the given size.
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Self {
            nrows,
            ncols,
            values: HashMap::new(),
        }	
    }

    /// Input a triplet into the sparse matrix, checking the row and column against the matrix size.
    pub fn insert(&mut self, row: usize, col: usize, val: P) {
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

    /// Read the value at the given row and column.
    pub fn get(&self, row: usize, col: usize) -> P {
	if row >= self.nrows || col >= self.ncols {
	    panic!("Triplet index ({}, {}) out of range for matrix size {}x{}",
		   row, col, self.nrows, self.ncols);
        }
        self.values.get(&(row, col)).copied().unwrap_or(P::zero())
    }
    
    /// Input a triplet into the sparse matrix, allowing the matrix to automatically
    /// resize to fit the new element.
    pub fn insert_unbounded(&mut self, row: usize, col: usize, val: P) {
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

    /// Read the value at the given row and column. This function won't check if
    /// the row and column are within the bounds of the matrix -- it will just return 0
    /// even if it is out of bounds.
    pub fn get_unbounded(&self, row: usize, col: usize) -> P {
	self.values.get(&(row, col)).copied().unwrap_or(P::zero())
    }

    /// Get the number of rows in the matrix.
    pub fn nrows(&self) -> usize {
	self.nrows
    }

    /// Get the number of columns in the matrix.
    pub fn ncols(&self) -> usize {
	self.ncols
    }

    /// Get the number of non-zero values in the matrix.
    pub fn nnz(&self) -> usize {
	self.values.len()
    }
}

impl<P: ValueType> fmt::Display for SparseMatrix<P> {
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

impl<P: ValueType> From<HashMap<(usize, usize), P>> for SparseMatrix<P> {
    fn from(values: HashMap<(usize, usize), P>) -> Self {
	let nrows = values.keys().max_by_key(|k| k.0).unwrap().0 + 1;
	let ncols = values.keys().max_by_key(|k| k.1).unwrap().1 + 1;
	Self {
	    nrows,
	    ncols,
	    values,
	}
    }
}

#[cfg(test)]
mod tests;
