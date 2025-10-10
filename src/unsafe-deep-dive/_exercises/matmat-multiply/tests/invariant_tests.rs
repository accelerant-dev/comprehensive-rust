use matmat::panic_heavy::matrix_multiplication;
use matmat::result::matrix_multiplication as result_matrix_multiplication;
use matmat::safe_unsafe::matrix_multiplication as safe_unsafe_matrix_multiplication;

const EPSILON: f64 = 1e-10;

fn matrix_to_refs(matrix: &[Vec<f64>]) -> Vec<&[f64]> {
    matrix.iter().map(|row| row.as_slice()).collect()
}

fn frobenius_norm(matrix: &[Vec<f64>]) -> f64 {
    matrix
        .iter()
        .map(|row| row.iter().map(|x| x * x).sum::<f64>())
        .sum::<f64>()
        .sqrt()
}

fn trace(matrix: &[Vec<f64>]) -> f64 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0.0;
    }
    let n = matrix.len().min(matrix[0].len());
    (0..n).map(|i| matrix[i][i]).sum()
}

fn determinant_2x2(matrix: &[Vec<f64>]) -> f64 {
    assert_eq!(matrix.len(), 2);
    assert_eq!(matrix[0].len(), 2);
    matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
}

#[test]
fn norm_submultiplicativity() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];

    let a_refs = matrix_to_refs(&a);
    let b_refs = matrix_to_refs(&b);
    let ab = matrix_multiplication(&a_refs, &b_refs);

    let norm_a = frobenius_norm(&a);
    let norm_b = frobenius_norm(&b);
    let norm_ab = frobenius_norm(&ab);

    assert!(
        norm_ab <= norm_a * norm_b + EPSILON,
        "Submultiplicativity violated: ||AB|| = {} > ||A|| * ||B|| = {} * {} = {}",
        norm_ab,
        norm_a,
        norm_b,
        norm_a * norm_b
    );
}

#[test]
fn trace_linearity() {
    let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0]];
    let b = vec![vec![9.0, 8.0, 7.0], vec![6.0, 5.0, 4.0], vec![3.0, 2.0, 1.0]];

    let scalar = 2.5;

    let mut scalar_a = vec![vec![0.0; 3]; 3];
    let mut a_plus_b = vec![vec![0.0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            scalar_a[i][j] = scalar * a[i][j];
            a_plus_b[i][j] = a[i][j] + b[i][j];
        }
    }

    let trace_a = trace(&a);
    let trace_b = trace(&b);
    let trace_scalar_a = trace(&scalar_a);
    let trace_a_plus_b = trace(&a_plus_b);

    assert!(
        (trace_scalar_a - scalar * trace_a).abs() < EPSILON,
        "Scalar linearity failed: tr({}A) = {} != {} * tr(A) = {} * {} = {}",
        scalar,
        trace_scalar_a,
        scalar,
        scalar,
        trace_a,
        scalar * trace_a
    );

    assert!(
        (trace_a_plus_b - (trace_a + trace_b)).abs() < EPSILON,
        "Additive linearity failed: tr(A+B) = {} != tr(A) + tr(B) = {} + {} = {}",
        trace_a_plus_b,
        trace_a,
        trace_b,
        trace_a + trace_b
    );
}

#[test]
fn determinant_multiplicativity_2x2() {
    let a = vec![vec![2.0, 3.0], vec![1.0, 4.0]];
    let b = vec![vec![5.0, 2.0], vec![3.0, 1.0]];

    let a_refs = matrix_to_refs(&a);
    let b_refs = matrix_to_refs(&b);
    let ab = matrix_multiplication(&a_refs, &b_refs);

    let det_a = determinant_2x2(&a);
    let det_b = determinant_2x2(&b);
    let det_ab = determinant_2x2(&ab);

    assert!(
        (det_ab - det_a * det_b).abs() < EPSILON,
        "Determinant multiplicativity failed: det(AB) = {} != det(A) * det(B) = {} * {} = {}",
        det_ab,
        det_a,
        det_b,
        det_a * det_b
    );
}

