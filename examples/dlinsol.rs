use std::env;
use std::process;

use csuperlu::comp_col::CompColMatrix;
use csuperlu::dense::DenseMatrix;
use csuperlu::simple_driver::simple_driver;
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

    let options = superlu_options_t::new();

    let mut perm_r = Vec::<i32>::with_capacity(num_rows);
    let mut perm_c = Vec::<i32>::with_capacity(num_columns);

    let mut stat = SuperLUStat_t::new();

    let SimpleSolution { mut x, lu: _ } =
        simple_driver(options, &mut a, &mut perm_c, &mut perm_r, b, &mut stat)
            .expect("Failed to solve linear system");

    x.print("X");

    // Access the solution matrix
    let solution = x.column_major_values();

    println!("{:?}", solution);
    println!("{}", distance(solution, x_true));
}
