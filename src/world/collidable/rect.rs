use util::math::Point3;
use util::mesh::gen;

use super::Collidable;


pub fn new(width: f32, height: f32, depth: f32, position: Point3) -> Collidable {
    Collidable::new(gen::rect(width, height, depth), None, position)
}
