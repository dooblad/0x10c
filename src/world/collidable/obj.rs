use graphics::mesh;
use util::math::Point3;

use super::Collidable;


pub fn new(file_name: &str, position: Point3) -> Collidable {
    Collidable::new(mesh::obj::new(file_name), position)
}
