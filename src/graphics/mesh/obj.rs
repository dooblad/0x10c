extern crate tobj;

use std::path::Path;

use graphics::mesh::Mesh;

pub fn new(file_name: &str) -> Mesh {
    println!("Loading \"{}\"", file_name);
    let mesh = {
        // TODO: Support materials.
        let (models, _) = tobj::load_obj(&Path::new(file_name)).unwrap();

        // TODO: Support multiple models.
        assert_eq!(models.len(), 1);
        // TODO: Don't clone... inefficient.
        models[0].mesh.clone()
    };

    Mesh::new(mesh.positions, Some(mesh.indices), Some(mesh.normals), None, None)
}
