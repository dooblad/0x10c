use gl::types::GLfloat;

use util::math::Point3;


/// A vector of non-indexed vertices, where each consecutive 9 (3 components * 3 points)
/// elements in the vector defines a face.
pub type BaseMesh = Vec<GLfloat>;

pub mod gen {
    use super::*;


    pub fn tetrahedron(size: f32) -> BaseMesh {
        // TODO: Looks funky, and not the good kind.
        let a = size;
        let b = size * 3.0f32.sqrt() / 2.0;
        let c = size / 2.0;
        vec![
            // Bottom
            0.0, 0.0, -a,
            c, 0.0, b,
            -c, 0.0, b,
            // Front
            -c, 0.0, b,
            c, 0.0, b,
            0.0, a, 0.0,
            // Back Left
            0.0, 0.0, -a,
            -c, 0.0, b,
            0.0, a, 0.0,
            // Back Right
            c, 0.0, b,
            0.0, 0.0, -a,
            0.0, a, 0.0,
        ]
    }

    pub fn cube(size: f32) -> BaseMesh {
        rect(size, size, size)
    }

    pub fn rect(width: f32, height: f32, depth: f32) -> BaseMesh {
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
}


// General Helper Functions

pub fn translate_vertices(verts: &mut BaseMesh, pos: Point3) {
    for i in 0..(verts.len() / 3) {
        for j in 0..3 {
            verts[i*3 + j] += pos[j]
        }
    }
}

pub fn expand_indices(base_positions: &BaseMesh, indices: &Vec<u32>) -> BaseMesh {
    let mut positions: BaseMesh = Vec::with_capacity(indices.len() * 3);
    for i in 0..indices.len() {
        positions.push(base_positions[(indices[i] * 3) as usize]);
        positions.push(base_positions[(indices[i] * 3 + 1) as usize]);
        positions.push(base_positions[(indices[i] * 3 + 2) as usize]);
    }
    positions
}

pub fn gen_normals(positions: &BaseMesh) -> BaseMesh {
    let mut normals: BaseMesh = Vec::with_capacity(positions.len());

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

