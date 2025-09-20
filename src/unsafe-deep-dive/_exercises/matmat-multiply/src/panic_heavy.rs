pub fn matrix_multiplication(a: &[&[f64]], b: &[&[f64]]) -> Vec<Vec<f64>> {
    if a.is_empty() {
        panic!("Matrix A cannot be empty");
    }
    if b.is_empty() {
        panic!("Matrix B cannot be empty");
    }

    let rows_a = a.len();
    let cols_a = a[0].len();

    if cols_a == 0 {
        panic!("Matrix A must have at least one column");
    }

    let rows_b = b.len();
    let cols_b = b[0].len();

    if cols_b == 0 {
        panic!("Matrix B must have at least one column");
    }

    if cols_a != rows_b {
        panic!(
            "Incompatible matrix dimensions: A has {} columns but B has {} rows",
            cols_a, rows_b
        );
    }

    for (i, row) in a.iter().enumerate() {
        if row.len() != cols_a {
            panic!(
                "Matrix A has inconsistent row lengths: row 0 has {} elements but row {} has {}",
                cols_a,
                i,
                row.len()
            );
        }
    }

    for (i, row) in b.iter().enumerate() {
        if row.len() != cols_b {
            panic!(
                "Matrix B has inconsistent row lengths: row 0 has {} elements but row {} has {}",
                cols_b,
                i,
                row.len()
            );
        }
    }

    let mut c = vec![vec![0.0; cols_b]; rows_a];

    // TODO: fast impl
    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}
