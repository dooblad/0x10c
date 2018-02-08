use graphics::mesh;
use util::math::Point3;

use super::Collidable;


pub fn new(width: f32, height: f32, depth: f32, position: Point3) -> Collidable {
    Collidable::new(mesh::rect::new(width, height, depth), position)
}
