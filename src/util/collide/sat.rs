use cgmath::{EuclideanSpace, InnerSpace};
use gl::types::GLfloat;

use graphics::mesh::util::gen_normals;
use util::collide::aabb::Range;
use util::math::{Point3, Vector3};


#[derive(Clone)]
pub struct Face {
    vertices: [Point3; 3],
    normal: Vector3,
}

// TODO: Store faces and vertices and have faces hold indices into the vertex vector.
pub struct CollisionMesh {
    // vertices: Vec<Point3>,
    faces: Vec<Face>,
}

impl CollisionMesh {
    /// `vertices` must not be indexed.  Each face is assumed to be a consecutive 9
    /// (3 components * 3 points) elements in the vector.
    pub fn new(vertices: Vec<GLfloat>) -> CollisionMesh {
        assert_eq!(vertices.len() % 9, 0);

        // TODO: Make more efficient.  `gen_normals` duplicates the normal for each vertex
        // of the triangle.
        let normals: Vec<GLfloat> = gen_normals(&vertices);

        let mut faces: Vec<Face> = Vec::with_capacity(vertices.len() / 9);

        let mut pos_iter = vertices.iter().peekable();
        let mut norm_iter = normals.iter();
        while pos_iter.peek().is_some() {
            let mut face_verts: [Point3; 3] = [
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
            ];
            for i in 0..3 {
                face_verts[i] = Point3 {
                    x: *pos_iter.next().unwrap(),
                    y: *pos_iter.next().unwrap(),
                    z: *pos_iter.next().unwrap(),
                };
            }
            // Each normal is currently duplicated 3 times, so we burn off the first two
            // duplicates in this loop.
            for _ in 0..6 {
                norm_iter.next().unwrap();
            }
            let norm = Vector3 {
                x: *norm_iter.next().unwrap(),
                y: *norm_iter.next().unwrap(),
                z: *norm_iter.next().unwrap(),
            };

            faces.push(Face {
                vertices: face_verts,
                normal: norm,
            });
        }

        CollisionMesh { faces }
    }

    pub fn collides_with(&self, other: &CollisionMesh) -> bool {
        let axes_to_test = self.all_axes(other);

        for axis in axes_to_test {
            let self_extents = Self::projected_extents(&self.faces, axis);
            let other_extents = Self::projected_extents(&other.faces, axis);

            if self_extents.min > other_extents.max ||
                self_extents.max < other_extents.min {
                return false;
            }
        }

        true
    }

    fn projected_extents(faces: &Vec<Face>, axis: Vector3) -> Range {
        let mut min = None;
        let mut max = None;
        for f in faces {
            for v in f.vertices.iter() {
                let projection = v.dot(axis);
                if min.is_none() || projection < min.unwrap() {
                    min = Some(projection);
                }
                if max.is_none() || projection > max.unwrap() {
                    max = Some(projection);
                }
            }
        }
        Range { min: min.unwrap(), max: max.unwrap() }
    }

    fn all_axes(&self, other: &CollisionMesh) -> Vec<Vector3> {
        const NULL_AXIS_UPPER_BOUND: f32 = 0.05;

        let mut result = Vec::with_capacity(
            self.faces.len() + other.faces.len() +
                (3 * self.faces.len() * 3 * other.faces.len()));

        for face in &self.faces {
            result.push(face.normal.clone());
        }
        for face in &other.faces {
            result.push(face.normal.clone());
        }
        for face_a in &self.faces {
            for i in 0..3 {
                let edge_a = face_a.vertices[(i + 1) % 3] - face_a.vertices[i];
                for face_b in &other.faces {
                    for j in 0..3 {
                        let edge_b = face_b.vertices[(j + 1) % 3] - face_a.vertices[j];
                        let mut axis = edge_a.cross(edge_b);
                        // If the axis is close to a zero vector, create a vector from one
                        // of `edge_a`s vertices to `edge_b`s vertices and use it for the
                        // cross product.
                        if axis.dot(axis) < NULL_AXIS_UPPER_BOUND {
                            axis = edge_a.cross(
                                face_b.vertices[j] - face_a.vertices[i]);
                            if axis.dot(axis) < NULL_AXIS_UPPER_BOUND {
                                // If it's still zero, then fuck it.
                                continue
                            }
                        } else {

                        }
                    }
                }
            }
        }
        result
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Can we even *make* shit?
    fn create_triangle() {
        let vertices = [
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0,
        ];
        let mesh = CollisionMesh::new(vertices.to_vec());
        assert_eq!(mesh.faces.len(), 1);
    }

    #[test]
    fn tetrahedra_colliding() {
        /*
         *  Like this, but in 3D.
         *
         *           /\ <-- a
         *          //\\
         *         //__\\
         *         /____\ <-- b
         */
        let a_verts = gen_tetrahedron(Point3::new(0.0, 0.5, 0.0));
        let b_verts = gen_tetrahedron(Point3::new(0.0, 0.0, 0.0));
        let a_mesh = CollisionMesh::new(a_verts);
        let b_mesh = CollisionMesh::new(b_verts);
        assert!(a_mesh.collides_with(&b_mesh));
    }

    #[test]
    fn tetrahedra_not_colliding() {
        /*
         *  Like this, but in 3D.
         *
         *           /\
         *          /  \ <-- a
         *         /____\
         *
         *           /\
         *          /  \ <-- b
         *         /____\
         */
        let a_verts = gen_tetrahedron(Point3::new(0.0, 1.5, 0.0));
        let b_verts = gen_tetrahedron(Point3::new(0.0, 0.0, 0.0));
        let a_mesh = CollisionMesh::new(a_verts);
        let b_mesh = CollisionMesh::new(b_verts);
        assert_eq!(a_mesh.collides_with(&b_mesh), false);
    }

    fn gen_tetrahedron(pos: Point3) -> Vec<GLfloat> {
        // Build base vertices.
        let frac_1_sqrt_3 = 1.0 / 3.0f32.sqrt();
        let mut verts = vec![
            // Bottom
            -frac_1_sqrt_3, 0.0, -0.5,
            frac_1_sqrt_3, 0.0, -0.5,
            0.0, 0.0, 0.5,
            // Front
            -frac_1_sqrt_3, 0.0, -0.5,
            frac_1_sqrt_3, 0.0, -0.5,
            0.0, 1.0, 0.0,
            // Back Left
            0.0, 0.0, 0.5,
            -frac_1_sqrt_3, 0.0, -0.5,
            0.0, 1.0, 0.0,
            // Back Right
            frac_1_sqrt_3, 0.0, -0.5,
            0.0, 0.0, 0.5,
            0.0, 1.0, 0.0,
        ];
        // Translate them by `pos`.
        for i in 0..(verts.len() / 3) {
            for j in 0..3 {
                verts[i*3 + j] += pos[j]
            }
        }
        verts
    }
}