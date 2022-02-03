#[derive(Debug)]
pub struct Node {
    id: i32,
    coords: Vec<f64>
}

impl Node {
	pub fn new(_id: i32, _x: f64, _y: f64) -> Node {
		Node {
			id: _id,
			coords: Vec::from([_x, _y])
		}
	}
}