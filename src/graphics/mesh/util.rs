use gl::types::GLfloat;

use util::math::Point3;

pub fn expand_indices(base_positions: &Vec<GLfloat>, indices: &Vec<u16>) -> Vec<GLfloat> {
    let mut positions: Vec<GLfloat> = Vec::with_capacity(indices.len() * 3);
    for i in 0..indices.len() {
        positions.push(base_positions[(indices[i] * 3) as usize]);
        positions.push(base_positions[(indices[i] * 3 + 1) as usize]);
        positions.push(base_positions[(indices[i] * 3 + 2) as usize]);
    }
    positions
}

pub fn generate_normals(positions: &Vec<GLfloat>) -> Vec<GLfloat> {
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