#[test]
fn orthogonal_matrix_properties() {
    let sqrt_2_inv = 1.0 / (2.0_f64).sqrt();
    let rotation_45 =
        vec![vec![sqrt_2_inv, -sqrt_2_inv], vec![sqrt_2_inv, sqrt_2_inv]];

    let rotation_45_refs = matrix_to_refs(&rotation_45);

    let rotation_90 = matrix_multiplication(&rotation_45_refs, &rotation_45_refs);

    let expected_rotation_90 = vec![vec![0.0, -1.0], vec![1.0, 0.0]];

    for i in 0..2 {
        for j in 0..2 {
            assert!(
                (rotation_90[i][j] - expected_rotation_90[i][j]).abs() < EPSILON,
                "90-degree rotation element ({}, {}) mismatch: {} != {}",
                i,
                j,
                rotation_90[i][j],
                expected_rotation_90[i][j]
            );
        }
    }

    let norm_preserved = frobenius_norm(&rotation_90);
    let expected_norm = (2.0_f64).sqrt();
    assert!(
        (norm_preserved - expected_norm).abs() < EPSILON,
        "Orthogonal matrix norm preservation failed: {} != {}",
        norm_preserved,
        expected_norm
    );
}

#[test]
fn projection_matrix_idempotency() {
    let u = vec![vec![1.0], vec![1.0], vec![1.0]];

    let u_refs = matrix_to_refs(&u);
    let ut = vec![vec![1.0, 1.0, 1.0]];
    let ut_refs = matrix_to_refs(&ut);

    let uut = matrix_multiplication(&u_refs, &ut_refs);

    let mut projection = vec![vec![0.0; 3]; 3];
    let norm_squared = 3.0;
    for i in 0..3 {
        for j in 0..3 {
            projection[i][j] = uut[i][j] / norm_squared;
        }
    }

    let proj_refs = matrix_to_refs(&projection);
    let projection_squared = matrix_multiplication(&proj_refs, &proj_refs);

    for i in 0..3 {
        for j in 0..3 {
            assert!(
                (projection_squared[i][j] - projection[i][j]).abs() < EPSILON,
                "Projection idempotency failed at ({}, {}): P² = {} != P = {}",
                i,
                j,
                projection_squared[i][j],
                projection[i][j]
            );
        }
    }
}

#[test]
fn nilpotent_matrix_properties() {
    let nilpotent =
        vec![vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 0.0]];

    let nilp_refs = matrix_to_refs(&nilpotent);

    let nilp_squared = matrix_multiplication(&nilp_refs, &nilp_refs);
    let expected_nilp_squared =
        vec![vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0]];

    for i in 0..3 {
        for j in 0..3 {
            assert!(
                (nilp_squared[i][j] - expected_nilp_squared[i][j]).abs() < EPSILON,
                "Nilpotent squared element ({}, {}) mismatch: {} != {}",
                i,
                j,
                nilp_squared[i][j],
                expected_nilp_squared[i][j]
            );
        }
    }

    let nilp_sq_refs = matrix_to_refs(&nilp_squared);
    let nilp_cubed = matrix_multiplication(&nilp_sq_refs, &nilp_refs);

    for i in 0..3 {
        for j in 0..3 {
            assert!(
                nilp_cubed[i][j].abs() < EPSILON,
                "Nilpotent cubed should be zero at ({}, {}): {}",
                i,
                j,
                nilp_cubed[i][j]
            );
        }
    }
}

#[test]
fn upper_triangular_closure() {
    let upper1 = vec![vec![1.0, 2.0, 3.0], vec![0.0, 4.0, 5.0], vec![0.0, 0.0, 6.0]];

    let upper2 =
        vec![vec![7.0, 8.0, 9.0], vec![0.0, 10.0, 11.0], vec![0.0, 0.0, 12.0]];

    let upper1_refs = matrix_to_refs(&upper1);
    let upper2_refs = matrix_to_refs(&upper2);

    let product = matrix_multiplication(&upper1_refs, &upper2_refs);

    for i in 0..3 {
        for j in 0..i {
            assert!(
                product[i][j].abs() < EPSILON,
                "Upper triangular closure failed: product[{}][{}] = {} should be 0",
                i,
                j,
                product[i][j]
            );
        }
    }

    assert!((product[0][0] - (1.0 * 7.0)).abs() < EPSILON);
    assert!((product[1][1] - (4.0 * 10.0)).abs() < EPSILON);
    assert!((product[2][2] - (6.0 * 12.0)).abs() < EPSILON);
}

