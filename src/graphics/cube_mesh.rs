extern crate image;

use cgmath;
use cgmath::One;
use glium;
use glium::Surface;
use std::f32;
use std::io::Cursor;
use std::ops::Sub;

use graphics::renderer;

pub trait Drawable {
    fn draw(&mut self, context: &mut renderer::RenderingContext);
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}

pub struct CubeMesh {
    model_matrix: cgmath::Matrix4<f32>,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    diffuse_texture: glium::texture::SrgbTexture2d,
    normal_map: glium::texture::Texture2d,
    // TODO: Remove this.
    time: u32,
}

impl CubeMesh {
    pub fn new(display: &glium::Display, size: f32) -> CubeMesh {
        implement_vertex!(Vertex, position, normal, tex_coords);

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

        let mut vertices: Vec<Vertex> = Vec::with_capacity(positions.len());
        for i in 0..positions.len() {
            let tex_coords = [
                [0.0, 0.0],
                [1.0, 0.0],
                [1.0, 1.0],
                [0.0, 0.0],
                [1.0, 1.0],
                [0.0, 1.0],
//                [0.0, 1.0],
//                [1.0, 1.0],
//                [0.0, 0.0],
//                [1.0, 0.0],
            ];
            vertices.push(Vertex {
                position: positions[i].into(),
                normal: normals[i].into(),
                tex_coords: tex_coords[i % tex_coords.len()],
            });
        }
        let vertex_buffer = glium::vertex::VertexBuffer::new(&display.clone(),
                                                             vertices.as_ref()).unwrap();

        let image = image::load(Cursor::new(&include_bytes!("../../tuto-14-diffuse.jpg")[..]),
                                image::JPEG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let diffuse_texture = glium::texture::SrgbTexture2d::new(&display.clone(), image).unwrap();

        let image = image::load(Cursor::new(&include_bytes!("../../tuto-14-normal.png")[..]),
                                image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let normal_map = glium::texture::Texture2d::new(&display.clone(), image).unwrap();

        CubeMesh {
            // Identity matrix.
            model_matrix: cgmath::Matrix4::one(),
            vertex_buffer,
            diffuse_texture,
            normal_map,
            time: 0,
        }
    }

    fn expand_indices(base_positions: &Vec<f32>, indices: &Vec<u16>) -> Vec<cgmath::Point3<f32>> {
        let mut positions: Vec<cgmath::Point3<f32>> = Vec::new();
        for i in 0..indices.len() {
            positions.push(cgmath::Point3 {
                x: base_positions[(indices[i] * 3) as usize],
                y: base_positions[(indices[i] * 3 + 1) as usize],
                z: base_positions[(indices[i] * 3 + 2) as usize],
            });
        }
        positions
    }

    fn generate_normals(positions: &Vec<cgmath::Point3<f32>>) -> Vec<cgmath::Vector3<f32>> {
        let mut normals: Vec<cgmath::Vector3<f32>> = Vec::new();

        let mut pos_iter = positions.iter().peekable();
        while pos_iter.peek().is_some() {
            let mut pos_vecs: Vec<cgmath::Point3<f32>> = Vec::with_capacity(3);

            // There are 3 components for each point and 3 points to form a triangle.
            for _ in 0..3 {
                pos_vecs.push(pos_iter.next().unwrap().clone());
            }

            let vec_diffs = [
                pos_vecs[1].sub(pos_vecs[0]),
                pos_vecs[2].sub(pos_vecs[0]),
            ];
            let normal = vec_diffs[0].cross(vec_diffs[1]);

            // Use the same normal for each point of a single triangle.
            for _ in 0..3 {
                normals.push(normal);
            }
        }

        normals
    }
}

impl Drawable for CubeMesh {
    fn draw(&mut self, context: &mut renderer::RenderingContext) {
        let light = [0.5 + 1.5 * f32::sin(self.time as f32 / 30f32), 0.4, -0.7f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let model : [[f32; 4]; 4] = self.model_matrix.into();
        let view : [[f32; 4]; 4] = context.camera.view_matrix().into();
        let perspective : [[f32; 4]; 4] = context.camera.projection_matrix().into();
        // TODO: How to pass around matrices?
        // Model will be per-mesh.
        let uniforms = uniform! {
            model: model,
            view: view,
            // TODO: s/perspective/projection
            perspective: perspective,
            u_light: light,
            diffuse_tex: &self.diffuse_texture,
            normal_tex: &self.normal_map,
        };

        context.target.draw(
            &self.vertex_buffer,
            glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &context.program,
            &uniforms,
            &params
        ).unwrap();

        self.time += 1;
    }
}
