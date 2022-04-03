mod matrix;

#[cfg(test)]
mod tests {
    #[test]
    fn it_tests_matrix_construction() {
        use crate::matrix::matrix;
        let shape = matrix::Matrix::new(2, 2).shape();
        let data = matrix::Matrix::new(2, 2).as_vec();
        let vec = vec![vec![0.0; 2]; 2];
        assert_eq!((vec, (2, 2)), (data, shape));
    }
}