#[test]
fn symmetric_matrix_properties() {
    let symmetric =
        vec![vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 5.0], vec![3.0, 5.0, 6.0]];

    let vector = vec![vec![1.0], vec![2.0], vec![3.0]];

    let sym_refs = matrix_to_refs(&symmetric);
    let vec_refs = matrix_to_refs(&vector);

    let result = matrix_multiplication(&sym_refs, &vec_refs);

    let vec_t = vec![vec![1.0, 2.0, 3.0]];
    let vec_t_refs = matrix_to_refs(&vec_t);

    let result_refs = matrix_to_refs(&result);
    let quadratic_form1 = matrix_multiplication(&vec_t_refs, &result_refs);
    let temp = matrix_multiplication(&vec_t_refs, &sym_refs);
    let temp_refs = matrix_to_refs(&temp);
    let quadratic_form2 = matrix_multiplication(&temp_refs, &vec_refs);

    assert!(
        (quadratic_form1[0][0] - quadratic_form2[0][0]).abs() < EPSILON,
        "Symmetric matrix quadratic form inconsistency: {} != {}",
        quadratic_form1[0][0],
        quadratic_form2[0][0]
    );

    let expected_qf = 1.0 * 1.0 * 1.0
        + 2.0 * 1.0 * 2.0
        + 3.0 * 1.0 * 3.0
        + 2.0 * 2.0 * 1.0
        + 4.0 * 2.0 * 2.0
        + 5.0 * 2.0 * 3.0
        + 3.0 * 3.0 * 1.0
        + 5.0 * 3.0 * 2.0
        + 6.0 * 3.0 * 3.0;

    assert!(
        (quadratic_form1[0][0] - expected_qf).abs() < EPSILON,
        "Symmetric quadratic form calculation error: {} != {}",
        quadratic_form1[0][0],
        expected_qf
    );
}

#[test]
fn trace_edge_cases() {
    let empty_matrix: Vec<Vec<f64>> = vec![];
    let matrix_with_empty_rows = vec![vec![]];

    assert_eq!(trace(&empty_matrix), 0.0, "Trace of empty matrix should be 0");
    assert_eq!(
        trace(&matrix_with_empty_rows),
        0.0,
        "Trace of matrix with empty rows should be 0"
    );

    let single_element = vec![vec![5.0]];
    assert_eq!(
        trace(&single_element),
        5.0,
        "Trace of 1x1 matrix should be the element value"
    );

    let rectangular = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    assert_eq!(trace(&rectangular), 6.0, "Trace of 2x3 matrix should be 1+5=6");

    let tall_rectangular = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
    assert_eq!(trace(&tall_rectangular), 5.0, "Trace of 3x2 matrix should be 1+4=5");
}

#[test]
fn norm_submultiplicativity_result() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];

    let a_refs = matrix_to_refs(&a);
    let b_refs = matrix_to_refs(&b);
    let ab = result_matrix_multiplication(&a_refs, &b_refs).unwrap();

    let norm_a = frobenius_norm(&a);
    let norm_b = frobenius_norm(&b);
    let norm_ab = frobenius_norm(&ab);

    assert!(
        norm_ab <= norm_a * norm_b + EPSILON,
        "Submultiplicativity violated: ||AB|| = {} > ||A|| * ||B|| = {} * {} = {}",
        norm_ab,
        norm_a,
        norm_b,
        norm_a * norm_b
    );
}

#[test]
fn norm_submultiplicativity_safe_unsafe() {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];

    let a_refs = matrix_to_refs(&a);
    let b_refs = matrix_to_refs(&b);
    let ab = safe_unsafe_matrix_multiplication(&a_refs, &b_refs).unwrap();

    let norm_a = frobenius_norm(&a);
    let norm_b = frobenius_norm(&b);
    let norm_ab = frobenius_norm(&ab);

    assert!(
        norm_ab <= norm_a * norm_b + EPSILON,
        "Submultiplicativity violated: ||AB|| = {} > ||A|| * ||B|| = {} * {} = {}",
        norm_ab,
        norm_a,
        norm_b,
        norm_a * norm_b
    );
}

