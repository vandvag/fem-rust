#[derive(Debug)]
pub struct Node {
    id: i32,
    coords: Vec<f32>
}

impl Node {
	pub fn new(_id: i32, _x: f32, _y: f32) -> Node {
		Node {
			id: _id,
			coords: Vec::from([_x, _y])
		}
	}
}