use crate::tuple::{Tuple2, Tuple3, Tuple4};
use std::ops::{Index, IndexMut, Mul};

macro_rules! matrix {
    ($name:ident, $tuple_type:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            data: [$tuple_type; $tuple_type::dim()],
        }

        impl $name {
            pub fn from_array(data: [[f64; $tuple_type::dim()]; $tuple_type::dim()]) -> Self {
                let mut result = Self::zeroes();
                for i in 0..$tuple_type::dim() {
                    result.data[i] = $tuple_type::from_array(data[i]);
                }
                result
            }

            pub fn zeroes() -> Self {
                Self {
                    data: [$tuple_type::default(); $tuple_type::dim()],
                }
            }

            pub fn eye() -> Self {
                let mut m = Self::zeroes();
                for i in 0..$tuple_type::dim() {
                    m[(i, i)] = 1.0;
                }
                m
            }

            pub const fn dim() -> (usize, usize) {
                ($tuple_type::dim(), $tuple_type::dim())
            }

            pub const fn rows() -> usize {
                $tuple_type::dim()
            }

            pub const fn cols() -> usize {
                $tuple_type::dim()
            }
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
                self.data.iter().zip(other.data.iter()).all(|(a, b)| a == b)
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
                let mut result = $result::zeroes();
                let rhs_t = rhs.transpose();
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
    ($matrix:ident, 2) => {
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

    ($matrix:ident, $submatrix_t:ident) => {
        impl $matrix {
            pub fn submatrix(&self, row: usize, col: usize) -> $submatrix_t {
                let mut result = $submatrix_t::zeroes();
                for i in 0..$matrix::rows() {
                    if i == row {
                        continue;
                    }
                    for j in 0..$matrix::cols() {
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
    ($matrix:ident, 2) => {
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

matrix!(Matrix2, Tuple2);
matrix_transpose!(Matrix2, Matrix2);
matrix_mul!(Matrix2, Matrix2, Matrix2);
matrix_submatrix!(Matrix2, 2);
matrix_minor!(Matrix2, 2);
matrix_inverse!(Matrix2);

matrix!(Matrix3, Tuple3);
matrix_transpose!(Matrix3, Matrix3);
matrix_mul!(Matrix3, Matrix3, Matrix3);
matrix_submatrix!(Matrix3, Matrix2);
matrix_minor!(Matrix3);
matrix_inverse!(Matrix3);

matrix!(Matrix4, Tuple4);
matrix_transpose!(Matrix4, Matrix4);
matrix_mul!(Matrix4, Matrix4, Matrix4);
matrix_submatrix!(Matrix4, Matrix3);
matrix_minor!(Matrix4);
matrix_inverse!(Matrix4);

#[rustfmt::skip]
pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    Matrix4::from_array([
        [1.0, 0.0, 0.0, x  ],
        [0.0, 1.0, 0.0, y  ],
        [0.0, 0.0, 1.0, z  ],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[rustfmt::skip]
pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
    Matrix4::from_array([
        [x,   0.0, 0.0, 0.0],
        [0.0, y,   0.0, 0.0],
        [0.0, 0.0, z,   0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[rustfmt::skip]
pub fn rotation_x(theta: f64) -> Matrix4 {
    let sin = theta.sin();
    let cos  = theta.cos();
    Matrix4::from_array([
        [1.0, 0.0,  0.0, 0.0],
        [0.0, cos, -sin, 0.0],
        [0.0, sin,  cos, 0.0],
        [0.0, 0.0,  0.0, 1.0],
    ])
}

#[rustfmt::skip]
pub fn rotation_y(theta: f64) -> Matrix4 {
    let sin = theta.sin();
    let cos  = theta.cos();
    Matrix4::from_array([
        [ cos, 0.0, sin, 0.0],
        [ 0.0, 1.0, 0.0, 0.0],
        [-sin, 0.0, cos, 0.0],
        [ 0.0, 0.0, 0.0, 1.0],
    ])
}

#[rustfmt::skip]
pub fn rotation_z(theta: f64) -> Matrix4 {
    let sin = theta.sin();
    let cos  = theta.cos();
    Matrix4::from_array([
        [cos, -sin, 0.0, 0.0],
        [sin,  cos, 0.0, 0.0],
        [0.0,  0.0, 1.0, 0.0],
        [0.0,  0.0, 0.0, 1.0],
    ])
}

#[rustfmt::skip]
pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4 {
    Matrix4::from_array([
        [1.0, xy,  xz,  0.0],
        [yx,  1.0, yz,  0.0],
        [zx,  zy,  1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

impl Matrix4 {
    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        translation(x, y, z) * self
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        scaling(x, y, z) * self
    }

    pub fn rotate_x(self, theta: f64) -> Self {
        rotation_x(theta) * self
    }

    pub fn rotate_y(self, theta: f64) -> Self {
        rotation_y(theta) * self
    }

    pub fn rotate_z(self, theta: f64) -> Self {
        rotation_z(theta) * self
    }

    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        shearing(xy, xz, yx, yz, zx, zy) * self
    }
}
