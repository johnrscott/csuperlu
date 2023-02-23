use csuperlu::sparse_matrix::SparseMatrix;

fn main() {
    let mut a = SparseMatrix::new(5, 5);
    a.add_value(1, 1, 0.5);
    a.add_value(2, 1, 1.5);
    println!("{:?}", a);
}

    
