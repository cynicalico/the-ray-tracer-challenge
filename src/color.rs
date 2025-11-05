use crate::utils::epsilon_eq;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        epsilon_eq(self.r, other.r) && epsilon_eq(self.g, other.g) && epsilon_eq(self.b, other.b)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Color::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Color::new(self.r * other, self.g * other, self.b * other)
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}
