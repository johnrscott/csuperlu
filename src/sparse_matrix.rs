use itertools::Itertools;
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
            values: HashMap::new(),
        }
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: f64) {
        if row >= self.num_rows || col >= self.num_cols {
            panic!("Index out of range");
        }
        self.values.insert((row, col), value);
    }

    pub fn get_value(&self, row: usize, col: usize) -> f64 {
        if row >= self.num_rows || col >= self.num_cols {
            panic!("Index out of range");
        }
        self.values.get(&(row, col)).copied().unwrap_or(0.0)
    }

    pub fn print_sorted(&self) {
	let sorted_keys = self.values.keys()
	    .sorted_unstable_by_key(|a| (a.1, a.0)); 
        for key in sorted_keys {
            println!("{:?} = {:?}", key, self.values[key]);
        }
    }
}
