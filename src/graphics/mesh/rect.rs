use graphics::mesh::Mesh;
use graphics::mesh::util::*;

pub fn new(width: f32, height: f32, depth: f32) -> Mesh {
    assert!(width > 0.0);
    assert!(height > 0.0);
    assert!(depth > 0.0);

    let w = width / 2.0;
    let h = height / 2.0;
    let d = depth / 2.0;
    let base_positions = vec![
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

    let positions = expand_indices(&base_positions, &indices);
    let normals = generate_normals(&positions);

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

    Mesh::new(positions, Some(normals), Some(tex_coords))
}
