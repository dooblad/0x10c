use entity::Entity;
use game::event_handler::EventHandler;
use graphics::Render;
use graphics::renderer::RenderingContext;
use graphics::mesh::pixel_quad::PixelQuad;
use hardware::dcpu::Dcpu;
use hardware::dcpu::assembler;
use hardware::keyboard;
use hardware::keyboard::Keyboard;
use util::collide::{AABB, Range};
use util::collide::Collide;
use util::math::Point3;
use world::EntitySlice;

const SCREEN_SIZE_IN_PIXELS: (u16, u16) = (128, 96);
const SCREEN_SIZE_IN_CELLS: (u16, u16) = (32, 12);
const CELL_SIZE_IN_PIXELS: (u16, u16) = (4, 8);

const BLINK_CYCLE_LENGTH: u32 = 60;

// Scale factor for the size of the screen.
const SCREEN_SCALE: f32 = 2.0;
const COLOR_PALETTE: [[u8; 3]; 16] = [
    [20, 12, 28],
    [68, 36, 52],
    [48, 52, 109],
    [78, 74, 78],
    [133, 76, 48],
    [52, 101, 36],
    [208, 70, 72],
    [117, 113, 97],
    [89, 125, 206],
    [210, 125, 44],
    [133, 149, 161],
    [109, 170, 44],
    [210, 170, 153],
    [109, 194, 202],
    [218, 212, 94],
    [222, 238, 214],
];

