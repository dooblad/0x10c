use glutin;
use std::collections::HashSet;

pub struct EventHandler {
    events_loop: glutin::EventsLoop,
    close_requested: bool,
    mouse_delta: (f64, f64),
    pressed_mouse_buttons: HashSet<u32>,
    // Store the currently-pressed keys and the keys that were pressed on the last tick.
    last_pressed_keys: HashSet<glutin::VirtualKeyCode>,
    pressed_keys: HashSet<glutin::VirtualKeyCode>,
}

impl EventHandler {
    pub fn new(events_loop: glutin::EventsLoop) -> EventHandler {
        EventHandler {
            events_loop,
            close_requested: false,
            mouse_delta: (0.0, 0.0),
            pressed_mouse_buttons: HashSet::new(),
            last_pressed_keys: HashSet::new(),
            pressed_keys: HashSet::new(),
        }
    }

    pub fn tick(&mut self) {
        let close_requested = &mut self.close_requested;
        let mouse_delta = &mut self.mouse_delta;
        let pressed_mouse_buttons = &mut self.pressed_mouse_buttons;
        // TODO: Make more efficient.
        self.last_pressed_keys = self.pressed_keys.clone();
        let pressed_keys = &mut self.pressed_keys;

        *mouse_delta = (0.0, 0.0);

        self.events_loop.poll_events(|event| {
            use glutin::Event::{Awakened, DeviceEvent, WindowEvent};

            match event {
                WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => *close_requested = true,
                    _ => (),
                },
                DeviceEvent { event, .. } => {
                    use glutin::DeviceEvent::*;
                    match event {
                        Added => (),
                        Removed => (),
                        Motion { axis, value, } => {
                            match axis {
                                0 => mouse_delta.0 += value,
                                1 => mouse_delta.1 += value,
                                _ => eprintln!("Unknown mouse axis {}", axis),
                            }
                        },
                        Button { button, state, } => {
                            match state {
                                glutin::ElementState::Pressed => {
                                    pressed_mouse_buttons.insert(button);
                                },
                                glutin::ElementState::Released => {
                                    pressed_mouse_buttons.remove(&button);
                                },
                            }
                        },
                        Key(input) => match input {
                            glutin::KeyboardInput { state, virtual_keycode: Some(key), .. } => {
                                match state {
                                    glutin::ElementState::Pressed => {
                                        pressed_keys.insert(key);
                                    },
                                    glutin::ElementState::Released => {
                                        pressed_keys.remove(&key);
                                    },
                                }
                            },
                            _ => {}
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

    pub fn mouse_delta(&self) -> (f64, f64) {
        return self.mouse_delta;
    }

    pub fn is_left_mouse_down(&self) -> bool {
        return self.pressed_mouse_buttons.contains(&1);
    }

    pub fn is_right_mouse_down(&self) -> bool {
        return self.pressed_mouse_buttons.contains(&3);
    }

    pub fn is_key_down(&self, key: &glutin::VirtualKeyCode) -> bool {
        return self.pressed_keys.contains(key);
    }

    pub fn is_key_pressed(&self, key: &glutin::VirtualKeyCode) -> bool {
        return self.pressed_keys.contains(key) && !self.last_pressed_keys.contains(key);
    }
}
