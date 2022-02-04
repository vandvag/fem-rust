use std::collections::HashMap;

use crate::material::Material;
use crate::node::Node;
use crate::element::Element;

// Function that prepares the discretization

pub fn discretization() {
    // Counter for Dirichlet dofs
    let mut num_dof_dirich = 0;
    // Counter for dofs that need to be computed
    let mut num_dof_solve = 0;
    // Total dofs
    let mut num_dof_total = 0;

    println!("==============================");
    println!("Preparing discretization");
    println!("==============================");

    // Create a vector containing all the different
    // material parameters
    let mut materials: Vec<Material> = Vec::new();

    let youngs = 1000_f64;
    let poisson = 0.5;

    materials.push(Material::new(youngs, poisson));

    // Read Geometry data
    // Here we are discretizing a 2D plate
    // with dimensions l_x (x) l_y
    // with div_x and div_y elements in x and y axis respectively

    let l_x = 10_f64;
    let l_y = 2_f64;
    let div_x = 20;
    let div_y = 5;

    /******************************
                Nodes
    ******************************/
    // Collect nodes in a hash map
    let num_nodes: i32 = (&div_x + 1) * (&div_y + 1);
    let mut nodes: HashMap<usize, Node> = HashMap::new();

    println!("==============================");
    println!("Nodes: ");
    // calculate size of one element
    let el_x = l_x / div_x as f64;
    let el_y = l_y / div_y as f64;

    for i in 0..(div_y + 1) {
        for j in 0..(div_x + 1) {
            // nodes[(i * (div_x + 1) * j) as usize] =
            //     Node::new(i * (div_x + 1) + j, j as f64* el_x, i as f64* el_y);
            nodes.insert(
                (i * (div_x + 1) + j) as usize,
                Node::new(i * (div_x + 1) + j, j as f64 * el_x, i as f64 * el_y),
            );
        }
    }

    /******************************
              Elements
    ******************************/
    let num_elems: i32 = (&div_x + 1) * (&div_y + 1);
    let mut elements: HashMap<usize, Element> = HashMap::new();

    println!("==============================");
    println!("Nodes: ");

    for i in 0..div_y {
        for j in 0..div_x {
            unimplemented!("You have to wait bro")
        }
    }
    
}
