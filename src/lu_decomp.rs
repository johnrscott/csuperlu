use crate::comp_col::CompColMatrix;
use crate::super_node::SuperNodeMatrix;
use crate::c::value_type::ValueType;

#[derive(Debug)]
pub struct LUDecomp<P: ValueType> {
    l: SuperNodeMatrix<P>,
    u: CompColMatrix<P>,
}

impl<P: ValueType> LUDecomp<P> {
    pub fn from_matrices(l: SuperNodeMatrix<P>, u: CompColMatrix<P>) -> Self {
        let l_c_super_matrix = l.super_matrix();
        let u_c_super_matrix = u.super_matrix();
        assert!(
            l_c_super_matrix.num_rows() == u_c_super_matrix.num_rows(),
            "Number of rows in L and U must match"
        );
        assert!(
            l_c_super_matrix.num_columns() == u_c_super_matrix.num_columns(),
            "Number of columns in L and U must match"
        );
        Self { l, u }
    }
    pub fn print(&mut self) {
        self.l.print("L");
        self.u.print("U");
    }
    pub fn value(&mut self, row: usize, col: usize) -> P {
        let l_c_super_matrix = self.l.super_matrix();
        let _u_c_super_matrix = self.u.super_matrix();
        assert!(
            row < l_c_super_matrix.num_rows(),
            "Row index out of range"
        );
        assert!(
            col < l_c_super_matrix.num_columns(),
            "Column index out of range"
        );
        // let _l_c_scformat = unsafe { &mut *(l_c_super_matrix.store::<SCformat>()) };
        // let _u_c_ncformat = unsafe { &mut *(u_c_super_matrix.store::<NCformat>()) };
        todo!("Finish this off later");
    }
}
