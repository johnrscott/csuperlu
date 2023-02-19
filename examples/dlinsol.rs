use std::env;
use std::process;

use csuperlu::c::options::colperm_t;
use csuperlu::c::stat::SuperLUStat_t;
use csuperlu::comp_col::CompColMatrix;
use csuperlu::dense::DenseMatrix;
use csuperlu::simple_driver::SimpleSolution;
use csuperlu::simple_driver::simple_driver;
use csuperlu::super_matrix::SuperMatrix;
use csuperlu::c::options::superlu_options_t;
use csuperlu::utils::distance;
use csuperlu::utils::multiply;

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
    let rhs = multiply(&mut a, &x_true);
    let mut b = DenseMatrix::from_vectors(num_rows, nrhs, rhs);

    b.print("b");

    let mut options = superlu_options_t::new();
    //options.ColPerm = colperm_t::NATURAL;

    let mut perm_r = Vec::<i32>::with_capacity(num_rows);
    let mut perm_c = Vec::<i32>::with_capacity(num_columns);

    let mut stat = SuperLUStat_t::new();

    let SimpleSolution {
        mut x,
	mut lu,
    } = simple_driver(options, &mut a, &mut perm_c, &mut perm_r, b, &mut stat)
        .expect("Failed to solve linear system");

    
    // Access the solution matrix
    // let solution = x.values();

    // println!("{:?}", solution);
    // println!("{}", distance(solution, known_true));
}
