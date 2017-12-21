use cgmath;
use cgmath::InnerSpace;
use std;

use util::math::{Matrix4, Point3, Rotation, Vector3};

pub struct Camera {
    view_matrix: Matrix4,
    projection_matrix: Matrix4,
    position: Point3,
    rotation: Rotation,
}

impl Camera {
    // TODO: This operates under the assumption that the window size never changes.
    // Make it update by checking for a window resize event and calling a
    // `set_window_size` method here.
    pub fn new(width: u32, height: u32) -> Camera {
        let fov: f32 = std::f32::consts::PI / 3.0;
        let aspect_ratio = width as f32 / height as f32;
        let z_far: f32 = 1024.0;
        let z_near: f32 = 0.1;

        let rotation = Rotation {
            horizontal_angle: 0.0,
            vertical_angle: 0.0,
        };

        let position = Point3::new(0.0, 0.0, 0.0f32);

        let view_matrix = Self::view_matrix_from(&position, &rotation);

        Camera {
            view_matrix,
            projection_matrix: cgmath::perspective(cgmath::Rad(fov), aspect_ratio, z_near, z_far),
            position,
            rotation,
        }
    }

    /*
pub fn tick(&mut self, event_handler: &event_handler::EventHandler) {
    let (mouse_x, mouse_y) = event_handler.mouse_delta();
    let mouse_x = mouse_x as f32 * ROTATION_SPEED;
    let mouse_y = mouse_y as f32 * ROTATION_SPEED;
    self.horizontal_angle += mouse_x / 100.0;
    self.vertical_angle += mouse_y / 100.0;

    // Prevent looking too far up or down, causing the camera to go upside down.
    self.vertical_angle = if self.vertical_angle > std::f32::consts::FRAC_PI_2 {
        std::f32::consts::FRAC_PI_2
    } else if self.vertical_angle < -std::f32::consts::FRAC_PI_2 {
        -std::f32::consts::FRAC_PI_2
    } else {
        self.vertical_angle
    };

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
    }
    */

    pub fn set_view(&mut self, position: &Point3, rotation: &Rotation) {
        self.position = position.clone();
        self.rotation = rotation.clone();

        self.view_matrix = Self::view_matrix_from(&self.position, &self.rotation);
    }

    fn view_matrix_from(position: &Point3, rotation: &Rotation) -> Matrix4 {
        let (facing_dir, _, up) = Self::characteristic_vectors(rotation);
        cgmath::Matrix4::look_at(position.clone(), position + facing_dir, up)
    }

    fn characteristic_vectors(rotation: &Rotation) -> (Vector3, Vector3, Vector3) {
        let &Rotation { horizontal_angle, vertical_angle } = rotation;

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

//        let up = facing_dir.cross(right).normalize();
        let up = right.cross(facing_dir).normalize();

        (facing_dir, right, up)
    }

    pub fn position(&self) -> Point3 {
        self.position
    }

    pub fn view_matrix(&self) -> Matrix4 {
        self.view_matrix
    }

    pub fn projection_matrix(&self) -> Matrix4 {
        self.projection_matrix
    }
}

