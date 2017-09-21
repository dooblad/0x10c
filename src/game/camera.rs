use cgmath;
use cgmath::InnerSpace;
use glium::glutin;
use std;
use std::ops::Neg;

use game::event_handler;

const MOVE_SPEED: f32 = 0.1;
const ROTATION_SPEED: f32 = 0.1;

pub struct Camera {
    pub view_matrix: cgmath::Matrix4<f32>,
    pub projection_matrix: cgmath::Matrix4<f32>,
    pub position: cgmath::Point3<f32>,
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
//        let position = cgmath::Point3::new(0.0, 0.0, 0.0f32);
        let position = cgmath::Point3::new(0.5, 0.2, -3.0f32);

        let view_matrix = Self::view_matrix_from(&position, horizontal_angle, vertical_angle);

        Camera {
            view_matrix,
            projection_matrix: cgmath::perspective(cgmath::Rad(fov), aspect_ratio, z_near, z_far),
            position,
            horizontal_angle,
            vertical_angle,
        }
    }

    pub fn tick(&mut self, event_handler: &event_handler::EventHandler) {
        let (mouse_x, mouse_y) = event_handler.mouse_delta();
        let mouse_x = mouse_x as f32 * ROTATION_SPEED;
        let mouse_y = mouse_y as f32 * ROTATION_SPEED;
        self.horizontal_angle += mouse_x / 100.0;
        self.vertical_angle += mouse_y / 100.0;

        let (facing_dir, right_dir, up_dir) = Self::characteristic_vectors(self.horizontal_angle,
                                                                           self.vertical_angle);
        let facing_dir = facing_dir * MOVE_SPEED;
        let right_dir = right_dir * MOVE_SPEED;
        let up_dir = up_dir * MOVE_SPEED;

        // Forward
        if event_handler.is_key_down(&glutin::VirtualKeyCode::W) {
            self.position += facing_dir;
        }
        // Backward
        if event_handler.is_key_down(&glutin::VirtualKeyCode::S) {
            self.position += facing_dir.neg();
        }
        // Left
        if event_handler.is_key_down(&glutin::VirtualKeyCode::A) {
            self.position += right_dir;
        }
        // Right
        if event_handler.is_key_down(&glutin::VirtualKeyCode::D) {
            self.position += right_dir.neg();
        }
        // Up
        if event_handler.is_key_down(&glutin::VirtualKeyCode::Space) {
            self.position += up_dir;
        }
        // Down
        if event_handler.is_key_down(&glutin::VirtualKeyCode::LShift) {
            self.position += up_dir.neg();
        }

        self.view_matrix = Camera::view_matrix_from(&self.position, self.horizontal_angle,
                                                    self.vertical_angle);
    }

    pub fn view_matrix(&self) -> cgmath::Matrix4<f32> {
        return self.view_matrix
    }

    pub fn projection_matrix(&self) -> cgmath::Matrix4<f32> {
        return self.projection_matrix
    }

    fn view_matrix_from(position: &cgmath::Point3<f32>, horizontal_angle: f32, vertical_angle: f32)
        -> cgmath::Matrix4<f32> {
        let (facing_dir, _, up) = Self::characteristic_vectors(horizontal_angle, vertical_angle);
        // TODO: Inline this function into this one.
//        Camera::make_view_matrix(&position, &(position + facing_dir), &up)
        cgmath::Matrix4::look_at(position.clone(), position + facing_dir, up)
    }

    fn characteristic_vectors(horizontal_angle: f32, vertical_angle: f32)
        -> (cgmath::Vector3<f32>, cgmath::Vector3<f32>, cgmath::Vector3<f32>) {
        let facing_dir = cgmath::Vector3 {
            x: vertical_angle.cos() * horizontal_angle.sin(),
            y: vertical_angle.sin(),
            z: vertical_angle.cos() * horizontal_angle.cos(),
        }.normalize();

        let right = cgmath::Vector3 {
            x: (horizontal_angle - std::f32::consts::PI / 2f32).sin(),
            y: 0.0,
            z: (horizontal_angle - std::f32::consts::PI / 2f32).cos(),
        }.normalize();

        let up = facing_dir.cross(right).normalize();

        (facing_dir, right, up)
    }
}

