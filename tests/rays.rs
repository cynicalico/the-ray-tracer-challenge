use trtc;

#[test]
fn test_creating_and_querying_a_ray() {
    let origin = trtc::point(1.0, 2.0, 3.0);
    let direction = trtc::vector(4.0, 5.0, 6.0);

    let r = trtc::Ray::new(origin, direction);

    assert_eq!(r.origin, origin);
    assert_eq!(r.direction, direction);
}

#[test]
fn test_computing_a_point_from_a_distance() {
    let r = trtc::Ray::new(trtc::point(2.0, 3.0, 4.0), trtc::vector(1.0, 0.0, 0.0));

    assert_eq!(trtc::position(&r, 0.0), trtc::point(2.0, 3.0, 4.0));
    assert_eq!(trtc::position(&r, 1.0), trtc::point(3.0, 3.0, 4.0));
    assert_eq!(trtc::position(&r, -1.0), trtc::point(1.0, 3.0, 4.0));
    assert_eq!(trtc::position(&r, 2.5), trtc::point(4.5, 3.0, 4.0));
}

#[test]
fn test_a_ray_intersects_a_sphere_at_two_points() {
    let r = trtc::Ray::new(trtc::point(0.0, 0.0, -5.0), trtc::vector(0.0, 0.0, 1.0));
    let s = trtc::Sphere::default();

    let xs = trtc::intersect(&s, &r);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);
}

#[test]
fn test_a_ray_intersects_a_sphere_at_a_tangent() {
    let r = trtc::Ray::new(trtc::point(0.0, 1.0, -5.0), trtc::vector(0.0, 0.0, 1.0));
    let s = trtc::Sphere::default();

    let xs = trtc::intersect(&s, &r);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn test_a_ray_misses_a_sphere() {
    let r = trtc::Ray::new(trtc::point(0.0, 2.0, -5.0), trtc::vector(0.0, 0.0, 1.0));
    let s = trtc::Sphere::default();

    let xs = trtc::intersect(&s, &r);

    assert_eq!(xs.count(), 0);
}

#[test]
fn test_a_ray_originates_inside_a_sphere() {
    let r = trtc::Ray::new(trtc::point(0.0, 0.0, 0.0), trtc::vector(0.0, 0.0, 1.0));
    let s = trtc::Sphere::default();

    let xs = trtc::intersect(&s, &r);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);
}

#[test]
fn test_a_sphere_is_behind_a_ray() {
    let r = trtc::Ray::new(trtc::point(0.0, 0.0, 5.0), trtc::vector(0.0, 0.0, 1.0));
    let s = trtc::Sphere::default();

    let xs = trtc::intersect(&s, &r);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
fn test_an_intersection_encapsulates_t_and_object() {
    let s = trtc::Sphere::default();

    let i = trtc::Intersection::new(3.5, &s);

    assert_eq!(i.t, 3.5);
    assert!(std::ptr::eq(i.object, &s));
}

#[test]
fn test_aggregating_intersections() {
    let s = trtc::Sphere::default();

    let i1 = trtc::Intersection::new(1.0, &s);
    let i2 = trtc::Intersection::new(2.0, &s);

    let mut xs = trtc::Intersections::new();
    xs.add(i1);
    xs.add(i2);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, 1.0);
    assert_eq!(xs[1].t, 2.0);
}

#[test]
fn test_intersect_sets_the_object_on_the_intersection() {
    let r = trtc::Ray::new(trtc::point(0.0, 0.0, -5.0), trtc::vector(0.0, 0.0, 1.0));
    let s = trtc::Sphere::default();

    let xs = trtc::intersect(&s, &r);

    assert_eq!(xs.count(), 2);
    assert!(std::ptr::eq(xs[0].object, &s));
    assert!(std::ptr::eq(xs[1].object, &s));
}

