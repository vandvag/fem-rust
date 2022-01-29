#[macro_use]
extern crate rulinalg;
mod material;
use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;

use material::Material;

fn main() {
    let a = Matrix::new(3, 3, vec![1, 0, 0, 0, 1, 0, 0, 0, 1]);
    let b = Vector::new(vec![3, 3, 3]);
    let mat = Material::new(1000 as f32, 0.3);

    println!("{}", mat.get_mat_matrix());
    println!("Hello, world!");
}
