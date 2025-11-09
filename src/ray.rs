use crate::{Matrix4, Tuple4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Tuple4,
    pub direction: Tuple4,
}

impl Ray {
    pub fn new(origin: Tuple4, direction: Tuple4) -> Self {
        Self { origin, direction }
    }
}

pub fn position(ray: &Ray, t: f64) -> Tuple4 {
    ray.origin + ray.direction * t
}

pub fn transform(ray: &Ray, t: &Matrix4) -> Ray {
    Ray::new(*t * ray.origin, *t * ray.direction)
}
