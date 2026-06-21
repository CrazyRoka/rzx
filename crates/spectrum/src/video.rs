use z80::Bus;

use crate::Spectrum16k;

const BORDER_TOP: usize = 48;
const BORDER_BOTTOM: usize = 56;
const BORDER_LEFT: usize = 48;
const BORDER_RIGHT: usize = 48;
const ACTIVE_DISPLAY_AREA_WIDTH: usize = 256;
const ACTIVE_DISPLAY_AREA_HEIGHT: usize = 192;

pub const CYCLES_PER_FRAME: usize = 69888;
pub const WINDOW_WIDTH: usize = BORDER_LEFT + ACTIVE_DISPLAY_AREA_WIDTH + BORDER_RIGHT;
pub const WINDOW_HEIGHT: usize = BORDER_TOP + ACTIVE_DISPLAY_AREA_HEIGHT + BORDER_BOTTOM;

enum Color {
    Black,
    Blue,
    Red,
    Magenta,
    Green,
    Cyan,
    Yellow,
    White,
}

impl Color {
    fn get_normal_color(&self) -> u32 {
        match self {
            Color::Black => 0xFF000000,
            Color::Blue => 0xFF0000CD,
            Color::Red => 0xFFCD0000,
            Color::Magenta => 0xFFCD00CD,
            Color::Green => 0xFF00CD00,
            Color::Cyan => 0xFF00CDCD,
            Color::Yellow => 0xFFCDCD00,
            Color::White => 0xFFCDCDCD,
        }
    }

    fn get_bright_color(&self) -> u32 {
        match self {
            Color::Black => 0xFF000000,
            Color::Blue => 0xFF0000FF,
            Color::Red => 0xFFFF0000,
            Color::Magenta => 0xFFFF00FF,
            Color::Green => 0xFF00FF00,
            Color::Cyan => 0xFF00FFFF,
            Color::Yellow => 0xFFFFFF00,
            Color::White => 0xFFFFFFFF,
        }
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Black,
            1 => Self::Blue,
            2 => Self::Red,
            3 => Self::Magenta,
            4 => Self::Green,
            5 => Self::Cyan,
            6 => Self::Yellow,
            7 => Self::White,
            _ => panic!("Unexpected color value {value}"),
        }
    }
}

pub struct ULA {
    cycles: u64,
    flash_counter: u8,
}

impl ULA {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            flash_counter: 0,
        }
    }

    pub fn render(&mut self, buffer: &mut [u32], cycles: u64, bus: &Spectrum16k) -> bool {
        self.cycles += cycles;
        if self.cycles < CYCLES_PER_FRAME as u64 {
            return false;
        }

        self.cycles -= CYCLES_PER_FRAME as u64;
        for y in 0..WINDOW_HEIGHT {
            for x in 0..WINDOW_WIDTH {
                if y < BORDER_TOP
                    || x < BORDER_LEFT
                    || y >= BORDER_TOP + ACTIVE_DISPLAY_AREA_HEIGHT
                    || x >= BORDER_LEFT + ACTIVE_DISPLAY_AREA_WIDTH
                {
                    let color: Color = Color::from(bus.border_color());
                    buffer[y * WINDOW_WIDTH + x] = color.get_normal_color();
                } else {
                    let area_y = y - BORDER_TOP;
                    let area_x = x - BORDER_LEFT;

                    let addr = 0x4000
                        + ((area_y & 0xc0) << 5)
                        + ((area_y & 0x38) << 2)
                        + ((area_y & 0x07) << 8)
                        + ((area_x & 0xF8) >> 3);
                    let pixel_byte = bus.read(addr as u16);
                    let pixel_bit = (pixel_byte >> (7 - (x & 0x07))) & 1;

                    let attr_addr = 0x5800 + ((area_y >> 3) << 5) + (area_x >> 3);
                    let attr = bus.read(attr_addr as u16);
                    let bright = (attr & 0x40) != 0;
                    let flash = (attr & 0x80) != 0;
                    let flash_active = flash && self.flash_counter >= 16;
                    let ink = (pixel_bit == 1) ^ flash_active;

                    let color_raw = if ink { attr & 0x07 } else { (attr >> 3) & 0x07 };
                    let color = Color::from(color_raw);
                    let rgba = if bright {
                        color.get_bright_color()
                    } else {
                        color.get_normal_color()
                    };

                    buffer[y * WINDOW_WIDTH + x] = rgba;
                }
            }
        }

        self.flash_counter = (self.flash_counter + 1) % 32;
        true
    }
}
