use thiserror::Error;

/// A matrix provided to `matrix_multiplication`
#[derive(Debug)]
pub enum Input {
    A,
    B,
}

#[derive(Debug, Error)]
pub enum Error {
    /// The given `Input` matrix has no rows.
    #[error("Matrix {0:?} has no rows")]
    NoRows(Input),

    /// The given `Input` matrix has no columns.
    #[error("Matrix {0:?} has no columns")]
    NoColumns(Input),
    /// The number of columns in matrix `a` column does not match the number of rows in matrix `b`.
    #[error(
        "Incompatible matrix dimensions: A has {a_columns} columns but B has {b_rows} rows"
    )]
    DimensionsIncompatible { a_columns: usize, b_rows: usize },

    /// The given matrix has differing number of rows.
    #[error("Matrix {matrix:?} has inconsistent row lengths ({lengths:?})")]
    RowLengthsDiffer { matrix: Input, lengths: Vec<usize> },
}

pub fn matrix_multiplication(
    a: &[&[f64]],
    b: &[&[f64]],
) -> Result<Vec<Vec<f64>>, Error> {
    if a.is_empty() {
        return Err(Error::NoRows(Input::A));
    }
    if b.is_empty() {
        return Err(Error::NoRows(Input::B));
    }

    let _rows_a = a.len();
    let cols_a = a[0].len();

    if cols_a == 0 {
        return Err(Error::NoColumns(Input::A));
    }

    let rows_b = b.len();
    let cols_b = b[0].len();

    if cols_b == 0 {
        return Err(Error::NoColumns(Input::B));
    }

    if cols_a != rows_b {
        return Err(Error::DimensionsIncompatible {
            a_columns: cols_a,
            b_rows: rows_b,
        });
    }

    for row in a.iter() {
        if row.len() != cols_a {
            let lengths: Vec<usize> = a.iter().map(|r| r.len()).collect();
            return Err(Error::RowLengthsDiffer { matrix: Input::A, lengths });
        }
    }

    for row in b.iter() {
        if row.len() != cols_b {
            let lengths: Vec<usize> = b.iter().map(|r| r.len()).collect();
            return Err(Error::RowLengthsDiffer { matrix: Input::B, lengths });
        }
    }

    let c = unsafe { matrix_multiplication_unchecked(a, b) };

    Ok(c)
}

/// Performs matrix multiplication on A and B, returning a new matrix.
///
/// This function implements the standard matrix multiplication algorithm `C = A × B`
/// where `C[i][j] = Σ(A[i][k] × B[k][j])` for all valid `k`.
///
/// ## Safety
///
/// Callers must ensure that the shapes of `a` and `b` comply with the rules of standard linear algebra:
///
/// - Both matrices must be non-empty (have at least one row)
/// - Both matrices must have at least one column in their first row
/// - All rows within each matrix must have the same length (rectangular matrices)
/// - The number of columns in `a` must equal the number of rows in `b`
///
/// Violating these constraints will result in undefined behavior and/or panics.
pub unsafe fn matrix_multiplication_unchecked(
    a: &[&[f64]],
    b: &[&[f64]],
) -> Vec<Vec<f64>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    let mut c = vec![vec![0.0; cols_b]; rows_a];

    for i in 0..rows_a {
        let c_row = &mut c[i];
        for k in 0..cols_a {
            unsafe {
                let a_ik = *a.get_unchecked(i).get_unchecked(k);
                let b_row = b.get_unchecked(k);

                for j in 0..cols_b {
                    *c_row.get_unchecked_mut(j) += a_ik * b_row.get_unchecked(j);
                }
            }
        }
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_multiplication() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];

        let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
        let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

        let result = matrix_multiplication(&a_refs, &b_refs).unwrap();

        assert_eq!(result, vec![vec![19.0, 22.0], vec![43.0, 50.0]]);
    }

    #[test]
    fn test_identity_multiplication() {
        let a = vec![vec![1.0, 2.0, 3.0]];
        let b = vec![vec![1.0], vec![0.0], vec![0.0]];

        let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
        let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

        let result = matrix_multiplication(&a_refs, &b_refs).unwrap();

        assert_eq!(result, vec![vec![1.0]]);
    }

    #[test]
    fn test_no_rows_error() {
        let a: Vec<Vec<f64>> = vec![];
        let b = vec![vec![1.0]];

        let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
        let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

        let result = matrix_multiplication(&a_refs, &b_refs);

        assert!(matches!(result, Err(Error::NoRows(Input::A))));
    }

    #[test]
    fn test_no_columns_error() {
        let a = vec![vec![]];
        let b = vec![vec![1.0]];

        let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
        let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

        let result = matrix_multiplication(&a_refs, &b_refs);

        assert!(matches!(result, Err(Error::NoColumns(Input::A))));
    }

    #[test]
    fn test_dimensions_incompatible_error() {
        let a = vec![vec![1.0, 2.0, 3.0]];
        let b = vec![vec![1.0], vec![2.0]];

        let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
        let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

        let result = matrix_multiplication(&a_refs, &b_refs);

        assert!(matches!(
            result,
            Err(Error::DimensionsIncompatible { a_columns: 3, b_rows: 2 })
        ));
    }

    #[test]
    fn test_row_lengths_differ_error() {
        let a = vec![vec![1.0, 2.0], vec![3.0]];
        let b = vec![vec![1.0], vec![2.0]];

        let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
        let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

        let result = matrix_multiplication(&a_refs, &b_refs);

        assert!(matches!(
            result,
            Err(Error::RowLengthsDiffer { matrix: Input::A, .. })
        ));
    }

    #[test]
    fn test_unsafe_multiplication() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = vec![vec![1.0, 2.0], vec![3.0, 1.0]];

        let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
        let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

        let result = unsafe { matrix_multiplication_unchecked(&a_refs, &b_refs) };

        assert_eq!(result, vec![vec![5.0, 5.0], vec![10.0, 5.0]]);
    }

    #[test]
    fn test_single_element_matrices() {
        let a = vec![vec![5.0]];
        let b = vec![vec![3.0]];

        let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
        let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

        let result = matrix_multiplication(&a_refs, &b_refs).unwrap();

        assert_eq!(result, vec![vec![15.0]]);
    }

    #[test]
    fn test_rectangular_matrices() {
        let a = vec![vec![1.0, 2.0, 3.0]];
        let b = vec![vec![4.0, 5.0], vec![6.0, 7.0], vec![8.0, 9.0]];

        let a_refs: Vec<&[f64]> = a.iter().map(|row| row.as_slice()).collect();
        let b_refs: Vec<&[f64]> = b.iter().map(|row| row.as_slice()).collect();

        let result = matrix_multiplication(&a_refs, &b_refs).unwrap();

        assert_eq!(result, vec![vec![40.0, 46.0]]);
    }
}
