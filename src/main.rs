mod material;
mod node;

use material::Material;
use node::Node;

fn main() {
    let mat = Material::new(1000_f32, 0.3);

    let node = Node::new(1, 0_f32, 1_f32);
    println!("{}", mat.get_mat_matrix());
    println!("New node: {:?}", node);
    println!("Hello, world!");
}