#[test]
fn determinant_multiplicativity_2x2_result() {
    let a = vec![vec![2.0, 3.0], vec![1.0, 4.0]];
    let b = vec![vec![5.0, 2.0], vec![3.0, 1.0]];

    let a_refs = matrix_to_refs(&a);
    let b_refs = matrix_to_refs(&b);
    let ab = result_matrix_multiplication(&a_refs, &b_refs).unwrap();

    let det_a = determinant_2x2(&a);
    let det_b = determinant_2x2(&b);
    let det_ab = determinant_2x2(&ab);

    assert!(
        (det_ab - det_a * det_b).abs() < EPSILON,
        "Determinant multiplicativity failed: det(AB) = {} != det(A) * det(B) = {} * {} = {}",
        det_ab,
        det_a,
        det_b,
        det_a * det_b
    );
}

#[test]
fn determinant_multiplicativity_2x2_safe_unsafe() {
    let a = vec![vec![2.0, 3.0], vec![1.0, 4.0]];
    let b = vec![vec![5.0, 2.0], vec![3.0, 1.0]];

    let a_refs = matrix_to_refs(&a);
    let b_refs = matrix_to_refs(&b);
    let ab = safe_unsafe_matrix_multiplication(&a_refs, &b_refs).unwrap();

    let det_a = determinant_2x2(&a);
    let det_b = determinant_2x2(&b);
    let det_ab = determinant_2x2(&ab);

    assert!(
        (det_ab - det_a * det_b).abs() < EPSILON,
        "Determinant multiplicativity failed: det(AB) = {} != det(A) * det(B) = {} * {} = {}",
        det_ab,
        det_a,
        det_b,
        det_a * det_b
    );
}

#[test]
fn projection_matrix_idempotency_result() {
    let u = vec![vec![1.0], vec![1.0], vec![1.0]];

    let u_refs = matrix_to_refs(&u);
    let ut = vec![vec![1.0, 1.0, 1.0]];
    let ut_refs = matrix_to_refs(&ut);

    let uut = result_matrix_multiplication(&u_refs, &ut_refs).unwrap();

    let mut projection = vec![vec![0.0; 3]; 3];
    let norm_squared = 3.0;
    for i in 0..3 {
        for j in 0..3 {
            projection[i][j] = uut[i][j] / norm_squared;
        }
    }

    let proj_refs = matrix_to_refs(&projection);
    let projection_squared =
        result_matrix_multiplication(&proj_refs, &proj_refs).unwrap();

    for i in 0..3 {
        for j in 0..3 {
            assert!(
                (projection_squared[i][j] - projection[i][j]).abs() < EPSILON,
                "Projection idempotency failed at ({}, {}): P² = {} != P = {}",
                i,
                j,
                projection_squared[i][j],
                projection[i][j]
            );
        }
    }
}

#[test]
fn projection_matrix_idempotency_safe_unsafe() {
    let u = vec![vec![1.0], vec![1.0], vec![1.0]];

    let u_refs = matrix_to_refs(&u);
    let ut = vec![vec![1.0, 1.0, 1.0]];
    let ut_refs = matrix_to_refs(&ut);

    let uut = safe_unsafe_matrix_multiplication(&u_refs, &ut_refs).unwrap();

    let mut projection = vec![vec![0.0; 3]; 3];
    let norm_squared = 3.0;
    for i in 0..3 {
        for j in 0..3 {
            projection[i][j] = uut[i][j] / norm_squared;
        }
    }

    let proj_refs = matrix_to_refs(&projection);
    let projection_squared =
        safe_unsafe_matrix_multiplication(&proj_refs, &proj_refs).unwrap();

    for i in 0..3 {
        for j in 0..3 {
            assert!(
                (projection_squared[i][j] - projection[i][j]).abs() < EPSILON,
                "Projection idempotency failed at ({}, {}): P² = {} != P = {}",
                i,
                j,
                projection_squared[i][j],
                projection[i][j]
            );
        }
    }
}
