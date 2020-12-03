mod cp437 {
    const CP437_TABLE: [char; 256] = 
    ['\0', '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼',
      '►', '◄', '↕', '‼', '¶', '§', '▬', '↨', '↑', '↓', '→', '←', '∟', '↔', '▲', '▼',
      ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/',
      '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?',
      '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
      'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_',
      '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
      'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '⌂',
      'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
      'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ',
      'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»',
      '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
      '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
      '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
      'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
      '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{A0}'];

    pub fn encode_char(character: char) -> Option<u8> {
        let code_point = character as usize;
        // Take the fast path for characters in printable ASCII.
        if code_point >= 0x20 && code_point <= 0x7E {
            Some(code_point as u8)
        // Otherwise do a table lookup.
        } else {
            match CP437_TABLE.iter().position(|&c| c == character) {
                None => None,
                Some(value) => Some(value as u8)
            }
        }
    }

    pub fn encode_char_lossy(character: char) -> u8 {
        encode_char(character).unwrap_or(0xFE) // Use ■ as a replacement character.
    }
}

use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black      = 0x0,
    Blue       = 0x1,
    Green      = 0x2,
    Cyan       = 0x3,
    Red        = 0x4,
    Magenta    = 0x5,
    Brown      = 0x6,
    LightGrey  = 0x7,
    DarkGrey   = 0x8,
    LightBlue  = 0x9,
    LightGreen = 0xA,
    LightCyan  = 0xB,
    LightRed   = 0xC,
    Pink       = 0xD,
    Yellow     = 0xE,
    White      = 0xF
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8);

impl ColourCode {
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct BufferEntry {
    cp437_code: u8,
    colours: ColourCode
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]
struct Buffer {
    entries: [[Volatile<BufferEntry>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct VgaConsole {
    active_column: usize,
    active_colours: ColourCode,
    buffer: &'static mut Buffer
}

impl VgaConsole {
    pub fn write_cp437_character(&mut self, cp437_code: u8) {
        if self.active_column >= BUFFER_WIDTH {
            self.new_line();
        }
        let row = BUFFER_HEIGHT - 1;
        let column = self.active_column;
        let colours = self.active_colours;
        self.buffer.entries[row][column].write(BufferEntry {
            cp437_code,
            colours
        });
        self.active_column += 1;
    }

    pub fn write_char(&mut self, character: char) {
        match character {
            '\n' => self.new_line(),
            ch => self.write_cp437_character(cp437::encode_char_lossy(ch))
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for ch in s.chars() {
            self.write_char(ch);
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH {
                let entry = self.buffer.entries[row][column].read();
                self.buffer.entries[row - 1][column].write(entry);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.active_column = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = BufferEntry {
            cp437_code: 0,
            colours: self.active_colours
        };
        for column in 0..BUFFER_WIDTH {
            self.buffer.entries[row][column].write(blank);
        }
    }
 }

impl core::fmt::Write for VgaConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl crate::console::Console for VgaConsole {}

lazy_static! {
    pub static ref VGA_CONSOLE: Mutex<VgaConsole> = Mutex::new(VgaConsole {
        active_column: 0,
        active_colours: ColourCode::new(Colour::Yellow, Colour::Black),
        buffer: unsafe { &mut *(0xB8000 as *mut Buffer ) }
    });
}
