mod c_dgssv;

use std::mem::MaybeUninit;
use crate::superlu::utils::{SuperLUStat_t, superlu_options_t, SuperMatrix,
			    colperm_t};
use crate::superlu::comp_col::CompColMatrix;
use crate::superlu::dense::DenseMatrix;
use crate::superlu::super_node::SuperNodeMatrix;

use c_dgssv::c_dgssv;

#[allow(non_snake_case)]
pub struct LUDecomp {
    pub L: SuperNodeMatrix,
    pub U: CompColMatrix,
    pub info: i32,
    pub stat: SuperLUStat_t,
}

#[allow(non_snake_case)]
pub fn dgssv(A: &mut CompColMatrix, mut b: DenseMatrix) -> LUDecomp {

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
	
    	c_dgssv(&mut options, &mut A.super_matrix, perm_c.as_mut_ptr(),
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
