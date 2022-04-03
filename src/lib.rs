mod matrix;
mod vector;
#[cfg(test)]
mod tests {
    #[test]
    fn it_tests_vector_construction() {
        use crate::vector::vector;
        let shape = vector::Vector::new(2).shape();
        let data = vector::Vector::new(2).as_vec();
        let vec = vec![0.0; 2];
        assert_eq!((vec, 2), (data, shape));
    }
    #[test]
    fn it_tests_matrix_construction() {
        use crate::matrix::matrix;
        let shape = matrix::Matrix::new(2, 2).shape();
        let data = matrix::Matrix::new(2, 2).as_vec();
        let vec = vec![vec![0.0; 2]; 2];
        assert_eq!((vec, (2, 2)), (data, shape));
    }
}
