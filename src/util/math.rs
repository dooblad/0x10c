use cgmath;
use cgmath::InnerSpace;

pub struct Rotation {
    pub horizontal_angle: f32,
    pub vertical_angle: f32,
}

impl Rotation {
    pub fn to_view_vec(&self) -> Vector3 {
        use std;

        // TBH, I'm not really sure why we need to add PI here, but this makes it so the
        // coordinate system is normal with zeroes for both horizontal and vertical
        // angles.
        let horizontal_angle = self.horizontal_angle + std::f32::consts::PI;
        Vector3 {
            x: self.vertical_angle.cos() * horizontal_angle.sin(),
            y: self.vertical_angle.sin(),
            z: self.vertical_angle.cos() * horizontal_angle.cos(),
        }.normalize()
    }
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

#[derive(Debug)]
pub struct Ray3 {
    pub pos: Point3,
    pub dir: Vector3,
}

// We're pretty much always going to be working with 32-bit floats, so we might as well define some
// convenience types.
pub type Vector2 = cgmath::Vector2<f32>;
pub type Vector3 = cgmath::Vector3<f32>;
pub type Point3 = cgmath::Point3<f32>;
pub type Matrix4 = cgmath::Matrix4<f32>;

