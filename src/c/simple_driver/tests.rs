use std::ops::AddAssign;

use num::{Float, Num, Complex};
use num::Zero;

use crate::c::{comp_col::{CompColMat, CompColRaw}, dense::{DenseRaw, DenseMat}, options::SimpleDriverOptions, simple_driver::SimpleDriver, stat::SuperluStat, value_type::ValueType};

use num::traits::real::Real;
use num::FromPrimitive;

use super::SimpleSolution;

// https://stackoverflow.com/questions/72679829/
// how-to-write-a-macro-to-turn-a-list-of-tuples-
// into-a-vec-of-complex-numbers
fn vec_cmplx<F: Float, const N: usize>(tuples: [(F, F); N]) -> Vec<Complex<F>> {
    tuples
        .into_iter()
        .map(|(re, im)| Complex { re, im })
        .collect()
}

fn distance<P: ValueType>(a: Vec<P>, b: Vec<P>) -> P::RealType {
    if a.len() == b.len() {
	let mut sum = P::RealType::zero();
	for n in 0..a.len() {
	    sum += P::abs(a[n] - b[n]) * P::abs(a[n] - b[n])
	}
	sum.sqrt()
    } else {
	panic!("a and b must be equal length in nearly_equal()")	
    }
}

fn check_linear_equation_solution<P: ValueType>(
    a: CompColRaw<P>,
    x_correct: Vec<P>,
    b: DenseRaw<P>,
    options: Option<SimpleDriverOptions>,
    perm_r: Option<Vec<i32>>) {
    if b.num_cols != 1 {
	panic!("This function is intended for single-column right-hand sides b")
    }

    let a = unsafe {
	CompColMat::from_raw(a)
    }.expect("Expected identity matrix to be valid");
    
    let b = DenseMat::from_raw(b)
        .expect("Expected rhs to be valid");
    
    // Use user-provided options, or make default options
    let options = match options {
	Some(options) => options,
	None => SimpleDriverOptions::new(),
    };
    
    // Make solver stats struct
    let mut stats = SuperluStat::new();
    
    // Solve the system
    let solution = unsafe {
	P::simple_driver(options,
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

    println!("{:?}", col_maj_vals);
    
    assert!(num_rows == a.num_rows());
    assert!(num_cols == 1);
    assert!(distance(col_maj_vals, x_correct) <
	    P::RealType::from_f64(1e-7).unwrap());

    // If a row permutation is passed as an argument, check it too
    if let Some(perm_r_correct) = perm_r {
	println!("Perm_r: {:?}", solution.perm_r);
	println!("Perm_c: {:?}", solution.perm_c);
	assert!(solution.perm_r == perm_r_correct)
    }
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
    check_linear_equation_solution(a, x_correct, b, None, None);
}

#[test]
fn test_3x3_real_matrix_solution() {

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
    check_linear_equation_solution(a, x_correct, b, None, None);
}

#[test]
fn test_2x2_complex_pauli_y_solution() {

    // This test checks the solution x to
    //
    //   a       x       b
    // [ 0 -i ] [ 1 ] = [ -2i ]
    // [ i  0 ] [ 2 ]   [   i ] 
    //
    // where a, x and b are complex
    let a = CompColRaw {
        num_rows: 2,
        non_zero_vals: vec_cmplx([(0.0, 1.0), (0.0, -1.0)]),
        row_indices: vec![1, 0],
        col_offsets: vec![0, 1, 2],
    };
    let x_correct = vec_cmplx([(1.0, 0.0), (2.0, 0.0)]);
    let b = DenseRaw {
	num_rows: 2,
	num_cols: 1,
	col_maj_vals: vec_cmplx([(0.0, -2.0), (0.0, 1.0)]),
    };
    check_linear_equation_solution(a, x_correct, b, None, None);
}

#[test]
fn test_3x3_complex_matrix_solution() {

    // This test checks the solution x to
    //
    //   a                        x        b
    // [ 0     -5+3i    8+7i ] [  1 ]   [ -26+18i ]
    // [ 3-10i     0   -3+9i ] [ 2i ] = [  -9+26i ] 
    // [ 3+i   -8+5i       0 ] [  4 ]   [  -7-15i ]
    //
    let num_rows = 3;
    let a = CompColRaw {
        num_rows,
        non_zero_vals: vec_cmplx([(3.0,-10.0), (3.0,1.0),
				  (-5.0,3.0), (-8.0,5.0),
				  (8.0,7.0), (-3.0,9.0)]),
	row_indices: vec![1, 2,
			  0, 2,
			  0, 1],
        col_offsets: vec![0, 2, 4, 6],
    };
    let x_correct = vec_cmplx([(1.0,0.0), (0.0,2.0), (4.0,0.0)]);
    let b = DenseRaw {
	num_rows,
	num_cols: 1,
	col_maj_vals: vec_cmplx([(26.0,18.0), (-9.0,26.0), (-7.0,-15.0)]),
    };
    check_linear_equation_solution(a, x_correct, b, None, None);
}

#[test]
fn test_4x4_real_row_perm_diagonal_solution() {

    // This test checks the solution x to
    //
    //   a                  x       b
    // [ 0   0   0   2 ] [  1 ]   [  -8 ]
    // [ 0  -1   0   0 ] [ -2 ] = [   2 ] 
    // [ 3   0   0   0 ] [  3 ]   [   3 ]
    // [ 0   0  -8   0 ] [ -4 ]   [ -24 ]
    //
    // Expect row permutation p = [ 3 1 0 2 ] (row
    // n is moved to p[n]). 
    let num_rows = 4;
    let a = CompColRaw {
        num_rows,
        non_zero_vals: vec![3.0, -1.0, -8.0, 2.0],
	row_indices: vec![2, 1, 3, 0],
        col_offsets: vec![0, 1, 2, 3, 4],
    };
    let x_correct = vec![1.0, -2.0, 3.0, -4.0];
    let b = DenseRaw {
	num_rows,
	num_cols: 1,
	col_maj_vals: vec![-8.0, 2.0, 3.0, -24.0],
    };
    let perm_r_correct = vec![3, 1, 0, 2];
    check_linear_equation_solution(a, x_correct, b, None, Some(perm_r_correct));
}
