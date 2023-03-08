//! This example is the same as the one in section 2.2 of the SuperLU manual.
//!
//! From the original source code:
//! " This is the small 5x5 example used in the Sections 2 and 3 of the
//!   Users’ Guide to illustrate how to call a SuperLU routine, and the
//!   matrix data structures used by SuperLU. "
//!
//! This example shows the equivalent rust code for the example in the
//! user guide.

use csuperlu::dense::DenseMatrix;
use csuperlu::c::options::ColumnPermPolicy;
use csuperlu::simple_driver::{SimpleSystem, SimpleSolution};
use csuperlu::c::stat::CSuperluStat;
use csuperlu::sparse_matrix::SparseMatrix;

fn main() {

    let num_rows = 5usize;
    
    let mut a = SparseMatrix::new();

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
    let a = a.compressed_column_format();

    // Make the RHS vector
    let nrhs = 1;
    let rhs = vec![1.0; num_rows];
    let b = DenseMatrix::from_vectors(num_rows, nrhs, rhs);

    let mut stat = CSuperluStat::new();

    let SimpleSolution {
	mut a,
	mut x,
	mut lu,
	..
    } = SimpleSystem {
	a,
	b,
    }.solve(&mut stat, ColumnPermPolicy::Natural)
	.expect("Failed to solve system");
    
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
