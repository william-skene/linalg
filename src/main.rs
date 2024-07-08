use linalg::Matrix;

fn main() {
    let mat1 = Matrix::from_2d_vec(2, 3, vec![vec![1000., 0., 1.], vec![0., 3., 5.]]).unwrap();
    let mat2 = Matrix::from_2d_vec(2, 2, vec![vec![11., 3.], vec![7., 11.]]).unwrap();
    println!("{mat1}\n");
    println!("{mat2}\n");
    println!("{}\n", mat2.clone() + mat2.clone());
    println!("{}\n", 15.0 * mat1.clone());
    println!("{}\n", (mat2.clone() * mat1.clone()).transpose());
    println!("{}\n", mat1.clone().transpose() * mat2.clone().transpose());
    println!("{}\n", mat2.pow(5));
}
