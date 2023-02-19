use std::env;
use std::process;

use csuperlu::comp_col::CompColMatrix;
use csuperlu::super_matrix::SuperMatrix;
use csuperlu::c::options::superlu_options_t;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
	println!("Please provide exactly one argument (the matrix file path)");
	process::exit(1);
    }
    
    let file_path = args[1].to_string();
    
    let mut a = CompColMatrix::<f64>::from_harwell_boeing(file_path);
    a.print("a");
    
    let mut _options = superlu_options_t::new();
}
