use std::ops::{self, Index, IndexMut};
use std::{cmp::min, usize};

use crate::sigma::sigma;

#[derive(Debug, Clone)]
pub struct NotInvertible;
pub struct Mat {
    pub row_vecs: Vec<Vec<f64>>,
    pub count_columns: usize,
    pub count_rows: usize,
}

impl Mat {
    pub fn clone(&self) -> Mat {
        let mut mat = Mat::new(self.count_rows, self.count_columns);
        mat.for_each(|mat, i, j| mat[i][j] = self[i][j]);
        mat
    }

    pub fn for_each<T>(&mut self, do_something: T)
    where
        T: Fn(&mut Mat, usize, usize) -> (),
    {
        for i in 0..self.count_rows {
            for j in 0..self.count_columns {
                do_something(self, i, j);
            }
        }
    }

    /// Calculate inverse matrix
    pub fn inverse(&self) -> Result<Mat, NotInvertible> {
        assert_eq!(
            self.count_rows, self.count_columns,
            "A matrix must be n by n"
        );
        // identity matrix
        let mut inverse_mat = Mat::new(self.count_rows, self.count_columns);
        let mut mat = self.clone();
        // use gaussian elimination to calculate inverse matrix
        for j in 0..self.count_columns {
            for i in j..self.count_rows {
                let element = mat[i][j];
                if element != 0.0 {
                    if i != j {
                        mat.swap_row(i, j);
                        inverse_mat.swap_row(i, j);
                    }
                    if element != 1.0 {
                        mat.multiply_row(i, 1.0 / element);
                        inverse_mat.multiply_row(i, 1.0 / element);
                    }
                    break;
                }
                if i == self.count_rows - 1 {
                    return Err(NotInvertible);
                }
            }
            for i in 0..self.count_rows {
                if i != j {
                    let element = mat[i][j];
                    if element != 0.0 {
                        mat.subtract_row(i, j, element);
                        inverse_mat.subtract_row(i, j, element);
                    }
                }
            }
        }
        Ok(inverse_mat)
    }

    /// Multiply a row with a scalar
    fn multiply_row(&mut self, row: usize, scalar: f64) {
        for k in 0..self.count_columns {
            self[row][k] = self[row][k] * scalar;
        }
    }

    /// Init Identity matrix
    pub fn new(count_rows: usize, count_columns: usize) -> Mat {
        Mat::new_diagonal(
            count_rows,
            count_columns,
            vec![1.0; min(count_columns, count_rows)],
        )
    }

    /// Init diagonal m by n matrix
    pub fn new_diagonal(count_rows: usize, count_columns: usize, diagonal: Vec<f64>) -> Mat {
        let row_vec: Vec<f64> = vec![0.0; count_columns];
        let mut row_vecs = vec![row_vec; count_rows];
        for i in 0..min(min(count_columns, count_rows), diagonal.len()) {
            row_vecs[i][i] = diagonal[i];
        }
        Mat {
            count_columns,
            count_rows,
            row_vecs,
        }
    }

    /// Subtract a row by another multiplied by scalar
    fn subtract_row(&mut self, row: usize, column: usize, scalar: f64) -> () {
        for k in 0..self.count_columns {
            self[row][k] = self[row][k] - self[column][k] * scalar;
        }
    }

    /// Swap rows
    fn swap_row(&mut self, row: usize, another_row: usize) -> () {
        for k in 0..self.count_columns {
            let temp_row = self[row][k];
            self[row][k] = self[another_row][k];
            self[another_row][k] = temp_row;
        }
    }
}

impl Index<usize> for Mat {
    type Output = Vec<f64>;
    fn index<'a>(&'a self, i: usize) -> &'a Vec<f64> {
        &self.row_vecs[i]
    }
}
impl IndexMut<usize> for Mat {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Vec<f64> {
        &mut self.row_vecs[i]
    }
}
impl_op_ex!(+|a: &Mat, b: &Mat| -> Mat {
    assert_eq_matrix_size(a, b);
    let mut new_mat = a.clone();
    new_mat.for_each(|new_mat, i, j| new_mat[i][j] += b[i][j]);
    new_mat
});
impl_op_ex!(-|a: &Mat, b: &Mat| -> Mat {
    assert_eq_matrix_size(a, b);
    let mut new_mat = a.clone();
    new_mat.for_each(|new_mat, i, j| new_mat[i][j] -= b[i][j]);
    new_mat
});
impl_op_ex!(*|a: &Mat, b: f64| -> Mat {
    let mut new_mat = a.clone();
    new_mat.for_each(|new_mat, i, j| new_mat[i][j] *= b);
    new_mat
});
impl_op_ex!(*|a: &Mat, b: &Mat| -> Mat {
    assert_eq!(a.count_columns, b.count_rows);
    let mut new_mat = Mat::new(a.count_rows, b.count_columns);
    new_mat
        .for_each(|new_mat, i, j| new_mat[i][j] = sigma(|k| a[i][k] * b[k][j], 0, a.count_columns));
    new_mat
});
impl_op_ex!(/|a: &Mat, b: f64| -> Mat {
    let mut new_mat = a.clone();
    new_mat.for_each(|new_mat, i, j| new_mat[i][j] /= b);
    new_mat
});

fn assert_eq_matrix_size(a: &Mat, b: &Mat) {
    assert_eq!(
        a.count_columns, b.count_columns,
        "Size of two matrices must be the same"
    );
    assert_eq!(
        a.count_rows, b.count_rows,
        "Size of twio matrices must be the same"
    );
}

#[cfg(test)]
mod test_matrix {
    use crate::matrix::Mat;

    #[test]
    fn test_matrix() {
        let mut identity = Mat::new(3, 3);
        let mut a = Mat::new(3, 3);
        a[0] = vec![1.5, 0.2, 2.0];
        a[1] = vec![0.5, 1.2, 1.0];
        a[2] = vec![0.0, 0.5, 3.0];
        let a_inverse = a.inverse().expect("Failed to calculate inverse");
        let a_x_a_inverse = a * a_inverse;
        let epsilon = 0.000000000000001;
        identity.for_each(|identitty, i, j| {
            let d = identitty[i][j] - a_x_a_inverse[i][j];
            assert!(-epsilon < d && d < epsilon);
        });
    }
}
