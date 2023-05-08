
use super::*;

#[test]
fn empty_matrix() {
    let a = SparseMat::<f64>::empty();
    assert_eq!(a.num_rows, 0);
    assert_eq!(a.num_rows(), 0);
    assert_eq!(a.num_cols, 0);
    assert_eq!(a.num_cols(), 0);
    assert_eq!(a.non_zero_vals.len(), 0);
    assert_eq!(a.num_non_zeros(), 0);
}

#[test]
fn new_matrix() {
    let a = SparseMat::<f64>::new(5, 8);
    assert_eq!(a.num_rows, 5);
    assert_eq!(a.num_rows(), 5);
    assert_eq!(a.num_cols, 8);
    assert_eq!(a.num_cols(), 8);
    assert_eq!(a.non_zero_vals.len(), 0);
    assert_eq!(a.num_non_zeros(), 0);
}

#[test]
fn insert_into_fixed_matrix() {
    let mut a = SparseMat::<f64>::new(5, 5);
    a.insert(1, 3, 1.5);
    a.insert(4, 2, 0.4);
    assert_eq!(a.non_zero_vals.len(), 2);
    assert_eq!(a.num_non_zeros(), 2);
    assert_eq!(a.get(1, 3), 1.5);
    assert_eq!(a.non_zero_vals[&(1, 3)], 1.5);
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
    let mut a = SparseMat::<f64>::new(5, 5);
    a.insert(6, 1, 1.9);
}

#[test]
#[should_panic]
fn invalid_read_of_fixed_matrix() {
    let mut a = SparseMat::<f64>::new(5, 5);
    a.insert(4, 1, 1.9);
    let _ = a.get(6, 7);
}

#[test]
fn insert_into_dynamic_matrix() {
    let mut a = SparseMat::<f64>::empty();
    a.insert_unbounded(1, 3, 1.5);
    a.insert_unbounded(4, 2, 0.4);
    assert_eq!(a.num_rows(), 5);
    assert_eq!(a.num_cols(), 4);
    assert_eq!(a.num_non_zeros(), 2);
    assert_eq!(a.get(1, 3), 1.5);
    assert_eq!(a.non_zero_vals[&(1, 3)], 1.5);
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
    let b = SparseMat::from(a.clone());
    assert_eq!(b.num_rows(), 4);
    assert_eq!(b.num_cols(), 3);
    assert_eq!(b.num_non_zeros(), 3);
    assert_eq!(b.get(2, 2), 0.5);
    assert_eq!(b.get(3, 1), 1.5);
    assert_eq!(b.get(0, 1), 1.3);
    assert_eq!(a, b.non_zero_vals().clone());
}

#[test]
fn resize_empty_matrix() {
    let mut a = SparseMat::<f64>::empty();
    a.resize(5, 6);
    assert_eq!(a.num_rows(), 5);
    assert_eq!(a.num_cols(), 6);

    a.resize(3, 4);
    assert_eq!(a.num_rows(), 3);
    assert_eq!(a.num_cols(), 4);

    a.resize(0, 0);
    assert_eq!(a.num_rows(), 0);
    assert_eq!(a.num_cols(), 0);
}

#[test]
fn resize_populated_matrix() {
    let mut a = SparseMat::<f64>::empty();
    a.insert_unbounded(2, 2, 0.5);
    a.insert_unbounded(3, 1, 1.5);
    a.insert_unbounded(0, 1, 1.3);

    a.resize(5, 6);
    assert_eq!(a.num_rows(), 5);
    assert_eq!(a.num_cols(), 6);

    a.resize(4, 3);
    assert_eq!(a.num_rows(), 4);
    assert_eq!(a.num_cols(), 3);
}

#[test]
#[should_panic]
fn invalid_resize_rows() {
    let mut a = SparseMat::<f64>::empty();
    a.insert_unbounded(2, 2, 0.5);
    a.insert_unbounded(3, 1, 1.5);
    a.insert_unbounded(0, 1, 1.3);
    a.resize_rows(1);
}

#[test]
#[should_panic]
fn invalid_resize_cols() {
    let mut a = SparseMat::<f64>::empty();
    a.insert_unbounded(2, 2, 0.5);
    a.insert_unbounded(3, 1, 1.5);
    a.insert_unbounded(0, 1, 1.3);
    a.resize_cols(2);
}

#[test]
fn concatenate_columns() {
    let mut a = SparseMat::empty();
    a.insert_unbounded(2, 2, 0.5);
    a.insert_unbounded(3, 1, 1.5);
    a.insert_unbounded(0, 1, 1.3);

    let mut b = SparseMat::empty();
    b.insert_unbounded(2, 5, 0.2);
    b.insert_unbounded(1, 1, 1.2);
    b.insert_unbounded(3, 1, 2.3);
    b.resize_cols(8);

    let mut c = SparseMat::new(4, 2);
    c.insert(0, 0, 3.6);
    c.insert(1, 1, 11.6);
    c.insert(2, 1, 4.2);
    
    let comb = SparseMat::concat_cols(vec![a, b, c]);
    assert_eq!(comb.get(2, 2), 0.5);
    assert_eq!(comb.get(3, 1), 1.5);
    assert_eq!(comb.get(0, 1), 1.3);
    assert_eq!(comb.get(2, 8), 0.2);
    assert_eq!(comb.get(1, 4), 1.2);
    assert_eq!(comb.get(3, 4), 2.3);
    assert_eq!(comb.get(0, 11), 3.6);
    assert_eq!(comb.get(1, 12), 11.6);
    assert_eq!(comb.get(2, 12), 4.2);
    assert_eq!(comb.num_rows(), 4);
    assert_eq!(comb.num_cols(), 13);
}

