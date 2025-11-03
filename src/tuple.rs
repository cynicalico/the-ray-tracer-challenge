use crate::utils::epsilon_eq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn is_point(self: &Self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(self: &Self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(self: &Self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

pub fn normalize(t: &Tuple) -> Tuple {
    let magnitude = t.magnitude();
    Tuple::new(
        t.x / magnitude,
        t.y / magnitude,
        t.z / magnitude,
        t.w / magnitude,
    )
}

pub fn dot(a: &Tuple, b: &Tuple) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

pub fn cross(a: &Tuple, b: &Tuple) -> Tuple {
    vector(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        epsilon_eq(self.x, other.x)
            && epsilon_eq(self.y, other.y)
            && epsilon_eq(self.z, other.z)
            && epsilon_eq(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Tuple::new(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
        )
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Tuple::new(
            self.x / other,
            self.y / other,
            self.z / other,
            self.w / other,
        )
    }
}
