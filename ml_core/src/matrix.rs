pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn add(&self, other: &Matrix) -> Self {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Matrix dimensions must match for addition");
        }

        let new_data: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();

        Self {
            rows: self.rows,
            cols: self.cols,
            data: new_data,
        }
    }

    pub fn dot(&self, other: &Matrix) -> Self {
        if self.cols != other.rows {
            panic!("Matrix dimensions do not match for dot product");
        }

        let mut new_data: Vec<f64> = vec![0.0; self.rows * other.cols];
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    new_data[i * other.cols + j] +=
                        self.data[i * self.cols + k] * other.data[k * other.cols + j];
                }
            }
        }

        Self {
            rows: self.rows,
            cols: other.cols,
            data: new_data,
        }
    }

    pub fn transpose(&self) -> Self {
        let mut new_data: Vec<f64> = vec![0.0; self.rows * self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                new_data[(j * self.rows) + i] = self.data[(i * self.cols) + j];
            }
        }

        Self {
            rows: self.cols,
            cols: self.rows,
            data: new_data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros() {
        let m = Matrix::zeros(2, 3);

        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 3);
        assert_eq!(m.data.len(), 6);

        assert_eq!(m.data, vec![0.0; 6]);
    }

    #[test]
    fn test_add() {
        let m1 = Matrix {
            rows: 2,
            cols: 2,
            data: vec![1.0, 2.0, 3.0, 4.0],
        };
        let m2 = Matrix {
            rows: 2,
            cols: 2,
            data: vec![5.0, 6.0, 7.0, 8.0],
        };
        let result = m1.add(&m2);

        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 2);
        assert_eq!(result.data, vec![6.0, 8.0, 10.0, 12.0]);
    }

    #[test]
    fn test_dot() {
        // 2x3 matrix
        let m1 = Matrix {
            rows: 2,
            cols: 3,
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        };
        // 3x2 matrix
        let m2 = Matrix {
            rows: 3,
            cols: 2,
            data: vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
        };
        let result = m1.dot(&m2);

        // Result should be 2x2
        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 2);
        assert_eq!(result.data, vec![58.0, 64.0, 139.0, 154.0]);
    }

    #[test]
    fn test_transpose() {
        let m = Matrix {
            rows: 2,
            cols: 3,
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        };
        let result = m.transpose();

        assert_eq!(result.rows, 3);
        assert_eq!(result.cols, 2);
        assert_eq!(result.data, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    }
}