#[allow(dead_code)]
const FONT: [[u16; 2]; 95] = [
    [
        // " "
        0b0000000000000000,
        0b0000000000000000,
    ],
    [
        // "!"
        0b0000000001011111,
        0b0000000000000000,
    ],
    [
        // '"'
        0b0000001100000000,
        0b0000001100000000,
    ],
    [
        // "#"
        0b0111111100010010,
        0b0111111100000000,
    ],
    [
        // "$"
        0b0010010001101011,
        0b0001001000000000,
    ],
    [
        // "%"
        0b0110000100011100,
        0b0100001100000000,
    ],
    [
        // "&"
        0b0011011001001001,
        0b0111010000000000,
    ],
    [
        // "'"
        0b0000000000000011,
        0b0000000000000000,
    ],
    [
        // "("
        0b0000000000111110,
        0b0100000100000000,
    ],
    [
        // ")"
        0b0100000100111110,
        0b0000000000000000,
    ],
    [
        // "*"
        0b0010101000011100,
        0b0010101000000000,
    ],
    [
        // "+"
        0b0000100000011100,
        0b0000100000000000,
    ],
    [
        // ","
        0b0100000000100000,
        0b0000000000000000,
    ],
    [
        // "-"
        0b0000100000001000,
        0b0000100000000000,
    ],
    [
        // "."
        0b0000000001000000,
        0b0000000000000000,
    ],
    [
        // "/"
        0b0110000000011100,
        0b0000001100000000,
    ],
    [
        // "0"
        0b0011111001001001,
        0b0011111000000000,
    ],
    [
        // "1"
        0b0100001001111111,
        0b0100000000000000,
    ],
    [
        // "2"
        0b0110001001011001,
        0b0100011000000000,
    ],
    [
        // "3"
        0b0010001001001001,
        0b0011011000000000,
    ],
    [
        // "4"
        0b0000111100001000,
        0b0111111100000000,
    ],
    [
        // "5"
        0b0100111101001001,
        0b0011000100000000,
    ],
    [
        // "6"
        0b0011111001001001,
        0b0011000100000000,
    ],
    [
        // "7"
        0b0110000100011001,
        0b0000011100000000,
    ],
    [
        // "8"
        0b0011101001000101,
        0b0011101000000000,
    ],
    [
        // "9"
        0b0010011001001001,
        0b0011111000000000,
    ],
    [
        // ":"
        0b0000000000100100,
        0b0000000000000000,
    ],
    [
        // ";"
        0b0100000000100100,
        0b0000000000000000,
    ],
    [
        // "<"
        0b0000100000010100,
        0b0010001000000000,
    ],
    [
        // "="
        0b0001010000010100,
        0b0001010000000000,
    ],
    [
        // ">"
        0b0010001000010100,
        0b0000100000000000,
    ],
    [
        // "?"
        0b0000001001011001,
        0b0000011000000000,
    ],
    [
        // "@"
        0b0011111001001101,
        0b0010111000000000,
    ],
    [
        // "A"
        0b0111111000001001,
        0b0111111000000000,
    ],
    [
        // "B"
        0b0111111101001001,
        0b0011011000000000,
    ],
    [
        // "C"
        0b0011111001000001,
        0b0010001000000000,
    ],
    [
        // "D"
        0b0111111101000001,
        0b0011111000000000,
    ],
    [
        // "E"
        0b0111111101001001,
        0b0100000100000000,
    ],
    [
        // "F"
        0b0111111100001001,
        0b0000000100000000,
    ],
    [
        // "G"
        0b0011111001000001,
        0b0111001000000000,
    ],
    [
        // "H"
        0b0111111100001000,
        0b0111111100000000,
    ],
    [
        // "I"
        0b0100000101111111,
        0b0100000100000000,
    ],
    [
        // "J"
        0b0011000001000000,
        0b0011111100000000,
    ],
    [
        // "K"
        0b0111111100001000,
        0b0111011100000000,
    ],
    [
        // "L"
        0b0111111101000000,
        0b0100000000000000,
    ],
    [
        // "M"
        0b0111111100000110,
        0b0111111100000000,
    ],
    [
        // "N"
        0b0111111100000001,
        0b0111111000000000,
    ],
    [
        // "O"
        0b0011111001000001,
        0b0011111000000000,
    ],
    [
        // "P"
        0b0111111100001001,
        0b0000011000000000,
    ],
    [
        // "Q"
        0b0011111001110001,
        0b0111111000000000,
    ],
    [
        // "R"
        0b0111111100001001,
        0b0111011000000000,
    ],
    [
        // "S"
        0b0010011001001001,
        0b0011001000000000,
    ],
    [
        // "T"
        0b0000000101111111,
        0b0000000100000000,
    ],
    [
        // "U"
        0b0111111101000000,
        0b0111111100000000,
    ],
    [
        // "V"
        0b0001111101100000,
        0b0001111100000000,
    ],
    [
        // "W"
        0b0111111100110000,
        0b0111111100000000,
    ],
    [
        // "X"
        0b0111011100001000,
        0b0111011100000000,
    ],
    [
        // "Y"
        0b0000011101111000,
        0b0000011100000000,
    ],
    [
        // "Z"
        0b0111000101001001,
        0b0100011100000000,
    ],
    [
        // "["
        0b0111111101000001,
        0b0000000000000000,
    ],
    [
        // "\"
        0b0000001100011100,
        0b0110000000000000,
    ],
    [
        // "]"
        0b0000000001000001,
        0b0111111100000000,
    ],
    [
        // "^"
        0b0000001000000001,
        0b0000001000000000,
    ],
    [
        // "_"
        0b0100000001000000,
        0b0100000000000000,
    ],
    [
        // "`"
        0b0000000000000001,
        0b0000001000000000,
    ],
    [
        // "a"
        0b0010010001010100,
        0b0111100000000000,
    ],
    [
        // "b"
        0b0111111101001000,
        0b0011000000000000,
    ],
    [
        // "c"
        0b0011100001000100,
        0b0010100000000000,
    ],
    [
        // "d"
        0b0011000001001000,
        0b0111111000000000,
    ],
    [
        // "e"
        0b0011100001010100,
        0b0101100000000000,
    ],
    [
        // "f"
        0b0000100001111110,
        0b0000100100000000,
    ],
    [
        // "g"
        0b0100100001010100,
        0b0011110000000000,
    ],
    [
        // "h"
        0b0111111000001000,
        0b0111100000000000,
    ],
    [
        // "i"
        0b0100010001111101,
        0b0100000000000000,
    ],
    [
        // "j"
        0b0010000001000000,
        0b0011110100000000,
    ],
    [
        // "k"
        0b0111111100010000,
        0b0110110000000000,
    ],
    [
        // "l"
        0b0100000101111111,
        0b0100000000000000,
    ],
    [
        // "m"
        0b0111110000011000,
        0b0111110000000000,
    ],
    [
        // "n"
        0b0111110000000100,
        0b0111100000000000,
    ],
    [
        // "o"
        0b0011000001001000,
        0b0011000000000000,
    ],
    [
        // "p"
        0b0111110000010100,
        0b0000100000000000,
    ],
    [
        // "q"
        0b0000100000010100,
        0b0111110000000000,
    ],
    [
        // "r"
        0b0111110000000100,
        0b0000100000000000,
    ],
    [
        // "s"
        0b0100100001010100,
        0b0010010000000000,
    ],
    [
        // "t"
        0b0000010000111110,
        0b0100010000000000,
    ],
    [
        // "u"
        0b0011110001000000,
        0b0111110000000000,
    ],
    [
        // "v"
        0b0001110001100000,
        0b0001110000000000,
    ],
    [
        // "w"
        0b0111110000110000,
        0b0111110000000000,
    ],
    [
        // "x"
        0b0110110000010000,
        0b0110110000000000,
    ],
    [
        // "y"
        0b0100110001010000,
        0b0011110000000000,
    ],
    [
        // "z"
        0b0110010001010100,
        0b0100110000000000,
    ],
    [
        // "{"
        0b0000100000110110,
        0b0100000100000000,
    ],
    [
        // "|"
        0b0000000001111111,
        0b0000000000000000,
    ],
    [
        // "}"
        0b0100000100110110,
        0b0000100000000000,
    ],
    [
        // "~"
        0b0000001000000011,
        0b0000000100000000,
    ],
];


