use core::ptr::null_mut;
use core::mem::transmute;
use core::time::Duration;
use core::arch::asm;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

// This must be initialized with Multiboot data in boot_entry_point.rs before use.
pub static mut BUFFER_BASE: *mut u8 = null_mut();

pub const BUFFER_WIDTH: usize = 1024;
pub const BUFFER_HEIGHT: usize = 768;
pub const BUFFER_BPP: u8 = 32;
pub const CURSOR_BLINK_TIME_MILLISECONDS: u32 = 500;
const UNIFONT_WIDTH: usize = 4096;
const UNIFONT_HEIGHT: usize = 8192;
const UNIFONT_GLYPH_WIDTH: usize = 16;
const UNIFONT_GLYPH_HEIGHT: usize = 16;
const UNIFONT_GLYPHS_PER_ROW: usize = UNIFONT_WIDTH / UNIFONT_GLYPH_WIDTH;
const BUFFER_ROWS: usize = BUFFER_HEIGHT / UNIFONT_GLYPH_HEIGHT;
const BUFFER_COLUMNS: usize = BUFFER_WIDTH / UNIFONT_GLYPH_WIDTH;

const UNIFONT: &[u8; (UNIFONT_WIDTH * UNIFONT_HEIGHT) / 8] = include_bytes!("../../../../Resources/Unifont.data");
const HALFWIDTH_RANGES: [(u32, u32); 4] = [
    // TODO: Autogenerate an exhaustive list of halfwidth characters.
    (0x0020, 0x03FE),
    (0x2500, 0x257F), // Box Drawing
    (0x2580, 0x259F), // Block Elements
    (0x1FB00, 0x1FBFF) // Symbols for Legacy Computing
];

type Colour = u32;

#[inline(always)]
fn test_bit(byte: u8, bit: u8) -> bool {
    let mask = 0b1000_0000 >> bit;
    byte & mask != 0
}

fn is_halfwidth(index: usize) -> bool {
    let index = index as u32;
    for range in HALFWIDTH_RANGES.iter() {
        if index >= range.0 && index <= range.1 {
            return true;
        }
    }
    false
}

#[repr(transparent)]
struct Buffer {
    entries: [[Volatile<Colour>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct VgaGraphicConsole {
    active_column: usize,
    active_row: usize,
    foreground_colour: Colour,
    background_colour: Colour,
    back_buffer: &'static mut Buffer,
    front_buffer: &'static mut Buffer
}

impl VgaGraphicConsole {
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
                self.back_buffer.entries[buffer_glyph_line][buffer_glyph_column_start + column].write(                    
                    if test_bit(UNIFONT[position / 8], position as u8 % 8) {
                        foreground_colour
                    } else {
                        background_colour
                    }
                );
            }
        }
    }

    fn write_glyph(&mut self, ch: char) {
        if self.active_column >= BUFFER_COLUMNS {
            self.new_line();
        }
        let column = self.active_column;
        let row = self.active_row;
        self.write_glyph_at(ch as usize, self.foreground_colour, self.background_colour, column, row);
        self.active_column += 1;
    }

    fn write_char(&mut self, character: char) {
        match character {
            '\n' => {
                self.toggle_cursor_cell(false); // Don't allow the cursor to show up in the backlog.
                self.new_line();
            },
            '\u{08}' => self.backspace(),
            ch => self.write_glyph(ch)
        }
    }

    fn write_string(&mut self, s: &str) {
        for ch in s.chars() {
            self.write_char(ch);
        }
        self.toggle_cursor_cell(true);
        self.sync();
    }

    fn new_line(&mut self) {
        if self.active_row < BUFFER_ROWS - 1 {
            self.active_column = 0;
            self.active_row += 1;
        } else {
            self.scroll_line();
        }
    }

    fn scroll_line(&mut self) {
        let line_size = BUFFER_WIDTH * UNIFONT_GLYPH_HEIGHT * (BUFFER_BPP as usize / 4) / 2;
        unsafe {
            let buffer_start = self.back_buffer as *mut Buffer as *mut u8;
            asm!("rep movsb",
            in("rsi") buffer_start.offset(line_size as isize),
            in("rdi") buffer_start,
            in("rcx") core::mem::size_of::<Buffer>() - line_size);
        };
        self.clear_character_row(BUFFER_ROWS - 1);
        self.active_column = 0;
    }

    fn backspace(&mut self) {
        self.toggle_cursor_cell(false); // Erase the cursor if it is visible.
        if self.active_column > 0 {
            let new_column = self.active_column - 1;
            self.write_glyph_at('\u{20}' as usize, self.foreground_colour, self.background_colour, new_column, BUFFER_ROWS - 1);
            self.active_column = new_column;
        }
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

    fn toggle_cursor_cell(&mut self, status: bool) {
        let ch = if status {
            '\u{2581}' // Lower one-eighth block
        } else {
            '\u{20}' // Space
        } as usize;
        if self.active_column >= BUFFER_COLUMNS {
            self.new_line();
        }   
        let (foreground, background, column, row) =
            (self.foreground_colour, self.background_colour, self.active_column, self.active_row);
        self.write_glyph_at(
            ch,
            foreground,
            background,
            column,
            row);
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

const BLINK_HANDLER: crate::timer::Handler = crate::timer::Handler::new(
    blink_cursor,
    Duration::from_millis(250),
    true
);

fn blink_cursor() {
    let mut status = CURSOR_STATUS.lock();
    *status = !*status;
    if let Some(mut console) = VGA_GRAPHIC_CONSOLE.try_lock() {
        console.toggle_cursor_cell(*status);
        console.sync();
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
    static ref CURSOR_STATUS: Mutex<bool> = Mutex::new(false);
}

lazy_static! {
    pub static ref VGA_GRAPHIC_CONSOLE: Mutex<VgaGraphicConsole> = {
        crate::timer::register_handler(BLINK_HANDLER).ok();
        Mutex::new(VgaGraphicConsole {
            active_column: 0,
            active_row: 0,
            foreground_colour: 0x00FF9900,
            background_colour: 0x00000000,
            back_buffer: unsafe { &mut BACK_BUFFER },
            front_buffer: unsafe { &mut *(BUFFER_BASE as *mut Buffer ) }
        })
    };
}