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

	let mut v = Vec::<f64>::new();
	let mut col_index = Vec::<usize>::new();
	let mut row_index = Vec::<usize>::new();

	col_index.push(0);
	let mut current_col = 0usize;
	let mut index = 0usize;
	
	for key in sorted_keys {
            println!("{:?} = {:?}", key, self.values[key]);
	    v.push(self.values[key]);
	    row_index.push(key.0);
	    if key.1 > current_col {
		col_index.push(index);
		current_col = key.1;
	    }
	    index += 1;
	}
	col_index.push(index);
	
	println!("{:?}", v);
	println!("{:?}", row_index);
	println!("{:?}", col_index);
    }
}
