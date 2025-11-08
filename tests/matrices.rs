use trtc;

#[test]
fn test_constructing_and_inspecting_a_4x4_matrix() {
    #[rustfmt::skip]
    let m = trtc::Matrix4::from_array([
        [ 1.0,  2.0,  3.0,  4.0],
        [ 5.5,  6.5,  7.5,  8.5],
        [ 9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
    ]);

    assert_eq!(m[(0, 0)], 1.0);
    assert_eq!(m[(0, 3)], 4.0);
    assert_eq!(m[(1, 0)], 5.5);
    assert_eq!(m[(1, 2)], 7.5);
    assert_eq!(m[(2, 2)], 11.0);
    assert_eq!(m[(3, 0)], 13.5);
    assert_eq!(m[(3, 2)], 15.5);
}

#[test]
fn test_a_2x2_matrix_ought_to_be_representable() {
    #[rustfmt::skip]
    let m = trtc::Matrix2::from_array([
        [-3.0,  5.0],
        [ 1.0, -2.0],
    ]);

    assert_eq!(m[(0, 0)], -3.0);
    assert_eq!(m[(0, 1)], 5.0);
    assert_eq!(m[(1, 0)], 1.0);
    assert_eq!(m[(1, 1)], -2.0);
}

#[test]
fn test_a_3x3_matrix_ought_to_be_representable() {
    #[rustfmt::skip]
    let m = trtc::Matrix3::from_array([
        [-3.0,  5.0,  0.0],
        [ 1.0, -2.0, -7.0],
        [ 0.0,  1.0,  1.0],
    ]);

    assert_eq!(m[(0, 0)], -3.0);
    assert_eq!(m[(1, 1)], -2.0);
    assert_eq!(m[(2, 2)], 1.0);
}

#[test]
fn test_matrix_equality_with_identical_matrices() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    #[rustfmt::skip]
    let b = trtc::Matrix4::from_array([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    assert_eq!(a, b);
}

#[test]
fn test_matrix_equality_with_different_matrices() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    #[rustfmt::skip]
    let b = trtc::Matrix4::from_array([
        [2.0, 3.0, 4.0, 5.0],
        [6.0, 7.0, 8.0, 9.0],
        [8.0, 7.0, 6.0, 5.0],
        [4.0, 3.0, 2.0, 1.0],
    ]);

    assert_ne!(a, b);
}

#[test]
fn test_multiplying_two_matrices() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    #[rustfmt::skip]
    let b = trtc::Matrix4::from_array([
        [-2.0, 1.0, 2.0,  3.0],
        [ 3.0, 2.0, 1.0, -1.0],
        [ 4.0, 3.0, 6.0,  5.0],
        [ 1.0, 2.0, 7.0,  8.0],
    ]);

    #[rustfmt::skip]
    assert_eq!(a * b, trtc::Matrix4::from_array([
        [20.0, 22.0,  50.0,  48.0],
        [44.0, 54.0, 114.0, 108.0],
        [40.0, 58.0, 110.0, 102.0],
        [16.0, 26.0,  46.0,  42.0],
    ]));
}

#[test]
fn test_a_matrix_multiplied_by_a_tuple() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let b = trtc::Tuple4::from_array([1.0, 2.0, 3.0, 1.0]);

    assert_eq!(a * b, trtc::Tuple4::from_array([18.0, 24.0, 33.0, 1.0]));
}

#[test]
fn test_multiplying_a_matrix_by_the_identity_matrix() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [0.0, 1.0, 2.0, 4.0],
        [1.0, 2.0, 4.0, 8.0],
        [2.0, 4.0, 8.0, 16.0],
        [4.0, 8.0, 16.0, 32.0],
    ]);

    assert_eq!(a * trtc::Matrix4::eye(), a);
}

#[test]
fn test_calculating_the_determinant_of_a_2x2_matrix() {
    #[rustfmt::skip]
    let a = trtc::Matrix2::from_array([
        [ 1.0, 5.0],
        [-3.0, 2.0],
    ]);

    assert_eq!(a.determinant(), 17.0);
}

#[test]
fn test_a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
    #[rustfmt::skip]
    let a = trtc::Matrix3::from_array([
        [ 1.0, 5.0,  0.0],
        [-3.0, 2.0,  7.0],
        [ 0.0, 6.0, -3.0],
    ]);

    #[rustfmt::skip]
    assert_eq!(a.submatrix(0, 2), trtc::Matrix2::from_array([
        [-3.0, 2.0],
        [ 0.0, 6.0],
    ]));
}

#[test]
fn test_a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [-6.0,  1.0,  1.0,  6.0],
        [-8.0,  5.0,  8.0,  6.0],
        [-1.0,  0.0,  8.0,  2.0],
        [-7.0,  1.0, -1.0,  1.0],
    ]);

    #[rustfmt::skip]
    assert_eq!(a.submatrix(2, 1), trtc::Matrix3::from_array([
        [-6.0,  1.0,  6.0],
        [-8.0,  8.0,  6.0],
        [-7.0, -1.0,  1.0],
    ]));
}

#[test]
fn test_calculating_a_minor_of_a_3x3_matrix() {
    #[rustfmt::skip]
    let a = trtc::Matrix3::from_array([
        [ 3.0,  5.0,  0.0],
        [ 2.0, -1.0, -7.0],
        [ 6.0, -1.0,  5.0],
    ]);

    let b = a.submatrix(1, 0);

    assert_eq!(b.determinant(), 25.0);
    assert_eq!(a.minor(1, 0), 25.0);
}

