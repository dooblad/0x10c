use std::path::Path;
use tobj;

use util::collide::OBJ_COLLISION_MESH_PREFIX;
use util::math::Point3;
use util::mesh::expand_indices;

use super::Collidable;


pub fn new(file_name: &str, position: Point3) -> Collidable {
    let (models, _) = tobj::load_obj(&Path::new(file_name)).unwrap();

    match models.len() {
        1 => {
            let mesh = &models[0].mesh;
            Collidable::new(expand_indices(&mesh.positions, &mesh.indices),
                            None, position)
        },
        2 => {
            assert!(models[0].name.starts_with(OBJ_COLLISION_MESH_PREFIX) ||
                models[1].name.starts_with(OBJ_COLLISION_MESH_PREFIX));

            let (render_mesh, collision_mesh) =
                if models[0].name.starts_with(OBJ_COLLISION_MESH_PREFIX) {
                    (&models[1].mesh, &models[0].mesh)
                } else {
                    (&models[0].mesh, &models[1].mesh)
                };
            Collidable::new(
                expand_indices(&render_mesh.positions, &render_mesh.indices),
                Some(expand_indices(&collision_mesh.positions, &collision_mesh.indices)),
                position)
        },
        _ => panic!("Too many models in given OBJ!"),
    }
}
