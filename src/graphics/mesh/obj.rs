extern crate tobj;

use std::path::Path;

use graphics::mesh::Mesh;
use graphics::texture::Texture;

pub fn new(file_name: &str) -> Mesh {
    println!("Loading \"{}\"", file_name);

    let (models, materials) = tobj::load_obj(&Path::new(file_name)).unwrap();

    // TODO: Support multiple models/materials.
    assert_eq!(models.len(), 1);
    assert_eq!(materials.len(), 1);

    // TODO: Don't clone... inefficient.
    let mesh = models[0].mesh.clone();

    if mesh.texcoords.len() == 0 {
        Mesh::new(mesh.positions, Some(mesh.indices), Some(mesh.normals), None, None)
    } else {
        let diffuse_texture = Texture::from(materials[0].diffuse_texture.as_str());
        Mesh::new(mesh.positions, Some(mesh.indices), Some(mesh.normals),
                  Some(mesh.texcoords), Some(diffuse_texture))
    }
}
