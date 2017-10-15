use cgmath::InnerSpace;
use glium::glutin::VirtualKeyCode;
use std;
use std::ops::Neg;

use game;
use graphics::Render;
use graphics::renderer;
use entity::Entity;
use util::f32::clamp;
use util::math::{Point3, Vector3, Rotation};
use util::collide::AABB;
use util::collide::Collide;

const PLAYER_AABB: (Point3, Point3) = (
    Point3 { x: -1.0, y: -2.0, z: -1.0 },
    Point3 { x: 1.0, y: 0.5, z: 1.0 },
);

const MOVE_SPEED: f32 = 0.04;
const JUMP_SPEED: f32 = 0.35;
const ROTATION_SPEED: f32 = 0.1;
const GRAVITY: f32 = 0.01;
const VELOCITY_DAMPENING_FACTOR: f32 = 0.8;

pub struct Player {
    aabb: AABB,
    // TODO: Add velocity.
    velocity: Vector3,
    rotation: Rotation,
}

impl Player {
    pub fn new() -> Player {
        Player {
            aabb: AABB::new(PLAYER_AABB.0, PLAYER_AABB.1, Point3 { x: 0.0, y: 0.0, z: 0.0 }),
            velocity: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: Rotation {
                horizontal_angle: 0.0,
                vertical_angle: 0.0,
            },
        }
    }

    fn movement_vectors(&self) -> (Vector3, Vector3, Vector3) {
        // Movement disregards the player's vertical view angle, because looking up and moving
        // forward should not cause the player to move up.
        let Rotation { horizontal_angle, .. } = self.rotation;

        let forward = Vector3 {
            x: horizontal_angle.sin(),
            y: 0.0,
            z: horizontal_angle.cos(),
        }.normalize();

        let right = Vector3 {
            x: (horizontal_angle - std::f32::consts::FRAC_PI_2).sin(),
            y: 0.0,
            z: (horizontal_angle - std::f32::consts::FRAC_PI_2).cos(),
        }.normalize();

        let up = right.cross(forward).normalize();

        (forward, right, up)
    }

    pub fn position(&self) -> &Point3 {
        &self.aabb.position()
    }

    pub fn rotation(&self) -> &Rotation {
        &self.rotation
    }

    fn update_rotation(&mut self, event_handler: &game::event_handler::EventHandler) {
        let (mouse_x, mouse_y) = event_handler.mouse_delta();
        let mouse_x = mouse_x as f32 * ROTATION_SPEED;
        let mouse_y = mouse_y as f32 * ROTATION_SPEED;
        // Moving the mouse in the +x axis corresponds to clockwise rotation, which will be negative
        // in radians.
        self.rotation.horizontal_angle -= mouse_x / 100.0;
        self.rotation.vertical_angle -= mouse_y / 100.0;

        // Prevent looking too far up or down, causing the camera to go upside down.
        self.rotation.vertical_angle = clamp(self.rotation.vertical_angle,
                                             -std::f32::consts::FRAC_PI_2,
                                             std::f32::consts::FRAC_PI_2);
    }

    fn update_velocity(&mut self, event_handler: &game::event_handler::EventHandler) {
        let (forward, right, up) = self.movement_vectors();

        // Forward
        if event_handler.is_key_down(&VirtualKeyCode::W) {
            self.velocity += MOVE_SPEED * forward;
        }
        // Backward
        if event_handler.is_key_down(&VirtualKeyCode::S) {
            self.velocity += MOVE_SPEED * forward.neg();
        }
        // Left
        if event_handler.is_key_down(&VirtualKeyCode::A) {
            self.velocity += MOVE_SPEED * right.neg();
        }
        // Right
        if event_handler.is_key_down(&VirtualKeyCode::D) {
            self.velocity += MOVE_SPEED * right;
        }

        self.velocity += GRAVITY * up.neg();

        // Up
        if event_handler.is_key_pressed(&VirtualKeyCode::Space) {
            self.velocity.y = 0.0;
            self.velocity += JUMP_SPEED * up;
        }
    }
}

impl Entity for Player {
    fn tick(&mut self, event_handler: &game::event_handler::EventHandler,
            collidables: &Vec<Box<Collide>>) {
        self.update_rotation(event_handler);

        self.update_velocity(event_handler);
        self.aabb.translate(self.velocity);
        self.velocity.x *= VELOCITY_DAMPENING_FACTOR;
        self.velocity.z *= VELOCITY_DAMPENING_FACTOR;

        println!("VELOCITY: {:?}", self.velocity);

        // Handle collisions.
        for collidable in collidables.iter() {
            for i in 0..2 {
                if let Some(mtv) = self.aabb.collide(collidable.aabb()) {
                    self.aabb.translate(mtv);

                    if mtv.x != 0.0 {
                        self.velocity.x = 0.0;
                    } else if mtv.y != 0.0 {
                        self.velocity.y = 0.0;
                    } else if mtv.z != 0.0 {
                        self.velocity.z = 0.0;
                    }
                }
            }
        }
    }
}

impl Collide for Player {
    fn aabb(&self) -> &AABB {
        &self.aabb
    }
}

impl Render for Player {
    fn render(&mut self, _: &mut renderer::RenderingContext) {

    }
}
