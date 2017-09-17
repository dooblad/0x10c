use glium;
use glium::glutin;

pub struct EventHandler {
    events_loop: glutin::EventsLoop,
    close_requested: bool,
}

impl EventHandler {
    pub fn new(events_loop: glutin::EventsLoop) -> EventHandler {
        EventHandler {
            events_loop,
            close_requested: false,
        }
    }

    pub fn tick(&mut self) {
        let close_requested = &mut self.close_requested;
        self.events_loop.poll_events(|event| {
            use glium::glutin::Event::{Awakened, DeviceEvent, WindowEvent};

            match event {
                WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::Closed => *close_requested = true,
                    _ => (),
                },
                DeviceEvent { event, .. } => {
                    use glium::glutin::DeviceEvent::{Added, Removed, Motion, Button, Key, Text};
                    match event {
                        Added => (),
                        Removed => (),
                        Motion { axis, value, } => {
                            println!("Axis: {}, Value: {}", axis, value);
                        },
                        Button { button, state, } => {
                            println!("Button: {}, State: {:?}", button, state);
                        },
                        Key(input) => {

                        },
                        Text { codepoint } => {
                            println!("Codepoint: {}", codepoint);
                        },
                    }
                },
                Awakened => (),
            }
        });
    }

    pub fn close_requested(&self) -> bool {
        return self.close_requested;
    }
}
