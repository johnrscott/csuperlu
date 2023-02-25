use csuperlu_sys::SuperMatrix as c_SuperMatrix;

pub trait SuperMatrix {
    fn super_matrix<'a>(&'a self) -> &c_SuperMatrix;

    /// The assumption is that the C library does not
    /// modify the super matrix when it prints. This is
    /// one of the unsafe assumptions of csuperlu.
    fn print(&self, what: &str);
}
