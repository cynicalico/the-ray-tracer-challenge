use trtc;

#[test]
fn test_constructing_and_inspecting_a_4x4_matrix() {
    #[rustfmt::skip]
    let m = trtc::Matrix4x4::from_array([
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
    let m = trtc::Matrix2x2::from_array([
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
    let m = trtc::Matrix3x3::from_array([
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
    let a = trtc::Matrix4x4::from_array([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    #[rustfmt::skip]
    let b = trtc::Matrix4x4::from_array([
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
    let a = trtc::Matrix4x4::from_array([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    #[rustfmt::skip]
    let b = trtc::Matrix4x4::from_array([
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
    let a = trtc::Matrix4x4::from_array([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    #[rustfmt::skip]
    let b = trtc::Matrix4x4::from_array([
        [-2.0, 1.0, 2.0,  3.0],
        [ 3.0, 2.0, 1.0, -1.0],
        [ 4.0, 3.0, 6.0,  5.0],
        [ 1.0, 2.0, 7.0,  8.0],
    ]);

    #[rustfmt::skip]
    assert_eq!(a * b, trtc::Matrix4x4::from_array([
        [20.0, 22.0,  50.0,  48.0],
        [44.0, 54.0, 114.0, 108.0],
        [40.0, 58.0, 110.0, 102.0],
        [16.0, 26.0,  46.0,  42.0],
    ]));
}
