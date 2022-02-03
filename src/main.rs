mod discretization;
mod material;
mod node;

use discretization::Discretization;
use material::Material;
use node::Node;

fn main() {
    let mat = Material::new(1000_f32, 0.3);
    let mat1 = mat.clone();
    let node1 = Node::new(1, 0_f32, 1_f32);
    let node2 = Node::new(2, 1_f32, 1_f32);
    let arr = [node1, node2];
    let discr = Discretization::new(Vec::from(arr), Vec::from([mat]));
    println!("{}", mat1.get_mat_matrix());
    println!("Hello, world!");
    println!("{:?}", discr);
}
