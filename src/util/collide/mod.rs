pub mod aabb;
pub mod sat;

use graphics::Render;

use util::collide::sat::CollisionMesh;


/// Models in an OBJ file with a name of the form "CM_<MODEL_NAME>" designate that model
/// to be the collision mesh for the model with name "<MODEL_NAME>".
pub const OBJ_COLLISION_MESH_PREFIX: &str = "CM";

/// Objects that can be collided with.
pub trait Collide : Render {
    fn collision_mesh(&self) -> &CollisionMesh;
}
