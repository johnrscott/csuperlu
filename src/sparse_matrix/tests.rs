
use super::*;

#[test]
fn empty_matrix() {
    let a = SparseMatrix::<f64>::empty();
    assert_eq!(a.nrows, 0);
    assert_eq!(a.ncols, 0);
    assert_eq!(a.values.len(), 0);
}


#[test]
fn new_matrix() {
    let a = SparseMatrix::<f64>::new(5, 8);
    assert_eq!(a.nrows, 5);
    assert_eq!(a.ncols, 8);
    assert_eq!(a.values.len(), 0);
}

#[test]
fn insert_into_fixed_matrix() {
    let mut a = SparseMatrix::<f64>::new(5, 5);
    a.insert(1, 3, 1.5);
    a.insert(4, 2, 0.4);
    assert_eq!(a.values.len(), 2);
    assert_eq!(a.get(1, 3), 1.5);
    assert_eq!(a.values[&(1, 3)], 1.5);
    assert_eq!(a.get(4, 2), 0.4);
    for i in 0..4 {
	for j in 0..4 {
	    if !((i == 1 && j == 3) || (i == 4 && j == 2)) {
		assert_eq!(a.get(i, j), 0.0);		
	    }
	}
    }
}

#[test]
#[should_panic]
fn invalid_insert_into_fixed_matrix() {
    let mut a = SparseMatrix::<f64>::new(5, 5);
    a.insert(6, 1, 1.9);
}
