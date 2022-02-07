mod discretization;
mod element;
mod material;
mod node;

use material::Material;
use std::collections::HashMap;
use {element::Element, node::Node};

fn main() {
    discretization::discretization();
    println!("Hello, world!");
}