#[test]
fn test_calculating_the_cofactor_of_a_3x3_matrix() {
    #[rustfmt::skip]
    let a = trtc::Matrix3::from_array([
        [ 3.0,  5.0,  0.0],
        [ 2.0, -1.0, -7.0],
        [ 6.0, -1.0,  5.0],
    ]);

    assert_eq!(a.minor(0, 0), -12.0);
    assert_eq!(a.cofactor(0, 0), -12.0);
    assert_eq!(a.minor(1, 0), 25.0);
    assert_eq!(a.cofactor(1, 0), -25.0);
}

#[test]
fn test_calculating_the_determinant_of_a_3x3_matrix() {
    #[rustfmt::skip]
    let a = trtc::Matrix3::from_array([
        [ 1.0, 2.0,  6.0],
        [-5.0, 8.0, -4.0],
        [ 2.0, 6.0,  4.0],
    ]);

    assert_eq!(a.cofactor(0, 0), 56.0);
    assert_eq!(a.cofactor(0, 1), 12.0);
    assert_eq!(a.cofactor(0, 2), -46.0);
    assert_eq!(a.determinant(), -196.0);
}

#[test]
fn test_calculating_the_determinant_of_a_4x4_matrix() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [-2.0, -8.0,  3.0,  5.0],
        [-3.0,  1.0,  7.0,  3.0],
        [ 1.0,  2.0, -9.0,  6.0],
        [-6.0,  7.0,  7.0, -9.0],
    ]);

    assert_eq!(a.cofactor(0, 0), 690.0);
    assert_eq!(a.cofactor(0, 1), 447.0);
    assert_eq!(a.cofactor(0, 2), 210.0);
    assert_eq!(a.cofactor(0, 3), 51.0);
    assert_eq!(a.determinant(), -4071.0);
}

#[test]
fn test_testing_an_invertible_matrix_for_invertibility() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [6.0,  4.0, 4.0,  4.0],
        [5.0,  5.0, 7.0,  6.0],
        [4.0, -9.0, 3.0, -7.0],
        [9.0,  1.0, 7.0, -6.0]
    ]);

    assert_eq!(a.determinant(), -2120.0);
    assert!(a.is_invertible());
}

#[test]
fn test_testing_a_noninvertible_matrix_for_invertibility() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [-4.0,  2.0, -2.0, -3.0],
        [ 9.0,  6.0,  2.0,  6.0],
        [ 0.0, -5.0,  1.0, -5.0],
        [ 0.0,  0.0,  0.0,  0.0]
    ]);

    assert_eq!(a.determinant(), 0.0);
    assert!(!a.is_invertible());
}

#[test]
fn test_calculating_the_inverse_of_a_matrix() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [-5.0,  2.0,  6.0, -8.0],
        [ 1.0, -5.0,  1.0,  8.0],
        [ 7.0,  7.0, -6.0, -7.0],
        [ 1.0, -3.0,  7.0,  4.0]
    ]);

    let b = a.inverse().unwrap();

    assert_eq!(a.determinant(), 532.0);
    assert_eq!(a.cofactor(2, 3), -160.0);
    assert_eq!(b[(3, 2)], -160.0 / 532.0);
    assert_eq!(a.cofactor(3, 2), 105.0);
    assert_eq!(b[(2, 3)], 105.0 / 532.0);

    #[rustfmt::skip]
    assert_eq!(b, trtc::Matrix4::from_array([
        [ 0.21805,  0.45113,  0.24060, -0.04511],
        [-0.80827, -1.45677, -0.44361,  0.52068],
        [-0.07895, -0.22368, -0.05263,  0.19737],
        [-0.52256, -0.81391, -0.30075,  0.30639]
    ]));
}

#[test]
fn test_calculating_the_inverse_of_another_matrix() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [ 8.0, -5.0,  9.0,  2.0],
        [ 7.0,  5.0,  6.0,  1.0],
        [-6.0,  0.0,  9.0,  6.0],
        [-3.0,  0.0, -9.0, -4.0]
    ]);

    #[rustfmt::skip]
    assert_eq!(a.inverse().unwrap(), trtc::Matrix4::from_array([
        [-0.15385, -0.15385, -0.28205, -0.53846],
        [-0.07692,  0.12308,  0.02564,  0.03077],
        [ 0.35897,  0.35897,  0.43590,  0.92308],
        [-0.69231, -0.69231, -0.76923, -1.92308]
    ]));
}

#[test]
fn test_calculating_the_inverse_of_a_third_matrix() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [ 9.0,  3.0,  0.0,  9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0,  9.0,  6.0,  4.0],
        [-7.0,  6.0,  6.0,  2.0]
    ]);

    #[rustfmt::skip]
    assert_eq!(a.inverse().unwrap(), trtc::Matrix4::from_array([
        [-0.04074, -0.07778,  0.14444, -0.22222],
        [-0.07778,  0.03333,  0.36667, -0.33333],
        [-0.02901, -0.14630, -0.10926,  0.12963],
        [ 0.17778,  0.06667, -0.26667,  0.33333]
    ]));
}

#[test]
fn test_multiplying_a_product_by_its_inverse() {
    #[rustfmt::skip]
    let a = trtc::Matrix4::from_array([
        [ 3.0, -9.0,  7.0,  3.0],
        [ 3.0, -8.0,  2.0, -9.0],
        [-4.0,  4.0,  4.0,  1.0],
        [-6.0,  5.0, -1.0,  1.0]
    ]);

    #[rustfmt::skip]
    let b = trtc::Matrix4::from_array([
        [ 8.0,  2.0,  2.0,  2.0],
        [ 3.0, -1.0,  7.0,  0.0],
        [ 7.0,  0.0,  5.0,  4.0],
        [ 6.0, -2.0,  0.0,  5.0]
    ]);

    let c = a * b;

    assert_eq!(c * b.inverse().unwrap(), a);
}
