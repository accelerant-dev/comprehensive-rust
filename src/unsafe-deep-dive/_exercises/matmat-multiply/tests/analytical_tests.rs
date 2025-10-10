use matmat::panic_heavy::matrix_multiplication;
use matmat::result::matrix_multiplication as result_matrix_multiplication;
use matmat::safe_unsafe::matrix_multiplication as safe_unsafe_matrix_multiplication;

const EPSILON: f64 = 1e-12;

fn assert_matrices_equal(a: &[Vec<f64>], b: &[Vec<f64>]) {
    assert_eq!(a.len(), b.len(), "Matrix row count mismatch");
    for (i, (row_a, row_b)) in a.iter().zip(b.iter()).enumerate() {
        assert_eq!(row_a.len(), row_b.len(), "Row {} length mismatch", i);
        for (j, (&val_a, &val_b)) in row_a.iter().zip(row_b.iter()).enumerate() {
            assert!(
                (val_a - val_b).abs() < EPSILON,
                "Matrix element mismatch at ({}, {}): {} != {}",
                i,
                j,
                val_a,
                val_b
            );
        }
    }
}

fn matrix_to_refs(matrix: &[Vec<f64>]) -> Vec<&[f64]> {
    matrix.iter().map(|row| row.as_slice()).collect()
}

#[test]
fn associativity_property() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    let c = vec![vec![9.0, 10.0], vec![11.0, 12.0]];

    let a_refs = matrix_to_refs(&a);
    let b_refs = matrix_to_refs(&b);
    let c_refs = matrix_to_refs(&c);

    let ab = matrix_multiplication(&a_refs, &b_refs);
    let ab_refs = matrix_to_refs(&ab);
    let abc_left = matrix_multiplication(&ab_refs, &c_refs);

    let bc = matrix_multiplication(&b_refs, &c_refs);
    let bc_refs = matrix_to_refs(&bc);
    let abc_right = matrix_multiplication(&a_refs, &bc_refs);

    assert_matrices_equal(&abc_left, &abc_right);
}

#[test]
fn distributivity_over_addition() {
    let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let b = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];
    let c = vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![3.0, 3.0]];

    let a_refs = matrix_to_refs(&a);
    let b_refs = matrix_to_refs(&b);
    let c_refs = matrix_to_refs(&c);

    let ab = matrix_multiplication(&a_refs, &b_refs);
    let ac = matrix_multiplication(&a_refs, &c_refs);

    let mut b_plus_c = vec![vec![0.0; 2]; 3];
    for i in 0..3 {
        for j in 0..2 {
            b_plus_c[i][j] = b[i][j] + c[i][j];
        }
    }

    let bc_refs = matrix_to_refs(&b_plus_c);
    let a_bc = matrix_multiplication(&a_refs, &bc_refs);

    let mut ab_plus_ac = vec![vec![0.0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            ab_plus_ac[i][j] = ab[i][j] + ac[i][j];
        }
    }

    assert_matrices_equal(&a_bc, &ab_plus_ac);
}

#[test]
fn scalar_multiplication_property() {
    let scalar = 3.0;
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];

    let a_refs = matrix_to_refs(&a);
    let b_refs = matrix_to_refs(&b);

    let mut scalar_a = vec![vec![0.0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            scalar_a[i][j] = scalar * a[i][j];
        }
    }

    let scalar_a_refs = matrix_to_refs(&scalar_a);
    let scalar_a_times_b = matrix_multiplication(&scalar_a_refs, &b_refs);

    let ab = matrix_multiplication(&a_refs, &b_refs);
    let mut scalar_ab = vec![vec![0.0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            scalar_ab[i][j] = scalar * ab[i][j];
        }
    }

    assert_matrices_equal(&scalar_a_times_b, &scalar_ab);
}

#[test]
fn transpose_property() {
    let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let b = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];

    let a_refs = matrix_to_refs(&a);
    let b_refs = matrix_to_refs(&b);

    let ab = matrix_multiplication(&a_refs, &b_refs);

    let a_transpose = vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]];
    let b_transpose = vec![vec![7.0, 9.0, 11.0], vec![8.0, 10.0, 12.0]];

    let bt_refs = matrix_to_refs(&b_transpose);
    let at_refs = matrix_to_refs(&a_transpose);
    let bt_at = matrix_multiplication(&bt_refs, &at_refs);

    let ab_transpose = vec![vec![ab[0][0], ab[1][0]], vec![ab[0][1], ab[1][1]]];

    assert_matrices_equal(&bt_at, &ab_transpose);
}

