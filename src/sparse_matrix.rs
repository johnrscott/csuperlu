use itertools::Itertools;
use std::fmt;
use std::collections::HashMap;

use crate::{comp_col::CompColMatrix, c::value_type::ValueType};

#[derive(Debug)]
pub struct SparseMatrix<P: ValueType<P>> {
    num_rows: usize,
    num_cols: usize,
    values: HashMap<(usize, usize), P>,
}

impl<P: ValueType<P>> SparseMatrix<P> {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        Self {
            num_rows,
            num_cols,
            values: HashMap::new(),
        }
    }

    // TODO: Can we overload something to make the input nicer, e.g a[row, col] = value
    pub fn set_value(&mut self, row: usize, col: usize, value: P) {
        if row >= self.num_rows || col >= self.num_cols {
            panic!("Index out of range");
        }
	// TODO: Do not insert into map when value = 0?
        self.values.insert((row, col), value);
    }

    pub fn get_value(&self, row: usize, col: usize) -> P {
        if row >= self.num_rows || col >= self.num_cols {
            panic!("Index out of range");
        }
        self.values.get(&(row, col)).copied().unwrap_or(P::zero())
    }

    pub fn compressed_column_format(&self) -> CompColMatrix<P> {
	// Sort in column order
	let sorted_keys = self.values.keys()
	    .sorted_unstable_by_key(|a| (a.1, a.0)); 

	let num_non_zeros = self.values.len();
	let mut non_zero_values = Vec::<P>::with_capacity(num_non_zeros);
	let mut column_offsets = Vec::<i32>::with_capacity(self.num_cols + 1);
	let mut row_indices = Vec::<i32>::with_capacity(num_non_zeros);

	column_offsets.push(0);
	let mut current_col = 0usize;
	
	for key in sorted_keys {
	    if key.1 > current_col {
		// Handle empty columns with this range
		for _ in 0..(key.1 - current_col) {
		    column_offsets.push(non_zero_values.len() as i32);
		}
		current_col = key.1;
	    }
	    non_zero_values.push(self.values[key]);
	    row_indices.push(key.0 as i32);
	}
	column_offsets.push(num_non_zeros as i32);

	CompColMatrix::from_vectors(self.num_rows, non_zero_values, row_indices, column_offsets)
    }

    pub fn num_rows(&self) -> usize {
	self.num_rows
    }

    pub fn num_cols(&self) -> usize {
	self.num_cols
    }

    pub fn num_non_zeros(&self) -> usize {
	self.values.len()
    }
    
}

impl<P: ValueType<P>> fmt::Display for SparseMatrix<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	writeln!(f, "{} x {} matrix, {} non-zero values", self.num_rows, self.num_cols, self.values.len())?;
	let sorted_keys = self.values.keys()
	    .sorted_unstable_by_key(|a| (a.1, a.0)); 
	for key in sorted_keys {
	    writeln!(f, "({}, {}) = {:?}", key.0, key.1, self.values[key])?;
	}
	Ok(())
    }
}