/// Data for configuring the displayed contents of a single cell on the monitor.
pub struct CellConfig {
    row: u16,
    column: u16,
    fg_color_idx: u16,
    bg_color_idx: u16,
    char_idx: u16,
}


pub struct Lem {
    aabb: AABB,
    screen: PixelQuad,
    blink_timer: u32,
    // TODO: Monitor's don't own the CPU or keyboard.
    dcpu: Dcpu,
    keyboard: Keyboard,
}

impl Lem {
    pub fn new(position: Point3) -> Lem {
        let s = SCREEN_SCALE / 2.0;
        let bounds = [
            Range { min: -s, max: s },
            Range { min: -s, max: s },
            Range { min: -0.05, max: 0.05 },
        ];
        let mut dcpu = Dcpu::new();
        // TODO: Make instructions lowercase (easier typing, yo).
        dcpu.load_program(&assembler::assemble(
            "\
; Initial screen coord
SET I, 0
; FG color idx
SET X, 15
; BG color idx
SET Y, 0
; Character idx
SET A, 0

; Start by initializing the cursor.
SET PC, set_blink

:loop
  IFE [0x9001], 0
  SET PC, loop

  ; Read a character.

  ; Head idx
  SET B, [0x9000]
  ; Grab element at head.
  SET A, [0x9002+B]
  ; Update head idx.
  ADD B, 1
  MOD B, 16
  SET [0x9000], B
  ; Decrement size.
  SUB [0x9001], 1

  ; Check for a backspace.
  IFE A, 0x10
  SET PC, del_char
  ; Make sure it's in the range [A-Z].
  IFL A, 0x20
  SET PC, loop
  IFG A, 0x7f
  SET PC, loop
  SUB A, 0x20

  :add_char
    ; Build monitor cell bits.
    SET J, 0
    BOR J, A
    SHL X, 12
    BOR J, X
    SHR X, 12
    SHL Y, 8
    BOR J, Y
    SHR Y, 8

    SET [0x8000+I], J
    ADD I, 1
    SET PC, set_blink

  :del_char
    IFE I, 0
    SET PC, loop
    SET [0x8000+I], 0
    SUB I, 1
    SET PC, set_blink

  :set_blink
    ; Make the cursor blink in its new position.
    SET J, 1
    SHL J, 7
    SET [0x8000+I], J
    SET PC, loop

; Do not call this subroutine if the keyboard buffer is empty.
;
; Result is placed in `A`.
;:pop_key
;    ;SET PUSH, B
;    SET B, [0x9000]
;    SET A, [0x9000+B]
;    SUB [0x9000], 1
;    ;SET B, POP
;    SET PC, POP

; I: Screen idx
; J: Char idx
; X: FG color idx
; Y: BG color idx
;:set_char
;SET A, 0
;BOR A, J
;SHL X, 12
;BOR A, X
;SHR X, 12
;SHL Y, 8
;BOR A, Y
;SHR Y, 8
            "
        ).unwrap());

        Lem {
            aabb: AABB::new(bounds, position),
            screen: PixelQuad::new(
                (SCREEN_SIZE_IN_PIXELS.0 as u32, SCREEN_SIZE_IN_PIXELS.1 as u32),
                SCREEN_SCALE, position),
            blink_timer: 0,
            dcpu,
            keyboard: Keyboard::new(),
        }
    }

