use matrix::Matrix;
mod matrix;

fn main() {
    let mat1 = Matrix::from_scalar(3, 2, 5.555);
    let mat2 = Matrix::from_2d_vec(2, 2, vec![vec![1., 2.], vec![3., 4.]]);
    println!("{mat1}");
    println!("{mat2}");
}
