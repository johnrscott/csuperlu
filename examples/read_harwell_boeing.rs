use std::env;
use csuperlu::super_matrix::SuperMatrix;

use csuperlu::comp_col::CompColMatrix;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
	println!("Please provide exactly one argument (the matrix file path)");
	process::exit(1);
    }
    
    let file_path = args[1].to_string();

    let mut a = CompColMatrix::<f64>::from_harwell_boeing(file_path);
    a.print("a");
}