#[test]
#[should_panic]
fn invalid_concatenate_columns() {
    let a = SparseMat::<f64>::new(5, 6);
    let b = SparseMat::<f64>::new(10, 6);
    let _comb = SparseMat::concat_cols(vec![a, b]);
}

#[test]
fn concatenate_rows() {
    let mut a = SparseMat::new(2, 5);
    a.insert(1, 1, 0.5);
    a.insert(0, 2, 1.5);
    a.insert(0, 4, 1.3);

    let mut b = SparseMat::new(4, 5);
    b.insert(0, 3, 0.2);
    b.insert(0, 2, 1.2);
    b.insert(0, 4, 2.3);

    let mut c = SparseMat::new(10, 5);
    c.insert(7, 0, 3.6);
    c.insert(4, 1, 11.6);
    c.insert(8, 2, 4.2);

    let comb = SparseMat::concat_rows(vec![a, b, c]);
    assert_eq!(comb.get(1, 1), 0.5);
    assert_eq!(comb.get(0, 2), 1.5);
    assert_eq!(comb.get(0, 4), 1.3);
    assert_eq!(comb.get(2, 3), 0.2);
    assert_eq!(comb.get(2, 2), 1.2);
    assert_eq!(comb.get(2, 4), 2.3);
    assert_eq!(comb.get(13, 0), 3.6);
    assert_eq!(comb.get(10, 1), 11.6);
    assert_eq!(comb.get(14, 2), 4.2);
    assert_eq!(comb.num_rows(), 16);
    assert_eq!(comb.num_cols(), 5);
}

#[test]
#[should_panic]
fn invalid_concatenate_rows() {
    let a = SparseMat::<f64>::new(5, 6);
    let b = SparseMat::<f64>::new(5, 10);
    let _comb = SparseMat::concat_rows(vec![a, b]);
}

#[test]
fn concatenate_both_dimensions() {
    let mut a = SparseMat::<f64>::new(2, 2);
    a.insert(0, 0, 1.0);
    a.insert(1, 1, 2.0);

    let mut b = SparseMat::<f64>::new(2, 3);
    b.insert(0, 0, 1.0);
    b.insert(1, 1, 2.0);

    let mut c = SparseMat::<f64>::new(3, 2);
    c.insert(0, 0, 1.0);
    c.insert(1, 1, 2.0);

    let mut d = SparseMat::<f64>::new(3, 3);
    d.insert(0, 0, 1.0);
    d.insert(1, 1, 2.0);
    d.insert(2, 2, 3.0);

    let comb = SparseMat::concat(vec![vec![a, b],
				      vec![c, d]]);
    assert_eq!(comb.get(0, 0), 1.0);
    assert_eq!(comb.get(1, 1), 2.0);
    assert_eq!(comb.get(0, 2), 1.0);
    assert_eq!(comb.get(1, 3), 2.0);
    assert_eq!(comb.get(2, 0), 1.0);
    assert_eq!(comb.get(3, 1), 2.0);
    assert_eq!(comb.get(2, 2), 1.0);
    assert_eq!(comb.get(3, 3), 2.0);
    assert_eq!(comb.get(4, 4), 3.0);
    assert_eq!(comb.num_cols(), 5);
    assert_eq!(comb.num_rows(), 5);
}

#[test]
fn transpose_matrix() {
    let mut a = SparseMat::new(4, 2);
    a.insert(3, 0, 3.6);
    a.insert(1, 1, 11.6);
    a.insert(2, 1, 4.2);

    let b = a.transpose();
    assert_eq!(b.get(0, 3), 3.6);
    assert_eq!(b.get(1, 1), 11.6);
    assert_eq!(b.get(1, 2), 4.2);
    assert_eq!(b.num_non_zeros(), 3);
    assert_eq!(b.num_rows(), 2);
    assert_eq!(b.num_cols(), 4);
}

#[test]
fn fancy_print() {
    let mut a = SparseMat::<f64>::new(2, 2);
    a.insert(0, 0, 1.0);
    a.insert(1, 1, 2.0);

    let mut b = SparseMat::<f64>::new(2, 3);
    b.insert(0, 0, 1.0);
    b.insert(1, 1, 2.0);

    let mut c = SparseMat::<f64>::new(3, 2);
    c.insert(0, 0, 1.0);
    c.insert(1, 1, 2.0);

    let mut d = SparseMat::<f64>::new(3, 3);
    d.insert(0, 0, 1.0);
    d.insert(1, 1, 2.0);
    d.insert(2, 2, 3.0);

    let comb = SparseMat::concat(vec![vec![a, b],
				      vec![c, d]]);

    comb.print_structure_old(2);
    let mut opts = PrintOptions::default();

    //opts.col_divisions.push(0);
    //opts.col_divisions.push(1);
    opts.col_divisions.push(2);
    //opts.col_divisions.push(3);
    opts.col_divisions.push(4);

    //opts.row_divisions.push(0);
    //opts.row_divisions.push(1);
    opts.row_divisions.push(2);
    //opts.row_divisions.push(3);
    opts.row_divisions.push(4);
    
    comb.print_structure(&opts);
}
