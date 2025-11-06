use crate::tuple::{Tuple2, Tuple3, Tuple4};
use std::ops::{Index, IndexMut, Mul};

macro_rules! matrix {
    ($name:ident, $rows:expr, $tuple_type:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            data: [$tuple_type; $rows],
        }

        impl $name {
            pub fn from_array(data: [[f64; $tuple_type::dim()]; $rows]) -> Self {
                let mut result = Self::zeroes();
                for i in 0..$rows {
                    result.data[i] = $tuple_type::from_array(data[i]);
                }
                result
            }

            pub fn zeroes() -> Self {
                Self {
                    data: [$tuple_type::default(); $rows],
                }
            }

            pub fn eye() -> Self {
                matrix!(@check_square $rows, $tuple_type::dim(), {
                    let mut m = Self::zeroes();
                    for i in 0..$rows {
                        m[(i, i)] = 1.0;
                    }
                    m
                })
            }

            pub const fn dim() -> (usize, usize) { ($rows, $tuple_type::dim()) }

            pub const fn rows() -> usize { $rows }

            pub const fn cols() -> usize { $tuple_type::dim() }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::zeroes()
            }
        }

        impl Index<(usize, usize)> for $name {
            type Output = f64;

            fn index(&self, index: (usize, usize)) -> &Self::Output {
                &self.data[index.0][index.1]
            }
        }

        impl IndexMut<(usize, usize)> for $name {
            fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
                &mut self.data[index.0][index.1]
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.data
                    .iter()
                    .zip(other.data.iter())
                    .all(|(a, b)| a == b)
            }
        }

        impl Mul<$tuple_type> for $name {
            type Output = $tuple_type;

            fn mul(self, rhs: $tuple_type) -> Self::Output {
                let mut result = $tuple_type::default();
                for i in 0..$tuple_type::dim() {
                    result[i] = self.data[i].dot(&rhs);
                }
                result
            }
        }
    };

    (@check_square $rows:expr, $cols:expr, $code:block) => {
        if $rows == $cols {
            $code
        } else {
            unimplemented!("Operation only valid for square matrices")
        }
    };
}

macro_rules! matrix_transpose {
    ($matrix:ident, $result:ident) => {
        impl $matrix {
            pub fn transpose(&self) -> $result {
                let mut result = $result::zeroes();
                for i in 0..$matrix::rows() {
                    for j in 0..$matrix::cols() {
                        result[(j, i)] = self[(i, j)];
                    }
                }
                result
            }
        }
    };
}

macro_rules! matrix_mul {
    ($lhs:ident, $rhs:ident, $result:ident) => {
        impl Mul<$rhs> for $lhs {
            type Output = $result;

            fn mul(self, rhs: $rhs) -> Self::Output {
                let rhs_t = rhs.transpose();
                let mut result = $result::zeroes();
                for i in 0..$lhs::rows() {
                    for j in 0..$rhs::cols() {
                        result[(i, j)] = self.data[i].dot(&rhs_t.data[j]);
                    }
                }
                result
            }
        }
    };
}

macro_rules! matrix_submatrix {
    ($matrix:ident, 2, 2) => {
        impl $matrix {
            pub fn submatrix(&self, row: usize, col: usize) -> f64 {
                match (row, col) {
                    (0, 0) => self.data[1][1],
                    (0, 1) => self.data[1][0],
                    (1, 0) => self.data[0][1],
                    (1, 1) => self.data[0][0],
                    _ => panic!("row and col must be in range [0, 1]"),
                }
            }
        }
    };

    ($matrix:ident, $rows:literal, $cols:literal, $submatrix_t:ident) => {
        impl $matrix {
            pub fn submatrix(&self, row: usize, col: usize) -> $submatrix_t {
                let mut result = $submatrix_t::zeroes();
                for i in 0..$rows {
                    if i == row {
                        continue;
                    }
                    for j in 0..$cols {
                        if j == col {
                            continue;
                        }

                        let res_i = if i > row { i - 1 } else { i };
                        let res_j = if j > col { j - 1 } else { j };
                        result[(res_i, res_j)] = self[(i, j)];
                    }
                }
                result
            }
        }
    };
}

macro_rules! matrix_minor {
    ($matrix:ident, 2, 2) => {
        impl $matrix {
            pub fn minor(&self, row: usize, col: usize) -> f64 {
                self.submatrix(row, col)
            }
        }
    };

    ($matrix:ident) => {
        impl $matrix {
            pub fn minor(&self, row: usize, col: usize) -> f64 {
                self.submatrix(row, col).determinant()
            }
        }
    };
}

