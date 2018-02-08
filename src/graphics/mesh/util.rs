use gl::types::GLfloat;

use util::math::Point3;

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

/*
TODO: This don't work right.
pub fn gen_indexed_normals(positions: &Vec<GLfloat>, indices: &Vec<u32>) -> Vec<GLfloat> {
    let mut normals: Vec<GLfloat> = vec![0.0; positions.len()];

    let mut idx_iter = indices.iter().peekable();
    let mut curr_indices: Vec<usize> = vec![0; 3];
    let mut curr_positions: Vec<Point3> = vec![Point3::new(0.0, 0.0, 0.0); 3];
    while idx_iter.peek().is_some() {
        for i in 0..3 {
            curr_indices[i] = *idx_iter.next().unwrap() as usize;
        }

        // Build a face from positions and the current indices.
        for i in 0..3 {
            for j in 0..3 {
                curr_positions[i][j] = positions[curr_indices[i]*3 + j];
            }
        }

        // Generate the normal for this face.
        let vec_diffs = [
            curr_positions[1] - curr_positions[0],
            curr_positions[2] - curr_positions[0],
        ];
        let curr_normal = vec_diffs[0].cross(vec_diffs[1]);

        // Use the same normal for every vertex of the face.
        for i in 0..3 {
            for j in 0..3 {
                normals[curr_indices[i]*3 + j] = curr_normal[j];
            }
        }
    }
    normals
}
*/
