use linalg::Matrix;

fn main() {
    let mat1 = Matrix::from_2d_vec(2, 3, vec![vec![1000., 0., 1.], vec![0., 3., 5.]]).unwrap();
    let mat2 = Matrix::from_2d_vec(2, 2, vec![vec![11., 3.], vec![7., 11.]]).unwrap();
    println!("{mat1}");
    println!("{mat2}");
    println!("{}", mat2.clone() + mat2.clone());
    println!("{}", 15.0 * mat1.clone());
    println!("{}", (mat2.clone() * mat1.clone()).transpose());
    println!("{}", mat1.clone().transpose() * mat2.clone().transpose());
    println!("{}", mat2.pow(5));
}
