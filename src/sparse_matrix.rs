use itertools::Itertools;
use std::fmt;
use std::collections::HashMap;

use crate::{comp_col::CompColMatrix, c::value_type::ValueType};

#[derive(Debug)]
pub struct SparseMatrix<P: ValueType> {
    num_rows: usize,
    num_cols: usize,
    values: HashMap<(usize, usize), P>,
}

impl<P: ValueType> SparseMatrix<P> {
    pub fn new() -> Self {
        Self {
            num_rows: 0,
            num_cols: 0,
            values: HashMap::new(),
        }
    }

    pub fn from_dict_of_keys(values: HashMap<(usize, usize), P>) -> Self {
	let num_rows = values.iter().max_by_key(|entry| entry.0.0).unwrap().0.0 + 1;
	let num_cols = values.iter().max_by_key(|entry| entry.0.1).unwrap().0.1 + 1;
	Self {
	    num_rows,
	    num_cols,
	    values
	}
    }

    // TODO: Can we overload something to make the input nicer, e.g a[row, col] = value
    pub fn set_value(&mut self, row: usize, col: usize, value: P) {
	// Keep track of new size
	if row >= self.num_rows {
	    self.num_rows = row + 1;
	}
	if col >= self.num_cols {
	    self.num_cols = col + 1;
	}	
	
	if value == P::zero() {
	    if self.values.contains_key(&(row, col)) {
		self.values.remove(&(row, col));
	    }
	}
	else {
            self.values.insert((row, col), value);
	}
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

    pub fn values(&self) -> &HashMap<(usize, usize), P> {
	&self.values
    }

    pub fn print_structure(&self) {
	for r in 0..self.num_rows {
	    for c in 0..self.num_cols {
		let res = self.values.get(&(r, c));
		match res {
		    None => print!("  "),
		    Some(_) => print!("x "),
		}
	    }
	    print!("\n");
	}
    }

    /// Allow resizing (shrinking and expanding) as long as contents are preserved
    /// Pads with additional rows and columns to fit new size
    pub fn resize(&mut self, num_rows: usize, num_cols: usize) {
	self.resize_rows(num_rows);
	self.resize_cols(num_cols);
    }

    pub fn resize_rows(&mut self, num_rows: usize) {
	let num_rows_actual = self.values.iter().max_by_key(|entry| entry.0.0).unwrap().0.0 + 1;
	if num_rows < num_rows_actual {
	    panic!("Cannot resize matrix (rows) to be smaller than its contents");
	}
	self.num_rows = num_rows;
    }

    pub fn resize_cols(&mut self, num_cols: usize) {
	let num_cols_actual = self.values.iter().max_by_key(|entry| entry.0.1).unwrap().0.1 + 1;
	if num_cols < num_cols_actual {
	    panic!("Cannot resize matrix (cols) to be smaller than its contents");
	}
	self.num_cols = num_cols;
    }

}

impl<P: ValueType> fmt::Display for SparseMatrix<P> {
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

