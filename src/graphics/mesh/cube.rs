use graphics::mesh::Mesh;
use graphics::mesh::rect;

pub fn new(size: f32) -> Mesh {
    rect::new(size, size, size)
}
