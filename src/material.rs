use std::fmt;
use std::fmt::Formatter;
use std::path::Display;
use ndarray::array;
use ndarray::Array2;

#[derive(Debug, Clone)]
pub struct Material {
    mat_matrix: Array2<f64>,
}

impl Material {
    pub fn new(e: f64, nu: f64) -> Material {
        // Change between plane stress and plain strain stress
        // TODO: find a way to change between plane stress and plane strain conditions
        /*
        let stress_cond = StressCondition::plane_stress;
        match stress_cond {
            plane_strain => Material::plane_strain_matrix(e: f64, nu: f64),
            plane_stress => Material::plane_stress_matrix(e, nu);
        }
        */
        let e1: f64 = e / (1.0 - nu * nu);
        let e2: f64 = nu * e1;
        let e3: f64 = e1 * (1.0 - 2.0 * nu) / 2.0;
        // let mat_matrix = Matrix::new(3, 3, vec![e1, e2, 0.0, e2, e1, 0.0, 0.0, 0.0, e3]);
        let mat_matrix = array![[e1, e2, 0_f64],
            [e2, e1, 0_f64],
            [0_f64, 0_f64, e3]];
        Material { mat_matrix }
    }

    pub fn get_mat_matrix(&self) -> Array2<f64> {
        self.mat_matrix
    }
}
