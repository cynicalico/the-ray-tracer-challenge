use crate::{Ray, Sphere, point, transform};
use std::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        Self { t, object }
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && std::ptr::eq(self.object, other.object)
    }
}

pub struct Intersections<'a> {
    pub intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new() -> Self {
        Self {
            intersections: Vec::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.intersections.len()
    }

    pub fn add(&mut self, i: Intersection<'a>) {
        self.intersections.push(i);
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

pub fn intersect<'a>(sphere: &'a Sphere, ray: &Ray) -> Intersections<'a> {
    let ray_t = transform(&ray, &sphere.transform.inverse().unwrap());
    let sphere_to_ray = ray_t.origin - point(0.0, 0.0, 0.0);

    let a = ray_t.direction.dot(&ray_t.direction);
    let b = 2.0 * ray_t.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        Intersections::new()
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        let mut intersections = Intersections::new();
        intersections.add(Intersection::new(t1, sphere));
        intersections.add(Intersection::new(t2, sphere));
        intersections
    }
}

pub fn hit<'a>(xs: &'a Intersections<'a>) -> Option<&'a Intersection<'a>> {
    xs.intersections
        .iter()
        .filter(|i| i.t > 0.0)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
}
