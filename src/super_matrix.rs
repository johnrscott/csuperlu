use crate::c::utils::c_SuperMatrix;

pub trait SuperMatrix {
    fn super_matrix<'a>(&'a mut self) -> &mut c_SuperMatrix;
}
