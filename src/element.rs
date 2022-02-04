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
use material::Material;

struct Element {
    id: i32,       // Element id
    n1: i32,       // Node 1 id
    n2: i32,       // Node 2 id
    n3: i32,       // Node 3 id
    n4: i32,       // Node 4 id
    mat: Material, // Material
}

impl Element {
    fn new(_id: i32, _n1: i32, _n2: i32, _n3: i32, _n4: i32, _mat: Material) -> Element {
        Element {
            _id,
            _n1,
            _n2,
            _n3,
            _n4,
            mat,
        }
    }
}
