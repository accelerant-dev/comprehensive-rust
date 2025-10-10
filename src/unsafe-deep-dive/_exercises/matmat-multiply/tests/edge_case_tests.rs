use matmat::panic_heavy::matrix_multiplication;
use matmat::result::{
    Error as ResultError, Input as ResultInput,
    matrix_multiplication as result_matrix_multiplication,
};
use matmat::safe_unsafe::{
    Error as SafeUnsafeError, Input as SafeUnsafeInput,
    matrix_multiplication as safe_unsafe_matrix_multiplication,
};

#[test]
#[should_panic]
fn empty_matrix_a() {
    let a: Vec<&[f64]> = vec![];
    let b = vec![vec![1.0]];
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    matrix_multiplication(&a, &b_refs);
}

#[test]
#[should_panic]
fn empty_matrix_b() {
    let a = vec![vec![1.0]];
    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b: Vec<&[f64]> = vec![];

    matrix_multiplication(&a_refs, &b);
}

#[test]
#[should_panic]
fn empty_row_in_matrix_a() {
    let a = vec![vec![]];
    let b = vec![vec![1.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    matrix_multiplication(&a_refs, &b_refs);
}

#[test]
#[should_panic]
fn empty_row_in_matrix_b() {
    let a = vec![vec![1.0]];
    let b = vec![vec![]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    matrix_multiplication(&a_refs, &b_refs);
}

#[test]
#[should_panic]
fn incompatible_dimensions_simple() {
    let a = vec![vec![1.0, 2.0]];
    let b = vec![vec![3.0], vec![4.0], vec![5.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    matrix_multiplication(&a_refs, &b_refs);
}

#[test]
#[should_panic]
fn incompatible_dimensions_complex() {
    let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let b = vec![vec![7.0, 8.0], vec![9.0, 10.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    matrix_multiplication(&a_refs, &b_refs);
}

#[test]
#[should_panic]
fn jagged_matrix_a() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0]];
    let b = vec![vec![6.0], vec![7.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    matrix_multiplication(&a_refs, &b_refs);
}

#[test]
#[should_panic]
fn jagged_matrix_b() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0], vec![6.0, 7.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    matrix_multiplication(&a_refs, &b_refs);
}

#[test]
#[should_panic]
fn both_matrices_jagged() {
    let a = vec![vec![1.0], vec![2.0, 3.0]];
    let b = vec![vec![4.0, 5.0], vec![6.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    matrix_multiplication(&a_refs, &b_refs);
}

#[test]
#[should_panic(expected = "Matrix A has inconsistent row lengths")]
fn specific_jagged_matrix_a_message() {
    let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0]];
    let b = vec![vec![6.0], vec![7.0], vec![8.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    matrix_multiplication(&a_refs, &b_refs);
}

#[test]
#[should_panic(expected = "Matrix B has inconsistent row lengths")]
fn specific_jagged_matrix_b_message() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0, 6.0, 7.0], vec![8.0, 9.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    matrix_multiplication(&a_refs, &b_refs);
}

#[test]
#[should_panic(expected = "Incompatible matrix dimensions")]
fn specific_incompatible_dimensions_message() {
    let a = vec![vec![1.0, 2.0, 3.0, 4.0]];
    let b = vec![vec![5.0], vec![6.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    matrix_multiplication(&a_refs, &b_refs);
}

#[test]
#[should_panic(expected = "Matrix A cannot be empty")]
fn specific_empty_matrix_a_message() {
    let a: Vec<&[f64]> = vec![];
    let b = vec![vec![1.0]];
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    matrix_multiplication(&a, &b_refs);
}

#[test]
#[should_panic(expected = "Matrix B cannot be empty")]
fn specific_empty_matrix_b_message() {
    let a = vec![vec![1.0]];
    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b: Vec<&[f64]> = vec![];

    matrix_multiplication(&a_refs, &b);
}

#[test]
fn empty_matrix_a_result() {
    let a: Vec<&[f64]> = vec![];
    let b = vec![vec![1.0]];
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = result_matrix_multiplication(&a, &b_refs);
    assert!(matches!(result, Err(ResultError::NoRows(ResultInput::A))));
}

#[test]
fn empty_matrix_b_result() {
    let a = vec![vec![1.0]];
    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b: Vec<&[f64]> = vec![];

    let result = result_matrix_multiplication(&a_refs, &b);
    assert!(matches!(result, Err(ResultError::NoRows(ResultInput::B))));
}

#[test]
fn empty_row_in_matrix_a_result() {
    let a = vec![vec![]];
    let b = vec![vec![1.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = result_matrix_multiplication(&a_refs, &b_refs);
    assert!(matches!(result, Err(ResultError::NoColumns(ResultInput::A))));
}

#[test]
fn empty_row_in_matrix_b_result() {
    let a = vec![vec![1.0]];
    let b = vec![vec![]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = result_matrix_multiplication(&a_refs, &b_refs);
    assert!(matches!(result, Err(ResultError::NoColumns(ResultInput::B))));
}

#[test]
fn incompatible_dimensions_result() {
    let a = vec![vec![1.0, 2.0]];
    let b = vec![vec![3.0], vec![4.0], vec![5.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = result_matrix_multiplication(&a_refs, &b_refs);
    assert!(matches!(
        result,
        Err(ResultError::DimensionsIncompatible { a_columns: 2, b_rows: 3 })
    ));
}

#[test]
fn jagged_matrix_a_result() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0]];
    let b = vec![vec![6.0], vec![7.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = result_matrix_multiplication(&a_refs, &b_refs);
    assert!(matches!(
        result,
        Err(ResultError::RowLengthsDiffer { matrix: ResultInput::A, .. })
    ));
}

#[test]
fn jagged_matrix_b_result() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0], vec![6.0, 7.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = result_matrix_multiplication(&a_refs, &b_refs);
    assert!(matches!(
        result,
        Err(ResultError::RowLengthsDiffer { matrix: ResultInput::B, .. })
    ));
}

#[test]
fn empty_matrix_a_safe_unsafe() {
    let a: Vec<&[f64]> = vec![];
    let b = vec![vec![1.0]];
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = safe_unsafe_matrix_multiplication(&a, &b_refs);
    assert!(matches!(result, Err(SafeUnsafeError::NoRows(SafeUnsafeInput::A))));
}

#[test]
fn empty_matrix_b_safe_unsafe() {
    let a = vec![vec![1.0]];
    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b: Vec<&[f64]> = vec![];

    let result = safe_unsafe_matrix_multiplication(&a_refs, &b);
    assert!(matches!(result, Err(SafeUnsafeError::NoRows(SafeUnsafeInput::B))));
}

#[test]
fn empty_row_in_matrix_a_safe_unsafe() {
    let a = vec![vec![]];
    let b = vec![vec![1.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = safe_unsafe_matrix_multiplication(&a_refs, &b_refs);
    assert!(matches!(result, Err(SafeUnsafeError::NoColumns(SafeUnsafeInput::A))));
}

#[test]
fn empty_row_in_matrix_b_safe_unsafe() {
    let a = vec![vec![1.0]];
    let b = vec![vec![]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = safe_unsafe_matrix_multiplication(&a_refs, &b_refs);
    assert!(matches!(result, Err(SafeUnsafeError::NoColumns(SafeUnsafeInput::B))));
}

#[test]
fn incompatible_dimensions_safe_unsafe() {
    let a = vec![vec![1.0, 2.0]];
    let b = vec![vec![3.0], vec![4.0], vec![5.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = safe_unsafe_matrix_multiplication(&a_refs, &b_refs);
    assert!(matches!(
        result,
        Err(SafeUnsafeError::DimensionsIncompatible { a_columns: 2, b_rows: 3 })
    ));
}

#[test]
fn jagged_matrix_a_safe_unsafe() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0]];
    let b = vec![vec![6.0], vec![7.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = safe_unsafe_matrix_multiplication(&a_refs, &b_refs);
    assert!(matches!(
        result,
        Err(SafeUnsafeError::RowLengthsDiffer { matrix: SafeUnsafeInput::A, .. })
    ));
}

#[test]
fn jagged_matrix_b_safe_unsafe() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0], vec![6.0, 7.0]];

    let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
    let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

    let result = safe_unsafe_matrix_multiplication(&a_refs, &b_refs);
    assert!(matches!(
        result,
        Err(SafeUnsafeError::RowLengthsDiffer { matrix: SafeUnsafeInput::B, .. })
    ));
}
