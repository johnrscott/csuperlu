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

use std::mem::MaybeUninit;
use csuperlu::c::comp_col::c_dCreate_CompCol_Matrix;
use csuperlu::c::utils::{
    SuperMatrix,
    Stype_t,
    Mtype_t,
    Dtype_t
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
    let mut a = vec![s, l, l, u, l, l, u, p, u, e, u, r];

    // Vector of ints of length nnz
    let mut asub = vec![0, 1, 4, 1, 2, 4, 0, 2, 0, 3, 3, 4];

    // Vector of ints of length n+1
    let mut xa = vec![0, 3, 6, 8, 10, 12];

    // Make the matrix
    let A = unsafe {
	let mut A = MaybeUninit::<SuperMatrix>::uninit();
	c_dCreate_CompCol_Matrix(A.as_mut_ptr(), m, n, nnz,
				 a.as_mut_ptr(), asub.as_mut_ptr(), xa.as_mut_ptr(),
				 Stype_t::SLU_NC, Dtype_t::SLU_D, Mtype_t::SLU_GE);
	A.assume_init()
    };
/*
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
    let (mut L, mut U, mut info) = unsafe {
	let mut L = MaybeUninit::<SuperMatrix>::uninit();
	let mut U = MaybeUninit::<SuperMatrix>::uninit();
	
	dgssv(&mut options, &mut A.super_matrix, perm_c.as_mut_ptr(),
	      perm_r.as_mut_ptr(),
	      L.as_mut_ptr(), U.as_mut_ptr(),
	      &mut B.super_matrix, &mut stat, &mut info);
	
	(
	    L.assume_init(),
	    U.assume_init(),
	    info
	)
    };

    A.print("A");
    U.print("U");
    L.print("L");
    B.print("B");    
*/
}
