use std::ops::AddAssign;

use num::Float;

use crate::c::{comp_col::{CompColMat, CompColRaw}, dense::{DenseRaw, DenseMat}, options::SimpleDriverOptions, simple_driver::SimpleDriver, stat::SuperluStat, value_type::ValueType};

use super::SimpleSolution;

fn distance(a: Vec<f64>, b: Vec<f64>) -> f64 {
    if a.len() == b.len() {
	let mut sum = 0.0;
	for n in 0..a.len() {
	    sum += (a[n] - b[n]) * (a[n] - b[n]);
	}
	f64::sqrt(sum)
    } else {
	panic!("a and b must be equal length in nearly_equal()")	
    }
}

fn check_linear_equation_solution(a: CompColRaw<f64>,
				  x_correct: Vec<f64>,
				  b: DenseRaw<f64>) {
    if b.num_cols != 1 {
	panic!("This function is intended for single-column right-hand sides b")
    }

    let a = unsafe {
	CompColMat::from_raw(a)
    }.expect("Expected identity matrix to be valid");
    
    let b = DenseMat::from_raw(b)
        .expect("Expected rhs to be valid");
    
    // Make solver options
    let options = SimpleDriverOptions::new();
    
    // Make solver stats struct
    let mut stats = SuperluStat::new();

    // Solve the system
    let solution = unsafe {
	f64::simple_driver(options,
			   &a,
			   None,
			   b,
			   &mut stats)
    }.expect("The solution should be valid");

    let DenseRaw {
	num_rows,
	num_cols,
	col_maj_vals,
    } = solution.x.to_raw();

    assert!(num_rows == a.num_rows());
    assert!(num_cols == 1);
    assert!(distance(col_maj_vals, x_correct) < 1e-7);
}

#[test]
fn test_2x2_real_identity_solution() {

    // This test checks the solution x to
    //
    //   a       x       b
    // [ 1 0 ] [ 1 ] = [ 1 ]
    // [ 0 1 ] [ 2 ]   [ 2 ] 
    //
    let a = CompColRaw {
        num_rows: 2,
        non_zero_vals: vec![1.0, 1.0],
        row_indices: vec![0, 1],
        col_offsets: vec![0, 1, 2],
    };
    let x_correct = vec![1.0, 2.0];
    let b = DenseRaw {
	num_rows: 2,
	num_cols: 1,
	col_maj_vals: vec![1.0, 2.0],
    };
    check_linear_equation_solution(a, x_correct, b);
}

#[test]
fn test_3x3_real_constant_matrix_solution() {

    // This test checks the solution x to
    //
    //   a              x        b
    // [ 1   -1   0 ] [ 1 ]   [ -1  ]
    // [ 0    0   1 ] [ 2 ] = [  3  ] 
    // [ 1  1.5  -5 ] [ 3 ]   [ -11 ]
    //
    let num_rows = 3;
    let a = CompColRaw {
        num_rows,
        non_zero_vals: vec![1.0, 1.0,
			    -1.0, 1.5,
			    1.0, -5.0],
        row_indices: vec![0, 2,
			  0, 2,
			  1, 2],
        col_offsets: vec![0, 2, 4, 6],
    };
    let x_correct = vec![1.0, 2.0, 3.0];
    let b = DenseRaw {
	num_rows,
	num_cols: 1,
	col_maj_vals: vec![-1.0, 3.0, -11.0],
    };
    check_linear_equation_solution(a, x_correct, b);
}
