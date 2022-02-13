use nalgebra::Matrix3;
use std::fmt::Display;
#[derive(Debug, Clone)]
pub struct Material {
    youngs: f64,
    nu: f64,
    mat_matrix: Matrix3<f64>,
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
        let mat_matrix: Matrix3<f64> = Matrix3::new(
            e1, e2, 0_f64, 
            e2, e1, 0_f64, 
            0_f64, 0_f64, e3);
        Material {
            youngs: e,
            nu,
            mat_matrix,
        }
    }

    pub fn get_mat_matrix(&self) -> Matrix3<f64> {
        self.mat_matrix
    }
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Material: \t({}, {})",
            self.youngs, self.nu
        )
    }
}