#[test]
fn identity_multiplication_property() {
    for n in 1..=5 {
        let mut identity = vec![vec![0.0; n]; n];
        for i in 0..n {
            identity[i][i] = 1.0;
        }

        let mut test_matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                test_matrix[i][j] = (i * n + j + 1) as f64;
            }
        }

        let id_refs = matrix_to_refs(&identity);
        let test_refs = matrix_to_refs(&test_matrix);

        let left_mult = matrix_multiplication(&id_refs, &test_refs);
        let right_mult = matrix_multiplication(&test_refs, &id_refs);

        assert_matrices_equal(&left_mult, &test_matrix);
        assert_matrices_equal(&right_mult, &test_matrix);
    }
}

#[test]
fn zero_multiplication_property() {
    for (m, n, p) in [(2, 3, 4), (1, 1, 1), (3, 2, 5)] {
        let mut test_matrix = vec![vec![0.0; n]; m];
        for i in 0..m {
            for j in 0..n {
                test_matrix[i][j] = (i * n + j + 1) as f64;
            }
        }

        let zero_left = vec![vec![0.0; n]; m];
        let zero_right = vec![vec![0.0; p]; n];

        let test_refs = matrix_to_refs(&test_matrix);
        let zero_left_refs = matrix_to_refs(&zero_left);
        let zero_right_refs = matrix_to_refs(&zero_right);

        let zero_result_left =
            matrix_multiplication(&zero_left_refs, &zero_right_refs);
        let zero_result_right = matrix_multiplication(&test_refs, &zero_right_refs);

        let expected_zero = vec![vec![0.0; p]; m];
        assert_matrices_equal(&zero_result_left, &expected_zero);
        assert_matrices_equal(&zero_result_right, &expected_zero);
    }
}

#[test]
fn block_matrix_multiplication() {
    let a11 = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let a12 = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    let a21 = vec![vec![9.0, 10.0], vec![11.0, 12.0]];
    let a22 = vec![vec![13.0, 14.0], vec![15.0, 16.0]];

    let b11 = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    let b12 = vec![vec![2.0, 3.0], vec![4.0, 5.0]];
    let b21 = vec![vec![6.0, 7.0], vec![8.0, 9.0]];
    let b22 = vec![vec![10.0, 11.0], vec![12.0, 13.0]];

    let full_a = vec![
        vec![1.0, 2.0, 5.0, 6.0],
        vec![3.0, 4.0, 7.0, 8.0],
        vec![9.0, 10.0, 13.0, 14.0],
        vec![11.0, 12.0, 15.0, 16.0],
    ];

    let full_b = vec![
        vec![1.0, 0.0, 2.0, 3.0],
        vec![0.0, 1.0, 4.0, 5.0],
        vec![6.0, 7.0, 10.0, 11.0],
        vec![8.0, 9.0, 12.0, 13.0],
    ];

    let full_a_refs = matrix_to_refs(&full_a);
    let full_b_refs = matrix_to_refs(&full_b);
    let full_result = matrix_multiplication(&full_a_refs, &full_b_refs);

    let a11_refs = matrix_to_refs(&a11);
    let a12_refs = matrix_to_refs(&a12);
    let _a21_refs = matrix_to_refs(&a21);
    let _a22_refs = matrix_to_refs(&a22);
    let b11_refs = matrix_to_refs(&b11);
    let b12_refs = matrix_to_refs(&b12);
    let b21_refs = matrix_to_refs(&b21);
    let b22_refs = matrix_to_refs(&b22);

    let c11_part1 = matrix_multiplication(&a11_refs, &b11_refs);
    let c11_part2 = matrix_multiplication(&a12_refs, &b21_refs);
    let mut c11 = vec![vec![0.0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            c11[i][j] = c11_part1[i][j] + c11_part2[i][j];
        }
    }

    let c12_part1 = matrix_multiplication(&a11_refs, &b12_refs);
    let c12_part2 = matrix_multiplication(&a12_refs, &b22_refs);
    let mut c12 = vec![vec![0.0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            c12[i][j] = c12_part1[i][j] + c12_part2[i][j];
        }
    }

    for i in 0..2 {
        for j in 0..2 {
            assert!((full_result[i][j] - c11[i][j]).abs() < EPSILON);
            assert!((full_result[i][j + 2] - c12[i][j]).abs() < EPSILON);
        }
    }
}

#[test]
fn rank_one_matrix_multiplication() {
    let u = vec![vec![1.0], vec![2.0], vec![3.0]];
    let v_t = vec![vec![4.0, 5.0, 6.0]];

    let u_refs = matrix_to_refs(&u);
    let vt_refs = matrix_to_refs(&v_t);

    let rank_one = matrix_multiplication(&u_refs, &vt_refs);

    for i in 0..3 {
        for j in 0..3 {
            let expected = u[i][0] * v_t[0][j];
            assert!(
                (rank_one[i][j] - expected).abs() < EPSILON,
                "Rank-1 matrix element ({}, {}) mismatch: {} != {}",
                i,
                j,
                rank_one[i][j],
                expected
            );
        }
    }
}

