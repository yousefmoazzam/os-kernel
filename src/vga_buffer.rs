use core::fmt;

use spin::{Lazy, Mutex};
use volatile::Volatile;

/// Memory address where the VGA buffer starts
static VGA_BUFFER_START: u32 = 0xB8000;

/// Available colours for displaying a character in the VGA text buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Colour (second) byte of an element in the VGA text buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8);

impl ColourCode {
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

/// Element in the VGA text buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenCharacter {
    ascii: u8,
    colour_code: ColourCode,
}

/// Columns in the VGA text buffer
const BUFFER_WIDTH: usize = 80;
/// Rows in the VGA text buffer
const BUFFER_HEIGHT: usize = 25;

/// VGA text buffer
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Byte to print when value is not in ASCII range
const UNPRINTABLE_BYTE_SUBSTITUTE: u8 = 0xFE;

/// Writer to VGA text buffer
pub struct Writer {
    column_position: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Writer {
    /// Write byte to buffer
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.write_new_line(),
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.write_new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                self.buffer.chars[row][self.column_position].write(ScreenCharacter {
                    ascii: byte,
                    colour_code: self.colour_code,
                });
                self.column_position += 1;
            }
        }
    }

    /// Write newline character to buffer
    pub fn write_new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Replace all characters in a row with the space character
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenCharacter {
            ascii: b' ',
            colour_code: self.colour_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    /// Write string to buffer
    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7E | b'\n' => self.write_byte(byte),
                _ => self.write_byte(UNPRINTABLE_BYTE_SUBSTITUTE),
            }
        }
    }
}

pub static WRITER: Lazy<Mutex<Writer>> = Lazy::new(|| {
    Mutex::new(Writer {
        column_position: 0,
        colour_code: ColourCode::new(Colour::LightCyan, Colour::Black),
        buffer: unsafe { &mut *(VGA_BUFFER_START as *mut Buffer) },
    })
});
