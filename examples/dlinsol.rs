use std::env;
use std::process;

use csuperlu::comp_col::CompColMatrix;
use csuperlu::dense::DenseMatrix;
use csuperlu::simple_driver::SimpleSystem;
use csuperlu::simple_driver::SimpleSolution;
use csuperlu::super_matrix::SuperMatrix;
use csuperlu::utils::distance;
use csuperlu_sys::options::superlu_options_t;
use csuperlu_sys::stat::SuperLUStat_t;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide exactly one argument (the matrix file path)");
        process::exit(1);
    }

    let file_path = args[1].to_string();

    let mut a = CompColMatrix::<f64>::from_harwell_boeing(file_path);
    let num_rows = a.num_rows();
    let num_columns = a.num_columns();
    a.print("a");

    // Make the true solution vector
    let x_true = vec![1.0; num_rows];

    // Make the RHS vector
    let nrhs = 1;
    let rhs = &mut a * &x_true;
    let b = DenseMatrix::from_vectors(num_rows, nrhs, rhs);

    b.print("b");

    let mut stat = SuperLUStat_t::new();

    let SimpleSolution {
	mut x,
	lu: _,
	column_perm: _,
	row_perm: _,
    } = SimpleSystem {
	a,
	b,
    }.solve(&mut stat).expect("Failed to solve linear system");

    x.print("X");

    // Access the solution matrix
    let solution = x.column_major_values();

    println!("{:?}", solution);
    println!("{}", distance(solution, x_true));
}
