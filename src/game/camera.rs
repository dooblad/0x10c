use cgmath;
use cgmath::InnerSpace;
use std;

use game::event_handler;

pub struct Camera {
    pub view_matrix: cgmath::Matrix4<f32>,
    pub projection_matrix: cgmath::Matrix4<f32>,
    pub position: cgmath::Vector3<f32>,
    pub horizontal_angle: f32,
    pub vertical_angle: f32,
}

impl Camera {
    // TODO: This operates under the assumption that the window size never changes.  Make it update
    // somehow.
    pub fn new(width: u32, height: u32) -> Camera {
        let fov: f32 = std::f32::consts::PI / 3.0;
        let aspect_ratio = width as f32 / height as f32;
        let z_far: f32 = 1024.0;
        let z_near: f32 = 0.1;

        let horizontal_angle = 0.0;
        let vertical_angle = 0.0;
        let position = cgmath::Vector3::new(0.0, 0.0, 0.0f32);

        let view_matrix = Camera::view_matrix_from(&position, horizontal_angle, vertical_angle);

        Camera {
            view_matrix,
            projection_matrix: cgmath::perspective(cgmath::Rad(fov), aspect_ratio, z_near, z_far),
            position,
            horizontal_angle,
            vertical_angle,
        }
    }

    pub fn tick(&mut self, event_handler: &event_handler::EventHandler) {
        self.view_matrix = Camera::view_matrix_from(&self.position, self.horizontal_angle,
                                                    self.vertical_angle);
    }

    fn view_matrix_from(position: &cgmath::Vector3<f32>, horizontal_angle: f32, vertical_angle: f32)
        -> cgmath::Matrix4<f32> {
        let (facing_dir, up) = Camera::characteristic_vectors(horizontal_angle, vertical_angle);
        Camera::view_matrix(&position, &(position + facing_dir), &up)
    }

    fn characteristic_vectors(horizontal_angle: f32, vertical_angle: f32)
        -> (cgmath::Vector3<f32>, cgmath::Vector3<f32>) {
        let facing_dir = cgmath::Vector3 {
            x: vertical_angle.cos() * horizontal_angle.sin(),
            y: vertical_angle.sin(),
            z: vertical_angle.cos() * horizontal_angle.cos(),
        };

        let right = cgmath::Vector3 {
            x: (horizontal_angle - std::f32::consts::PI / 2f32).sin(),
            y: 0.0,
            z: (horizontal_angle - std::f32::consts::PI / 2f32).cos(),
        };

        let up = facing_dir.cross(right);

        (facing_dir, up)
    }

    /*
    fn projection_matrix(width: u32, height: u32) -> cgmath::Matrix4<f32> {
        // TODO: Is this shit backwards?
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = std::f32::consts::PI / 3.0;

        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        cgmath::Matrix4::from([
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar+znear)/(zfar-znear), 1.0],
            [0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
        ])
    }
    */

    fn view_matrix(pos: &cgmath::Vector3<f32>, dir: &cgmath::Vector3<f32>,
                   up: &cgmath::Vector3<f32>) -> cgmath::Matrix4<f32> {
        let dir = dir.normalize();

        let s = cgmath::Vector3 {
            x: up.y * dir.z - up.z * dir.y,
            y: up.z * dir.x - up.x * dir.z,
            z: up.x * dir.y - up.y * dir.x,
        }.normalize();

        let u = cgmath::Vector3 {
            x: dir.y * s.z - dir.z * s.y,
            y: dir.z * s.x - dir.x * s.z,
            z: dir.x * s.y - dir.y * s.x,
        };

        let p = cgmath::Vector3 {
            x: -pos.x * s.x - pos.y * s.y - pos.z * s.z,
            y: -pos.x * u.x - pos.y * u.y - pos.z * u.z,
            z: -pos.x * dir.x - pos.y * dir.y - pos.z * dir.z
        };

        cgmath::Matrix4::from([
            [s.x, u.x, dir.x, 0.0],
            [s.y, u.y, dir.y, 0.0],
            [s.z, u.z, dir.z, 0.0],
            [p.x, p.y, p.z, 1.0],
        ])
    }
}

