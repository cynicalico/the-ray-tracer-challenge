use trtc;

#[test]
fn test_multiplying_by_a_translation_matrix() {
    let transform = trtc::translation(5.0, -3.0, 2.0);
    let p = trtc::point(-3.0, 4.0, 5.0);

    assert_eq!(transform * p, trtc::point(2.0, 1.0, 7.0));
}

#[test]
fn test_multiplying_by_the_inverse_of_a_translation_matrix() {
    let transform = trtc::translation(5.0, -3.0, 2.0);
    let inv = transform.inverse().unwrap();
    let p = trtc::point(-3.0, 4.0, 5.0);

    assert_eq!(inv * p, trtc::point(-8.0, 7.0, 3.0));
}

#[test]
fn test_translation_does_not_affect_vectors() {
    let transform = trtc::translation(5.0, -3.0, 2.0);
    let v = trtc::vector(-3.0, 4.0, 5.0);

    assert_eq!(transform * v, v);
}

#[test]
fn test_a_scaling_matrix_applied_to_a_point() {
    let transform = trtc::scaling(2.0, 3.0, 4.0);
    let p = trtc::point(-4.0, 6.0, 8.0);

    assert_eq!(transform * p, trtc::point(-8.0, 18.0, 32.0));
}

#[test]
fn test_a_scaling_matrix_applied_to_a_vector() {
    let transform = trtc::scaling(2.0, 3.0, 4.0);
    let v = trtc::vector(-4.0, 6.0, 8.0);

    assert_eq!(transform * v, trtc::vector(-8.0, 18.0, 32.0));
}

#[test]
fn test_multiplying_by_the_inverse_of_a_scaling_matrix() {
    let transform = trtc::scaling(2.0, 3.0, 4.0);
    let inv = transform.inverse().unwrap();
    let v = trtc::vector(-4.0, 6.0, 8.0);

    assert_eq!(inv * v, trtc::vector(-2.0, 2.0, 2.0));
}

#[test]
fn test_reflection_is_scaling_by_a_negative_value() {
    let transform = trtc::scaling(-1.0, 1.0, 1.0);
    let p = trtc::point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, trtc::point(-2.0, 3.0, 4.0));
}

#[test]
fn test_rotating_a_point_around_the_x_axis() {
    let p = trtc::point(0.0, 1.0, 0.0);
    let half_quarter = trtc::rotation_x(std::f64::consts::PI / 4.0);
    let full_quarter = trtc::rotation_x(std::f64::consts::PI / 2.0);

    assert_eq!(
        half_quarter * p,
        trtc::point(
            0.0,
            std::f64::consts::SQRT_2 / 2.0,
            std::f64::consts::SQRT_2 / 2.0
        )
    );
    assert_eq!(full_quarter * p, trtc::point(0.0, 0.0, 1.0));
}

#[test]
fn test_rotating_a_point_around_the_y_axis() {
    let p = trtc::point(0.0, 0.0, 1.0);
    let half_quarter = trtc::rotation_y(std::f64::consts::PI / 4.0);
    let full_quarter = trtc::rotation_y(std::f64::consts::PI / 2.0);

    assert_eq!(
        half_quarter * p,
        trtc::point(
            std::f64::consts::SQRT_2 / 2.0,
            0.0,
            std::f64::consts::SQRT_2 / 2.0
        )
    );
    assert_eq!(full_quarter * p, trtc::point(1.0, 0.0, 0.0));
}

#[test]
fn test_rotating_a_point_around_the_z_axis() {
    let p = trtc::point(0.0, 1.0, 0.0);
    let half_quarter = trtc::rotation_z(std::f64::consts::PI / 4.0);
    let full_quarter = trtc::rotation_z(std::f64::consts::PI / 2.0);

    assert_eq!(
        half_quarter * p,
        trtc::point(
            -std::f64::consts::SQRT_2 / 2.0,
            std::f64::consts::SQRT_2 / 2.0,
            0.0
        )
    );
    assert_eq!(full_quarter * p, trtc::point(-1.0, 0.0, 0.0));
}

#[test]
fn test_a_shearing_transformation_moves_x_in_proportion_to_y() {
    let transform = trtc::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let p = trtc::point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, trtc::point(5.0, 3.0, 4.0));
}

#[test]
fn test_a_shearing_transformation_moves_x_in_proportion_to_z() {
    let transform = trtc::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let p = trtc::point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, trtc::point(6.0, 3.0, 4.0));
}

#[test]
fn test_a_shearing_transformation_moves_y_in_proportion_to_x() {
    let transform = trtc::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let p = trtc::point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, trtc::point(2.0, 5.0, 4.0));
}

#[test]
fn test_a_shearing_transformation_moves_y_in_proportion_to_z() {
    let transform = trtc::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let p = trtc::point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, trtc::point(2.0, 7.0, 4.0));
}

#[test]
fn test_a_shearing_transformation_moves_z_in_proportion_to_x() {
    let transform = trtc::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let p = trtc::point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, trtc::point(2.0, 3.0, 6.0));
}

#[test]
fn test_a_shearing_transformation_moves_z_in_proportion_to_y() {
    let transform = trtc::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let p = trtc::point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, trtc::point(2.0, 3.0, 7.0));
}

#[test]
fn test_individual_transformation_operations_are_applied_in_sequence() {
    let p = trtc::point(1.0, 0.0, 1.0);
    let a = trtc::rotation_x(std::f64::consts::PI / 2.0);
    let b = trtc::scaling(5.0, 5.0, 5.0);
    let c = trtc::translation(10.0, 5.0, 7.0);

    let p2 = a * p;
    assert_eq!(p2, trtc::point(1.0, -1.0, 0.0));

    let p3 = b * p2;
    assert_eq!(p3, trtc::point(5.0, -5.0, 0.0));

    let p4 = c * p3;
    assert_eq!(p4, trtc::point(15.0, 0.0, 7.0));
}

#[test]
fn test_chained_transformations_must_be_applied_in_reverse_order() {
    let p = trtc::point(1.0, 0.0, 1.0);
    let a = trtc::rotation_x(std::f64::consts::PI / 2.0);
    let b = trtc::scaling(5.0, 5.0, 5.0);
    let c = trtc::translation(10.0, 5.0, 7.0);

    let t = c * b * a;
    assert_eq!(t * p, trtc::point(15.0, 0.0, 7.0));
}

#[test]
fn test_fluent_transformations_are_applied_in_correct_order() {
    let p = trtc::point(1.0, 0.0, 1.0);
    let t = trtc::Matrix4::eye()
        .rotate_x(std::f64::consts::PI / 2.0)
        .scale(5.0, 5.0, 5.0)
        .translate(10.0, 5.0, 7.0);

    assert_eq!(t * p, trtc::point(15.0, 0.0, 7.0));
}
