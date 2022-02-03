use crate::material::Material;
use crate::node::Node;

#[derive(Debug)]
pub struct Discretization {
    nodes: Vec<Node>,
    materials: Vec<Material>,
}

impl Discretization {
    pub fn new(_nodes: Vec<Node>, _materials: Vec<Material>) -> Discretization {
        Discretization {
            nodes: _nodes,
            materials: _materials,
        }
    }

    pub fn assign_dofs() {
        panic!("Not implemented yet!");
    }

    pub fn assemble_static_linear() {
        panic!("Not implemented yet!");
    }

    pub fn assemble_fext() {
        panic!("Not implemented yet!");
    }
}
