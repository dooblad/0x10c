pub mod camera;
pub mod event_handler;

use glutin;
use graphics;
use std::thread;
use std::time::{Duration, Instant};

use entity;
use world;

const WINDOW_DIMENSIONS: (u32, u32) = (800, 500);
static WINDOW_TITLE: &'static str = "0x10c";

pub struct Game {
    current_state: Box<GameState>,
    event_handler: event_handler::EventHandler,
}

impl Game {
    pub fn new() -> Game {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_dimensions(WINDOW_DIMENSIONS.0, WINDOW_DIMENSIONS.1)
            .with_title(WINDOW_TITLE);
        let context = glutin::ContextBuilder::new();
        // TODO: Update and add gl-rs initialization shit in Display constructor.
        let display = graphics::Display::new(window, context, &events_loop).unwrap();

        {
            let gl_window = display.gl_window();
            // Make cursor invisible.
            gl_window.set_cursor(glutin::MouseCursor::NoneCursor);
            // Confine cursor to this window.
            gl_window.set_cursor_state(glutin::CursorState::Grab).unwrap();
        }

        Game {
            current_state: Box::new(MainGameState::new(display)),
            event_handler: event_handler::EventHandler::new(events_loop),
        }
    }

    pub fn run(&mut self) {
        let mut accumulator = Duration::new(0, 0);
        let mut previous_clock = Instant::now();

        let event_handler = &mut self.event_handler;
        loop {
            event_handler.tick();
            if event_handler.close_requested() {
                break;
            }

            let now = Instant::now();
            accumulator += now - previous_clock;
            previous_clock = now;

            let fixed_time_stamp = Duration::new(0, 16666667);
            while accumulator >= fixed_time_stamp {
                accumulator -= fixed_time_stamp;

                self.current_state.tick(&event_handler);
                self.current_state.render();
            }

            thread::sleep(fixed_time_stamp - accumulator);
        }
    }
}

//
// Game State Shit
//

trait GameState {
    fn tick(&mut self, event_handler: &event_handler::EventHandler);
    fn render(&mut self);
}

struct MainGameState<'a> {
    camera: camera::Camera,
    world: world::World<'a>,
}

impl<'a> MainGameState<'a> {
    pub fn new(display: graphics::Display) -> MainGameState {
        let camera = camera::Camera::new(WINDOW_DIMENSIONS.0, WINDOW_DIMENSIONS.1);
        let player = entity::player::Player::new();
        let world = world::World::new(player, display);

        MainGameState {
            camera,
            world,
        }
    }
}

impl<'a> GameState for MainGameState<'a> {
    fn tick(&mut self, event_handler: &event_handler::EventHandler) {
        self.world.tick(event_handler);
    }

    fn render(&mut self) {
        {
            let player = self.world.player();
            self.camera.set_view(player.position(), player.rotation());
        }

        self.world.render(&self.camera);
    }
}

