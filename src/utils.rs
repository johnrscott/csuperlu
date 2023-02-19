use crate::c::comp_col::CCompColMatrix;

/// Todo implement properly
pub fn distance<P: CCompColMatrix<P>>(v1: &[P], v2: Vec<P>) -> P {
    let mut value = P::zero();
    for n in 0..v2.len() {
	value = value + (v1[n] - v2[n]) * (v1[n] - v2[n]);
    }
    value
}
