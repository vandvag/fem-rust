use rulinalg::matrix::Matrix;

enum StressCondition {
	plane_stress,
	plane_strain,
}

pub struct Material {
	mat_matrix: Matrix<f32>,
}

impl Material {

	pub fn new(E: f32, nu: f32) -> Material {
		// Change between plane stress and plain strain stress
		// TODO: find a way to change between plane stress and plane strain conditions
		/*
		let stress_cond = StressCondition::plane_stress;
		match stress_cond {
			plane_strain => {
				let vor: f32 = E / ((1.0 + nu) * (1.0 - 2.0 * nu));
				let e1: f32 = vor * (1.0 - nu);
				let e2: f32 = vor * nu;
				let e3: f32 = vor * (1.0 - 2.0 * nu) / 2.0;
			}
			plane_stress => {
				let e1: f32 = E / (1.0 - nu * nu);
				let e2: f32 = nu * e1;
				let e3: f32 = e1 * (1.0 - 2.0 * nu) / 2.0;
			}
		}
		*/
		// plane stress settings
		let e1: f32 = E / (1.0 - nu * nu);
		let e2: f32 = nu * e1;
		let e3: f32 = e1 * (1.0 - 2.0 * nu) / 2.0;
		let mat_matrix = Matrix::new(3, 3, vec![e1, e2, 0.0, e2, e1, 0.0, 0.0, 0.0, e3]);
		Material {
			mat_matrix: mat_matrix,
		}
	}

	pub fn get_mat_matrix(&self) -> &Matrix<f32> {
		&self.mat_matrix
	}
}
