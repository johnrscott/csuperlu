//! This example shows how to reuse the column permutation
//! and row permutation from a previous solution. Do this if
//! the next matrix $A$ has similar values to the previous $A$

use csuperlu::comp_col::CompColMatrix;
use csuperlu::dense::DenseMatrix;
use csuperlu::c::options::ColumnPermPolicy;
use csuperlu::simple_driver::{SimpleSolution, SimpleSystem, SamePattern, SimilarValues};
use csuperlu::c::stat::CSuperluStat;

fn main() {
    // Matrix dimensions
    let num_rows = 5usize;
    let num_columns = 5usize;

    // Matrix elements
    let s: f64 = 19.0;
    let u: f64 = 21.0;
    let p: f64 = 16.0;
    let e: f64 = 5.0;
    let r: f64 = 18.0;
    let l: f64 = 12.0;

    // Vector of doubles of length nnz
    let non_zero_values = vec![s, l, l, u, l, l, u, p, u, e, u, r];

    // Vector of ints of length nnz
    let row_indices = vec![0, 1, 4, 1, 2, 4, 0, 2, 0, 3, 3, 4];

    // Vector of ints of length num_columns + 1
    let column_offsets = vec![0, 3, 6, 8, 10, 12];

    // Make the left-hand side matrix
    let a = CompColMatrix::from_vectors(num_rows, non_zero_values, row_indices, column_offsets);

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
	row_perm,
	..
    } = SimpleSystem {
	a,
	b,
    }.solve(&mut stat, ColumnPermPolicy::ColAMD)
	.expect("Failed to solve linear system");

    println!("First solution gave Pc = {:?}", column_perm);
    println!("First solution gave Pr = {:?}", row_perm);

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
	row_perm,
	..
    } = SimilarValues {
	a,
	b,
	column_perm,
	row_perm,
    }.solve(&mut stat)
	.expect("Failed to solve linear system");

    println!("Second solution gave Pc = {:?}", column_perm);
    println!("Second solution gave Pr = {:?}", row_perm);
    
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
