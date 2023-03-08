
use super::*;

#[test]
fn empty_matrix() {
    let a = SparseMatrix::<f64>::empty();
    assert_eq!(a.nrows, 0);
    assert_eq!(a.nrows(), 0);
    assert_eq!(a.ncols, 0);
    assert_eq!(a.ncols(), 0);
    assert_eq!(a.values.len(), 0);
    assert_eq!(a.nnz(), 0);
}

#[test]
fn new_matrix() {
    let a = SparseMatrix::<f64>::new(5, 8);
    assert_eq!(a.nrows, 5);
    assert_eq!(a.nrows(), 5);
    assert_eq!(a.ncols, 8);
    assert_eq!(a.ncols(), 8);
    assert_eq!(a.values.len(), 0);
    assert_eq!(a.nnz(), 0);
}

#[test]
fn insert_into_fixed_matrix() {
    let mut a = SparseMatrix::<f64>::new(5, 5);
    a.insert(1, 3, 1.5);
    a.insert(4, 2, 0.4);
    assert_eq!(a.values.len(), 2);
    assert_eq!(a.nnz(), 2);
    assert_eq!(a.get(1, 3), 1.5);
    assert_eq!(a.values[&(1, 3)], 1.5);
    assert_eq!(a.get(4, 2), 0.4);
    for i in 0..5 {
	for j in 0..5 {
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

#[test]
#[should_panic]
fn invalid_read_of_fixed_matrix() {
    let mut a = SparseMatrix::<f64>::new(5, 5);
    a.insert(4, 1, 1.9);
    let _ = a.get(6, 7);
}

#[test]
fn insert_into_dynamic_matrix() {
    let mut a = SparseMatrix::<f64>::empty();
    a.insert_unbounded(1, 3, 1.5);
    a.insert_unbounded(4, 2, 0.4);
    assert_eq!(a.nrows(), 5);
    assert_eq!(a.ncols(), 4);
    assert_eq!(a.nnz(), 2);
    assert_eq!(a.get(1, 3), 1.5);
    assert_eq!(a.values[&(1, 3)], 1.5);
    assert_eq!(a.get(4, 2), 0.4);
    // Check up to 5x5 even though it isn't that size
    for i in 0..5 {
	for j in 0..5 {
	    if !((i == 1 && j == 3) || (i == 4 && j == 2)) {
		assert_eq!(a.get_unbounded(i, j), 0.0);		
	    }
	}
    }
}

#[test]
fn matrix_from_hashmap() {
    let mut a = HashMap::<(usize, usize), f64>::new();
    a.insert((2, 2), 0.5);
    a.insert((3, 1), 1.5);
    a.insert((0, 1), 1.3);
    let b = SparseMatrix::from(a);
    assert_eq!(b.nrows(), 4);
    assert_eq!(b.ncols(), 3);
    assert_eq!(b.nnz(), 3);
    assert_eq!(b.get(2, 2), 0.5);
    assert_eq!(b.get(3, 1), 1.5);
    assert_eq!(b.get(0, 1), 1.3);
}
