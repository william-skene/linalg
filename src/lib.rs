use std::cmp::max;
use std::fmt::{self, Display};
use std::ops::{Add, Index, IndexMut, Mul, MulAssign};
#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn from_scalar(n_rows: usize, n_cols: usize, val: f64) -> Self {
        Matrix {
            rows: n_rows,
            cols: n_cols,
            data: vec![val; n_cols * n_rows],
        }
    }

    pub fn from_2d_vec(n_rows: usize, n_cols: usize, data: Vec<Vec<f64>>) -> Result<Self, String> {
        let mut data_formatted = Vec::<f64>::with_capacity(n_rows * n_cols);
        if data.len() != n_rows {
            return Err("Inconsistent row length".to_owned());
        }

        for row in data {
            if row.len() != n_cols {
                return Err("Inconsistent column length".to_owned());
            }
            data_formatted.extend(row);
        }

        Ok(Matrix {
            rows: n_rows,
            cols: n_cols,
            data: data_formatted,
        })
    }

    pub fn identity(size: usize) -> Self {
        let mut data = vec![0.0; size * size];
        for i in 0..size {
            data[i * size + i] = 1.0;
        }
        Matrix {
            rows: size,
            cols: size,
            data: data,
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        // Return the shape of the matrix in the form (rows, cols)
        (self.rows, self.cols)
    }

    pub fn transpose(self) -> Self {
        let mut ret = Matrix {
            rows: self.cols,
            cols: self.rows,
            data: vec![0.; self.cols * self.rows],
        };
        for i in 0..ret.rows {
            for j in 0..ret.cols {
                ret[(i, j)] = self[(j, i)];
            }
        }
        ret
    }

    pub fn pow(self, pow: usize) -> Self {
        if self.rows != self.cols {
            panic!("Can only raise square matrices to a power.");
        }
        if pow == 0 {
            return Matrix::identity(self.rows);
        }

        Self::pow_helper(self.clone(), pow)
    }

    fn pow_helper(mat: Self, pow: usize) -> Self {
        if pow == 0 {
            return Self::identity(mat.rows);
        } else if pow % 2 == 0 {
            return Self::pow_helper(mat.clone() * mat.clone(), pow / 2);
        } else {
            return mat.clone() * Self::pow_helper(mat.clone() * mat.clone(), pow / 2);
        }
    }
}

impl Add for Matrix {
    type Output = Matrix;
    fn add(self, other: Matrix) -> Self::Output {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Matrices of different shapes cannot be added together. Left({}, {}), Right({}, {})", self.rows, self.cols, other.rows, other.cols);
        } else {
            return Matrix {
                rows: self.rows,
                cols: self.cols,
                data: self
                    .data
                    .iter()
                    .zip(other.data.iter())
                    .map(|(x, y)| x + y)
                    .collect(),
            };
        }
    }
}

// Matrix Multiplication
impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        // Check that dims are correct
        if self.cols != rhs.rows {
            panic!(
                "LHS cols must be same as RHS rows to multiply. LHS: ({},{}), RHS: ({}, {})",
                self.rows, self.cols, rhs.rows, rhs.cols
            );
        }
        let mut out = Matrix::from_scalar(self.rows, rhs.cols, 0.);

        for i in 0..out.rows {
            for j in 0..out.cols {
                let mut el = 0.;
                for k in 0..self.cols {
                    el += self[(i, k)] * rhs[(k, j)];
                }
                out[(i, j)] = el;
            }
        }

        out
    }
}
impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Matrix) {
        // Check that dims are correct
        if self.cols != rhs.rows {
            panic!(
                "LHS cols must be same as RHS rows to multiply. LHS: ({},{}), RHS: ({}, {})",
                self.rows, self.cols, rhs.rows, rhs.cols
            );
        }
        let mut out = Matrix::from_scalar(self.rows, rhs.cols, 0.);

        for i in 0..out.rows {
            for j in 0..out.cols {
                let mut el = 0.;
                for k in 0..self.cols {
                    el += self[(i, k)] * rhs[(k, j)];
                }
                out[(i, j)] = el;
            }
        }

        *self = out;
    }
}
// Scalar Multiplication
impl Mul<Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        rhs * self
    }
}

impl Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(mut self, rhs: f64) -> Self::Output {
        for el in &mut self.data {
            *el *= rhs;
        }
        self
    }
}

fn number_of_digits(number: f64) -> i64 {
    let tol = 1e-8;
    if number.abs() < tol {
        return 1;
    }
    return (number.log(10.0) + tol).floor() as i64 + 1;
}

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = " ";
        let tol = 1e-8;
        let mut s = "".to_string();
        let mut max_num_len = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                let elem = self[(row, col)];
                max_num_len = max(number_of_digits(elem), max_num_len);
            }
        }

        for row in 0..self.rows {
            for col in 0..self.cols {
                let elem = self[(row, col)];
                let mut num_len = number_of_digits(elem);
                if elem.abs() < tol {
                    num_len = 1
                }
                for _ in 0..(max_num_len - num_len) {
                    s.push_str(&sep);
                }
                s.push_str(&format!("{}", elem)[..]);
                if col != self.cols - 1 {
                    s.push_str(&sep);
                }
            }
            s.push('\n');
        }
        s.push_str("Shape: ");
        s.push_str(&self.rows.to_string());
        s.push('x');
        s.push_str(&self.cols.to_string());
        write!(f, "{}", s)
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, (i, j): (usize, usize)) -> &f64 {
        if i < self.rows && j < self.cols {
            return &self.data[i * self.cols + j];
        } else {
            panic!(
                "index out of bounds: the shape is ({}, {}) but the index is ({}, {}).",
                self.rows, self.cols, i, j
            )
        }
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut f64 {
        if i < self.rows && j < self.cols {
            return &mut self.data[i * self.cols + j];
        } else {
            panic!(
                "index out of bounds: the shape is ({}, {}) but the index is ({}, {}).",
                self.rows, self.cols, i, j
            )
        }
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.clone(),
        }
    }
}
