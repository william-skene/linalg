use linalg::Matrix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let mat1 = Matrix::from_2d_vec(2, 2, vec![vec![1., 1.], vec![1., 1.]]).unwrap();
        let mat2 = Matrix::from_2d_vec(2, 2, vec![vec![1., 1.], vec![1., 1.]]).unwrap();
        assert_eq!(mat1, mat2);
    }

    #[test]
    fn test_neq_same_shape() {
        let mat1 = Matrix::from_2d_vec(2, 2, vec![vec![1., 2.], vec![2., 3.]]).unwrap();
        let mat2 = Matrix::from_2d_vec(2, 2, vec![vec![2., 3.], vec![3., 4.]]).unwrap();
        assert!(mat1 != mat2);
    }

    #[test]
    fn test_neq_diff_shape() {
        let mat1 = Matrix::from_2d_vec(2, 3, vec![vec![1., 2., 4.], vec![2., 3., 4.]]).unwrap();
        let mat2 = Matrix::from_2d_vec(2, 2, vec![vec![2., 3.], vec![3., 4.]]).unwrap();
        assert!(mat1 != mat2);
    }

    #[test]
    fn test_add() {
        let mat1 = Matrix::from_2d_vec(2, 2, vec![vec![1., 2.], vec![2., 3.]]).unwrap();
        let mat2 = Matrix::from_2d_vec(2, 2, vec![vec![2., 3.], vec![3., 4.]]).unwrap();
        let expected = Matrix::from_2d_vec(2, 2, vec![vec![3., 5.], vec![5., 7.]]).unwrap();

        let result = mat1.clone() + mat2.clone();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_square_transpose() {
        let mat = Matrix::from_2d_vec(2, 2, vec![vec![1., 2.], vec![3., 4.]]).unwrap();
        let expected = Matrix::from_2d_vec(2, 2, vec![vec![1., 3.], vec![2., 4.]]).unwrap();

        let result = mat.transpose();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rect_transpose() {
        let mat = Matrix::from_2d_vec(2, 3, vec![vec![1., 2., 3.], vec![4., 5., 6.]]).unwrap();
        let expected =
            Matrix::from_2d_vec(3, 2, vec![vec![1., 4.], vec![2., 5.], vec![3., 6.]]).unwrap();

        let result = mat.transpose();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_pow_zero_returns_identity() {
        let mat = Matrix::from_2d_vec(2, 2, vec![vec![1., 2.], vec![3., 4.]]).unwrap();
        let expected = Matrix::identity(2);

        let result = mat.pow(0);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pow_1_returns_mat() {
        let mat = Matrix::from_2d_vec(2, 2, vec![vec![1., 2.], vec![3., 4.]]).unwrap();
        let result = mat.pow(1);

        assert_eq!(mat, result);
    }

    #[test]
    fn test_pow_even_returns_mat() {
        let mat = Matrix::from_2d_vec(2, 2, vec![vec![1., 2.], vec![3., 4.]]).unwrap();
        let expected = Matrix::from_2d_vec(2, 2, vec![vec![199., 290.], vec![435., 634.]]).unwrap();
        let result = mat.pow(4);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_pow_odd_returns_mat() {
        let mat = Matrix::from_2d_vec(2, 2, vec![vec![1., 2.], vec![3., 4.]]).unwrap();
        let expected = Matrix::from_2d_vec(2, 2, vec![vec![37., 54.], vec![81., 118.]]).unwrap();
        let result = mat.pow(3);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_pow_not_square_panics() {
        let mat =
            Matrix::from_2d_vec(3, 2, vec![vec![1., 2.], vec![3., 4.], vec![1., 2.]]).unwrap();
        let result = std::panic::catch_unwind(|| mat.pow(3));
        assert!(result.is_err());
    }
}
