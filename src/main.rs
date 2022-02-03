mod material;

use material::Material;

fn main() {
    let mat = Material::new(1000_f32, 0.3);

    println!("{}", mat.get_mat_matrix());
    println!("Hello, world!");
}
