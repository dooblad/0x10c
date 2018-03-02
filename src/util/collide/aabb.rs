use util::collide::sat::CollisionMesh;
use util::math::Point3;
use util::mesh::gen;
use util::mesh::translate_vertices;


/*
pub struct AABB {
    bounds: [Range; 3],
    position: Point3,
}
*/

#[derive(Clone, Debug)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

// TODO: Generate an AABB that's actually a CollisionMesh.
pub fn new(bounds: [Range; 3], position: Point3) -> CollisionMesh {
    let x_len = bounds[0].max - bounds[0].min;
    let y_len = bounds[1].max - bounds[1].min;
    let z_len = bounds[2].max - bounds[2].min;
    let mut mesh = gen::rect(
        x_len,
        y_len,
        z_len,
    );
    let translation = Point3 {
        x: bounds[0].max - (x_len / 2.0),
        y: bounds[1].max - (y_len / 2.0),
        z: bounds[2].max - (z_len / 2.0),
    };
    translate_vertices(&mut mesh, translation);
    CollisionMesh::new(mesh, Some(position))
}

/*
impl AABB {
    pub fn new(bounds: [Range; 3], position: Point3) -> AABB {
        AABB {
            bounds,
            position,
        }
    }

    pub fn translate(&mut self, v: Vector3) {
        self.position += v;
    }

    pub fn position(&self) -> &Point3 {
        &self.position
    }

    pub fn set_position(&mut self, position: Point3) {
        self.position = position;
    }

    pub fn collide(&self, intent: Vector3, other: &AABB) -> Option<Vector3> {
        let intended_position = self.position + intent;
        let other_sum = other.minkowski_sum(self);
        let other_bounds = other_sum.world_bounds();

        let mut min_mtv_axis = 0;
        let mut min_resolve_val = std::f32::INFINITY;
        for i in 0..3 {
            if intended_position[i] > other_bounds[i].min && intended_position[i] < other_bounds[i].max {
                let resolve_val = if intent[i] > 0.0 {
                    -(intended_position[i] - other_bounds[i].min)
                } else {
                    -(intended_position[i] - other_bounds[i].max)
                };

                if resolve_val.abs() < min_resolve_val.abs() {
                    min_mtv_axis = i;
                    min_resolve_val = resolve_val;
                }
            } else {
                return None;
            }
        }

        let mut mtv = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        mtv[min_mtv_axis] = min_resolve_val;

        return Some(mtv);
    }

    fn minkowski_sum(&self, other: &AABB) -> AABB {
        let mut sum_bounds = [
            Range { min: 0.0, max: 0.0 },
            Range { min: 0.0, max: 0.0 },
            Range { min: 0.0, max: 0.0 },
        ];
        for i in 0..3 {
            sum_bounds[i].min = self.bounds[i].min - other.bounds[i].max;
            sum_bounds[i].max = self.bounds[i].max - other.bounds[i].min;
        }
        AABB {
            bounds: sum_bounds,
            position: self.position.clone(),
        }
    }

    fn world_bounds(&self) -> [Range; 3] {
        let mut world_bounds = [
            self.bounds[0].clone(),
            self.bounds[1].clone(),
            self.bounds[2].clone(),
        ];
        for i in 0..3 {
            world_bounds[i].min += self.position[i];
            world_bounds[i].max += self.position[i];
        }
        world_bounds
    }
}

impl Clone for AABB {
    fn clone(&self) -> AABB {
        let bounds: [Range; 3] = [
            self.bounds[0].clone(),
            self.bounds[1].clone(),
            self.bounds[2].clone(),
        ];

        AABB {
            bounds,
            position: self.position.clone(),
        }
    }
}
*/
