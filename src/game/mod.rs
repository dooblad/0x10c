pub mod camera;
pub mod event_handler;

use glium;
use glium::glutin;
use std::thread;
use std::time::{Duration, Instant};

use graphics::renderer;

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
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        Game {
            current_state: Box::new(MainGameState::new(display)),
            event_handler: event_handler::EventHandler::new(events_loop),
        }
    }

    pub fn run(&mut self) {
        let mut accumulator = Duration::new(0, 0);
        let mut previous_clock = Instant::now();

        loop {
            self.event_handler.tick();
            if self.event_handler.close_requested() {
                break;
            }

            let now = Instant::now();
            accumulator += now - previous_clock;
            previous_clock = now;

            let fixed_time_stamp = Duration::new(0, 16666667);
            while accumulator >= fixed_time_stamp {
                accumulator -= fixed_time_stamp;

                self.current_state.tick();
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
    fn tick(&mut self);
    fn render(&mut self);
}

struct MainGameState {
    camera: camera::Camera,
    renderer: renderer::Renderer,
}

impl MainGameState {
    pub fn new(display: glium::Display) -> MainGameState {
        MainGameState {
            camera: camera::Camera::new(WINDOW_DIMENSIONS.0, WINDOW_DIMENSIONS.1),
            renderer: renderer::Renderer::new(display),
        }
    }
}

impl GameState for MainGameState {
    fn tick(&mut self) {
    }

    fn render(&mut self) {
        self.renderer.render()
    }
}

