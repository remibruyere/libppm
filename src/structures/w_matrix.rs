/// A matrix is represented here
pub struct W_matrix {
    /// A matrix should have an array integer and a size. The size should match the real size of the matrix.
    pub weight: Vec<u8>,
    pub size: usize,
}

impl W_matrix {
    /// Returns a matrix with the given params.
    ///
    /// # Arguments
    ///
    /// * `weight` - An array that contain integer that represent a matrix
    /// * `size` - A size that represente the size of the matrix
    fn new(weight: Vec<u8>, size: usize) -> W_matrix {
        W_matrix {
            weight: weight,
            size: size,
        }
    }

    /// Returns the total weight of the matrix
    ///
    /// # Arguments
    ///
    /// * `self` - an instance of matrix
    ///
    /// # Example:
    /// Given: a matrix [0, 1, 0, 2, 3, 2, 0, 1, 0]
    /// When: you call this function on it
    /// Then: you should get 9
    pub fn get_total_weight(&self) -> u128 {
        let mut total = 0;
        for w in &self.weight {
            total = total + w;
        }
        total as u128
    }
}
