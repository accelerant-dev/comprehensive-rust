use matmat::panic_heavy::matrix_multiplication;

#[test]
fn basic_2x2_multiplication() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = matrix_multiplication(&a_refs, &b_refs);

    let expected = vec![vec![19.0, 22.0], vec![43.0, 50.0]];
    assert_eq!(result, expected);
}

#[test]
fn three_by_three_multiplication() {
    let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0]];
    let b = vec![vec![9.0, 8.0, 7.0], vec![6.0, 5.0, 4.0], vec![3.0, 2.0, 1.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = matrix_multiplication(&a_refs, &b_refs);

    let expected = vec![
        vec![30.0, 24.0, 18.0],
        vec![84.0, 69.0, 54.0],
        vec![138.0, 114.0, 90.0],
    ];
    assert_eq!(result, expected);
}

#[test]
fn rectangular_matrices() {
    let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let b = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = matrix_multiplication(&a_refs, &b_refs);

    let expected = vec![vec![58.0, 64.0], vec![139.0, 154.0]];
    assert_eq!(result, expected);
}

#[test]
fn identity_matrix() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let identity = vec![vec![1.0, 0.0], vec![0.0, 1.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let id_refs: Vec<&[f64]> = identity.iter().map(|row| row.as_slice()).collect();

    let result = matrix_multiplication(&a_refs, &id_refs);

    assert_eq!(result, a);
}

#[test]
fn zero_matrix() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let zero = vec![vec![0.0, 0.0], vec![0.0, 0.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let zero_refs: Vec<&[f64]> = zero.iter().map(|row| row.as_slice()).collect();

    let result = matrix_multiplication(&a_refs, &zero_refs);

    let expected = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
    assert_eq!(result, expected);
}

#[test]
fn single_element_matrices() {
    let a = vec![vec![3.0]];
    let b = vec![vec![4.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = matrix_multiplication(&a_refs, &b_refs);

    let expected = vec![vec![12.0]];
    assert_eq!(result, expected);
}

#[test]
fn vector_multiplication() {
    let a = vec![vec![1.0, 2.0, 3.0]];
    let b = vec![vec![4.0], vec![5.0], vec![6.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = matrix_multiplication(&a_refs, &b_refs);

    let expected = vec![vec![32.0]];
    assert_eq!(result, expected);
}

#[test]
fn negative_numbers() {
    let a = vec![vec![-1.0, 2.0], vec![3.0, -4.0]];
    let b = vec![vec![5.0, -6.0], vec![-7.0, 8.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = matrix_multiplication(&a_refs, &b_refs);

    let expected = vec![vec![-19.0, 22.0], vec![43.0, -50.0]];
    assert_eq!(result, expected);
}

#[test]
fn fractional_numbers() {
    let a = vec![vec![0.5, 1.5], vec![2.5, 3.5]];
    let b = vec![vec![0.1, 0.2], vec![0.3, 0.4]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = matrix_multiplication(&a_refs, &b_refs);

    let expected = vec![vec![0.5, 0.7], vec![1.3, 1.9]];

    for (i, row) in result.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            assert!((val - expected[i][j]).abs() < 1e-10);
        }
    }
}