    pub fn set_cell(&mut self, cell_config: CellConfig) {
        let CellConfig {
            row, column, fg_color_idx, bg_color_idx, char_idx,
        } = cell_config;

        assert!(row < SCREEN_SIZE_IN_CELLS.1);
        assert!(column < SCREEN_SIZE_IN_CELLS.0);
        assert!((fg_color_idx as usize) < COLOR_PALETTE.len());
        assert!((bg_color_idx as usize) < COLOR_PALETTE.len());
        assert!((char_idx as usize) < FONT.len());

        let fg_color = COLOR_PALETTE[fg_color_idx as usize];
        let bg_color = COLOR_PALETTE[bg_color_idx as usize];

        // Screen coordinates start from the top, unlike `PixelQuad`s, so we flip the cell
        // coords.
        let adjusted_row = SCREEN_SIZE_IN_CELLS.1 - row - 1;
        // The very first index of the cell.
        let row_offs = adjusted_row * SCREEN_SIZE_IN_PIXELS.0 * CELL_SIZE_IN_PIXELS.1;
        let col_offs = column * CELL_SIZE_IN_PIXELS.0;
        let cell_base_idx = (4 * (row_offs + col_offs)) as usize;

        for i in 0..CELL_SIZE_IN_PIXELS.1 {
            // Gotta also flip the pixel coords.
            let adjusted_i = CELL_SIZE_IN_PIXELS.1 - i - 1;
            for j in 0..CELL_SIZE_IN_PIXELS.0 {
                // An individual pixel index within the cell.
                let pixel_idx = (adjusted_i * 4) * SCREEN_SIZE_IN_PIXELS.0;
                let pixel_idx = pixel_idx + (j * 4);
                let pixel_idx = pixel_idx as usize;

                let color = if self.is_font_pixel(i, j, char_idx) {
                    fg_color
                } else {
                    bg_color
                };

                let pixels = self.screen.pixels();
                for k in 0..3 {
                    pixels[cell_base_idx + pixel_idx + k] = color[k];
                }
                pixels[cell_base_idx + pixel_idx + 3] = 255;
            }
        }
    }

    /// Returns true if `i` and `j` correspond to a `1` in the character at `char_idx` for
    /// the currently mapped font.
    ///
    /// # Arguments
    ///
    /// `i` - The row of the pixel in the character.
    /// `j` - The column of the pixel in the character.
    fn is_font_pixel(&self, i: u16, j: u16, char_idx: u16) -> bool {
        let mut j = j;
        let word_index;

        if j <= 1 {  // It's in the first font word.
            word_index = 0;
        } else {  // It's in the second font word.
            j -= 2;
            word_index = 1;
        }

        // Since we're shifting right, and the higher-order octet of a single font word
        // contains the first column, we use `1 - j` to process the higher-order octet
        // first.
        let shift_by = (1 - j) * CELL_SIZE_IN_PIXELS.1 + i;
        (FONT[char_idx as usize][word_index] >> shift_by) & 0x1 == 1
    }

    fn decode_cell_word(word: u16) -> (u16, u16, bool, u16) {
        let fg_col_idx = word >> 12;
        let bg_col_idx = (word >> 8) & 0b1111;
        let blink = ((word >> 7) & 0b1) == 1;
        let char_idx = word & 0b1111111;
        (fg_col_idx, bg_col_idx, blink, char_idx)
    }
}

impl Render for Lem {
    fn render(&mut self, context: &mut RenderingContext) {
        context.push_shader_state();
        context.bind_shader(String::from("unlit"));
        self.screen.render(context);
        context.pop_shader_state();
    }
}

impl Collide for Lem {
    fn aabb(&self) -> &AABB {
        &self.aabb
    }
}

impl Entity for Lem {
    fn tick(&mut self, event_handler: &EventHandler, _: &Vec<Box<Collide>>,
            _: EntitySlice) {
        if self.keyboard.focused() {
            keyboard::try_push_key(&mut self.dcpu, event_handler);
        }

        for _ in 0..4 {
            self.dcpu.tick();
        }

        // TODO: Don't hardcode mapping to 0x8000.
        let should_blink = self.blink_timer < (BLINK_CYCLE_LENGTH / 2);
        let start_addr: u16 = 0x8000;
        let end: u16 = (SCREEN_SIZE_IN_CELLS.0 * SCREEN_SIZE_IN_CELLS.1) as u16;
        for i in 0..end {
            let (fg_col_idx, mut bg_col_idx, blinkable, char_idx) =
                Self::decode_cell_word(self.dcpu.mem(start_addr + i));
            if blinkable && should_blink {
                // If we're blinking, then put the background color as far apart as it can
                // be from its actual color.
                bg_col_idx = (bg_col_idx + (COLOR_PALETTE.len() as u16 / 2)) %
                    COLOR_PALETTE.len() as u16;
            }
            self.set_cell(CellConfig {
                row: i / SCREEN_SIZE_IN_CELLS.0,
                column: i % SCREEN_SIZE_IN_CELLS.0,
                fg_color_idx: fg_col_idx,
                bg_color_idx: bg_col_idx,
                char_idx,
            });
        }
        self.screen.update();

        self.blink_timer = (self.blink_timer + 1) % BLINK_CYCLE_LENGTH;
    }

    fn interactable(&self) -> bool {
        true
    }

    fn interact(&mut self) {
        self.keyboard.set_focused(true);
    }

    fn stop_interact(&mut self) {
        self.keyboard.set_focused(false);
    }
}
