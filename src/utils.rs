use crate::{comp_col::CompColMatrix, c::comp_col::CCompColMatrix};

/// Todo implement properly
pub fn distance(v1: &[f64], v2: Vec<f64>) -> f64 {
    let mut val = 0.0;
    for n in 0..v2.len() {
	val += (v1[n] - v2[n]) * (v1[n] - v2[n]);
    }
    val
}

/// Naive matrix multiplication which loops over all
/// each full row of the sparse matrix and adds up the
/// results.
pub fn multiply<P: CCompColMatrix<P>>(a: &mut CompColMatrix<P>, x: &Vec<P>)
				      -> Vec<P> {
    assert!(a.num_columns() == x.len(), "Cannot multiply; incompatible dimensions");
    let mut b = Vec::<P>::new();
    for row in 0..a.num_rows() {
	let mut value = P::zero();
	for column in 0..a.num_columns() {
	    value = value + (a.value(row, column) * x[row]);
	}
	b.push(value);
    }
    b
}
