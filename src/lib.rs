mod matrix;
mod vector;
#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn it_tests_vector_construction() {
        use crate::vector::vector;
        let shape = vector::Vector::new(2).shape();
        let data = vector::Vector::new(2).as_vec();
        let vec = vec![0.0; 2];
        assert_eq!((vec, 2), (data, shape));
    }
    #[test]
    fn it_tests_vector_to_scalar_addition() {
        use crate::vector::vector;
        let mut v = vector::Vector::new(2);
        v += 1;
        assert_eq!(v.as_vec(), vec![1.0, 1.0]);
    }
    #[test]
    fn it_tests_vector_to_scalar_subtraction() {
        use crate::vector::vector;
        let mut v = vector::Vector::new(2);
        v += 2;
        v -= 1;
        assert_eq!(v.as_vec(), vec![1.0, 1.0]);
    }
    #[test]
    fn it_tests_vector_to_scalar_multiplication() {
        use crate::vector::vector;
        let mut v = vector::Vector::new(2);
        v += 1;
        v *= 2;
        assert_eq!(v.as_vec(), vec![2.0, 2.0]);
    }
    #[test]
    fn it_tests_vector_to_scalar_division() {
        use crate::vector::vector;
        let mut v = vector::Vector::new(2);
        v += 2;
        v /= 2;
        assert_eq!(v.as_vec(), vec![1.0, 1.0]);
    }
    #[test]
    fn it_tests_vector_to_vector_addition() {
        use crate::vector::vector;
        let mut v1 = vector::Vector::new(2);
        v1.set(vec![2.0, 3.0]);
        let mut v2 = vector::Vector::new(2);
        v2.set(vec![2.0, 3.0]);
        v1 += v2;
        assert_eq!(v1.as_vec(), [4.0, 6.0]);
    }
    #[test]
    fn it_tests_vector_to_vector_subtraction() {
        use crate::vector::vector;
        let mut v1 = vector::Vector::new(2);
        v1.set(vec![2.0, 3.0]);
        let mut v2 = vector::Vector::new(2);
        v2.set(vec![2.0, 3.0]);
        v1 -= v2;
        assert_eq!(v1.as_vec(), [0.0, 0.0]);
    }
    #[test]
    fn it_tests_vector_to_vector_multimplication() {
        use crate::vector::vector;
        let mut v1 = vector::Vector::new(2);
        v1.set(vec![2.0, 3.0]);
        let mut v2 = vector::Vector::new(2);
        v2.set(vec![2.0, 3.0]);
        v1 *= v2;
        assert_eq!(v1.as_vec(), [4.0, 9.0]);
    }
    #[test]
    fn it_tests_vector_to_vector_division() {
        use crate::vector::vector;
        let mut v1 = vector::Vector::new(2);
        v1.set(vec![2.0, 3.0]);
        let mut v2 = vector::Vector::new(2);
        v2.set(vec![2.0, 3.0]);
        v1 -= v2;
        assert_eq!(v1.as_vec(), [0.0, 0.0]);
    }
    #[test]
    fn it_tests_dot_product_of_two_vectors() {
        use crate::vector::vector;
        let mut v1 = vector::Vector::new(3);
        v1.set(vec![1.0, 2.0, 3.0]);
        let mut v2 = vector::Vector::new(3);
        v2.set( vec![4.0, 5.0, 6.0]);
        assert_eq!(32.0, v1.dot(v2));
    }
    #[test]
    fn it_tests_matrix_construction() {
        use crate::matrix::matrix;
        let shape = matrix::Matrix::new(2, 2).shape();
        let data = matrix::Matrix::new(2, 2).as_vec();
        let vec = vec![vec![0.0; 2]; 2];
        assert_eq!((vec, (2, 2)), (data, shape));
    }
    #[test]
    fn it_tests_matrix_ones_construction() {
        use crate::matrix::matrix;
        let shape = matrix::Matrix::ones(2, 2).shape();
        let data = matrix::Matrix::ones(2, 2).as_vec();
        let vec = vec![vec![1.; 2]; 2];
        assert_eq!((vec, (2, 2)), (data, shape));
    }
    #[test]
    fn it_tests_random_matrix_construction() {
        use crate::matrix::matrix;
        let m = matrix::Matrix::random((2, 2), (-1., 0.));
        assert_ne!(m.as_vec()[0][0], m.as_vec()[0][1]);
        assert_ne!(m.as_vec()[1][0], m.as_vec()[1][1]);
    }
    #[test]
    fn it_tests_matrix_set() {
        use crate::matrix::matrix;
        let mut m = matrix::Matrix::random((2, 2), (-1., 0.));
        m.set(vec![vec![0.; 2]; 2]);
        let vec = vec![vec![0.; 2]; 2];
        assert_eq!(m.as_vec(), vec);
    }
    #[test]
    fn it_tests_matrix_multiplication() {
        use crate::matrix::matrix;
        let mut m = matrix::Matrix::new(2, 3);
        m.set(vec![vec![1., 2., 3.], vec![4., 5., 6.]]);
        let mut m2 = matrix::Matrix::new(3, 2);
        m2.set(vec![vec![1., 2.], vec![3., 4.], vec![5., 6.]]);
        let res = vec![vec![22., 28.], vec![49., 64.]];
        assert_eq!(res, matrix::Matrix::dot(m, m2).as_vec());
    }
}
