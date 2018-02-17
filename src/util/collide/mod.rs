pub mod aabb;
pub mod sat;

use graphics::Render;

use self::aabb::AABB;

// TODO: Make this return an option for objects that aren't collidable.
/// Objects that can be collided with.
pub trait Collide : Render {
    fn aabb(&self) -> &AABB;
}

