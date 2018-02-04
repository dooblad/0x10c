use cgmath;
use cgmath::InnerSpace;
use std;

use util::math::{Matrix4, Point3, Rotation, Vector3};


const FOV: f32 = std::f32::consts::PI / 3.0;
const Z_RANGE: (f32, f32) = (0.1, 100.0);

#[derive(Clone)]
pub struct Camera {
    view_matrix: Matrix4,
    projection_matrix: Matrix4,
    position: Point3,
    rotation: Rotation,
    fov: f32,
    z_range: (f32, f32),
}

impl Camera {
    // TODO: This operates under the assumption that the window size never changes.
    // Make it update by checking for a window resize event and calling a
    // `set_window_size` method here.
    pub fn new(width: u32, height: u32) -> Camera {
        let rotation = Rotation {
            horizontal_angle: 0.0,
            vertical_angle: 0.0,
        };
        let position = Point3::new(0.0, 0.0, 0.0f32);
        let view_matrix = Self::view_matrix_from(&position, &rotation);
        let aspect_ratio = width as f32 / height as f32;
        let fov = FOV;
        let z_range = Z_RANGE;

        Camera {
            view_matrix,
            projection_matrix: cgmath::perspective(cgmath::Rad(FOV), aspect_ratio,
                                                   z_range.0, z_range.1),
            position,
            rotation,
            fov,
            z_range,
        }
    }

    pub fn set_view(&mut self, position: &Point3, rotation: &Rotation) {
        self.position = position.clone();
        self.rotation = rotation.clone();

        self.view_matrix = Self::view_matrix_from(&self.position, &self.rotation);
    }

    pub fn set_position(&mut self, position: &Point3) {
        self.position = position.clone();

        self.view_matrix = Self::view_matrix_from(&self.position, &self.rotation);
    }

    fn view_matrix_from(position: &Point3, rotation: &Rotation) -> Matrix4 {
        let (facing_dir, _, up) = Self::characteristic_vectors(rotation);
        cgmath::Matrix4::look_at(position.clone(), position + facing_dir, up)
    }

    fn characteristic_vectors(rotation: &Rotation) -> (Vector3, Vector3, Vector3) {
        let &Rotation { horizontal_angle, vertical_angle } = rotation;
        // TBH, I'm not really sure why we need to add PI here, but this makes it so the
        // coordinate system is normal with zeroes for both horizontal and vertical
        // angles.
        let horizontal_angle = horizontal_angle + std::f32::consts::PI;

        let facing_dir = Vector3 {
            x: vertical_angle.cos() * horizontal_angle.sin(),
            y: vertical_angle.sin(),
            z: vertical_angle.cos() * horizontal_angle.cos(),
        }.normalize();

        let right = Vector3 {
            x: (horizontal_angle - std::f32::consts::PI / 2f32).sin(),
            y: 0.0,
            z: (horizontal_angle - std::f32::consts::PI / 2f32).cos(),
        }.normalize();

        let up = right.cross(facing_dir).normalize();

        (facing_dir, right, up)
    }

    pub fn position(&self) -> Point3 {
        self.position
    }

    pub fn rotation(&self) -> &Rotation {
        &self.rotation
    }

    pub fn view_matrix(&self) -> Matrix4 {
        self.view_matrix
    }

    pub fn projection_matrix(&self) -> Matrix4 {
        self.projection_matrix
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn z_range(&self) -> (f32, f32) {
        self.z_range
    }
}

