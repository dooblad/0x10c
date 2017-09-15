use glium;
use glium::glutin;
use std::thread;
use std::time::{Duration, Instant};

mod renderer;

const WINDOW_DIMENSIONS: (u32, u32) = (800, 500);
static WINDOW_TITLE: &'static str = "0x10c";

pub struct Game {
    current_state: Box<GameState>,
    events_loop: glutin::EventsLoop,
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
            events_loop,
        }
    }

    pub fn run(&mut self) {
        let mut accumulator = Duration::new(0, 0);
        let mut previous_clock = Instant::now();

        let events_loop = &mut self.events_loop;
        let current_state = &mut self.current_state;
        let mut close_requested = false;

        loop {
            events_loop.poll_events(|event| {
                // TODO: Handle key inputs.
                match event {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::Closed => close_requested = true,
                        _ => (),
                    },
                    _ => (),
                }
            });
            if close_requested {
                break;
            }

            let now = Instant::now();
            accumulator += now - previous_clock;
            previous_clock = now;

            let fixed_time_stamp = Duration::new(0, 16666667);
            while accumulator >= fixed_time_stamp {
                accumulator -= fixed_time_stamp;

                current_state.tick();
                current_state.render();
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
    renderer: renderer::Renderer,
}

impl MainGameState {
    pub fn new(display: glium::Display) -> MainGameState {
        MainGameState {
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

