use crate::utils::epsilon_eq;

macro_rules! matrix {
    ($name:ident, $rows:expr, $cols:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            data: [[f64; $cols]; $rows],
        }

        impl $name {
            pub fn zeroes() -> Self {
                Self {
                    data: [[0.0; $cols]; $rows],
                }
            }

            pub fn eye() -> Self {
                matrix!(@check_square $rows, $cols, {
                    let mut m = Self::zeroes();
                    for i in 0..$rows {
                        m[(i, i)] = 1.0;
                    }
                    m
                })
            }

            pub fn from_array(data: [[f64; $cols]; $rows]) -> Self {
                Self { data }
            }

            pub const fn rows(&self) -> usize {
                $rows
            }

            pub const fn cols(&self) -> usize {
                $cols
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::zeroes()
            }
        }

        impl std::ops::Index<(usize, usize)> for $name {
            type Output = f64;

            fn index(&self, index: (usize, usize)) -> &Self::Output {
                &self.data[index.0][index.1]
            }
        }

        impl std::ops::IndexMut<(usize, usize)> for $name {
            fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
                &mut self.data[index.0][index.1]
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.data
                    .iter()
                    .flatten()
                    .zip(other.data.iter().flatten())
                    .all(|(&a, &b)| epsilon_eq(a, b))
            }
        }

        matrix_mul!($name, $name, $name);
    };

    (@check_square $rows:expr, $cols:expr, $code:block) => {
        if $rows == $cols {
            $code
        } else {
            unimplemented!("Operation only valid for square matrices")
        }
    };
}

macro_rules! matrix_mul {
    ($lhs:ident, $rhs:ident, $result:ident) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = $result;

            fn mul(self, other: $rhs) -> Self::Output {
                let mut result = $result::zeroes();
                for i in 0..self.rows() {
                    for j in 0..other.cols() {
                        for k in 0..self.cols() {
                            result[(i, j)] += self[(i, k)] * other[(k, j)];
                        }
                    }
                }
                result
            }
        }
    };
}

matrix!(Matrix2x2, 2, 2);
matrix!(Matrix3x3, 3, 3);
matrix!(Matrix4x4, 4, 4);
