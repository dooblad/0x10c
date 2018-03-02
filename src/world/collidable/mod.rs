pub mod cube;
pub mod obj;
pub mod rect;

use graphics::Render;
use graphics::mesh::Mesh;
use graphics::renderer::RenderingContext;
use util::collide::Collide;
use util::collide::sat::CollisionMesh;
use util::math::Point3;
use util::mesh::BaseMesh;
use util::mesh::gen_normals;


pub struct Collidable {
    collision_mesh: CollisionMesh,
    // aabb: AABB,
    render_mesh: Mesh,
}

/// Private AABB struct for doing quick tests to filter out objects that definitely aren't
/// colliding.
struct AABB {
    // TODO: Implement
}

impl Collidable {
    /// When no `collision_mesh` is given, `mesh` is used for both rendering and
    /// colliding.
    pub fn new(mesh: BaseMesh, collision_mesh: Option<BaseMesh>,
               position: Point3) -> Collidable {
        let cm_base = match collision_mesh {
            Some(cm) => cm,
            None => mesh.clone(),
        };
        let collision_mesh = CollisionMesh::new(cm_base, Some(position));
        // TODO: Calculate the convex hull of `mesh` to create a collision mesh.
        let render_mesh_normals = gen_normals(&mesh);
        let mut render_mesh = Mesh::new(mesh, None, Some(render_mesh_normals), None,
                                        None);
        render_mesh.move_to(position);

        Collidable {
            collision_mesh,
            render_mesh,
        }
    }
}

impl Render for Collidable {
    fn render(&mut self, context: &mut RenderingContext) {
        self.render_mesh.render(context);
    }
}

impl Collide for Collidable {
    fn collision_mesh(&self) -> &CollisionMesh {
        &self.collision_mesh
    }
}
