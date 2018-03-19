use cgmath::{InnerSpace, EuclideanSpace};
use gl::types::GLfloat;
use std;

use util::collide::aabb::Range;
use util::math::{Point3, Vector3, Ray3};
use util::mesh::gen_normals;
use util::mesh::BaseMesh;


#[derive(Clone)]
pub struct Face {
    vertices: [Point3; 3],
    normal: Vector3,
}

// TODO: Store faces and vertices and have faces hold indices into the vertex vector.
pub struct CollisionMesh {
    faces: Vec<Face>,
    aabb: AABB,
    position: Point3,
}

/// Private AABB struct for doing quick tests to filter out objects that definitely aren't
/// colliding.
struct AABB {
    bounds: [Range; 3],
}

impl CollisionMesh {
    pub fn new(vertices: BaseMesh, position: Option<Point3>) -> CollisionMesh {
        assert_eq!(vertices.len() % 9, 0);

        let position = position.unwrap_or(Point3::new(0.0, 0.0, 0.0));

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

        CollisionMesh {
            faces,
            aabb: AABB::new(Self::bounds(&vertices)),
            position,
        }
    }

    pub fn collide_with(&self, other: &CollisionMesh) -> Option<Vector3> {
        // Define a translation vector to move `other` into our coordinate space.  This is
        // more efficient than both meshes translating by their position vectors, because
        // only one set of vertices needs to be translated.
        let other_translation = Some(other.position - self.position);

        // If their AABBs don't collide, then the precise meshes definitely don't collide.
        if !self.aabb.collides_with(&other.aabb, other_translation.unwrap()) {
            return None;
        }

        // TODO: We'll probably need that O(n^2) edge check eventually.
        // When collisions get more complicated, but we can probably optimize it so once
        // the current minimum is less than the velocity or something, then we know it's
        // the minimum axis.

        let mut mtv: Option<f32> = None;
        let mut min_axis: Option<Vector3> = None;
        for face in self.faces.iter().chain(other.faces.iter()) {
            if !self.test_axis(other, other_translation, face.normal,
                           &mut mtv, &mut min_axis) {
                return None;
            }
        }

        let mtv = mtv.unwrap();
        if mtv.abs() == 0.0 {
            // Discard "ghost" collisions.  They actually happen.  Or at least, they
            // happened at some point and idk.
            return None;
        }

        Some(mtv * min_axis.unwrap())
    }

    pub fn collide_with_ray(&self, ray: Ray3) -> bool {
        // Check pg. 199 of Real Time Collision Detection for an explanation of the algo,
        // my dude.
        let mut t_min = 0.0f32;
        let mut t_max = std::f32::INFINITY;
        for face in &self.faces {
            let denom = face.normal.dot(ray.dir);
            let dist = face.normal.dot(face.vertices[0] - ray.pos);
            if denom == 0.0 {
                if dist < 0.0 {
                    return false;
                }
            } else {
                let t = dist / denom;
                if denom < 0.0 {
                    if t > t_min {
                        t_min = t;
                    }
                } else {
                    if t < t_max {
                        t_max = t;
                    }
                }
                if t_min > t_max {
                    return false;
                }
            }
            /*
            let plane_norm = face.normal;
            // Can use any of the face's vertices as a point for the plane.
            let plane_pos = face.vertices[0];
            // Intuitively, this value tells us how much closer incrementing `t` will // bring us to the plane, except it will be negative when the plane's normal
            // is facing towards us.
            let vec_dot = plane_norm.dot(ray.dir);
            if vec_dot == 0.0 {
                continue;
            }
            // "Time" parameter at which the ray contacts the plane.
            let t = plane_norm.dot(plane_pos - ray.pos) / vec_dot;
            if vec_dot > 0.0 {
                // Face is away from us, so adjust `t_max`.
                if t < t_max {
                    t_max = t;
                }
            } else if vec_dot < 0.0 {
                // Face is towards us, so adjust `t_min`.
                if t > t_min {
                    t_min = t;
                }
            }
            // Ignore the case where the dot product == 0.0.

            // Check that the interval defined by the checks above is empty.  If it is,
            // there's no collision.
            if t_max < t_min {
                return false;
            }
            */
        }
        true
    }

    pub fn translate(&mut self, v: Vector3) {
        self.position += v;
    }

    pub fn position(&self) -> &Point3 {
        &self.position
    }

    /// Returns true if there is a collision on the given `axis`.
    fn test_axis(&self, other: &CollisionMesh, other_translation: Option<Vector3>,
                 axis: Vector3, mtv: &mut Option<GLfloat>,
                 min_axis: &mut Option<Vector3>) -> bool {
        let self_extents = Self::projected_extents(self, axis, None);
        let other_extents = Self::projected_extents(other, axis, other_translation);

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
            return false;
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
        } else {
            panic!("(Supposedly) impossible case reached with: \n
\tself_extents:  {:?},
\tother_extents: {:?},
\taxis:          {:?},
                ", self_extents, other_extents, axis);
        }

        // Check against MTV.
        if mtv.is_none() || tv.abs() < mtv.unwrap().abs() {
            *mtv = Some(tv);
            *min_axis = Some(axis);
        }
        true
    }

    /// Find the minimum and maximum points when projecting.
    fn projected_extents(mesh: &CollisionMesh, axis: Vector3,
                         translation: Option<Vector3>) -> Range {
        let mut min = None;
        let mut max = None;
        for f in &mesh.faces {
            for v in f.vertices.iter() {
                let v = match translation {
                    Some(t) => v + t,
                    None => v.clone(),
                };
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

    /// Finds extrema in this mesh's vertices (useful for creating AABBs).
    fn bounds(vertices: &BaseMesh) -> [Range; 3] {
        let mut result = [
            Range { min: vertices[0], max: vertices[0] },
            Range { min: vertices[1], max: vertices[1] },
            Range { min: vertices[2], max: vertices[2] },
        ];
        for i in 0..(vertices.len() / 3) {
            for j in 0..3 {
                if vertices[i*3 + j] < result[j].min {
                    result[j].min = vertices[i*3 + j];
                } else if vertices[i*3 + j] > result[j].max {
                    result[j].max = vertices[i*3 + j];
                }
            }
        }
        result
    }
}

impl AABB {
    pub fn new(bounds: [Range; 3]) -> AABB {
        AABB { bounds }
    }

    pub fn collides_with(&self, other: &AABB, other_translation: Vector3) -> bool {
        for i in 0..3 {
            if self.bounds[i].max < (other.bounds[i].min + other_translation[i]) ||
                self.bounds[i].min > (other.bounds[i].max + other_translation[i]) {
                return false;
            }
        }
        true
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
        let mesh = CollisionMesh::new(vertices.to_vec(), None);
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
        CollisionMesh::new(vertices, pos)
    }
}

