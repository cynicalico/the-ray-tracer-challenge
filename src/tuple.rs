use crate::utils::epsilon_eq;
use std::ops::{Add, Div, Mul, Neg, Sub};

macro_rules! tuple {
    ($name:ident, $dim:literal, [$($field:ident),*], [$($idx:expr),*]) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            $(pub $field: f64,)*
        }

        impl $name {
            pub fn from_array(data: [f64; $dim]) -> Self {
                Self { $($field: data[$idx]),* }
            }

            pub fn magnitude(&self) -> f64 {
                (0.0 $(+ self.$field * self.$field)*).sqrt()
            }

            pub fn normalize(&self) -> Self {
                let magnitude = self.magnitude();
                Self {
                    $($field: self.$field / magnitude),*
                }
            }

            pub fn dot(&self, other: &Self) -> f64 {
                0.0 $(+ self.$field * other.$field)*
            }

            pub const fn dim() -> usize { $dim }
        }

        impl Default for $name {
            fn default() -> Self {
                Self { $($field: 0.0),* }
            }
        }

        impl std::ops::Index<usize> for $name {
            type Output = f64;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    $($idx => &self.$field,)*
                    _ => panic!("Index out of bounds"),
                }
            }
        }

        impl std::ops::IndexMut<usize> for $name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    $($idx => &mut self.$field,)*
                    _ => panic!("Index out of bounds"),
                }
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                true $(& epsilon_eq(self.$field, other.$field))*
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self {
                    $($field: self.$field + other.$field),*
                }
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self {
                    $($field: self.$field - other.$field),*
                }
            }
        }

        impl Neg for $name {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    $($field: -self.$field),*
                }
            }
        }

        impl Mul<f64> for $name {
            type Output = Self;

            fn mul(self, scalar: f64) -> Self {
                Self {
                    $($field: self.$field * scalar),*
                }
            }
        }

        impl Div<f64> for $name {
            type Output = Self;

            fn div(self, scalar: f64) -> Self {
                Self {
                    $($field: self.$field / scalar),*
                }
            }
        }
    };
}

tuple!(Tuple2, 2, [x, y], [0, 1]);
tuple!(Tuple3, 3, [x, y, z], [0, 1, 2]);
tuple!(Tuple4, 4, [x, y, z, w], [0, 1, 2, 3]);

impl Tuple3 {
    pub fn cross(&self, other: &Self) -> Self {
        Self::from_array([
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        ])
    }
}

impl Tuple4 {
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::from_array([
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
            0.0,
        ])
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple4 {
    Tuple4::from_array([x, y, z, 1.0])
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple4 {
    Tuple4::from_array([x, y, z, 0.0])
}
