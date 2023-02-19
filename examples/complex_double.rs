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

use csuperlu::c::options::{colperm_t, superlu_options_t};
use csuperlu::c::stat::{c_StatPrint, SuperLUStat_t};
use csuperlu::comp_col::CompColMatrix;
use csuperlu::dense::DenseMatrix;
use csuperlu::simple_driver::{simple_driver, SimpleSolution};
use csuperlu::super_matrix::SuperMatrix;
use num::Complex;


fn main() {
    // Matrix dimensions
    let m: i32 = 5;
    let n: i32 = 5;

    // Number of non-zeros
    let nnz: i32 = 12;

    // Matrix elements
    let s = Complex::new(19.0, 0.0);
    let u = Complex::new(21.0, 0.0);
    let p = Complex::new(16.0, 0.0);
    let e = Complex::new(5.0, 0.0);
    let r = Complex::new(18.0, 0.0);
    let l = Complex::new(12.0, 0.0);

    // Vector of doubles of length nnz
    let a = vec![s, l, l, u, l, l, u, p, u, e, u, r];

    // Vector of ints of length nnz
    let asub = vec![0, 1, 4, 1, 2, 4, 0, 2, 0, 3, 3, 4];

    // Vector of ints of length n+1
    let xa = vec![0, 3, 6, 8, 10, 12];

    // Make the left-hand side matrix
    let mut a = CompColMatrix::from_vectors(m, n, nnz, a, asub, xa);

    // Make the RHS vector
    let nrhs = 1;
    let rhs = vec![Complex::new(1.0, 0.0); m as usize];
    let b = DenseMatrix::from_vectors(m, nrhs, rhs);

    let mut options = superlu_options_t::new();
    options.ColPerm = colperm_t::NATURAL;

    let mut perm_r = Vec::<i32>::with_capacity(m as usize);
    let mut perm_c = Vec::<i32>::with_capacity(n as usize);

    let mut stat = SuperLUStat_t::new();

    let SimpleSolution {
        mut x,
	mut lu,
        mut info,
    } = simple_driver(options, &mut a, &mut perm_c, &mut perm_r, b, &mut stat);

    // Print the performance statistics
    c_StatPrint(&mut stat);

    // Print solution
    a.print("A");
    lu.print();
    
    println!("{:?}", a.nonzero_values());
    println!("{:?}", a.column_pointers());
    println!("{:?}", a.row_indices());

    println!("{}", a.value(0,0));

    x.print("X");
    let x_vals = x.values();
    println!("{:?}", x_vals);
    
}
