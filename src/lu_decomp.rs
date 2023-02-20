use crate::csuperlu_sys::super_matrix::{c_SCformat, c_NCformat};
use crate::comp_col::CompColMatrix;
use crate::super_node::SuperNodeMatrix;
use crate::super_matrix::SuperMatrix;
use crate::value_type::ValueType;

pub struct LUDecomp<P: ValueType<P>> {
    l: SuperNodeMatrix<P>,
    u: CompColMatrix<P>,
}

impl<P: ValueType<P>> LUDecomp<P> {
    pub fn from_matrices(l: SuperNodeMatrix<P>, u: CompColMatrix<P>) -> Self {
	let l_c_super_matrix = l.super_matrix();
	let u_c_super_matrix = u.super_matrix();
	assert!(l_c_super_matrix.nrow == u_c_super_matrix.nrow,
		"Number of rows in L and U must match");
	assert!(l_c_super_matrix.ncol == u_c_super_matrix.ncol,
		"Number of columns in L and U must match");
	Self {
	    l,
	    u,
	}
    }
    pub fn print(&mut self) {
	self.l.print("L");
	self.u.print("U");
    }
    pub fn value(&mut self, row: usize, col: usize) -> P {
	let l_c_super_matrix = self.l.super_matrix();
	let u_c_super_matrix = self.u.super_matrix();
	assert!(row < l_c_super_matrix.nrow as usize,
		"Row index out of range");
	assert!(col < l_c_super_matrix.ncol as usize,
		"Column index out of range");
	let _l_c_scformat = unsafe {
	    &mut *(l_c_super_matrix.Store as *mut c_SCformat)
	};
	let _u_c_ncformat = unsafe {
	    &mut *(u_c_super_matrix.Store as *mut c_NCformat)
	};
	todo!("Finish this off later");
    }
}