#[test]
fn commutative_diagonal_matrices() {
    let diag_a = vec![vec![2.0, 0.0, 0.0], vec![0.0, 3.0, 0.0], vec![0.0, 0.0, 4.0]];

    let diag_b = vec![vec![5.0, 0.0, 0.0], vec![0.0, 6.0, 0.0], vec![0.0, 0.0, 7.0]];

    let a_refs = matrix_to_refs(&diag_a);
    let b_refs = matrix_to_refs(&diag_b);

    let ab = matrix_multiplication(&a_refs, &b_refs);
    let ba = matrix_multiplication(&b_refs, &a_refs);

    assert_matrices_equal(&ab, &ba);

    for i in 0..3 {
        let expected = diag_a[i][i] * diag_b[i][i];
        assert!((ab[i][i] - expected).abs() < EPSILON);
    }
}

#[test]
fn associativity_property_result() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    let c = vec![vec![9.0, 10.0], vec![11.0, 12.0]];

    let a_refs = matrix_to_refs(&a);
    let b_refs = matrix_to_refs(&b);
    let c_refs = matrix_to_refs(&c);

    let ab = result_matrix_multiplication(&a_refs, &b_refs).unwrap();
    let ab_refs = matrix_to_refs(&ab);
    let abc_left = result_matrix_multiplication(&ab_refs, &c_refs).unwrap();

    let bc = result_matrix_multiplication(&b_refs, &c_refs).unwrap();
    let bc_refs = matrix_to_refs(&bc);
    let abc_right = result_matrix_multiplication(&a_refs, &bc_refs).unwrap();

    assert_matrices_equal(&abc_left, &abc_right);
}

#[test]
fn associativity_property_safe_unsafe() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    let c = vec![vec![9.0, 10.0], vec![11.0, 12.0]];

    let a_refs = matrix_to_refs(&a);
    let b_refs = matrix_to_refs(&b);
    let c_refs = matrix_to_refs(&c);

    let ab = safe_unsafe_matrix_multiplication(&a_refs, &b_refs).unwrap();
    let ab_refs = matrix_to_refs(&ab);
    let abc_left = safe_unsafe_matrix_multiplication(&ab_refs, &c_refs).unwrap();

    let bc = safe_unsafe_matrix_multiplication(&b_refs, &c_refs).unwrap();
    let bc_refs = matrix_to_refs(&bc);
    let abc_right = safe_unsafe_matrix_multiplication(&a_refs, &bc_refs).unwrap();

    assert_matrices_equal(&abc_left, &abc_right);
}

#[test]
fn identity_multiplication_property_result() {
    for n in 1..=5 {
        let mut identity = vec![vec![0.0; n]; n];
        for i in 0..n {
            identity[i][i] = 1.0;
        }

        let mut test_matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                test_matrix[i][j] = (i * n + j + 1) as f64;
            }
        }

        let id_refs = matrix_to_refs(&identity);
        let test_refs = matrix_to_refs(&test_matrix);

        let left_mult = result_matrix_multiplication(&id_refs, &test_refs).unwrap();
        let right_mult = result_matrix_multiplication(&test_refs, &id_refs).unwrap();

        assert_matrices_equal(&left_mult, &test_matrix);
        assert_matrices_equal(&right_mult, &test_matrix);
    }
}

#[test]
fn identity_multiplication_property_safe_unsafe() {
    for n in 1..=5 {
        let mut identity = vec![vec![0.0; n]; n];
        for i in 0..n {
            identity[i][i] = 1.0;
        }

        let mut test_matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                test_matrix[i][j] = (i * n + j + 1) as f64;
            }
        }

        let id_refs = matrix_to_refs(&identity);
        let test_refs = matrix_to_refs(&test_matrix);

        let left_mult =
            safe_unsafe_matrix_multiplication(&id_refs, &test_refs).unwrap();
        let right_mult =
            safe_unsafe_matrix_multiplication(&test_refs, &id_refs).unwrap();

        assert_matrices_equal(&left_mult, &test_matrix);
        assert_matrices_equal(&right_mult, &test_matrix);
    }
}

#[test]
fn matrix_power_consistency() {
    let a = vec![vec![1.0, 1.0], vec![0.0, 1.0]];

    let a_refs = matrix_to_refs(&a);

    let a_squared = matrix_multiplication(&a_refs, &a_refs);
    let a_squared_refs = matrix_to_refs(&a_squared);
    let a_cubed_via_square = matrix_multiplication(&a_squared_refs, &a_refs);

    let a_cubed_direct = matrix_multiplication(&a_refs, &a_squared_refs);

    assert_matrices_equal(&a_cubed_via_square, &a_cubed_direct);

    let expected_a_cubed = vec![vec![1.0, 3.0], vec![0.0, 1.0]];
    assert_matrices_equal(&a_cubed_direct, &expected_a_cubed);
}
