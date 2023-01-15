use crate::c::utils::c_SuperMatrix;

pub trait SuperMatrix {
    fn super_matrix() -> &mut c_SuperMatrix;
}
