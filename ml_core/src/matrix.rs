#[allow(dead_code)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

#[allow(dead_code)]
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
}
