use crate::c::super_matrix::c_SuperMatrix;

pub trait SuperMatrix {
    fn super_matrix<'a>(&'a mut self) -> &mut c_SuperMatrix;
    fn print(&mut self, what: &str);
}
