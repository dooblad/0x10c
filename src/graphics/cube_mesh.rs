extern crate image;

use cgmath;
use glium;
use glium::Surface;
use std;
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
    vertex_buffer: glium::VertexBuffer<Vertex>,
    diffuse_texture: glium::texture::SrgbTexture2d,
    normal_map: glium::texture::Texture2d,
    // TODO: Remove this.
    time: u32,
}

impl CubeMesh {
    pub fn new(display: &glium::Display, size: f32) -> CubeMesh {
        /*
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

        let vertex_buffer = {
            // TODO: Is this the right place to be calling this macro?
            implement_vertex!(Vertex, position, normal);
            let mut vertices: Vec<Vertex> = Vec::with_capacity(positions.len());
            let vert_iter = positions.chunks(3).zip(normals.chunks(3)).map(|(p, n)| {
                Vertex {
                    position: [p[0], p[1], p[2]],
                    normal: [n[0], n[1], n[2]],
                }
            });

            for vert in vert_iter {
                vertices.push(vert);
            }

            glium::VertexBuffer::new(
                &display.clone(),
                vertices.as_ref(),
            ).unwrap()
        };
        */
        implement_vertex!(Vertex, position, normal, tex_coords);

        let vertex_buffer = glium::vertex::VertexBuffer::new(&display.clone(), &[
            Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
        ]).unwrap();

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
            vertex_buffer,
            diffuse_texture,
            normal_map,
            time: 0,
        }
    }

    fn expand_indices(base_positions: &Vec<f32>, indices: &Vec<u16>) -> Vec<f32> {
        let mut positions: Vec<f32> = Vec::new();
        for i in 0..indices.len() {
            for j in 0..3 {
                positions.push(base_positions[(indices[i] * 3 + j) as usize]);
            }
        }
        positions
    }

    fn generate_normals(positions: &Vec<f32>) -> Vec<f32> {
        let mut normals: Vec<f32> = Vec::new();

        let mut pos_iter = positions.iter().peekable();
        while pos_iter.peek().is_some() {
            let mut pos_vecs: Vec<cgmath::Vector3<f32>> = Vec::with_capacity(3);

            // There are 3 components for each point and 3 points to form a triangle.
            for _ in 0..3 {
                pos_vecs.push(cgmath::Vector3 {
                    x: pos_iter.next().unwrap().clone(),
                    y: pos_iter.next().unwrap().clone(),
                    z: pos_iter.next().unwrap().clone(),
                });
            }

            let vec_diffs = [
                pos_vecs[1].sub(pos_vecs[0]),
                pos_vecs[2].sub(pos_vecs[0]),
            ];
            let normal = vec_diffs[0].cross(vec_diffs[1]);

            for _ in 0..3 {
                normals.push(normal.x);
                normals.push(normal.y);
                normals.push(normal.z);
            }
        }

        normals
    }
}

impl Drawable for CubeMesh {
    fn draw(&mut self, context: &mut renderer::RenderingContext) {
        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];
        let view = view_matrix(&[0.5, 0.2, -3.0], &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]);

        let perspective = {
            let (width, height) = context.target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = std::f32::consts::PI / 3.0;

            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar+znear)/(zfar-znear), 1.0],
                [0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
            ]
        };

        let light = [0.5 + 1.5 * f32::sin(self.time as f32 / 30f32), 0.4, 0.7f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let uniforms = uniform! {
            model: model,
            view: view,
            perspective: perspective,
            u_light: light,
            diffuse_tex: &self.diffuse_texture,
            normal_tex: &self.normal_map,
        };

        context.target.draw(
            &self.vertex_buffer,
            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            &context.program,
            &uniforms,
            &params
        ).unwrap();

        self.time += 1;
    }
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
