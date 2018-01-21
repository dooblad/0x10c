use glutin::VirtualKeyCode;
use glutin::VirtualKeyCode::*;

use game::event_handler::EventHandler;
use hardware::dcpu::Dcpu;


const MAPPING_LOCATION: u16 = 0x9000;
const HEAD_LOCATION: u16 = MAPPING_LOCATION;
const SIZE_LOCATION: u16 = MAPPING_LOCATION + 1;
const DATA_LOCATION: u16 = MAPPING_LOCATION + 2;
const BUFFER_CAPACITY: u16 = 16;

const PRINTABLE_CHAR_OFFSET: u16 = 0x20;
// Defines the ordering of ASCII characters, starting at the first printable character.
const KEY_CODE_MAPPINGS: [(bool, VirtualKeyCode); 95] = [
    (false, Space),
    (true, Key1),
    (true, Apostrophe),
    (true, Key3),
    (true, Key4),
    (true, Key5),
    (true, Key7),
    (false, Apostrophe),
    (true, Key9),
    (true, Key0),
    (true, Key8),
    (true, Equals),
    (false, Comma),
    (false, Subtract),
    (false, Period),
    (false, Slash),
    (false, Key0),
    (false, Key1),
    (false, Key2),
    (false, Key3),
    (false, Key4),
    (false, Key5),
    (false, Key6),
    (false, Key7),
    (false, Key8),
    (false, Key9),
    (true, Semicolon),
    (false, Semicolon),
    (true, Comma),
    (false, Equals),
    (true, Period),
    (true, Slash),
    (true, Key2),
    (true, A),
    (true, B),
    (true, C),
    (true, D),
    (true, E),
    (true, F),
    (true, G),
    (true, H),
    (true, I),
    (true, J),
    (true, K),
    (true, L),
    (true, M),
    (true, N),
    (true, O),
    (true, P),
    (true, Q),
    (true, R),
    (true, S),
    (true, T),
    (true, U),
    (true, V),
    (true, W),
    (true, X),
    (true, Y),
    (true, Z),
    (false, LBracket),
    (false, Backslash),
    (false, RBracket),
    (true, Key6),
    (true, Subtract),
    (false, Grave),
    (false, A),
    (false, B),
    (false, C),
    (false, D),
    (false, E),
    (false, F),
    (false, G),
    (false, H),
    (false, I),
    (false, J),
    (false, K),
    (false, L),
    (false, M),
    (false, N),
    (false, O),
    (false, P),
    (false, Q),
    (false, R),
    (false, S),
    (false, T),
    (false, U),
    (false, V),
    (false, W),
    (false, X),
    (false, Y),
    (false, Z),
    (true, LBracket),
    (true, Backslash),
    (true, RBracket),
    (true, Grave),
];


pub struct Keyboard {
    focused: bool,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            focused: false,
        }
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
}

pub fn try_push_key(dcpu: &mut Dcpu, event_handler: &EventHandler) {
    let is_shifted = event_handler.is_key_down(&LShift) ||
        event_handler.is_key_down(&RShift);

    let mut key_code = 0;
    for (i, &(needs_shift, key)) in KEY_CODE_MAPPINGS.iter().enumerate() {
        if needs_shift == is_shifted && event_handler.is_key_pressed(&key) {
            key_code = (i as u16) + PRINTABLE_CHAR_OFFSET;
            break;
        }
    }

    if key_code == 0 {
        key_code = if event_handler.is_key_pressed(&Back) {
            0x10
        } else if event_handler.is_key_pressed(&Return) {
            0x11
        } else if event_handler.is_key_pressed(&Insert) {
            0x12
        } else if event_handler.is_key_pressed(&Delete) {
            0x13
        } else if event_handler.is_key_pressed(&Up) {
            0x80
        } else if event_handler.is_key_pressed(&Down) {
            0x81
        } else if event_handler.is_key_pressed(&Left) {
            0x82
        } else if event_handler.is_key_pressed(&Right) {
            0x83
        } else if event_handler.is_key_pressed(&LShift) ||
            event_handler.is_key_pressed(&RShift) {
            0x90
        } else if event_handler.is_key_pressed(&LControl) ||
            event_handler.is_key_pressed(&RControl) {
            0x91
        } else {
            0x0
        };
    }

    if key_code != 0 {
        let head = dcpu.mem(HEAD_LOCATION);
        let size = dcpu.mem(SIZE_LOCATION);
        if size < BUFFER_CAPACITY {
            let insert_idx = (head + size) % BUFFER_CAPACITY;
            dcpu.set_mem(DATA_LOCATION + insert_idx, key_code);
            dcpu.set_mem(SIZE_LOCATION, size + 1);
        }
    }
}
