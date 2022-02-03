mod discretization;
mod material;
mod node;

use material::Material;
use node::Node;

fn main() {
    discretization::discretization();
    println!("Hello, world!");
}
