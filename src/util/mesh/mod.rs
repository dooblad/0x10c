extern crate tobj;

use gl::types::GLfloat;
use std::path::Path;

use util::math::Point3;


pub mod gen {
    use super::*;


    pub fn tetrahedron(size: f32) -> Vec<GLfloat> {
        // Build base vertices.
        let s = size;
        let frac_1_sqrt_3 = s / 3.0f32.sqrt();
        let frac_1_2 = s / 0.5;
        vec![
            // Bottom
            -frac_1_sqrt_3, 0.0, -frac_1_2,
            frac_1_sqrt_3, 0.0, -frac_1_2,
            0.0, 0.0, frac_1_2,
            // Front
            -frac_1_sqrt_3, 0.0, -frac_1_2,
            frac_1_sqrt_3, 0.0, -frac_1_2,
            0.0, s, 0.0,
            // Back Left
            0.0, 0.0, frac_1_2,
            -frac_1_sqrt_3, 0.0, -frac_1_2,
            0.0, s, 0.0,
            // Back Right
            frac_1_sqrt_3, 0.0, -frac_1_2,
            0.0, 0.0, frac_1_2,
            0.0, s, 0.0,
        ]
    }

    pub fn cube(size: f32) -> Vec<GLfloat> {
        rect(size, size, size)
    }

    pub fn rect(width: f32, height: f32, depth: f32) -> Vec<GLfloat> {
        assert!(width > 0.0);
        assert!(height > 0.0);
        assert!(depth > 0.0);

        let w = width / 2.0;
        let h = height / 2.0;
        let d = depth / 2.0;
        let positions = vec![
            // Front
            -w, -h, d,
            w, -h, d,
            w, h, d,
            -w, h, d,
            // Back
            -w, -h, -d,
            w, -h, -d,
            w, h, -d,
            -w, h, -d,
        ];

        let indices: Vec<u32> = vec![
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

        expand_indices(&positions, &indices)
    }

    pub fn obj(file_name: &str) -> Vec<GLfloat> {
        let (models, materials) = tobj::load_obj(&Path::new(file_name)).unwrap();

        // TODO: Support multiple models/materials.
        assert_eq!(models.len(), 1);
        assert_eq!(materials.len(), 1);

        // TODO: Don't clone... inefficient.
        let mesh = &models[0].mesh;

        expand_indices(&mesh.positions, &mesh.indices)
    }
}


// General Helper Functions

pub fn translate_vertices(verts: &mut Vec<GLfloat>, pos: Point3) {
    for i in 0..(verts.len() / 3) {
        for j in 0..3 {
            verts[i*3 + j] += pos[j]
        }
    }
}

pub fn expand_indices(base_positions: &Vec<GLfloat>, indices: &Vec<u32>) -> Vec<GLfloat> {
    let mut positions: Vec<GLfloat> = Vec::with_capacity(indices.len() * 3);
    for i in 0..indices.len() {
        positions.push(base_positions[(indices[i] * 3) as usize]);
        positions.push(base_positions[(indices[i] * 3 + 1) as usize]);
        positions.push(base_positions[(indices[i] * 3 + 2) as usize]);
    }
    positions
}

pub fn gen_normals(positions: &Vec<GLfloat>) -> Vec<GLfloat> {
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

