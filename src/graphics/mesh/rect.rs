use graphics::mesh::Mesh;
use util::mesh::gen::rect;
use util::mesh::gen_normals;


pub fn new(width: f32, height: f32, depth: f32) -> Mesh {
    let positions = rect(width, height, depth);
    let normals = gen_normals(&positions);

    /*
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
    */

    Mesh::new(positions, None, Some(normals), None, None)
}

