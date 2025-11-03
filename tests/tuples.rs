use trtc;

#[test]
fn test_a_tuple_with_w_eq_1_is_a_point() {
    let a = trtc::Tuple::new(4.3, -4.2, 3.1, 1.0);

    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 1.0);
    assert!(a.is_point());
    assert!(!a.is_vector());
}

#[test]
fn test_a_tuple_with_w_eq_0_is_a_vector() {
    let a = trtc::Tuple::new(4.3, -4.2, 3.1, 0.0);

    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 0.0);
    assert!(!a.is_point());
    assert!(a.is_vector());
}

#[test]
fn test_point_creates_tuples_with_w_eq_1() {
    let a = trtc::point(4.0, -4.0, 3.0);

    assert_eq!(a, trtc::Tuple::new(4.0, -4.0, 3.0, 1.0));
}

#[test]
fn test_vector_creates_tuples_with_w_eq_0() {
    let a = trtc::vector(4.0, -4.0, 3.0);

    assert_eq!(a, trtc::Tuple::new(4.0, -4.0, 3.0, 0.0));
}

#[test]
fn test_adding_two_tuples() {
    let a1 = trtc::Tuple::new(3.0, -2.0, 5.0, 1.0);
    let a2 = trtc::Tuple::new(-2.0, 3.0, 1.0, 0.0);

    assert_eq!(a1 + a2, trtc::Tuple::new(1.0, 1.0, 6.0, 1.0));
}

#[test]
fn test_subtracting_two_points() {
    let p1 = trtc::point(3.0, 2.0, 1.0);
    let p2 = trtc::point(5.0, 6.0, 7.0);

    assert_eq!(p1 - p2, trtc::vector(-2.0, -4.0, -6.0));
}

#[test]
fn test_subtracting_a_vector_from_a_point() {
    let p = trtc::point(3.0, 2.0, 1.0);
    let v = trtc::vector(5.0, 6.0, 7.0);

    assert_eq!(p - v, trtc::point(-2.0, -4.0, -6.0));
}

#[test]
fn test_subtracting_two_vectors() {
    let v1 = trtc::vector(3.0, 2.0, 1.0);
    let v2 = trtc::vector(5.0, 6.0, 7.0);

    assert_eq!(v1 - v2, trtc::vector(-2.0, -4.0, -6.0));
}

#[test]
fn test_subtracting_a_vector_from_the_zero_vector() {
    let zero = trtc::vector(0.0, 0.0, 0.0);
    let v = trtc::vector(1.0, -2.0, 3.0);

    assert_eq!(zero - v, trtc::vector(-1.0, 2.0, -3.0));
}

#[test]
fn test_negating_a_tuple() {
    let a = trtc::Tuple::new(1.0, -2.0, 3.0, -4.0);

    assert_eq!(-a, trtc::Tuple::new(-1.0, 2.0, -3.0, 4.0));
}

#[test]
fn test_multiplying_a_tuple_by_a_scalar() {
    let a = trtc::Tuple::new(1.0, -2.0, 3.0, -4.0);

    assert_eq!(a * 3.5, trtc::Tuple::new(3.5, -7.0, 10.5, -14.0));
}

#[test]
fn test_multiplying_a_tuple_by_a_fraction() {
    let a = trtc::Tuple::new(1.0, -2.0, 3.0, -4.0);

    assert_eq!(a * 0.5, trtc::Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn test_dividing_a_tuple_by_a_scalar() {
    let a = trtc::Tuple::new(1.0, -2.0, 3.0, -4.0);

    assert_eq!(a / 2.0, trtc::Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn test_computing_the_magnitude_of_vector_1_0_0() {
    let v = trtc::vector(1.0, 0.0, 0.0);

    assert_eq!(v.magnitude(), 1.0);
}

#[test]
fn test_computing_the_magnitude_of_vector_0_1_0() {
    let v = trtc::vector(0.0, 1.0, 0.0);

    assert_eq!(v.magnitude(), 1.0);
}

#[test]
fn test_computing_the_magnitude_of_vector_0_0_1() {
    let v = trtc::vector(0.0, 0.0, 1.0);

    assert_eq!(v.magnitude(), 1.0);
}

#[test]
fn test_computing_the_magnitude_of_vector_1_2_3() {
    let v = trtc::vector(1.0, 2.0, 3.0);

    assert_eq!(v.magnitude(), 14.0f64.sqrt());
}

#[test]
fn test_computing_the_magnitude_of_vector_n1_n2_n3() {
    let v = trtc::vector(-1.0, -2.0, -3.0);

    assert_eq!(v.magnitude(), 14.0f64.sqrt());
}

#[test]
fn test_normalizing_vector_4_0_0_gives_1_0_0() {
    let v = trtc::vector(4.0, 0.0, 0.0);

    assert_eq!(trtc::normalize(&v), trtc::vector(1.0, 0.0, 0.0));
}

#[test]
fn test_normalizing_vector_1_2_3() {
    let v = trtc::vector(1.0, 2.0, 3.0);

    assert_eq!(
        trtc::normalize(&v),
        trtc::vector(
            1.0 / 14.0f64.sqrt(),
            2.0 / 14.0f64.sqrt(),
            3.0 / 14.0f64.sqrt()
        )
    );
}

#[test]
fn test_the_magnitude_of_a_normalized_vector() {
    let v = trtc::vector(1.0, 2.0, 3.0);

    let norm = trtc::normalize(&v);

    assert_eq!(norm.magnitude(), 1.0);
}

#[test]
fn test_the_dot_product_of_two_tuples() {
    let a = trtc::vector(1.0, 2.0, 3.0);
    let b = trtc::vector(2.0, 3.0, 4.0);

    assert_eq!(trtc::dot(&a, &b), 20.0);
}

#[test]
fn test_the_cross_product_of_two_vectors() {
    let a = trtc::vector(1.0, 2.0, 3.0);
    let b = trtc::vector(2.0, 3.0, 4.0);

    assert_eq!(trtc::cross(&a, &b), trtc::vector(-1.0, 2.0, -1.0));
    assert_eq!(trtc::cross(&b, &a), trtc::vector(1.0, -2.0, 1.0));
}
