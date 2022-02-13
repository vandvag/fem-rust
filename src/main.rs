// mod discretization;
// mod element;
mod material;
// mod node;

use material::Material;
// use std::collections::HashMap;
// use {element::Element, node::Node};

fn main() {
    // discretization::discretization();
    let mat = Material::new(1000.0, 0.5);
    println!("{}", &mat);
    println!("{}", mat.get_mat_matrix());
    println!("Hello, world!");
}
