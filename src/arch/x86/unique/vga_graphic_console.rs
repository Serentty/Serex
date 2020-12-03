use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

const BUFFER_WIDTH: usize = 320;
const BUFFER_HEIGHT: usize = 200;
const UNIFONT_WIDTH: usize = 4096;
const UNIFONT_HEIGHT: usize = 4096;
const UNIFONT_GLYPH_WIDTH: usize = 16;
const UNIFONT_GLYPH_HEIGHT: usize = 16;
const UNIFONT_GLYPHS_PER_ROW: usize = UNIFONT_WIDTH / UNIFONT_GLYPH_WIDTH;
const BUFFER_ROWS: usize = BUFFER_HEIGHT / UNIFONT_GLYPH_HEIGHT;
const BUFFER_COLUMNS: usize = BUFFER_WIDTH / UNIFONT_GLYPH_WIDTH;

static UNIFONT_BMP: &[u8; 16777216] = include_bytes!("../../../../Resources/Unifont BMP.data");



#[repr(transparent)]
struct Buffer {
    entries: [[Volatile<u8>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

impl Buffer {
    fn write_glyph_at(&mut self, mut index: usize, foreground_colour: u8, background_colour: u8, x: usize, y: usize) {
        if index > 0xFFFF {
            index = 0xFFFD; // For now, replace characters above hte BMP with
                            // a replacment charcter.
        }
        let unifont_row = index / UNIFONT_GLYPHS_PER_ROW;
        let unifont_column = index % UNIFONT_GLYPHS_PER_ROW;
        let glyph_start_y = unifont_row * UNIFONT_GLYPH_HEIGHT;
        let glyph_start_x = unifont_column * UNIFONT_GLYPH_WIDTH;
        for row in 0..UNIFONT_GLYPH_HEIGHT {
            let buffer_glyph_line = y * UNIFONT_GLYPH_HEIGHT + row;
            let buffer_glyph_column_start = x * UNIFONT_GLYPH_WIDTH;
            for column in 0..UNIFONT_GLYPH_WIDTH {
                self.entries[buffer_glyph_line][buffer_glyph_column_start + column].write(
                    if UNIFONT_BMP[(glyph_start_y + row) * UNIFONT_WIDTH + (glyph_start_x + column)] != 0 {
                        foreground_colour
                    } else {
                        background_colour
                    }
                );
            }
        }
    }
}

pub struct VgaGraphicConsole {
    active_column: usize,
    foreground_colour: u8,
    background_colour: u8,
    buffer: &'static mut Buffer
}

impl VgaGraphicConsole {
    fn write_glyph(&mut self, ch: char) {
        if self.active_column >= BUFFER_COLUMNS {
            self.new_line();
        }
        let row = BUFFER_ROWS - 1;
        let column = self.active_column;
        self.buffer.write_glyph_at(ch as usize, self.foreground_colour, self.background_colour, column, row);
        self.active_column += 1;
    }

    pub fn write_char(&mut self, character: char) {
        match character {
            '\n' => self.new_line(),
            ch => self.write_glyph(ch)
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for ch in s.chars() {
            self.write_char(ch);
        }
    }

    fn new_line(&mut self) {
        for row in (1 * UNIFONT_GLYPH_HEIGHT)..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH {
                let entry = self.buffer.entries[row][column].read();
                self.buffer.entries[row - UNIFONT_GLYPH_HEIGHT][column].write(entry);
            }
        }
        self.clear_character_row(BUFFER_ROWS - 1);
        self.active_column = 0;
    }

    fn clear_character_row(&mut self, row: usize) {
        let character_row_start_line = row * UNIFONT_GLYPH_HEIGHT;
        for row in 0..UNIFONT_GLYPH_HEIGHT {
            let line = character_row_start_line + row;
            for column in 0..BUFFER_WIDTH {
                self.buffer.entries[line][column].write(self.background_colour);
            }
        }           
    }
 }

impl core::fmt::Write for VgaGraphicConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl crate::console::Console for VgaGraphicConsole {}

lazy_static! {
    pub static ref VGA_GRAPHIC_CONSOLE: Mutex<VgaGraphicConsole> = Mutex::new(VgaGraphicConsole {
        active_column: 0,
        foreground_colour: 0x0F,
        background_colour: 0x00,
        buffer: unsafe { &mut *(0xA0000 as *mut Buffer ) }
    });
}
