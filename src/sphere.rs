use crate::Matrix4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4,
}

impl Sphere {
    pub fn new(transform: Matrix4) -> Self {
        Self { transform }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(Matrix4::eye())
    }
}
