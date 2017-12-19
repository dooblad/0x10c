use cgmath::SquareMatrix;
use gl::types::GLfloat;
//use image;
use std::f32;
//use std::io::Cursor;

use graphics;
use graphics::Render;
use graphics::renderer;
use util::collide::{AABB, Range};
use util::collide::Collide;
use util::math::{Matrix4, Point3, Vector3};

pub struct CollidableCube {
    aabb: AABB,
//    velocity: Vector3,
    model_matrix: Matrix4,
    mesh: graphics::Mesh,
//    diffuse_texture: graphics::Texture,
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

        let mesh = graphics::Mesh::new(positions, Some(normals), Some(tex_coords));

        /*
        let image = image::load(
            Cursor::new(&include_bytes!("../../tuto-14-diffuse.jpg")[..]),
            image::JPEG
        ).unwrap().to_rgba();
        let diffuse_texture = graphics::Texture::new(image);
        */

        let bounds = [
            Range { min: -s, max: s },
            Range { min: -s, max: s },
            Range { min: -s, max: s },
        ];

        CollidableCube {
            aabb: AABB::new(bounds, position),
//            velocity,
            model_matrix: Matrix4::identity(),
            mesh,
//            diffuse_texture,
        }
    }

    fn model_matrix(&mut self) -> Matrix4 {
        // The rightmost column of a model matrix is where translation data is stored.
        let position = self.aabb.position();
        self.model_matrix[3][0] = position[0];
        self.model_matrix[3][1] = position[1];
        self.model_matrix[3][2] = position[2];

        self.model_matrix
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
    fn render(&mut self, context: &mut renderer::RenderingContext) {
        // TODO: Replace the commented-out segments.
        /*
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        */

//        let model: [[f32; 4]; 4] = self.model_matrix().into();
//        let view: [[f32; 4]; 4] = context.camera.view_matrix().into();
//        let projection: [[f32; 4]; 4] = context.camera.projection_matrix().into();
//        let color: [f32; 3] = [0.2, 0.2, 1.0];

        {
            let mut uniforms = context.program.uniforms();
            uniforms.send_matrix_4fv("model", self.model_matrix());
            uniforms.send_matrix_4fv("view", context.camera.view_matrix());
            uniforms.send_matrix_4fv("projection", context.camera.projection_matrix());
            uniforms.send_3fv("color", Vector3::new(0.2, 0.2, 1.0));
        }

        context.draw(
            &self.mesh,
        );
        /*
        let uniforms = uniform! {
            model: model,
            view: view,
            projection: projection,
            color: color,
        };

        context.target.draw(
            &self.vertex_buffer,
            glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &context.program,
            &uniforms,
            &params
        ).unwrap();
        */
    }
}
