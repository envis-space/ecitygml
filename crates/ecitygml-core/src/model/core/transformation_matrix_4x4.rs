use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformationMatrix4x4 {
    matrix: nalgebra::Matrix4<f64>,
}

impl TransformationMatrix4x4 {
    pub fn new(matrix: nalgebra::Matrix4<f64>) -> Self {
        Self { matrix }
    }

    pub fn identity() -> Self {
        Self {
            matrix: nalgebra::Matrix4::identity(),
        }
    }

    /// Constructs from 16 values in row-major order (as used in GML).
    pub fn try_from_row_major(values: Vec<f64>) -> Result<Self, Error> {
        if values.len() != 16 {
            return Err(Error::InvalidMatrixSize {
                found: values.len(),
            });
        }
        Ok(Self {
            matrix: nalgebra::Matrix4::from_row_slice(&values),
        })
    }

    /// Returns the values in row-major order (as used in GML).
    pub fn to_row_major(&self) -> Vec<f64> {
        (0..4)
            .flat_map(|row| (0..4).map(move |col| self.matrix[(row, col)]))
            .collect()
    }

    pub fn matrix(&self) -> &nalgebra::Matrix4<f64> {
        &self.matrix
    }

    pub fn set_matrix(&mut self, matrix: nalgebra::Matrix4<f64>) {
        self.matrix = matrix;
    }
}
