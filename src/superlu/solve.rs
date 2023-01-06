use libc;
use std::mem::MaybeUninit;
use crate::superlu::utils::{SuperLUStat_t, superlu_options_t, SuperMatrix,
			    colperm_t};
use crate::superlu::comp_col::CompColMatrix;
use crate::superlu::dense::DenseMatrix;
use crate::superlu::super_node::SuperNodeMatrix;

#[link(name = "superlu")]
extern {
    fn dgssv(options: *mut superlu_options_t,
	     A: *mut SuperMatrix,
	     perm_c: *mut libc::c_int,
	     perm_r: *mut libc::c_int,
	     L: *mut SuperMatrix,
	     U: *mut SuperMatrix,
	     B: *mut SuperMatrix,
	     stat: *mut SuperLUStat_t,
	     info: *mut libc::c_int);
}

#[allow(non_snake_case)]
pub struct LUDecomp {
    pub L: SuperNodeMatrix,
    pub U: CompColMatrix,
    pub info: i32,
    pub stat: SuperLUStat_t,
}

#[allow(non_snake_case)]
pub fn dgssv_solve(A: &mut CompColMatrix, mut b: DenseMatrix) -> LUDecomp {

    let mut options = superlu_options_t::new();
    options.ColPerm = colperm_t::NATURAL;

    let num_rows = A.super_matrix.num_rows();
    let num_cols = A.super_matrix.num_cols();
    let mut perm_r = Vec::<i32>::with_capacity(num_rows);
    let mut perm_c = Vec::<i32>::with_capacity(num_cols);
    let mut stat = SuperLUStat_t::new();    
    let mut info = 0;
    unsafe {
    	let mut L = MaybeUninit::<SuperMatrix>::uninit();
    	let mut U = MaybeUninit::<SuperMatrix>::uninit();
	
    	dgssv(&mut options, &mut A.super_matrix, perm_c.as_mut_ptr(),
    	      perm_r.as_mut_ptr(),
    	      L.as_mut_ptr(), U.as_mut_ptr(),
    	      &mut b.super_matrix, &mut stat, &mut info);

    	LUDecomp {
    	    L: SuperNodeMatrix::from_super_matrix(L.assume_init()),
    	    U: CompColMatrix::from_super_matrix(U.assume_init()),
    	    info,
    	    stat,
    	}
    }
}
