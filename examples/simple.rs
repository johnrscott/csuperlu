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

use csuperlu::c::comp_col::{
    c_Destroy_CompCol_Matrix,
};
use csuperlu::c::dense::c_Destroy_SuperMatrix_Store;
use csuperlu::c::utils::{
    Stype_t,
    Mtype_t,
    Dtype_t,
};
use csuperlu::c::super_node::{
    c_dPrint_SuperNode_Matrix,
    c_Destroy_SuperNode_Matrix,
};
use csuperlu::c::stat::{
    SuperLUStat_t,
    c_StatPrint,
};
use csuperlu::c::options::{
    superlu_options_t,
    colperm_t,
};
use csuperlu::comp_col::{
    dCreate_CompCol_Matrix,
    dPrint_CompCol_Matrix,
};
use csuperlu::dense::dCreate_Dense_Matrix;
use csuperlu::simple_driver::{
    dgssv,
    DgssvSolution,
};

fn main() {

    // Matrix dimensions
    let m: i32 = 5;
    let n: i32 = 5;

    // Number of non-zeros
    let nnz: i32 = 12;

    // Matrix elements
    let s: f64 = 19.0;
    let u: f64 = 21.0;
    let p: f64 = 16.0;
    let e: f64 = 5.0;
    let r: f64 = 18.0;
    let l: f64 = 12.0;
    
    // Vector of doubles of length nnz
    let a = vec![s, l, l, u, l, l, u, p, u, e, u, r];

    // Vector of ints of length nnz
    let asub = vec![0, 1, 4, 1, 2, 4, 0, 2, 0, 3, 3, 4];

    // Vector of ints of length n+1
    let xa = vec![0, 3, 6, 8, 10, 12];
    
    // Make the matrix
    let mut A = dCreate_CompCol_Matrix(m, n, nnz, a, asub, xa,
     				       Stype_t::SLU_NC, Dtype_t::SLU_D,
     				       Mtype_t::SLU_GE);
    
    // Make the RHS vector
    let nrhs = 1;
    let mut rhs = vec![1.0; m as usize];
    let B = dCreate_Dense_Matrix(m, nrhs, &mut rhs, m,
     				 Stype_t::SLU_DN, Dtype_t::SLU_D,
     				 Mtype_t::SLU_GE);
    
    let mut options = superlu_options_t::new();
    options.ColPerm = colperm_t::NATURAL;
    
    let perm_r = Vec::<i32>::with_capacity(m as usize);
    let perm_c = Vec::<i32>::with_capacity(n as usize);

    let stat = SuperLUStat_t::new();

    let DgssvSolution {
	mut X,
	mut L,
	mut U,
	mut stat,
	mut info
    } = dgssv(options, &mut A, perm_c, perm_r, B, stat);
    
    // Print the performance statistics
    c_StatPrint(&mut stat);

    dPrint_CompCol_Matrix("A", &mut A);
    dPrint_CompCol_Matrix("U", &mut U);

    let c_str = std::ffi::CString::new("L").unwrap();
    c_dPrint_SuperNode_Matrix(c_str.as_ptr() as *mut libc::c_char, &mut L);

    c_Destroy_CompCol_Matrix(&mut A);
    c_Destroy_SuperMatrix_Store(&mut X);
    c_Destroy_SuperNode_Matrix(&mut L);
    c_Destroy_CompCol_Matrix(&mut U);
}
