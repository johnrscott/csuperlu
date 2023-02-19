use crate::c::super_matrix::c_SuperMatrix;

pub trait SuperMatrix {
    fn super_matrix<'a>(&'a self) -> &c_SuperMatrix;
    fn print(&self, what: &str);
}
