use crate::Spectrum;
use z80::Bus;

const VERTICAL_RETRACE: usize = 16;
const BORDER_TOP: usize = 48;
const BORDER_BOTTOM: usize = 56;
const BORDER_LEFT: usize = 48;
const BORDER_RIGHT: usize = 48;
const ACTIVE_DISPLAY_AREA_WIDTH: usize = 256;
const ACTIVE_DISPLAY_AREA_HEIGHT: usize = 192;

pub const WINDOW_WIDTH: usize = BORDER_LEFT + ACTIVE_DISPLAY_AREA_WIDTH + BORDER_RIGHT;
pub const WINDOW_HEIGHT: usize = BORDER_TOP + ACTIVE_DISPLAY_AREA_HEIGHT + BORDER_BOTTOM;
const CYCLES_PER_SCANLINE: usize = 224;
pub const CYCLES_PER_FRAME: usize = (WINDOW_HEIGHT + VERTICAL_RETRACE) * CYCLES_PER_SCANLINE;

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
    fn get_color(&self, bright: bool) -> u32 {
        if bright {
            self.get_bright_color()
        } else {
            self.get_normal_color()
        }
    }

    fn get_normal_color(&self) -> u32 {
        match self {
            Color::Black => 0x000000FF,
            Color::Blue => 0x0000CDFF,
            Color::Red => 0xCD0000FF,
            Color::Magenta => 0xCD00CDFF,
            Color::Green => 0x00CD00FF,
            Color::Cyan => 0x00CDCDFF,
            Color::Yellow => 0xCDCD00FF,
            Color::White => 0xCDCDCDFF,
        }
    }

    fn get_bright_color(&self) -> u32 {
        match self {
            Color::Black => 0x000000FF,
            Color::Blue => 0x0000FFFF,
            Color::Red => 0xFF0000FF,
            Color::Magenta => 0xFF00FFFF,
            Color::Green => 0x00FF00FF,
            Color::Cyan => 0x00FFFFFF,
            Color::Yellow => 0xFFFF00FF,
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

    pub fn render(&mut self, buffer: &mut [u32], cycles: u64, bus: &Spectrum) -> bool {
        let mut current_line = self.cycles as usize / CYCLES_PER_SCANLINE;
        self.cycles += cycles;
        let frame_completed = self.cycles >= CYCLES_PER_FRAME as u64;
        if frame_completed {
            self.cycles -= CYCLES_PER_FRAME as u64;
            self.flash_counter = (self.flash_counter + 1) % 32;
        }

        let next_line = self.cycles as usize / CYCLES_PER_SCANLINE;
        while current_line != next_line {
            let window_line = current_line.checked_sub(VERTICAL_RETRACE);
            current_line = (current_line + 1) % (WINDOW_HEIGHT + VERTICAL_RETRACE);
            if let Some(y) = window_line {
                self.draw_line(buffer, bus, y);
            }
        }

        frame_completed
    }

    #[inline(always)]
    fn draw_line(&mut self, buffer: &mut [u32], bus: &Spectrum, y: usize) {
        let border_color = Color::from(bus.border_color()).get_normal_color();
        for x in (0..BORDER_LEFT)
            .into_iter()
            .chain((0 + BORDER_LEFT + ACTIVE_DISPLAY_AREA_WIDTH..WINDOW_WIDTH).into_iter())
        {
            buffer[y * WINDOW_WIDTH + x] = border_color;
        }

        if y < BORDER_TOP || y >= BORDER_TOP + ACTIVE_DISPLAY_AREA_HEIGHT {
            for x in BORDER_LEFT..BORDER_LEFT + ACTIVE_DISPLAY_AREA_WIDTH {
                buffer[y * WINDOW_WIDTH + x] = border_color;
            }
            return;
        }

        for x in (BORDER_LEFT..BORDER_LEFT + ACTIVE_DISPLAY_AREA_WIDTH).step_by(8) {
            let area_y = y - BORDER_TOP;
            let area_x = x - BORDER_LEFT;

            let addr = 0x4000
                + ((area_y & 0xc0) << 5)
                + ((area_y & 0x38) << 2)
                + ((area_y & 0x07) << 8)
                + ((area_x & 0xF8) >> 3);
            let attr_addr = 0x5800 + ((area_y >> 3) << 5) + (area_x >> 3);

            let pixel_byte = bus.read(addr as u16);
            let attr = bus.read(attr_addr as u16);

            let bright = (attr & 0x40) != 0;
            let flash = (attr & 0x80) != 0;
            let flash_active = flash && self.flash_counter >= 16;

            let mut ink_color = Color::from(attr & 0x07).get_color(bright);
            let mut paper_color = Color::from((attr >> 3) & 0x07).get_color(bright);

            if flash_active {
                std::mem::swap(&mut ink_color, &mut paper_color);
            }

            for bit in 0..8 {
                let pixel_bit = (pixel_byte >> (7 - bit)) & 1;
                let color = if pixel_bit == 1 {
                    ink_color
                } else {
                    paper_color
                };

                buffer[y * WINDOW_WIDTH + x + bit] = color;
            }
        }
    }
}
