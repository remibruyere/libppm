pub struct W_matrix {
    pub weight: Vec<u8>,
    pub size: usize,
}

impl W_matrix {
    fn new(weight: Vec<u8>, size: usize) -> W_matrix {
        W_matrix {
            weight: weight,
            size: size,
        }
    }

    pub fn get_total_weight(&self) -> u128 {
        let mut total = 0;
        for w in &self.weight {
            total = total + w;
        }
        total as u128
    }
}
