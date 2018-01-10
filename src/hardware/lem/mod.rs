use entity::Entity;
use game::event_handler::EventHandler;
use graphics::Render;
use graphics::renderer::RenderingContext;
use graphics::mesh::pixel_quad::PixelQuad;
use hardware::dcpu::Dcpu;
use hardware::dcpu::assembler;
use util::collide::{AABB, Range};
use util::collide::Collide;
use util::math::Point3;
use world::EntitySlice;

const SCREEN_SIZE_IN_PIXELS: (u16, u16) = (128, 96);
const SCREEN_SIZE_IN_CELLS: (u16, u16) = (32, 12);
const CELL_SIZE_IN_PIXELS: (u16, u16) = (4, 8);

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
const FONT: [[u16; 2]; 27] = [
    [
        // " "
        0b0000000000000000,
        0b0000000000000000,
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
        0b0111111101000001,
        0b0100000100000000,
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
        0b0000100100000000,
    ],
    [
        // "G"
        0b0111111101000001,
        0b0111000100000000,
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
        0b0111111100000100,
        0b0111101100000000,
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
        0b0111111100011100,
        0b0111111100000000,
    ],
    [
        // "O"
        0b0111111101000001,
        0b0111111100000000,
    ],
    [
        // "P"
        0b0111111100001001,
        0b0000011000000000,
    ],
    [
        // "Q"
        // TODO: Look better.
        0b0011111100110001,
        0b0111111100000000,
    ],
    [
        // "R"
        0b0111111100000101,
        0b0111101000000000,
    ],
    [
        // "S"
        0b0100011001001001,
        0b0011000100000000,
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
        0b0110001100011100,
        0b0110001100000000,
    ],
    [
        // "Y"
        0b0000011101111000,
        0b0000011100000000,
    ],
    [
        // "Z"
        0b0110000101011101,
        0b0100001100000000,
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
    // TODO: Monitor's don't own the CPU.
    dcpu: Dcpu,
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
        dcpu.load_program(&assembler::assemble(
            "\
SET I, 0  ; Screen index
SET J, 0  ; Char index
SET X, 0  ; FG color index
SET Y, 15  ; BG color index

:loop
SET A, 0
BOR A, J
SHL X, 12
BOR A, X
SHR X, 12
SHL Y, 8
BOR A, Y
SHR Y, 8

SET [0x8000+I], A

ADD I, 1
ADD J, 1
ADD X, 1
SUB Y, 1

MOD I, 386
MOD J, 27
MOD X, 16
IFE Y, 0
SET Y, 16

SET PC, loop
            "
        ).unwrap());

        Lem {
            aabb: AABB::new(bounds, position),
            screen: PixelQuad::new(
                (SCREEN_SIZE_IN_PIXELS.0 as u32, SCREEN_SIZE_IN_PIXELS.1 as u32),
                SCREEN_SCALE, position),
            dcpu,
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
    fn tick(&mut self, _: &EventHandler, _: &Vec<Box<Collide>>, _: EntitySlice) {
        self.dcpu.tick();

        let start_addr: u16 = 0x8000;
        let end: u16 = (SCREEN_SIZE_IN_CELLS.0 * SCREEN_SIZE_IN_CELLS.1) as u16;
        for i in 0..end {
            let (fg_col_idx, bg_col_idx, blink, char_idx) = Self::decode_cell_word(
                self.dcpu.mem(start_addr + i));
            self.set_cell(CellConfig {
                row: i / SCREEN_SIZE_IN_CELLS.0,
                column: i % SCREEN_SIZE_IN_CELLS.0,
                fg_color_idx: fg_col_idx,
                bg_color_idx: bg_col_idx,
                char_idx,
            });
        }
        // TODO: Don't hardcode mapping to 0x8000.
        // TODO: Blinking characters if B bit is set.
        self.screen.update();
    }
}
