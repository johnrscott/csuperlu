use crate::c::comp_col::CCreateCompColMatrix;
use crate::c::super_node::CSuperNodeMatrixUtils;
use crate::comp_col::CompColMatrix;
use crate::super_node::SuperNodeMatrix;
use crate::super_matrix::SuperMatrix;

pub struct LUDecomp<P>
where P: CSuperNodeMatrixUtils<P> + CCreateCompColMatrix<P> {
    l: SuperNodeMatrix<P>,
    u: CompColMatrix<P>,
}

impl<P> LUDecomp<P>
where P: CSuperNodeMatrixUtils<P> + CCreateCompColMatrix<P> {
    pub fn from_matrices(l: SuperNodeMatrix<P>, u: CompColMatrix<P>) -> Self {
	Self {
	    l,
	    u,
	}
    }
    pub fn print(&mut self) {
	self.l.print("L");
	self.u.print("U");
    }
}
