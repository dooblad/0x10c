use graphics::mesh::Mesh;
use util::mesh::gen::tetrahedron;
use util::mesh::gen_normals;


pub fn new(size: f32) -> Mesh {
    let positions = tetrahedron(size);
    let normals = gen_normals(&positions);
    Mesh::new(positions, None, Some(normals), None, None)
}
