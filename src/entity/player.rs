use cgmath::InnerSpace;
use glutin::VirtualKeyCode;
use std;
use std::ops::Neg;

use game::event_handler::EventHandler;
use graphics::renderer;
use graphics::Render;
use entity::Entity;
use util::f32::clamp;
use util::math::{Point3, Vector3, Rotation};
use util::collide::{AABB, Range};
use util::collide::Collide;
use world::EntitySlice;

const PLAYER_BOUNDS: [Range; 3] = [
    Range { min: -1.0, max: 1.0 },
    Range { min: -3.0, max: 0.5 },
    Range { min: -1.0, max: 1.0 },
];

const MOVE_SPEED: f32 = 0.04;
const JUMP_SPEED: f32 = 0.35;
const ROTATION_SPEED: f32 = 0.1;
const GRAVITY: f32 = 0.02;
const VELOCITY_DAMPENING_FACTOR: f32 = 0.8;

pub struct Player {
    aabb: AABB,
    velocity: Vector3,
    rotation: Rotation,
    fly_mode: bool,
    // If the player is using a computer, for example.
    input_captured: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            aabb: AABB::new(PLAYER_BOUNDS, Point3 { x: 0.0, y: 0.0, z: 0.0 }),
            velocity: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: Rotation {
                horizontal_angle: 0.0,
                vertical_angle: 0.0,
            },
            fly_mode: false,
            input_captured: false,
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
        }.neg().normalize();

        let right = Vector3 {
            x: (horizontal_angle - std::f32::consts::FRAC_PI_2).sin(),
            y: 0.0,
            z: (horizontal_angle - std::f32::consts::FRAC_PI_2).cos(),
        }.neg().normalize();

        let up = right.cross(forward).normalize();

        (forward, right, up)
    }

    pub fn position(&self) -> &Point3 {
        &self.aabb.position()
    }

    pub fn rotation(&self) -> &Rotation {
        &self.rotation
    }

    fn update_rotation(&mut self, event_handler: &EventHandler) {
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

    fn update_velocity(&mut self, event_handler: &EventHandler) {
        let (forward, right, up) = self.movement_vectors();

        // TODO: Separate input capture and velocity updating, so we can do a clean check
        // for `input_captured`.
        if !self.input_captured {
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

            if self.fly_mode {
                // Up
                if event_handler.is_key_down(&VirtualKeyCode::Space) {
                    self.velocity += MOVE_SPEED * up;
                }
                // Down
                if event_handler.is_key_down(&VirtualKeyCode::LShift) {
                    self.velocity -= MOVE_SPEED * up;
                }

                self.velocity.y *= VELOCITY_DAMPENING_FACTOR;
            } else {
                self.velocity += GRAVITY * up.neg();

                // Up
                if event_handler.is_key_pressed(&VirtualKeyCode::Space) {
                    self.velocity.y = 0.0;
                    self.velocity += JUMP_SPEED * up;
                }
            }
        }

        self.velocity.x *= VELOCITY_DAMPENING_FACTOR;
        self.velocity.z *= VELOCITY_DAMPENING_FACTOR;
    }

    /// Checks for and responds to a collision with the given collidable.
    ///
    /// # Arguments
    ///
    /// * `velocity_delta` - How much the velocity will change after the collision phase.
    fn collide<C: ?Sized + Collide>(&mut self, collidable: &C,
                                    velocity_delta: &mut Vector3) {
        if let Some(mtv) = self.aabb.collide(self.velocity, collidable.aabb()) {
            let mtv_axis = if mtv.x != 0.0 {
                0
            } else if mtv.y != 0.0 {
                1
            } else if mtv.z != 0.0 {
                2
            } else {
                panic!("Received `Some` MTV with all-zero components.");
            };

            velocity_delta[mtv_axis] = -self.velocity[mtv_axis];

            self.aabb.translate(mtv);
        }
    }
}

impl Entity for Player {
    fn tick(&mut self, event_handler: &EventHandler,
            collidables: &Vec<Box<Collide>>,
            entities: EntitySlice) {
        if !self.input_captured {
            if event_handler.is_key_pressed(&VirtualKeyCode::V) {
                self.fly_mode = !self.fly_mode;
            }

            self.update_rotation(event_handler);
        }

        self.update_velocity(event_handler);

        // Handle collisions.

        // We want to update the velocity only after we've used it to translate for the
        // current frame.
        let mut velocity_delta = Vector3 { x: 0.0, y: 0.0, z: 0.0 };

        // First, check for collisions with static collidables.
        for collidable in collidables {
            self.collide(&**collidable, &mut velocity_delta);
        }
        // Then, check for collisions/interactions with entities.
        for entity in entities.into_iter() {
            self.collide(&**entity, &mut velocity_delta);
            if entity.interactable() {
                // TODO: Ray collision.
                if event_handler.is_key_pressed(&VirtualKeyCode::Return) {
                    self.input_captured = true;
                    (& mut**entity).interact();
                } else if event_handler.is_key_pressed(&VirtualKeyCode::Escape) {
                    self.input_captured = false;
                    (& mut**entity).stop_interact();
                }
            }
        }

        self.aabb.translate(self.velocity);
        self.velocity += velocity_delta;
    }

    fn interactable(&self) -> bool {
        false
    }

    fn interact(&mut self) { }

    fn stop_interact(&mut self) { }
}

impl Collide for Player {
    fn aabb(&self) -> &AABB {
        &self.aabb
    }
}

impl Render for Player {
    fn render(&mut self, _: &mut renderer::RenderingContext) { }
}
