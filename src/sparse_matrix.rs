use std::collections::HashMap;

#[derive(Debug)]
pub struct SparseMatrix {
    num_rows: usize,
    num_cols: usize,
    values: HashMap<(usize, usize), f64>,
}

impl SparseMatrix {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
	Self {
	    num_rows,
	    num_cols,
	    values: HashMap::new()
	}
    }

    pub fn add_value(&mut self, row: usize, col: usize, value: f64) {
	if row >= self.num_rows || col >= self.num_cols { 
	    panic!("Index out of range");
	}
	self.values.insert((row, col), value);
    }
}
