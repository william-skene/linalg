use std::fmt::{Display, self};
use std::ptr::eq;

#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>
}

impl Matrix {
    pub fn from_scalar(n_rows: usize, n_cols: usize, val: f64) -> Self {
        Matrix {
            rows: n_rows,
            cols: n_cols,
            data: vec![vec![val; n_cols]; n_rows] // Potentially a better way, something about non localized data https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
        }
    }

    pub fn from_2d_vec(n_rows: usize, n_cols: usize, data: Vec<Vec<f64>>) -> Self {
        Matrix {
            rows: n_rows,
            cols: n_cols,
            data: data // Potentially a better way, something about non localized data https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        // Return the shape of the matrix in the form (rows, cols)
        (self.rows, self.cols)
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = ", ";
        let mut s = "".to_string();

        for row in &self.data {
            for elem in row {
                s.push_str(&elem.to_string());
                if !eq(elem, row.last().unwrap()) {
                    s.push_str(&sep);
                }
            }
            s.push('\n');
        }
        s.push_str("Size: ");
        s.push_str(&self.rows.to_string());
        s.push('x');
        s.push_str(&self.cols.to_string());
        write!(f, "{}", s)
    }
}