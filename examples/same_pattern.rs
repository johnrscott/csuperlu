//! This example shows how to reuse the column permutation from
//! a previous solution. Do this if the sparse structure of the matrix
//! is the same as the first solution.

use csuperlu::dense::DenseMatrix;
use csuperlu::c::options::ColumnPermPolicy;
use csuperlu::simple_driver::{SimpleSolution, SimpleSystem, SamePattern};
use csuperlu::c::stat::CSuperluStat;
use csuperlu::sparse_matrix::SparseMatrix;

fn main() {
    let num_rows = 5usize;
    let num_columns = 5usize;
    
    let mut a = SparseMatrix::new(num_rows, num_columns);

    // Matrix elements
    let s: f64 = 19.0;
    let u: f64 = 21.0;
    let p: f64 = 16.0;
    let e: f64 = 5.0;
    let r: f64 = 18.0;
    let l: f64 = 12.0;
    // Set values
    a.set_value(0, 0, s);
    a.set_value(1, 1, u);
    a.set_value(2, 2, p);
    a.set_value(3, 3, e);
    a.set_value(4, 4, r);

    a.set_value(1, 0, l);
    a.set_value(2, 1, l);
    a.set_value(4, 0, l);
    a.set_value(4, 1, l);

    a.set_value(0, 2, u);
    a.set_value(0, 3, u);
    a.set_value(3, 4, u);
    
    // Make the left-hand side matrix
    let mut a = a.compressed_column_format();
    
    // Make the RHS vector
    let nrhs = 1;
    let rhs = vec![1.0; num_rows];
    let b = DenseMatrix::from_vectors(num_rows, nrhs, rhs);

    let mut stat = CSuperluStat::new();

    let SimpleSolution {
	mut a,
	mut x,
	mut lu,
	column_perm,
	..
    } = SimpleSystem {
	a,
	b,
    }.solve(&mut stat, ColumnPermPolicy::ColAMD)
	.expect("Failed to solve linear system");

    // Print the column permutation
    println!("First solution gave: {:?}", column_perm);

    // Recreate b
    let nrhs = 1;
    let rhs = vec![1.0; num_rows];
    let b = DenseMatrix::from_vectors(num_rows, nrhs, rhs);
    
    // Now solve again with the same pattern
    let SimpleSolution {
	mut a,
	mut x,
	mut lu,
	column_perm,
	..
    } = SamePattern {
	a,
	b,
	column_perm,
    }.solve(&mut stat)
	.expect("Failed to solve linear system");

    // Print the column permutation
    println!("Second solution gave: {:?}", column_perm);
    
    // Print the performance statistics
    stat.print();
    
    // Print solution
    a.print("A");
    lu.print();

    println!("{:?}", a.non_zero_values());
    println!("{:?}", a.column_offsets());
    println!("{:?}", a.row_indices());

    println!("{}", a.value(0, 0));

    x.print("X");
    let x_vals = x.column_major_values();
    println!("{:?}", x_vals);
}
