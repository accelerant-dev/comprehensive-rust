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

    let rows_a = a.len();
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

    let mut c = vec![vec![0.0; cols_b]; rows_a];

    // Cache-friendly implementation with loop reordering
    for i in 0..rows_a {
        for k in 0..cols_a {
            let a_ik = a[i][k];
            for j in 0..cols_b {
                c[i][j] += a_ik * b[k][j];
            }
        }
    }

    Ok(c)
}
