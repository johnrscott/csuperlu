use super::{*, super_matrix::CCompColMat};

#[test]
fn test_comp_col_matrix() {
    let non_zero_vals = vec![1.0, 2.0];
    let row_indices = vec![1, 2];
    let col_offsets = vec![0, 1, 2];
    let a = unsafe {
	CCompColMat::from_vectors(2, non_zero_vals,
				  row_indices, col_offsets)
	    .expect("Failed to create matrix")
    };
    assert_eq!(a.num_cols(), 2);
    assert_eq!(a.num_rows(), 2);
}
