pub mod cube;
pub mod obj;
pub mod rect;

use graphics::Render;
use graphics::mesh::Mesh;
use graphics::renderer::RenderingContext;
use util::collide::aabb::AABB;
use util::collide::Collide;
use util::math::Point3;


pub struct Collidable {
    aabb: AABB,
    mesh: Mesh,
}

impl Collidable {
    pub fn new(mut mesh: Mesh, position: Point3) -> Collidable {
        // TODO: If/when we add velocity, update the mesh's position as well.
        mesh.move_to(position);

        Collidable {
            aabb: AABB::new(mesh.bounds(), position),
            mesh,
        }
    }
}

impl Render for Collidable {
    fn render(&mut self, context: &mut RenderingContext) {
        self.mesh.render(context);
    }
}

impl Collide for Collidable {
    fn aabb(&self) -> &AABB {
        &self.aabb
    }
}
