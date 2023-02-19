/// To implement properly
pub fn distance(v1: &[f64], v2: Vec<f64>) -> f64 {
    let mut val = 0.0;
    for n in 0..v2.len() {
	val += (v1[n] - v2[n]) * (v1[n] - v2[n]);
    }
    val
}
