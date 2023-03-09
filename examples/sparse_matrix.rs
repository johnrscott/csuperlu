use csuperlu::sparse_matrix::SparseMat;

fn main() {
    let mut a = SparseMat::new(5, 5);
    // Matrix elements
    let s: f64 = 19.0;
    let u: f64 = 21.0;
    let p: f64 = 16.0;
    let e: f64 = 5.0;
    let r: f64 = 18.0;
    let l: f64 = 12.0;
    // Set values
    a.insert(0, 0, s);
    a.insert(1, 1, u);
    a.insert(2, 2, p);
    a.insert(3, 3, e);
    a.insert(4, 4, r);

    a.insert(1, 0, l);
    a.insert(2, 1, l);
    a.insert(4, 0, l);
    a.insert(4, 1, l);

    a.insert(0, 2, u);
    a.insert(0, 3, u);
    a.insert(3, 4, u);
    a.insert(4, 3, 1.0);
    a.insert(4, 3, 0.0);
    
    println!("{a}");
    println!("{}x{}: {}", a.num_rows(), a.num_cols(), a.num_non_zeros());
    println!("{:?}", a.non_zero_vals());
    a.print_structure(3);
    
    let ccf = a.compressed_column_format();
    ccf.print("CCF");
    
}
