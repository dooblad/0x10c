use util::math::Point3;
use util::mesh::gen;

use super::Collidable;


pub fn new(size: f32, position: Point3) -> Collidable {
    Collidable::new(gen::cube(size), None, position)
}
