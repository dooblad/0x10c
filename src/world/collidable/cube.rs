use gl::types::GLfloat;
use std::f32;

use graphics::Render;
use graphics::mesh;
use graphics::mesh::Mesh;
use graphics::renderer::RenderingContext;
use util::collide::{AABB, Range};
use util::collide::Collide;
use util::math::Point3;

pub struct Cube {
    aabb: AABB,
    mesh: Mesh,
}

impl Cube {
    pub fn new(size: f32, position: Point3) -> Cube {
        let s = size / 2.0;
        let bounds = [
            Range { min: -s, max: s },
            Range { min: -s, max: s },
            Range { min: -s, max: s },
        ];

        let mut mesh = mesh::cube::new(size);
        // TODO: If/when we add velocity, update the mesh's position as well.
        mesh.move_to(position);

        Cube {
            aabb: AABB::new(bounds, position),
            mesh,
        }
    }
}

impl Collide for Cube {
    fn aabb(&self) -> &AABB {
        &self.aabb
    }
}

impl Render for Cube {
    fn render(&mut self, context: &mut RenderingContext) {
        self.mesh.render(context);
    }
}
