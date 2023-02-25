//! This example is the same as the one in section 2.2 of the SuperLU manual.
//!
//! From the original source code:
//! " This is the small 5x5 example used in the Sections 2 and 3 of the
//!   Users’ Guide to illustrate how to call a SuperLU routine, and the
//!   matrix data structures used by SuperLU. "
//!
//! This code calls all the same functions as the C code, but with names
//! prefixed with c_. However, there are a few differences compared to the
//! C code. First, Rust vectors are used instead of C style vectors (allocated
//! with malloc). Second, Rust requires unsafe blocks whenever a struct is
//! initialised by a function taking the uninitialised struct as an input/output
//! parameter.

use csuperlu::comp_col::CompColMatrix;
use csuperlu::dense::DenseMatrix;
use csuperlu::simple_driver::{ColumnPermPolicy, SimpleSolution, SimpleSystem};
use csuperlu::super_matrix::SuperMatrix;
use csuperlu_sys::stat::{c_StatPrint, SuperLUStat_t};

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
    let mut a = CompColMatrix::from_vectors(num_rows, non_zero_values, row_indices, column_offsets);

    // Make the RHS vector
    let nrhs = 1;
    let rhs = vec![1.0; num_rows];
    let b = DenseMatrix::from_vectors(num_rows, nrhs, rhs);

    let mut stat = SuperLUStat_t::new();

    let SimpleSolution { mut x, mut lu, .. } = SimpleSystem { a: &a, b }
        .solve(&mut stat, ColumnPermPolicy::Natural)
        .expect("Failed to solve linear system");

    // Print the performance statistics
    c_StatPrint(&mut stat);

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
