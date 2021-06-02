use std::ops::{Index, IndexMut};
use std::{cmp::min, usize};

pub fn binomial(n: usize, k: usize) -> u32 {
    let mut pascal_triangle: Vec<Vec<u32>> = Vec::with_capacity(n + 1);
    for i in 0..n + 1 {
        let mut row_i = Vec::with_capacity(i + 1);
        for j in 0..i + 1 {
            row_i[j] = if j == 0 || j == i {
                1
            } else {
                pascal_triangle[i - 1][j - 1] + pascal_triangle[i - 1][j]
            };
        }
        pascal_triangle[i] = row_i;
    }
    pascal_triangle[n][k]
}

pub fn sigma(a: fn(k: i32) -> f64, from: i32, to: i32) -> f64 {
    let mut sum = 0.0;
    for k in from..to {
        sum = sum + a(k);
    }
    sum
}

pub fn sigma_vec(a: fn(k: i32) -> (f64, f64), from: i32, to: i32) -> (f64, f64) {
    let mut sum = (0.0, 0.0);
    for k in from..to {
        let ak = a(k);
        sum = (sum.0 + ak.0, sum.1 + ak.1);
    }
    sum
}

pub struct Mat {
    pub row_vecs: Vec<Vec<f64>>,
    pub count_columns: usize,
    pub count_rows: usize,
}

impl Mat {
    pub fn new(count_rows: usize, count_columns: usize) -> Mat {
        let row_vec: Vec<f64> = vec![0.0; count_columns];
        let mut row_vecs = vec![row_vec; count_rows];
        for i in 0..min(count_columns, count_rows) {
            row_vecs[i][i] = 1.0;
        }
        Mat {
            count_columns,
            count_rows,
            row_vecs,
        }
    }

    pub fn inverse() {
        todo!()
    }

    pub fn clone(&self) -> Mat {
        let mut mat = Mat::new(self.count_rows, self.count_columns);
        for i in 0..mat.count_rows - 1 {
            for j in 0..mat.count_columns - 1 {
                mat[i][j] = self[i][j];
            }
        }
        mat
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
