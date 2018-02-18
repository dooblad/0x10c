use cgmath::{EuclideanSpace, InnerSpace};
use gl::types::GLfloat;

use util::collide::aabb::Range;
use util::math::{Point3, Vector3};
use util::mesh::{gen_normals, translate_vertices};


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
    pub fn new(mut vertices: Vec<GLfloat>, pos: Option<Point3>) -> CollisionMesh {
        assert_eq!(vertices.len() % 9, 0);

        // TODO: Store a position, rather than translating *every* vertex in the mesh.
        if let Some(p) = pos {
            translate_vertices(&mut vertices, p);
        }

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

    // TODO: Make more efficient by translating one of the meshes into the other's
    // coordinate space.
    pub fn collide_with(&self, other: &CollisionMesh) -> Option<Vector3> {
        let axes_to_test = self.all_axes(other);

        // TODO: Better variable names.
        let mut mtv: Option<f32> = None;
        let mut min_axis: Option<Vector3> = None;
        for axis in axes_to_test {
            let self_extents = Self::projected_extents(self, axis);
            let other_extents = Self::projected_extents(other, axis);

            // Below, "()" pairs represent our interval, "[]" pairs represent `other`s
            // interval, and "."s represent how much of our interval is colliding with
            // `other`s.
            //
            // The goal is to find the smallest vector that moves *ourselves* out of the
            // collision.
            let tv;
            if self_extents.min > other_extents.max ||
                self_extents.max < other_extents.min {
                // Non-overlapping intervals
                // [__] (__) or
                // (__) [__]
                return None;
            } else if self_extents.min >= other_extents.min &&
                self_extents.max <= other_extents.max {
                // We're engulfed.
                // [__(..)__]
                //
                // First, find which side is easier to push ourselves to.

                // How far we need to move left to not collide.
                let left_dist = self_extents.max - other_extents.min;
                // How far we need to move right to not collide.
                let right_dist = other_extents.max - self_extents.min;
                tv = if left_dist < right_dist {
                    // Negate to go from a distance to a direction.
                    -left_dist
                } else {
                    right_dist
                };
            } else if self_extents.min <= other_extents.min &&
                self_extents.max >= other_extents.max {
                // They're engulfed.
                // (..[..]..)
                //
                // Basipally symmetric to the previous case.
                let left_dist = self_extents.max - other_extents.min;
                let right_dist = other_extents.max - self_extents.min;
                tv = if left_dist < right_dist {
                    -left_dist
                } else {
                    right_dist
                };
            // TODO: Move these tests up in the conditional chain.  Should be more likely.
            } else if self_extents.min <= other_extents.min &&
                self_extents.max <= other_extents.max {
                // We're penetrating from the left.
                // (__[..)__]
                tv = - (self_extents.max - other_extents.min);
            } else if self_extents.min <= other_extents.max &&
                self_extents.max >= other_extents.max {
                // We're penetrating from the right.
                // [__(..]__)
                tv = other_extents.max - self_extents.min;
                /*
            } else if self_extents.min == other_extents.min &&
                self_extents.max == other_extents.max {
                // Ranges are literally equal.
                // [..]
                // (..)
                // Need to choose an arbitrary direction to push it.  I flipped a coin,
                // and it came up left, so left it is.
                tv = - (self_extents.max - self_extents.min);
                */
            } else {
                panic!("(Supposedly) impossible case reached with: \n
\tself_extents:  {:?},
\tother_extents: {:?},
\taxis:          {:?},
                ", self_extents, other_extents, axis);
            }

            // Check against MTV.
            if mtv.is_none() || tv.abs() < mtv.unwrap().abs() {
                mtv = Some(tv);
                min_axis = Some(axis);
            }
        }

        Some(mtv.unwrap() * min_axis.unwrap())
    }

    /// Find the minimum and maximum points when projecting
    fn projected_extents(mesh: &CollisionMesh, axis: Vector3) -> Range {
        let mut min = None;
        let mut max = None;
        for f in &mesh.faces {
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

    /// Produces a vector of normalized axes to test for a separating axis between this
    /// mesh and `other`.
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
                            } else {
                                result.push(axis.normalize());
                            }
                        } else {
                            result.push(axis.normalize());
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
    use util::mesh::gen::{cube, tetrahedron};

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
    fn cube_colliding() {
        /*
         *         _____
         *        |     | <-- a
         *        |_____|
         *        |_____|
         *        |     |
         *        |_____| <-- b
         */
        let mut a_mesh = gen(cube(1.0), Point3::new(0.0, 0.9, 0.0));
        let mut b_mesh = gen(cube(1.0), Point3::new(0.0, 0.0, 0.0));
        match a_mesh.collide_with(&b_mesh) {
            Some(mtv) => {
                // `a` should be moved pretty much straight upwards.
                assert!(mtv.y > mtv.x && mtv.y > mtv.z);
            },
            None => panic!(),
        };
    }

    #[test]
    fn cube_not_colliding() {
        /*
         *         _____
         *        |     |
         *        |     | <-- a
         *        |_____|
         *         _____
         *        |     |
         *        |     | <-- b
         *        |_____|
         */
        let mut a_mesh = gen(cube(1.0), Point3::new(0.0, 1.1, 0.0));
        let mut b_mesh = gen(cube(1.0), Point3::new(0.0, 0.0, 0.0));
        assert!(a_mesh.collide_with(&b_mesh).is_none());
    }

    #[test]
    fn cube_corner_colliding() {
        /*
         *       _____
         *      |     | <-- a
         *      |    _|___
         *      |___|_|   |
         *          |     | <-- b
         *          |_____|
         */
        let mut a_mesh = gen(cube(1.0), Point3::new(-0.9, 0.9, 0.0));
        let mut b_mesh = gen(cube(1.0), Point3::new(0.0, 0.0, 0.0));
        match a_mesh.collide_with(&b_mesh) {
            Some(mtv) => {
                // `a` should be moved more along the `x` and `y` axes, than the `z` axis.
                // And it shouldn't be a zero vector.
                assert!(mtv.x.abs() >= mtv.z.abs() &&
                    mtv.y.abs() >= mtv.z.abs() &&
                    mtv.dot(mtv) > 0.0);
            },
            None => panic!(),
        };
    }

    #[test]
    fn tetrahedra_colliding() {
        /*
         *           /\
         *          /  \ <-- a
         *         /_/\_\
         *          /  \ <-- b
         *         /____\
         */
        let mut a_mesh = gen(tetrahedron(1.0), Point3::new(0.0, 0.9, 0.0));
        let mut b_mesh = gen(tetrahedron(1.0), Point3::new(0.0, 0.0, 0.0));
        match a_mesh.collide_with(&b_mesh) {
            Some(mtv) => {
                // `a` should be moved pretty much straight upwards.
                assert!(mtv.y > mtv.x && mtv.y > mtv.z);
            },
            None => panic!(),
        };
    }

    #[test]
    fn tetrahedra_not_colliding() {
        /*
         *           /\
         *          /  \ <-- a
         *         /____\
         *
         *           /\
         *          /  \ <-- b
         *         /____\
         */
        let mut a_mesh = gen(tetrahedron(1.0), Point3::new(0.0, 1.1, 0.0));
        let mut b_mesh = gen(tetrahedron(1.0), Point3::new(0.0, 0.0, 0.0));
        assert!(a_mesh.collide_with(&b_mesh).is_none());
    }

    fn gen(vertices: &mut Vec<GLfloat>, pos: Point3) -> CollisionMesh {
        use util::mesh::translate_vertices;

        translate_vertices(vertices, pos);
        CollisionMesh::new(vertices)
    }
}