macro_rules! matrix_inverse {
    ($matrix:ident) => {
        impl $matrix {
            pub fn cofactor(&self, row: usize, col: usize) -> f64 {
                let minor = self.minor(row, col);
                if (row + col) % 2 == 0 { minor } else { -minor }
            }

            pub fn determinant(&self) -> f64 {
                let mut result = 0.0;
                for i in 0..$matrix::rows() {
                    result += self[(i, 0)] * self.cofactor(i, 0);
                }
                result
            }

            pub fn is_invertible(&self) -> bool {
                self.determinant() != 0.0
            }

            pub fn inverse(&self) -> Option<Self> {
                if !self.is_invertible() {
                    return None;
                }

                let mut result = Self::zeroes();
                let determinant = self.determinant();
                for i in 0..$matrix::rows() {
                    for j in 0..$matrix::cols() {
                        result[(j, i)] = self.cofactor(i, j) / determinant;
                    }
                }
                Some(result)
            }
        }
    };
}

matrix!(Matrix2x2, 2, Tuple2);
matrix_transpose!(Matrix2x2, Matrix2x2);
matrix_mul!(Matrix2x2, Matrix2x2, Matrix2x2);
matrix_mul!(Matrix2x2, Matrix2x3, Matrix2x3);
matrix_mul!(Matrix2x2, Matrix2x4, Matrix2x4);
matrix_submatrix!(Matrix2x2, 2, 2);
matrix_minor!(Matrix2x2, 2, 2);
matrix_inverse!(Matrix2x2);

matrix!(Matrix3x3, 3, Tuple3);
matrix_transpose!(Matrix3x3, Matrix3x3);
matrix_mul!(Matrix3x3, Matrix3x2, Matrix3x2);
matrix_mul!(Matrix3x3, Matrix3x3, Matrix3x3);
matrix_mul!(Matrix3x3, Matrix3x4, Matrix3x4);
matrix_submatrix!(Matrix3x3, 3, 3, Matrix2x2);
matrix_minor!(Matrix3x3);
matrix_inverse!(Matrix3x3);

matrix!(Matrix4x4, 4, Tuple4);
matrix_transpose!(Matrix4x4, Matrix4x4);
matrix_mul!(Matrix4x4, Matrix4x2, Matrix4x2);
matrix_mul!(Matrix4x4, Matrix4x3, Matrix4x3);
matrix_mul!(Matrix4x4, Matrix4x4, Matrix4x4);
matrix_submatrix!(Matrix4x4, 4, 4, Matrix3x3);
matrix_minor!(Matrix4x4);
matrix_inverse!(Matrix4x4);

matrix!(Matrix2x3, 2, Tuple3);
matrix_transpose!(Matrix2x3, Matrix3x2);
matrix_mul!(Matrix2x3, Matrix3x2, Matrix2x2);
matrix_mul!(Matrix2x3, Matrix3x3, Matrix2x3);
matrix_mul!(Matrix2x3, Matrix3x4, Matrix2x4);

matrix!(Matrix2x4, 2, Tuple4);
matrix_transpose!(Matrix2x4, Matrix4x2);
matrix_mul!(Matrix2x4, Matrix4x2, Matrix2x2);
matrix_mul!(Matrix2x4, Matrix4x3, Matrix2x3);
matrix_mul!(Matrix2x4, Matrix4x4, Matrix2x4);

matrix!(Matrix3x2, 3, Tuple2);
matrix_transpose!(Matrix3x2, Matrix2x3);
matrix_mul!(Matrix3x2, Matrix2x2, Matrix3x2);
matrix_mul!(Matrix3x2, Matrix2x3, Matrix3x3);
matrix_mul!(Matrix3x2, Matrix2x4, Matrix3x4);

matrix!(Matrix3x4, 3, Tuple4);
matrix_transpose!(Matrix3x4, Matrix4x3);
matrix_mul!(Matrix3x4, Matrix4x2, Matrix3x2);
matrix_mul!(Matrix3x4, Matrix4x3, Matrix3x3);
matrix_mul!(Matrix3x4, Matrix4x4, Matrix3x4);

matrix!(Matrix4x2, 4, Tuple2);
matrix_transpose!(Matrix4x2, Matrix2x4);
matrix_mul!(Matrix4x2, Matrix2x2, Matrix4x2);
matrix_mul!(Matrix4x2, Matrix2x3, Matrix4x3);
matrix_mul!(Matrix4x2, Matrix2x4, Matrix4x4);

matrix!(Matrix4x3, 4, Tuple3);
matrix_transpose!(Matrix4x3, Matrix3x4);
matrix_mul!(Matrix4x3, Matrix3x2, Matrix4x2);
matrix_mul!(Matrix4x3, Matrix3x3, Matrix4x3);
matrix_mul!(Matrix4x3, Matrix3x4, Matrix4x4);
