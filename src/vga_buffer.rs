use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// This tells the compiler that not every variant of the enum
// needs to be used in code somewhere.
#[allow(dead_code)]

// Like Haskell, the compiler can be told to provide
// default implmentations for the follwing traits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

// This makes the compiler store each variant in a u8;
// there is no u4 available in Rust so u8 is the smallest we have.
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// Same traits as above
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

// Tells the compiler that the data layout for ColorCode
// is the same as that of its single field, in this case u8
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

// Tells the compiler to layout the order of the fields
// like a C struct
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// Using the Volatile generic type tells the compiler
// that writing to this buffer involves side effects,
// in this case the screen, and should not be optimized away
// because we only write to it and never read from it.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

// This exposes a `Writer` for any other module to easily use
// without having to instantiate an object and keep a reference to it.
// But we need to use the `lazy_static!` macro to allow us
// to initialize `WRITER` at _runtime_ not compile time.
lazy_static! {
    // Wrapping our writer in a `Mutex` allows to both 
    // expose a static instance and keep the compiler
    // happy by guaranteeing synchronized access to the writer
    // _and_ allow for local mutation of its state.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

impl Writer {
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                // Note that this is leveraging the Volatile type/interface
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
}

// This is how we mix in `Write` trait for our Writer,
// by implementing the only function, `write_str`,
// so that we can take advantage of Rust's `write!` and 
// `writeln!` macros their formatting capabiities.
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
