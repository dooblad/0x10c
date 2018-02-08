use graphics::mesh;
use util::math::Point3;

use super::Collidable;


pub fn new(size: f32, position: Point3) -> Collidable {
    Collidable::new(mesh::cube::new(size), position)
}
