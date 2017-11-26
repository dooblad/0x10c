use cgmath;

pub struct Rotation {
    pub horizontal_angle: f32,
    pub vertical_angle: f32,
}

impl Clone for Rotation {
    fn clone(&self) -> Rotation {
        Rotation {
            horizontal_angle: self.horizontal_angle,
            vertical_angle: self.vertical_angle,
        }
    }

    fn clone_from(&mut self, source: &Rotation) {
        self.horizontal_angle = source.horizontal_angle;
        self.vertical_angle = source.vertical_angle;
    }
}

// We're pretty much always going to be working with 32-bit floats, so we might as well define some
// convenience types.
pub type Vector2 = cgmath::Vector2<f32>;
pub type Vector3 = cgmath::Vector3<f32>;
pub type Point3 = cgmath::Point3<f32>;
pub type Matrix4 = cgmath::Matrix4<f32>;

