use gl::types::GLfloat;
use std::f32;

use graphics::Render;
use graphics::mesh::Mesh;
use graphics::renderer::RenderingContext;
use util::collide::{AABB, Range};
use util::collide::Collide;
use util::math::Point3;

pub struct CollidableCube {
    aabb: AABB,
    mesh: Mesh,
}

impl CollidableCube {
    pub fn new(size: f32, position: Point3) -> CollidableCube {
        assert!(size > 0.0);

        let s = size / 2.0;
        let base_positions = vec![
            // Front
            -s, -s, s,
            s, -s, s,
            s, s, s,
            -s, s, s,
            // Back
            -s, -s, -s,
            s, -s, -s,
            s, s, -s,
            -s, s, -s,
        ];

        let indices = vec![
            // Front
            0, 1, 2,
            0, 2, 3,
            // Back
            5, 4, 7,
            5, 7, 6,
            // Left
            4, 0, 3,
            4, 3, 7,
            // Right
            1, 5, 6,
            1, 6, 2,
            // Top
            3, 2, 6,
            3, 6, 7,
            // Bottom
            4, 5, 1,
            4, 1, 0,
        ];

        let positions = Self::expand_indices(&base_positions, &indices);
        let normals = Self::generate_normals(&positions);

        let base_tex_coords = [
            // Lower right triangle
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            // Upper left triangle
            0.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
        ];
        let num_tex_coords = (positions.len() / 3) * 2;
        let mut tex_coords = Vec::with_capacity(num_tex_coords);
        for i in 0..num_tex_coords {
            tex_coords.push(base_tex_coords[i % base_tex_coords.len()]);
        }

        let mut mesh = Mesh::new(positions, Some(normals), Some(tex_coords));
        // TODO: If/when we add velocity, update the mesh's position as well.
        mesh.move_to(position);

        let bounds = [
            Range { min: -s, max: s },
            Range { min: -s, max: s },
            Range { min: -s, max: s },
        ];

        CollidableCube {
            aabb: AABB::new(bounds, position),
            mesh,
        }
    }

    fn expand_indices(base_positions: &Vec<GLfloat>, indices: &Vec<u16>) -> Vec<GLfloat> {
        let mut positions: Vec<GLfloat> = Vec::with_capacity(indices.len() * 3);
        for i in 0..indices.len() {
            positions.push(base_positions[(indices[i] * 3) as usize]);
            positions.push(base_positions[(indices[i] * 3 + 1) as usize]);
            positions.push(base_positions[(indices[i] * 3 + 2) as usize]);
        }
        positions
    }

    fn generate_normals(positions: &Vec<GLfloat>) -> Vec<GLfloat> {
        let mut normals: Vec<GLfloat> = Vec::with_capacity(positions.len());

        let mut pos_iter = positions.iter().peekable();
        while pos_iter.peek().is_some() {
            let mut pos_vecs: Vec<Point3> = Vec::with_capacity(3);

            // There are 3 components for each point and 3 points to form a triangle.
            for _ in 0..3 {
                pos_vecs.push(Point3 {
                    x: pos_iter.next().unwrap().clone(),
                    y: pos_iter.next().unwrap().clone(),
                    z: pos_iter.next().unwrap().clone(),
                });
            }

            let vec_diffs = [
                pos_vecs[1] - pos_vecs[0],
                pos_vecs[2] - pos_vecs[0],
            ];
            let normal = vec_diffs[0].cross(vec_diffs[1]);

            // Use the same normal for each point of a single triangle.
            for _ in 0..3 {
                normals.push(normal.x);
                normals.push(normal.y);
                normals.push(normal.z);
            }
        }
        normals
    }
}

impl Collide for CollidableCube {
    fn aabb(&self) -> &AABB {
        &self.aabb
    }
}

impl Render for CollidableCube {
    fn render(&mut self, context: &mut RenderingContext) {
        self.mesh.draw(context);
    }
}
