use core::ptr::null_mut;
use core::mem::transmute;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

// This must be initialized with Multiboot data in boot_entry_point.rs before use.
pub static mut BUFFER_BASE: *mut u8 = null_mut();

pub const BUFFER_WIDTH: usize = 1024;
pub const BUFFER_HEIGHT: usize = 768;
pub const BUFFER_BPP: u8 = 32;
const UNIFONT_WIDTH: usize = 4096;
const UNIFONT_HEIGHT: usize = 8192;
const UNIFONT_GLYPH_WIDTH: usize = 16;
const UNIFONT_GLYPH_HEIGHT: usize = 16;
const UNIFONT_GLYPHS_PER_ROW: usize = UNIFONT_WIDTH / UNIFONT_GLYPH_WIDTH;
const BUFFER_ROWS: usize = BUFFER_HEIGHT / UNIFONT_GLYPH_HEIGHT;
const BUFFER_COLUMNS: usize = BUFFER_WIDTH / UNIFONT_GLYPH_WIDTH;

static UNIFONT: &[u8; (UNIFONT_WIDTH * UNIFONT_HEIGHT) / 8] = include_bytes!("../../../../Resources/Unifont.data");

type Colour = u32;

#[inline(always)]
fn test_bit(byte: u8, bit: u8) -> bool {
    let mask = 0b1000_0000 >> bit;
    byte & mask != 0
}

fn is_halfwidth(index: usize) -> bool {
    false
}

#[repr(transparent)]
struct Buffer {
    entries: [[Volatile<Colour>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

impl Buffer {
    fn write_glyph_at(&mut self, mut index: usize, foreground_colour: Colour, background_colour: Colour, x: usize, y: usize) {
        if index > 0x1FFFF {
            index = 0xFFFD; // Characters above the SMP are not covered
                            // by the font, so replace them.
                            
        }
        let unifont_row = index / UNIFONT_GLYPHS_PER_ROW;
        let unifont_column = index % UNIFONT_GLYPHS_PER_ROW;
        let glyph_start_y = unifont_row * UNIFONT_GLYPH_HEIGHT;
        let glyph_start_x = unifont_column * UNIFONT_GLYPH_WIDTH;
        for row in 0..UNIFONT_GLYPH_HEIGHT {
            let buffer_glyph_line = y * UNIFONT_GLYPH_HEIGHT + row;
            let buffer_glyph_column_start = x * UNIFONT_GLYPH_WIDTH;
            for column in 0..UNIFONT_GLYPH_WIDTH {
                let position = (glyph_start_y + row) * UNIFONT_WIDTH + (glyph_start_x + (column / if is_halfwidth(index) { 2 } else { 1 }));
                self.entries[buffer_glyph_line][buffer_glyph_column_start + column].write(                    
                    if test_bit(UNIFONT[position / 8], position as u8 % 8) {
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
    foreground_colour: Colour,
    background_colour: Colour,
    back_buffer: &'static mut Buffer,
    front_buffer: &'static mut Buffer
}

impl VgaGraphicConsole {
    fn write_glyph(&mut self, ch: char) {
        if self.active_column >= BUFFER_COLUMNS {
            self.new_line();
        }
        let row = BUFFER_ROWS - 1;
        let column = self.active_column;
        self.back_buffer.write_glyph_at(ch as usize, self.foreground_colour, self.background_colour, column, row);
        self.active_column += 1;
    }

    fn write_char(&mut self, character: char) {
        match character {
            '\n' => self.new_line(),
            ch => self.write_glyph(ch)
        }
    }

    fn write_string(&mut self, s: &str) {
        for ch in s.chars() {
            self.write_char(ch);
        }
        self.sync();
    }

    fn new_line(&mut self) {
        for row in (1 * UNIFONT_GLYPH_HEIGHT)..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH {
                let entry = self.back_buffer.entries[row][column].read();
                self.back_buffer.entries[row - UNIFONT_GLYPH_HEIGHT][column].write(entry);
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
                self.back_buffer.entries[line][column].write(self.background_colour);
            }
        }           
    }

    fn sync(&mut self) {        
        unsafe {
            asm!("rep movsb",
            in("rsi") self.back_buffer,
            in("rdi") self.front_buffer,
            in("rcx") core::mem::size_of::<Buffer>());
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

static mut BACK_BUFFER: Buffer = unsafe { transmute([[0 as Colour; BUFFER_WIDTH]; BUFFER_HEIGHT]) };

lazy_static! {
    pub static ref VGA_GRAPHIC_CONSOLE: Mutex<VgaGraphicConsole> = Mutex::new(VgaGraphicConsole {
        active_column: 0,
        foreground_colour: 0x0000FFFF,
        background_colour: 0x00000000,
        back_buffer: unsafe { &mut BACK_BUFFER },
        front_buffer: unsafe { &mut *(BUFFER_BASE as *mut Buffer ) }
    });
}
