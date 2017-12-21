use std::f32;

use graphics::Render;
use graphics::mesh;
use graphics::mesh::Mesh;
use graphics::renderer::RenderingContext;
use util::collide::{AABB, Range};
use util::collide::Collide;
use util::math::Point3;

pub struct Rect {
    aabb: AABB,
    mesh: Mesh,
}

impl Rect {
    pub fn new(width: f32, height: f32, depth: f32, position: Point3) -> Rect {
        let w = width / 2.0;
        let h = height / 2.0;
        let d = depth / 2.0;
        let bounds = [
            Range { min: -w, max: w },
            Range { min: -h, max: h },
            Range { min: -d, max: d },
        ];

        let mut mesh = mesh::rect::new(width, height, depth);
        // TODO: If/when we add velocity, update the mesh's position as well.
        mesh.move_to(position);

        Rect {
            aabb: AABB::new(bounds, position),
            mesh,
        }
    }
}

impl Collide for Rect {
    fn aabb(&self) -> &AABB {
        &self.aabb
    }
}

impl Render for Rect {
    fn render(&mut self, context: &mut RenderingContext) {
        self.mesh.render(context);
    }
}
