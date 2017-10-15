use graphics::Render;
use std;
use util::math::{Point3, Vector3};
use util::f32::abs;

pub struct AABB {
    corners: (Point3, Point3),
    position: Point3,
}

pub trait Collide : Render {
    fn aabb(&self) -> &AABB;
}

#[derive(Debug)]
struct Range {
    min: f32,
    max: f32,
}

impl AABB {
    pub fn new(a_corner: Point3, b_corner: Point3, position: Point3) -> AABB {
        AABB {
            corners: (a_corner, b_corner),
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

    pub fn collide(&self, other: &AABB) -> Option<Vector3> {
        let self_extrema = self.extrema();
        let other_extrema = other.extrema();

        let epsilon = 0.00001;

        // (index, magnitude)
        let mut min_translation: Option<(usize, f32)> = None;
        for i in 0..3 {
            let mut translation = 0.0;
            if self_extrema[i].max < other_extrema[i].min ||
                self_extrema[i].min > other_extrema[i].max {
                // Not colliding.
                return None;
            } else if self_extrema[i].min <= other_extrema[i].min &&
                self_extrema[i].max >= other_extrema[i].min {
                // Colliding from the left.
                translation = (other_extrema[i].min - self_extrema[i].max) - epsilon;
            } else if self_extrema[i].max >= other_extrema[i].max &&
                self_extrema[i].min <= other_extrema[i].max {
                // Colliding from the right.
                translation = (other_extrema[i].max - self_extrema[i].min) + epsilon;
            } else if self_extrema[i].min <= other_extrema[i].min &&
                self_extrema[i].max >= other_extrema[i].max {
                // Engulfing the other object.
                let left_translation = -(other_extrema[i].min - self_extrema[i].min) - epsilon;
                let right_translation = (self_extrema[i].max - other_extrema[i].max) + epsilon;

                translation = if abs(left_translation) < abs(right_translation) {
                    left_translation
                } else {
                    right_translation
                }
            } else if self_extrema[i].min >= other_extrema[i].min &&
                self_extrema[i].max <= other_extrema[i].max {
                // The other object is engulfing us.
                let left_translation = -(self_extrema[i].min - other_extrema[i].min) - epsilon;
                let right_translation = (other_extrema[i].max - self_extrema[i].max) + epsilon;

                translation = if abs(left_translation) < abs(right_translation) {
                    left_translation
                } else {
                    right_translation
                }
            } else {
                panic!("We should have covered all cases!");
            }

            min_translation = match min_translation {
                None => Some((i, translation)),
                Some((j, v)) => if abs(translation) < abs(v) {
                    Some((i, translation))
                } else {
                    Some((j, v))
                }
            }
        }

        let (min_index, min_translation) = min_translation.unwrap();
        let mut mtv = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        mtv[min_index] = min_translation;

        Some(mtv)
    }

    fn extrema(&self) -> [Range; 3] {
        let mut result: [Range; 3] = [
            Range { min: 0.0, max: 0.0 },
            Range { min: 0.0, max: 0.0 },
            Range { min: 0.0, max: 0.0 },
        ];

        for i in 0..3 {
            let (min, max) = if self.corners.0[i] < self.corners.1[i] {
                (self.corners.0[i], self.corners.1[i])
            } else {
                (self.corners.1[i], self.corners.0[i])
            };
            result[i] = Range {
                min: min + self.position[i],
                max: max + self.position[i],
            };
        }
        result
    }
}

impl Clone for AABB {
    fn clone(&self) -> AABB {
        AABB {
            corners: (self.corners.0.clone(), self.corners.1.clone()),
            position: self.position.clone(),
        }
    }
}