#[test]
fn test_the_hit_when_all_intersections_have_positive_t() {
    let s = trtc::Sphere::default();
    let i1 = trtc::Intersection::new(1.0, &s);
    let i2 = trtc::Intersection::new(2.0, &s);

    let mut xs = trtc::Intersections::new();
    xs.add(i1);
    xs.add(i2);

    let i = trtc::hit(&xs);
    assert!(i.is_some());
    assert_eq!(i.unwrap(), &i1);
}

#[test]
fn test_the_hit_when_some_intersections_have_negative_t() {
    let s = trtc::Sphere::default();
    let i1 = trtc::Intersection::new(-1.0, &s);
    let i2 = trtc::Intersection::new(1.0, &s);

    let mut xs = trtc::Intersections::new();
    xs.add(i1);
    xs.add(i2);

    let i = trtc::hit(&xs);
    assert!(i.is_some());
    assert_eq!(i.unwrap(), &i2);
}

#[test]
fn test_the_hit_when_all_intersections_have_negative_t() {
    let s = trtc::Sphere::default();
    let i1 = trtc::Intersection::new(-2.0, &s);
    let i2 = trtc::Intersection::new(-1.0, &s);

    let mut xs = trtc::Intersections::new();
    xs.add(i1);
    xs.add(i2);

    let i = trtc::hit(&xs);
    assert!(i.is_none());
}

#[test]
fn test_the_hit_is_always_lowest_nonnegative_intersection() {
    let s = trtc::Sphere::default();
    let i1 = trtc::Intersection::new(5.0, &s);
    let i2 = trtc::Intersection::new(7.0, &s);
    let i3 = trtc::Intersection::new(-3.0, &s);
    let i4 = trtc::Intersection::new(2.0, &s);

    let mut xs = trtc::Intersections::new();
    xs.add(i1);
    xs.add(i2);
    xs.add(i3);
    xs.add(i4);

    let i = trtc::hit(&xs);
    assert!(i.is_some());
    assert_eq!(i.unwrap(), &i4);
}

#[test]
fn test_translating_a_ray() {
    let r = trtc::Ray::new(trtc::point(1.0, 2.0, 3.0), trtc::vector(0.0, 1.0, 0.0));
    let m = trtc::translation(3.0, 4.0, 5.0);

    let r2 = trtc::transform(&r, &m);

    assert_eq!(r2.origin, trtc::point(4.0, 6.0, 8.0));
    assert_eq!(r2.direction, trtc::vector(0.0, 1.0, 0.0));
}

#[test]
fn test_scaling_a_ray() {
    let r = trtc::Ray::new(trtc::point(1.0, 2.0, 3.0), trtc::vector(0.0, 1.0, 0.0));
    let m = trtc::scaling(2.0, 3.0, 4.0);

    let r2 = trtc::transform(&r, &m);

    assert_eq!(r2.origin, trtc::point(2.0, 6.0, 12.0));
    assert_eq!(r2.direction, trtc::vector(0.0, 3.0, 0.0));
}

#[test]
fn test_a_spheres_default_transformation() {
    let s = trtc::Sphere::default();

    assert_eq!(s.transform, trtc::Matrix4::eye());
}

#[test]
fn test_changing_a_spheres_transformation() {
    let mut s = trtc::Sphere::default();
    let t = trtc::translation(2.0, 3.0, 4.0);

    s.transform = t;

    assert_eq!(s.transform, t);
}

#[test]
fn test_intersecting_a_scaled_sphere_with_a_ray() {
    let r = trtc::Ray::new(trtc::point(0.0, 0.0, -5.0), trtc::vector(0.0, 0.0, 1.0));
    let mut s = trtc::Sphere::default();

    s.transform = trtc::scaling(2.0, 2.0, 2.0);
    let xs = trtc::intersect(&s, &r);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
}

#[test]
fn test_intersecting_a_translated_sphere_with_a_ray() {
    let r = trtc::Ray::new(trtc::point(0.0, 0.0, -5.0), trtc::vector(0.0, 0.0, 1.0));
    let mut s = trtc::Sphere::default();

    s.transform = trtc::translation(5.0, 0.0, 0.0);
    let xs = trtc::intersect(&s, &r);

    assert_eq!(xs.count(), 0);
}
