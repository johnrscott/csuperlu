//! This example is the same as the one in section 2.2 of the SuperLU manual. 
//!
//! From the original source code:
//! " This is the small 5x5 example used in the Sections 2 and 3 of the
//!   Users’ Guide to illustrate how to call a SuperLU routine, and the
//!   matrix data structures used by SuperLU. "
//!
//! Compared to the original C code, this code is much cleaner, and much
//! safer. However, function names similar to SuperLU have been kept to make
//! porting code as easy as possible
//!

extern crate csuperlu;

use csuperlu::superlu::dgssv::{dgssv, LUDecomp};
use csuperlu::superlu::comp_col::CompColMatrix;
use csuperlu::superlu::dense::DenseMatrix;
use csuperlu::superlu::utils::{superlu_options_t, colperm_t,
			       SuperLUStat_t, SuperMatrix};
use std::mem::MaybeUninit;

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
    let mut A = CompColMatrix::new(m, n, nnz, a, asub, xa);    
    
    // Make the RHS vector
    let nrhs = 1;
    let mut rhs = vec![1.0; m as usize];
    let mut B = DenseMatrix::new(m, nrhs, &mut rhs);

    let mut options = superlu_options_t::new();
    options.ColPerm = colperm_t::NATURAL;
    
    let mut perm_r = Vec::<i32>::with_capacity(m as usize);
    let mut perm_c = Vec::<i32>::with_capacity(n as usize);

    let mut stat = SuperLUStat_t::new();    
    let mut info = 0;
    let mut lu_decomp = unsafe {
	let mut L = MaybeUninit::<SuperMatrix>::uninit();
	let mut U = MaybeUninit::<SuperMatrix>::uninit();
	
	dgssv(&mut options, &mut A.super_matrix, perm_c.as_mut_ptr(),
	      perm_r.as_mut_ptr(),
	      L.as_mut_ptr(), U.as_mut_ptr(),
	      &mut B.super_matrix, &mut stat, &mut info);

	LUDecomp {
	    L: SuperNodeMatrix::from_super_matrix(L.assume_init()),
	    U: CompColMatrix::from_super_matrix(U.assume_init()),
	    info
	}
    };

    A.print("A");
    lu_decomp.U.print("U");
    lu_decomp.L.print("L");
    B.print("B");    
}
