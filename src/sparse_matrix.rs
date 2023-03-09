//! Create and manipulate sparse matrices

use itertools::Itertools;
use std::fmt;
use std::collections::HashMap;

use crate::c::value_type::ValueType;
use crate::comp_col::CompColMatrix;

#[derive(Debug, PartialEq, Clone)]
pub struct SparseMat<P: ValueType> {
    num_rows: usize,
    num_cols: usize,
    non_zero_vals: HashMap<(usize, usize), P>,
}

impl<P: ValueType> SparseMat<P> {
    /// Create an empty sparse matrix with zero rows and columns.
    pub fn empty() -> Self {
        Self {
            num_rows: 0,
            num_cols: 0,
            non_zero_vals: HashMap::new(),
        }
    }

    /// Create an empty sparse matrix of the given size.
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        Self {
            num_rows,
            num_cols,
            non_zero_vals: HashMap::new(),
        }	
    }

    /// Input a triplet into the sparse matrix, checking the row and column against the matrix size.
    pub fn insert(&mut self, row: usize, col: usize, val: P) {
	if row >= self.num_rows || col >= self.num_cols {
	    panic!("Triplet index ({}, {}) out of range for matrix size {}x{}",
		   row, col, self.num_rows, self.num_cols);
	}

	if val == P::zero() {
	    if self.non_zero_vals.contains_key(&(row, col)) {
		self.non_zero_vals.remove(&(row, col));
	    }
	}
	else {
            self.non_zero_vals.insert((row, col), val);
	}
    }

    /// Read the value at the given row and column.
    pub fn get(&self, row: usize, col: usize) -> P {
	if row >= self.num_rows || col >= self.num_cols {
	    panic!("Triplet index ({}, {}) out of range for matrix size {}x{}",
		   row, col, self.num_rows, self.num_cols);
        }
        self.non_zero_vals.get(&(row, col)).copied().unwrap_or(P::zero())
    }
    
    /// Input a triplet into the sparse matrix, allowing the matrix to automatically
    /// resize to fit the new element.
    pub fn insert_unbounded(&mut self, row: usize, col: usize, val: P) {
	if val == P::zero() {
	    if self.non_zero_vals.contains_key(&(row, col)) {
		self.non_zero_vals.remove(&(row, col));
	    }
	}
	else {
            self.non_zero_vals.insert((row, col), val);
	    if row >= self.num_rows {
		self.num_rows = row + 1;
	    }
	    if col >= self.num_cols {
		self.num_cols = col + 1;
	    }
	}
    }

    /// Read the value at the given row and column. This function won't check if
    /// the row and column are within the bounds of the matrix -- it will just return 0
    /// even if it is out of bounds.
    pub fn get_unbounded(&self, row: usize, col: usize) -> P {
	self.non_zero_vals.get(&(row, col)).copied().unwrap_or(P::zero())
    }

    /// Get the number of rows in the matrix.
    pub fn num_rows(&self) -> usize {
	self.num_rows
    }

    /// Get the number of columns in the matrix.
    pub fn num_cols(&self) -> usize {
	self.num_cols
    }

    /// Get the number of non-zero values in the matrix.
    pub fn num_non_zeros(&self) -> usize {
	self.non_zero_vals.len()
    }

    /// Return the non-zero values in a hashmap.
    pub fn non_zero_vals(&self) -> &HashMap<(usize, usize), P> {
	&self.non_zero_vals
    }

    /// Allow resizing (shrinking and expanding) as long as contents are preserved
    /// Pads with additional rows and columns to fit new size
    pub fn resize(&mut self, num_rows: usize, num_cols: usize) {
	self.resize_rows(num_rows);
	self.resize_cols(num_cols);
    }

    /// Allow resizing (shrinking and expanding) as long as contents are preserved
    /// Pads with additional rows to fit new size
    pub fn resize_rows(&mut self, num_rows: usize) {
	let num_rows_actual = match self.non_zero_vals.keys().max_by_key(|k| k.0) {
	    Some(max_index) => max_index.0 + 1,
	    None => 0,
	};
	if num_rows < num_rows_actual {
	    panic!("Contents of matrix fit into {num_rows_actual} rows, cannot resize to {num_rows} rows.");
	}
	self.num_rows = num_rows;
    }

    /// Allow resizing (shrinking and expanding) as long as contents are preserved
    /// Pads with additional columns to fit new size
    pub fn resize_cols(&mut self, num_cols: usize) {
	let num_cols_actual = match self.non_zero_vals.keys().max_by_key(|k| k.1) {
	    Some(max_index) => max_index.1 + 1,
	    None => 0,
	};
	if num_cols < num_cols_actual {
	    panic!("Contents of matrix fit into {num_cols_actual} cols, cannot resize to {num_cols} cols.");
	}
	self.num_cols = num_cols;
    }

    /// TODO: Change this to new compressed column format, could use 'from' trait
    pub fn compressed_column_format(&self) -> CompColMatrix<P> {	
	// Sort in column order
	let sorted_keys = self.non_zero_vals.keys()
	    .sorted_unstable_by_key(|a| (a.1, a.0)); 

	let num_non_zeros = self.num_non_zeros();
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
	    non_zero_values.push(self.non_zero_vals[key]);
	    row_indices.push(key.0 as i32);
	}
	column_offsets.push(num_non_zeros as i32);

	CompColMatrix::from_vectors(self.num_rows, non_zero_values, row_indices, column_offsets)
    }

    /// Lots of janky stuff going on here, look away...
    pub fn print_structure(&self, division: usize) {
	let mut row_divider = String::new();
	print!("   ");
	for i in 0..division {
	    row_divider.push_str("──");
	    if i / 10 == 0 {
		print!("  ");
	    } else {
		print!("{} ", i / 10);
	    }
	}
	row_divider.push('┼');
	print!("  ");
	for i in 0..(self.num_cols-division) {
	    row_divider.push_str("──");
	    if i / 10 == 0 {
		print!("  ");
	    } else {
		print!("{} ", i / 10);
	    }
	}
	print!("\n   ");
	for i in 0..division {
	    print!("{} ", i % 10);
	}
	print!("  ");
	for i in 0..(self.num_cols-division) {
	    print!("{} ", i % 10);
	}
	print!("\n");

	let mut current_row = 0;
	for r in 0..self.num_rows {
	    if r == division {
		current_row = 0;
		println!("   {row_divider}");
	    } 
	    print!("{:2} ", current_row);
	    current_row += 1;
	    for c in 0..self.num_cols {
		if c == division {
		    print!("│ ");
		}
		let res = self.non_zero_vals.get(&(r, c));
		match res {
		    None => print!("  "),
		    Some(_) => print!("x "),
		}
	    }
	    print!("\n");
	}
    }
}

impl<P: ValueType> fmt::Display for SparseMat<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	writeln!(f, "{} x {} matrix, {} non-zero values",
		 self.num_rows, self.num_cols, self.num_non_zeros())?;
	let sorted_keys = self.non_zero_vals.keys()
	    .sorted_unstable_by_key(|a| (a.1, a.0)); 
	for key in sorted_keys {
	    writeln!(f, "({}, {}) = {:?}", key.0, key.1, self.non_zero_vals[key])?;
	}
	Ok(())
    }
}

impl<P: ValueType> From<HashMap<(usize, usize), P>> for SparseMat<P> {
    fn from(non_zero_vals: HashMap<(usize, usize), P>) -> Self {
	let num_rows = non_zero_vals.keys().max_by_key(|k| k.0).unwrap().0 + 1;
	let num_cols = non_zero_vals.keys().max_by_key(|k| k.1).unwrap().1 + 1;
	Self {
	    num_rows,
	    num_cols,
	    non_zero_vals,
	}
    }
}

#[cfg(test)]
mod tests;
