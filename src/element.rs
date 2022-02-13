use std::{collections::HashMap, fmt::Display};
use crate::{material::Material, node::Node};
use nalgebra::{SMatrix};

/// Implementation for a q1 2D finite element
/// node 1: +---------------------+ : node 2
/// 		|					  |
/// 		|					  |
/// 		|					  |
/// 		|					  |
/// 		|					  |
/// 		|					  |
/// 		|					  |
/// node 4: +---------------------+ : node 3

// TODO Element struct needs to include a reference to a
// TODO discretization struct, in order to be able to access
// TODO the vector containing all the node objects
// TODO
/*
Element need to contatin the following fields:
    -- id: i32 -> Id number of the element
    -- idMatrix: Array1<i32> -> ID Matrix of the element
    -- nodes: Vec<&Node> -> Vector containing references to the structure nodes
    -- mateial: &Material -> Reference to the material of the element
*/

pub struct Element<'a> {
    id: i32,
    // Element id
    nodes: Vec<&'a Node>,
    // Vector of nodes
    mat: &'a Material, // Reference to a material
}

impl Element<'_> {
    pub fn new<'a>(
        _id: i32,
        _n1: i32,
        _n2: i32,
        _n3: i32,
        _n4: i32,
        _mat: &'a Material,
        _nodes: &'a HashMap<i32, Node>,
    ) -> Element<'a> {
        let mut nds: Vec<&Node> = Vec::new();

        nds.push(&_nodes[&_n1]);
        nds.push(&_nodes[&_n2]);
        nds.push(&_nodes[&_n3]);
        nds.push(&_nodes[&_n4]);

        Element {
            id: _id,
            nodes: nds,
            mat: _mat,
        }
    }

    pub fn get_statlin(&self) -> SMatrix<f64, 8, 8> {
        // Calculates the static linear stiffness matrix for the element

        // let mut _ele_stiffness: Array2<f64> = zeros((8, 8));
        let mut _ele_stiffness: SMatrix<f64, 8, 8> = SMatrix::zeros();
        // This array contains the data required for the Gauss integration
        // scheme. Column 0 contains the position of the Gauss points
        // and column 1 contains the Gauss points factors
        let int_data: Array2<f64> =
            array![[-1.0 / 3.0_f64.sqrt(), 1.0], [1.0 / 3.0_f64.sqrt(), 1.0]];

        let mut deriv: Array2<f64> = zeros((4, 2));
        // xi variable used for shape function derivation
        let mut xi = 0_f64;
        let mut eta = 0_f64;
        let mut omega_gp = 0_f64;

        for i in 0..2 {
            for j in 0..2 {
                xi = int_data[[i, 0]];
                eta = int_data[[j, 0]];
                omega_gp = int_data[[i, 1]] * int_data[[j, 1]];

                deriv[[0, 0]] = -0.25 * (1.0 - eta);
                deriv[[0, 1]] = -0.25 * (1.0 - xi);
                deriv[[1, 1]] = 0.25 * (1.0 - eta);
                deriv[[2, 0]] = 0.25 * (1.0 + eta);
                deriv[[2, 1]] = 0.25 * (1.0 + xi);
                deriv[[3, 0]] = -0.25 * (1.0 + eta);
                deriv[[3, 1]] = 0.25 * (1.0 - xi);

                // Jacobian matrix
                // TODO For the jacobian I need to know the coordinates of the
                // TODO nodes of the element
                let mut jaco: Array2<f64> = zeros((2, 2));

                for k in 0..4 {
                    jaco[[0, 0]] += deriv[[k, 0]] * self.nodes[k].get_coords()[0];
                    jaco[[0, 1]] += deriv[[k, 0]] * self.nodes[k].get_coords()[1];
                    jaco[[0, 0]] += deriv[[k, 1]] * self.nodes[k].get_coords()[0];
                    jaco[[0, 0]] += deriv[[k, 1]] * self.nodes[k].get_coords()[1];
                }

                // det = ndarray_linalg::Determinant::det(&jaco).unwrap();
                det = jaco.det();
                let jaco_inv = jaco.inv();

                // B operator
                let mut bop: Array2<f64> = zeros((3, 8));
                for k in 0..4 {
                    let node_start = k * 2;
                    bop[[0, node_start + 0]] =
                        jaco_inv[[0, 0]] * deriv[[k, 0]] + jaco_inv[[0, 1]] * deriv[[k, 1]];
                    bop[[1, node_start + 1]] =
                        jaco_inv[[1, 0]] * deriv[[k, 0]] + jaco_inv[[1, 1]] * deriv[[k, 1]];
                    bop[[2, node_start + 0]] = bop[[1, node_start + 1]];
                    bop[[2, node_start + 1]] = bop[[0, node_start + 0]];
                }
                let boptr = bop.t();
                _ele_stiffness =
                    _ele_stiffness + omega_gp * det * boptr * self.mat.get_mat_matrix() * bop;
            }
        }
        _ele_stiffness
    }
}

impl Display for Element<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Element: {}\tNodes: ({}, {}, {}, {})",
            self.id,
            self.nodes[0].get_id(),
            self.nodes[1].get_id(),
            self.nodes[2].get_id(),
            self.nodes[3].get_id()
        )
    }
}
