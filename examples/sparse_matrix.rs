use csuperlu::sparse_matrix::SparseMatrix;

fn main() {
    let mut a = SparseMatrix::new(5, 5);
    // Matrix elements
    let s: f64 = 19.0;
    let u: f64 = 21.0;
    let p: f64 = 16.0;
    let e: f64 = 5.0;
    let r: f64 = 18.0;
    let l: f64 = 12.0;
    // Set values
    a.set_value(0, 0, s);
    a.set_value(1, 1, u);
    a.set_value(2, 2, p);
    a.set_value(3, 3, e);
    a.set_value(4, 4, r);

    a.set_value(1, 0, l);
    a.set_value(2, 1, l);
    a.set_value(4, 0, l);
    a.set_value(4, 1, l);

    a.set_value(0, 2, u);
    a.set_value(0, 3, u);
    a.set_value(3, 4, u);

    a.print_sorted();

    println!("{:?}", a);
}
