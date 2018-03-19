use cgmath::InnerSpace;
use glutin::VirtualKeyCode;
use std;
use std::ops::Neg;

use game::event_handler::EventHandler;
use graphics::renderer;
use graphics::Render;
use graphics::mesh::obj;
use entity::Entity;
use util::f32::clamp;
use util::math::{Point3, Ray3, Rotation, Vector2, Vector3};
use util::collide::aabb;
use util::collide::aabb::Range;
use util::collide::Collide;
use util::collide::sat::CollisionMesh;
use world::TickConfig;


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
    collision_mesh: CollisionMesh,
    velocity: Vector3,
    rotation: Rotation,
    on_ground: bool,
    fly_mode: bool,
    // If the player is using a computer, for example.
    input_captured: bool,
}

impl Player {
    pub fn new() -> Player {
        let test_position = Point3 {
            x: 7.0,
            y: 3.0,
            z: 0.0,
        };
        let mut test_render_mesh = obj::new("res/ramp.obj");
        test_render_mesh.move_to(test_position);
        let position = Point3 { x: 0.0, y: 4.0, z: 0.0 };
        Player {
            collision_mesh: aabb::new(PLAYER_BOUNDS, position),
            velocity: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: Rotation {
                horizontal_angle: 0.0,
                vertical_angle: 0.0,
            },
            on_ground: false,
            fly_mode: true,
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
        &self.collision_mesh.position()
    }

    pub fn velocity(&self) -> &Vector3 {
        &self.velocity
    }

    pub fn rotation(&self) -> &Rotation {
        &self.rotation
    }

    fn update_rotation(&mut self, mouse_delta: Vector2) {
        let Vector2 { x: mouse_x, y: mouse_y } = mouse_delta;

        // Moving the mouse in the +x axis corresponds to clockwise rotation, which will be negative
        // in radians.
        self.rotation.horizontal_angle -= mouse_x / 100.0;
        self.rotation.vertical_angle -= mouse_y / 100.0;

        // Prevent looking too far up or down, causing the camera to go upside down.
        self.rotation.vertical_angle = clamp(self.rotation.vertical_angle,
                                             -std::f32::consts::FRAC_PI_2,
                                             std::f32::consts::FRAC_PI_2);
    }

    fn mouse_delta(&self, event_handler: &EventHandler) -> Vector2 {
        let (mouse_x, mouse_y) = event_handler.mouse_delta();
        Vector2 {
            x: mouse_x as f32 * ROTATION_SPEED,
            y: mouse_y as f32 * ROTATION_SPEED,
        }
    }

    fn keyboard_delta(&mut self, event_handler: &EventHandler) -> Vector3 {
        let (forward, right, up) = self.movement_vectors();

        let mut velocity_delta = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        if !self.input_captured {
            // Forward
            if event_handler.is_key_down(&VirtualKeyCode::W) {
                velocity_delta += MOVE_SPEED * forward;
            }
            // Backward
            if event_handler.is_key_down(&VirtualKeyCode::S) {
                velocity_delta += MOVE_SPEED * forward.neg();
            }
            // Left
            if event_handler.is_key_down(&VirtualKeyCode::A) {
                velocity_delta += MOVE_SPEED * right.neg();
            }
            // Right
            if event_handler.is_key_down(&VirtualKeyCode::D) {
                velocity_delta += MOVE_SPEED * right;
            }

            if self.fly_mode {
                // Up
                if event_handler.is_key_down(&VirtualKeyCode::Space) {
                    velocity_delta += MOVE_SPEED * up;
                }
                // Down
                if event_handler.is_key_down(&VirtualKeyCode::LShift) {
                    velocity_delta -= MOVE_SPEED * up;
                }

                velocity_delta.y *= VELOCITY_DAMPENING_FACTOR;
            } else {
                velocity_delta += GRAVITY * up.neg();

                // Up
                if self.on_ground && event_handler.is_key_pressed(&VirtualKeyCode::Space) {
                    velocity_delta.y = 0.0;
                    velocity_delta += JUMP_SPEED * up;
                    self.on_ground = false;
                }
            }
        }

        velocity_delta
    }

    /// Checks for and responds to a collision with the given collidable.
    ///
    /// # Arguments
    ///
    /// * `velocity_delta` - How much the velocity will change after the collision phase.
    fn collide<C: ?Sized + Collide>(&mut self, collidable: &C) {
        if let Some(mtv) = self.collision_mesh.collide_with(collidable.collision_mesh()) {
            let x_abs = mtv.x.abs();
            let y_abs = mtv.y.abs();
            let z_abs = mtv.z.abs();
            if y_abs > x_abs && y_abs > z_abs {
                if mtv.y > 0.0 {
                    self.on_ground = true;
                }
                self.velocity.y = 0.0;
            } else if x_abs > y_abs && x_abs > z_abs {
                self.velocity.x = 0.0;
            } else if z_abs > x_abs && z_abs > y_abs {
                self.velocity.z = 0.0;
            }
            self.collision_mesh.translate(mtv);
        }
    }
}

impl Entity for Player {
    fn tick(&mut self, config: TickConfig) {
        let event_handler = config.event_handler;

        // Process input.
        {
            let mut mouse_delta = Vector2 { x: 0.0, y: 0.0 };
            let mut velocity_delta = Vector3 { x: 0.0, y: 0.0, z: 0.0 };

            if !self.input_captured {
                if event_handler.is_key_pressed(&VirtualKeyCode::V) {
                    self.fly_mode = !self.fly_mode;
                }

                mouse_delta += self.mouse_delta(event_handler);
                velocity_delta += self.keyboard_delta(event_handler);
            }
            self.update_rotation(mouse_delta);

            self.velocity += velocity_delta;
            self.velocity.x *= VELOCITY_DAMPENING_FACTOR;
            if self.fly_mode {
                self.velocity.y *= VELOCITY_DAMPENING_FACTOR;
            }
            self.velocity.z *= VELOCITY_DAMPENING_FACTOR;
        }

        // Handle collisions.
        {
            self.collision_mesh.translate(self.velocity);

            // First, check for collisions with static collidables.
            for collidable in config.collidables.iter() {
                self.collide(&**collidable);
            }
            // Then, check for collisions/interactions with entities.
            for entity in config.entities.into_iter() {
                let mut entity = & mut**entity;
                self.collide(entity);
                if entity.interactable() {
                    if event_handler.is_key_pressed(&VirtualKeyCode::E) {
                        let collides = {
                            let view_ray = Ray3 {
                                pos: self.collision_mesh.position().clone(),
                                dir: self.rotation.to_view_vec(),
                            };
                            // TODO: This shit don't work.  You need some visual debugging
                            // tools for meshes and vectors and shit.
                            entity.collision_mesh().collide_with_ray(view_ray)
                        };

                        if collides {
                            self.input_captured = true;
                            entity.interact();
                        }
                    } else if event_handler.is_key_pressed(&VirtualKeyCode::Escape) {
                        self.input_captured = false;
                        entity.stop_interact();
                    }
                }
            }
        }
    }

    fn interactable(&self) -> bool { false }

    fn interact(&mut self) { }

    fn stop_interact(&mut self) { }
}

impl Collide for Player {
    fn collision_mesh(&self) -> &CollisionMesh {
        &self.collision_mesh
    }
}

impl Render for Player {
    fn render(&mut self, _: &mut renderer::RenderingContext) { }
}
