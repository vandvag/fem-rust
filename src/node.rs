use std::fmt::Display;
pub struct Node {
    id: i32,
    coords: Vec<f64>,
}

impl Node {
    pub fn new(_id: i32, _x: f64, _y: f64) -> Node {
        Node {
            id: _id,
            coords: Vec::from([_x, _y]),
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

	pub fn get_coords(&self) -> &Vec<f64> {
		&self.coords
	}
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Node {}: \t({}, {})",
            self.id, self.coords[0], self.coords[1]
        )
    }
}
