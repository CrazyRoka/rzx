use std::{cell::RefCell, rc::Rc};

use z80::Bus;

use crate::Keyboard;

pub struct Spectrum16k {
    rom: [u8; 0x4000], // 0x0000 – 0x3FFF
    ram: [u8; 0x4000], // 0x4000 - 0x7FFF
    //IO
    keyboard: Rc<RefCell<Keyboard>>,
    // ULA
    border_color: u8,
    mic: bool,
    ear: bool,
}

impl Spectrum16k {
    pub fn new(rom: &[u8], keyboard: Rc<RefCell<Keyboard>>) -> Self {
        assert_eq!(rom.len(), 0x4000, "16K model requires a 16KB ROM");
        let mut rom_clone = [0; 0x4000];
        rom_clone.copy_from_slice(rom);

        Spectrum16k {
            rom: rom_clone,
            ram: [0xFF; 0x4000],
            keyboard,
            border_color: 0,
            mic: false,
            ear: false,
        }
    }

    pub fn border_color(&self) -> u8 {
        self.border_color
    }

    pub fn ear_state(&self) -> bool {
        self.ear
    }

    pub fn mic_state(&self) -> bool {
        self.mic
    }
}

impl Bus for Spectrum16k {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],
            0x4000..=0x7FFF => self.ram[(addr & 0x3FFF) as usize],
            0x8000..=0xFFFF => {
                dbg!("Attempt to read upper RAM area on 16K model", addr);
                0xFF
            }
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x3FFF => {
                dbg!("Attempt to overwrite ROM area", addr, value);
            }
            0x4000..=0x7FFF => self.ram[(addr & 0x3FFF) as usize] = value,
            0x8000..=0xFFFF => {
                dbg!(
                    "Attempt to overwrite upper RAM area on 16K model",
                    addr,
                    value
                );
            }
        }
    }

    fn port_read(&self, port: u16) -> u8 {
        if (port & 0xFF) != 0xFE {
            dbg!("Unexpectede port read at address {port:#04X}");
            // TODO: handle floating bus
            return 0xFF;
        }

        let mut keyboard_state = 0x1F;
        for row in 0..8 {
            if ((port >> 8) & (1 << row)) == 0 {
                keyboard_state &= self.keyboard.borrow().read_row(row);
            }
        }

        keyboard_state |= 0xA0;
        keyboard_state |= (self.ear as u8) << 6;

        keyboard_state
    }

    fn port_write(&mut self, port: u16, value: u8) {
        if (port & 0x01) == 1 {
            dbg!("Unexpected even port write", port, value);
            return;
        }

        self.border_color = value & 0x07;
        self.mic = (value & 0x08) == 0x08;
        self.ear = (value & 0x10) == 0x10;

        dbg!(
            "Received port write",
            self.border_color,
            self.mic,
            self.ear,
            port,
            value
        );

        if (port & 0xFF) != 0xFE {
            dbg!("Received unexpected port write", port, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a 16KB ROM filled with a recognisable repeating pattern.
    /// Address `i` holds `(i & 0xFF)` as a byte. This means:
    ///   0x0000 -> 0x00, 0x0001 -> 0x01, ... 0x00FF -> 0xFF, 0x0100 -> 0x00, ...
    fn make_rom() -> Vec<u8> {
        (0..0x4000).map(|i| (i & 0xFF) as u8).collect()
    }

    fn make_keyboard() -> Rc<RefCell<Keyboard>> {
        Rc::new(RefCell::new(Keyboard::new()))
    }

    fn make_spectrum16k() -> Spectrum16k {
        Spectrum16k::new(&make_rom(), make_keyboard())
    }

    // -----------------------------------------------------------------
    // Construction / validation
    // -----------------------------------------------------------------

    #[test]
    fn new_accepts_exactly_16kb_rom() {
        // Should not panic
        let _mem = Spectrum16k::new(&[0u8; 0x4000], make_keyboard());
    }

    #[test]
    #[should_panic(expected = "16K model requires a 16KB ROM")]
    fn new_rejects_rom_shorter_than_16kb() {
        let _mem = Spectrum16k::new(&[0u8; 0x3FFF], make_keyboard());
    }

    #[test]
    #[should_panic(expected = "16K model requires a 16KB ROM")]
    fn new_rejects_rom_longer_than_16kb() {
        let _mem = Spectrum16k::new(&[0u8; 0x4001], make_keyboard());
    }

    // -----------------------------------------------------------------
    // ROM reads (0x0000 ..= 0x3FFF)
    // -----------------------------------------------------------------

    #[test]
    fn rom_read_returns_loaded_bytes() {
        let mem = make_spectrum16k();
        assert_eq!(mem.read(0x0000), 0x00);
        assert_eq!(mem.read(0x0001), 0x01);
        assert_eq!(mem.read(0x00FE), 0xFE);
        assert_eq!(mem.read(0x00FF), 0xFF);
        assert_eq!(mem.read(0x0100), 0x00); // pattern wraps
        assert_eq!(mem.read(0x0101), 0x01);
        assert_eq!(mem.read(0x01FE), 0xFE);
        assert_eq!(mem.read(0x3FFF), 0xFF); // last ROM byte
    }

    #[test]
    fn rom_write_is_silently_ignored() {
        let mut mem = make_spectrum16k();
        mem.write(0x0000, 0xAA);
        mem.write(0x1000, 0xBB);
        mem.write(0x3FFF, 0xCC);
        assert_eq!(mem.read(0x0000), 0x00);
        assert_eq!(mem.read(0x1000), 0x00);
        assert_eq!(mem.read(0x3FFF), 0xFF);
    }

    // -----------------------------------------------------------------
    // RAM reads/writes (0x4000 ..= 0x7FFF)
    // -----------------------------------------------------------------

    #[test]
    fn ram_powers_up_to_0xff() {
        let mem = make_spectrum16k();
        for addr in [0x4000, 0x4001, 0x5DC0, 0x7FFE, 0x7FFF] {
            assert_eq!(
                mem.read(addr),
                0xFF,
                "RAM at 0x{:04X} not 0xFF at power-up",
                addr
            );
        }
    }

    #[test]
    fn ram_write_then_read_roundtrip() {
        let mut mem = make_spectrum16k();
        mem.write(0x4000, 0x42);
        mem.write(0x7FFF, 0x99);
        mem.write(0x5DC0, 0x12);
        assert_eq!(mem.read(0x4000), 0x42);
        assert_eq!(mem.read(0x7FFF), 0x99);
        assert_eq!(mem.read(0x5DC0), 0x12);
    }

    #[test]
    fn ram_overwrite_replaces_previous_value() {
        let mut mem = make_spectrum16k();
        for &v in &[0x01u8, 0x02, 0x00, 0xFF, 0x80] {
            mem.write(0x4000, v);
            assert_eq!(mem.read(0x4000), v);
        }
    }

    #[test]
    fn ram_addresses_are_independent() {
        let mut mem = make_spectrum16k();
        mem.write(0x4000, 0xAA);
        mem.write(0x4001, 0xBB);
        mem.write(0x7FFE, 0xCC);
        mem.write(0x7FFF, 0xDD);
        assert_eq!(mem.read(0x4000), 0xAA);
        assert_eq!(mem.read(0x4001), 0xBB);
        assert_eq!(mem.read(0x7FFE), 0xCC);
        assert_eq!(mem.read(0x7FFF), 0xDD);
    }

    #[test]
    fn ram_all_256_byte_values_round_trip() {
        let mut mem = make_spectrum16k();
        for b in 0..=255u8 {
            mem.write(0x4000, b);
            assert_eq!(mem.read(0x4000), b, "byte 0x{:02X} did not round-trip", b);
        }
    }

    #[test]
    fn ram_full_range_is_addressable() {
        // Slow-ish but catches off-by-one indexing bugs in one shot.
        let mut mem = make_spectrum16k();
        for addr in 0x4000..=0x7FFFu16 {
            mem.write(addr, (addr & 0xFF) as u8);
        }
        for addr in 0x4000..=0x7FFFu16 {
            assert_eq!(
                mem.read(addr),
                (addr & 0xFF) as u8,
                "mismatch at 0x{:04X}",
                addr
            );
        }
    }

    // -----------------------------------------------------------------
    // Unmapped region (0x8000 ..= 0xFFFF) — this is what distinguishes
    // the 16K model from the 48K model.
    // -----------------------------------------------------------------

    #[test]
    fn unmapped_read_returns_0xff() {
        let mem = make_spectrum16k();
        assert_eq!(mem.read(0x8000), 0xFF);
        assert_eq!(mem.read(0xC000), 0xFF);
        assert_eq!(mem.read(0xFFFF), 0xFF);
    }

    #[test]
    fn unmapped_write_does_not_crash_and_is_ignored() {
        let mut mem = make_spectrum16k();
        // Should not panic
        mem.write(0x8000, 0x42);
        mem.write(0xC000, 0x99);
        mem.write(0xFFFF, 0x00);
        // Reads should still return 0xFF
        assert_eq!(mem.read(0x8000), 0xFF);
        assert_eq!(mem.read(0xC000), 0xFF);
        assert_eq!(mem.read(0xFFFF), 0xFF);
    }

    // -----------------------------------------------------------------
    // Boundary tests
    // -----------------------------------------------------------------

    #[test]
    fn boundary_first_rom_byte() {
        let mem = make_spectrum16k();
        assert_eq!(mem.read(0x0000), 0x00);
    }

    #[test]
    fn boundary_last_rom_byte() {
        let mut mem = make_spectrum16k();
        assert_eq!(mem.read(0x3FFF), 0xFF);
        mem.write(0x3FFF, 0x42); // ROM — ignored
        assert_eq!(mem.read(0x3FFF), 0xFF);
    }

    #[test]
    fn boundary_first_ram_byte() {
        let mut mem = make_spectrum16k();
        assert_eq!(mem.read(0x4000), 0xFF); // power-up
        mem.write(0x4000, 0x42);
        assert_eq!(mem.read(0x4000), 0x42);
    }

    #[test]
    fn boundary_last_ram_byte() {
        let mut mem = make_spectrum16k();
        assert_eq!(mem.read(0x7FFF), 0xFF); // power-up
        mem.write(0x7FFF, 0x42);
        assert_eq!(mem.read(0x7FFF), 0x42);
    }

    #[test]
    fn boundary_first_unmapped_byte() {
        let mut mem = make_spectrum16k();
        // 0x7FFF is RAM, 0x8000 is unmapped — a single byte separates two regions.
        mem.write(0x7FFF, 0x11);
        mem.write(0x8000, 0x22); // ignored
        assert_eq!(mem.read(0x7FFF), 0x11);
        assert_eq!(mem.read(0x8000), 0xFF);
    }

    #[test]
    fn boundary_last_addressable_byte() {
        let mut mem = make_spectrum16k();
        assert_eq!(mem.read(0xFFFF), 0xFF);
        mem.write(0xFFFF, 0x42); // ignored
        assert_eq!(mem.read(0xFFFF), 0xFF);
    }

    // -----------------------------------------------------------------
    // Port 0xFE — OUT (Border color)
    // -----------------------------------------------------------------

    #[test]
    fn port_fe_write_border_color() {
        let mut mem = make_spectrum16k();
        for color in 0..=0x07 {
            mem.port_write(0xFE, color);
            assert_eq!(mem.border_color(), color);
        }
    }

    #[test]
    fn port_fe_write_border_uses_only_low_3_bits() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x08); // 0b00001000 — low 3 bits are 000
        assert_eq!(mem.border_color(), 0);
    }

    #[test]
    fn port_fe_write_border_value_255_gives_7() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0xFF);
        assert_eq!(mem.border_color(), 7);
    }

    #[test]
    fn port_fe_write_border_value_15_gives_7() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x0F); // 0b00001111
        assert_eq!(mem.border_color(), 7);
    }

    // -----------------------------------------------------------------
    // Port 0xFE — OUT (MIC bit)
    // -----------------------------------------------------------------

    #[test]
    fn port_fe_write_mic_bit_clear() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x00);
        assert!(!mem.mic_state());
    }

    #[test]
    fn port_fe_write_mic_bit_set() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x08); // bit 3
        assert!(mem.mic_state());
    }

    #[test]
    fn port_fe_write_mic_bit_toggles() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x08);
        assert!(mem.mic_state());
        mem.port_write(0xFE, 0x00);
        assert!(!mem.mic_state());
        mem.port_write(0xFE, 0x08);
        assert!(mem.mic_state());
    }

    // -----------------------------------------------------------------
    // Port 0xFE — OUT (EAR bit)
    // -----------------------------------------------------------------

    #[test]
    fn port_fe_write_ear_bit_clear() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x00);
        assert!(!mem.ear_state());
    }

    #[test]
    fn port_fe_write_ear_bit_set() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x10); // bit 4
        assert!(mem.ear_state());
    }

    #[test]
    fn port_fe_write_ear_bit_toggles() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x10);
        assert!(mem.ear_state());
        mem.port_write(0xFE, 0x00);
        assert!(!mem.ear_state());
    }

    // -----------------------------------------------------------------
    // Port 0xFE — OUT (Combined field updates)
    // -----------------------------------------------------------------

    #[test]
    fn port_fe_write_updates_all_fields_simultaneously() {
        let mut mem = make_spectrum16k();
        // border=5, mic=1, ear=1 -> 0b000110101 = 0x1D
        mem.port_write(0xFE, 0x1D);
        assert_eq!(mem.border_color(), 5);
        assert!(mem.mic_state());
        assert!(mem.ear_state());
    }

    #[test]
    fn port_fe_write_overwrites_previous_state() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x1F); // border=7, mic=1, ear=1
        mem.port_write(0xFE, 0x02); // border=2, mic=0, ear=0
        assert_eq!(mem.border_color(), 2);
        assert!(!mem.mic_state());
        assert!(!mem.ear_state());
    }

    #[test]
    fn port_fe_write_border_preserves_mic_and_ear() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x18); // border=0, mic=1, ear=1
        mem.port_write(0xFE, 0x05); // border=5, mic=0, ear=0
        // Second write clears mic and ear because all bits are written together
        assert_eq!(mem.border_color(), 5);
        assert!(!mem.mic_state());
        assert!(!mem.ear_state());
    }

    // -----------------------------------------------------------------
    // Port 0xFE — OUT (Port address handling)
    // -----------------------------------------------------------------

    #[test]
    fn port_write_even_address_0xfe_affects_ula() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x03);
        assert_eq!(mem.border_color(), 3);
    }

    #[test]
    fn port_write_even_address_0x1fe_affects_ula() {
        let mut mem = make_spectrum16k();
        mem.port_write(0x1FE, 0x05);
        assert_eq!(mem.border_color(), 5);
    }

    #[test]
    fn port_write_even_address_0x00_affects_ula() {
        let mut mem = make_spectrum16k();
        mem.port_write(0x00, 0x06);
        assert_eq!(mem.border_color(), 6);
    }

    #[test]
    fn port_write_different_even_ports_affect_same_state() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x01);
        assert_eq!(mem.border_color(), 1);
        mem.port_write(0x1FE, 0x02);
        assert_eq!(mem.border_color(), 2);
        mem.port_write(0xFEFE, 0x03); // Even port with high bits set
        assert_eq!(mem.border_color(), 3);
    }

    #[test]
    fn port_write_odd_address_does_not_affect_ula() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFF, 0x07); // Odd port
        assert_eq!(mem.border_color(), 0); // Unchanged from initial
    }

    #[test]
    fn port_write_odd_address_0xff_does_not_affect_ula() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFF, 0xFF); // Odd port, all bits set
        assert_eq!(mem.border_color(), 0);
        assert!(!mem.mic_state());
        assert!(!mem.ear_state());
    }

    // -----------------------------------------------------------------
    // Port 0xFE — IN (Basic keyboard structure)
    // -----------------------------------------------------------------

    #[test]
    fn port_fe_read_no_keys_pressed_returns_0x1f_in_low_bits() {
        let mem = make_spectrum16k();
        assert_eq!(mem.port_read(0xFEFE) & 0x1F, 0x1F);
    }

    #[test]
    fn port_fe_read_bit_5_is_always_1() {
        let mem = make_spectrum16k();
        assert_ne!(mem.port_read(0xFEFE) & 0x20, 0);
    }

    #[test]
    fn port_fe_read_bit_7_is_always_1() {
        let mem = make_spectrum16k();
        assert_ne!(mem.port_read(0xFEFE) & 0x80, 0);
    }

    #[test]
    fn port_fe_read_no_keys_returns_0xff_when_ear_high() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x10); // EAR high
        assert_eq!(mem.port_read(0xFEFE), 0xFF);
    }

    // // -----------------------------------------------------------------
    // // Port 0xFE — IN (Single key presses per row)
    // // -----------------------------------------------------------------

    // #[test]
    // fn port_fe_read_row_0_bit_0_cleared() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(0, 0b11110);
    //     assert_eq!(mem.port_read(0xFEFE) & 0x1F, 0b11110);
    // }

    // #[test]
    // fn port_fe_read_row_0_bit_1_cleared() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(0, 0b11101);
    //     assert_eq!(mem.port_read(0xFEFE) & 0x1F, 0b11101);
    // }

    // #[test]
    // fn port_fe_read_row_0_bit_4_cleared() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(0, 0b01111);
    //     assert_eq!(mem.port_read(0xFEFE) & 0x1F, 0b01111);
    // }

    // #[test]
    // fn port_fe_read_row_1_single_key() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(1, 0b11011);
    //     assert_eq!(mem.port_read(0xFDFE) & 0x1F, 0b11011);
    // }

    // #[test]
    // fn port_fe_read_row_2_single_key() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(2, 0b10111);
    //     assert_eq!(mem.port_read(0xFBFE) & 0x1F, 0b10111);
    // }

    // #[test]
    // fn port_fe_read_row_3_single_key() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(3, 0b01111);
    //     assert_eq!(mem.port_read(0xF7FE) & 0x1F, 0b01111);
    // }

    // #[test]
    // fn port_fe_read_row_4_single_key() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(4, 0b11110);
    //     assert_eq!(mem.port_read(0xEFFE) & 0x1F, 0b11110);
    // }

    // #[test]
    // fn port_fe_read_row_5_single_key() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(5, 0b11101);
    //     assert_eq!(mem.port_read(0xDFFE) & 0x1F, 0b11101);
    // }

    // #[test]
    // fn port_fe_read_row_6_single_key() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(6, 0b11011);
    //     assert_eq!(mem.port_read(0xBFFE) & 0x1F, 0b11011);
    // }

    // #[test]
    // fn port_fe_read_row_7_single_key() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(7, 0b10111);
    //     assert_eq!(mem.port_read(0x7FFE) & 0x1F, 0b10111);
    // }

    // // -----------------------------------------------------------------
    // // Port 0xFE — IN (Multiple keys in same row)
    // // -----------------------------------------------------------------

    // #[test]
    // fn port_fe_read_two_keys_same_row() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(0, 0b11000); // bits 0 and 1 cleared
    //     assert_eq!(mem.port_read(0xFEFE) & 0x1F, 0b11000);
    // }

    // #[test]
    // fn port_fe_read_three_keys_same_row() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(1, 0b10000); // bits 0, 1, 2 cleared
    //     assert_eq!(mem.port_read(0xFDFE) & 0x1F, 0b10000);
    // }

    // #[test]
    // fn port_fe_read_all_keys_in_row_pressed() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(0, 0b00000);
    //     assert_eq!(mem.port_read(0xFEFE) & 0x1F, 0b00000);
    // }

    // // -----------------------------------------------------------------
    // // Port 0xFE — IN (Row independence)
    // // -----------------------------------------------------------------

    // #[test]
    // fn port_fe_read_rows_are_independent() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(0, 0b00000); // All keys pressed in row 0
    //     mem.set_keyboard_row(1, 0b11111); // No keys pressed in row 1

    //     assert_eq!(mem.port_read(0xFEFE) & 0x1F, 0b00000);
    //     assert_eq!(mem.port_read(0xFDFE) & 0x1F, 0b11111);
    // }

    // #[test]
    // fn port_fe_read_modifying_one_row_does_not_affect_other() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(0, 0b11110);
    //     mem.set_keyboard_row(1, 0b11101);

    //     assert_eq!(mem.port_read(0xFEFE) & 0x1F, 0b11110);

    //     mem.set_keyboard_row(0, 0b01111); // Change row 0
    //     assert_eq!(mem.port_read(0xFEFE) & 0x1F, 0b01111);
    //     assert_eq!(mem.port_read(0xFDFE) & 0x1F, 0b11101); // Row 1 unchanged
    // }

    // // -----------------------------------------------------------------
    // // Port 0xFE — IN (Multiple row selection / AND behavior)
    // // -----------------------------------------------------------------

    // #[test]
    // fn port_fe_read_two_rows_are_anded() {
    //     let mut mem = make_spectrum16k();
    //     // Row 0: bit 0 pressed -> 0b11110
    //     // Row 1: bit 1 pressed -> 0b11101
    //     // AND: 0b11110 & 0b11101 = 0b11100
    //     mem.set_keyboard_row(0, 0b11110);
    //     mem.set_keyboard_row(1, 0b11101);

    //     // Port 0xFCFE: high byte 0xFC = 11111100, bits 0 and 1 low -> rows 0 and 1
    //     assert_eq!(mem.port_read(0xFCFE) & 0x1F, 0b11100);
    // }

    // #[test]
    // fn port_fe_read_anded_rows_show_combined_pressed_keys() {
    //     let mut mem = make_spectrum16k();
    //     // Row 0: bits 0,1 pressed -> 0b11100
    //     // Row 1: bits 2,3 pressed -> 0b10011
    //     // AND: 0b11100 & 0b10011 = 0b10000
    //     mem.set_keyboard_row(0, 0b11100);
    //     mem.set_keyboard_row(1, 0b10011);

    //     assert_eq!(mem.port_read(0xFCFE) & 0x1F, 0b10000);
    // }

    // #[test]
    // fn port_fe_read_all_eight_rows_anded_no_keys() {
    //     let mem = make_spectrum16k();
    //     // All rows have no keys pressed (0b11111)
    //     // Port 0x00FE: high byte 0x00, all bits low -> all rows selected
    //     assert_eq!(mem.port_read(0x00FE) & 0x1F, 0b11111);
    // }

    // #[test]
    // fn port_fe_read_all_eight_rows_anded_with_keys() {
    //     let mut mem = make_spectrum16k();
    //     // Each row has a different bit pressed
    //     for row in 0..8u8 {
    //         let pressed_bit = 1 << row; // Row 0 -> bit 0, row 1 -> bit 1, etc.
    //         mem.set_keyboard_row(row, !(pressed_bit & 0x1F) & 0x1F);
    //     }
    //     // AND of all rows: each row has a different bit cleared, so AND clears all bits
    //     assert_eq!(mem.port_read(0x00FE) & 0x1F, 0b00000);
    // }

    // #[test]
    // fn port_fe_read_three_rows_anded() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(0, 0b11110); // bit 0 pressed
    //     mem.set_keyboard_row(1, 0b11111); // no keys
    //     mem.set_keyboard_row(2, 0b11101); // bit 1 pressed

    //     // Port 0xF8FE: high byte 0xF8 = 11111000, bits 0,1,2 low -> rows 0,1,2
    //     // AND: 0b11110 & 0b11111 & 0b11101 = 0b11100
    //     assert_eq!(mem.port_read(0xF8FE) & 0x1F, 0b11100);
    // }

    // -----------------------------------------------------------------
    // Port 0xFE — IN (EAR bit 6 — Issue 3 default)
    // -----------------------------------------------------------------

    #[test]
    fn port_fe_read_ear_bit_6_initial_state() {
        // At power-up with ULA register = 0x00, EAR output is low
        // On Issue 3: EAR low -> bit 6 = 0
        let mem = make_spectrum16k();
        assert_eq!(mem.port_read(0xFEFE) & 0x40, 0);
    }

    #[test]
    fn port_fe_read_ear_bit_6_set_when_ear_output_high() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x10); // EAR bit high
        assert_ne!(mem.port_read(0xFEFE) & 0x40, 0);
    }

    #[test]
    fn port_fe_read_ear_bit_6_cleared_when_ear_output_low() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x10); // EAR high first
        assert_ne!(mem.port_read(0xFEFE) & 0x40, 0);

        mem.port_write(0xFE, 0x00); // EAR low
        assert_eq!(mem.port_read(0xFEFE) & 0x40, 0);
    }

    #[test]
    fn port_fe_read_ear_bit_6_toggles_with_ear_output() {
        let mut mem = make_spectrum16k();
        for _ in 0..4 {
            mem.port_write(0xFE, 0x10);
            assert_ne!(mem.port_read(0xFEFE) & 0x40, 0);

            mem.port_write(0xFE, 0x00);
            assert_eq!(mem.port_read(0xFEFE) & 0x40, 0);
        }
    }

    #[test]
    fn port_fe_read_ear_bit_6_not_affected_by_mic_when_ear_high() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x18); // EAR=1, MIC=1
        assert_ne!(mem.port_read(0xFEFE) & 0x40, 0);

        mem.port_write(0xFE, 0x10); // EAR=1, MIC=0
        assert_ne!(mem.port_read(0xFEFE) & 0x40, 0);
    }

    #[test]
    fn port_fe_read_ear_bit_6_not_affected_by_mic_when_ear_low() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x08); // EAR=0, MIC=1
        assert_eq!(mem.port_read(0xFEFE) & 0x40, 0);

        mem.port_write(0xFE, 0x00); // EAR=0, MIC=0
        assert_eq!(mem.port_read(0xFEFE) & 0x40, 0);
    }

    // #[test]
    // fn port_fe_read_ear_bit_6_independent_of_keyboard() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(0, 0b00000); // All keys pressed
    //     mem.port_write(0xFE, 0x10); // EAR high

    //     let result = mem.port_read(0xFEFE);
    //     assert_ne!(result & 0x40, 0); // EAR bit still set
    //     assert_eq!(result & 0x1F, 0b00000); // Keyboard still works
    // }

    // -----------------------------------------------------------------
    // Floating bus (odd port reads)
    // -----------------------------------------------------------------

    #[test]
    fn port_read_odd_address_0xff_returns_0xff() {
        let mem = make_spectrum16k();
        assert_eq!(mem.port_read(0xFF), 0xFF);
    }

    #[test]
    fn port_read_odd_address_0x01_returns_0xff() {
        let mem = make_spectrum16k();
        assert_eq!(mem.port_read(0x01), 0xFF);
    }

    #[test]
    fn port_read_odd_address_0xfeff_returns_0xff() {
        let mem = make_spectrum16k();
        assert_eq!(mem.port_read(0xFEFF), 0xFF);
    }

    // -----------------------------------------------------------------
    // Initial state verification
    // -----------------------------------------------------------------

    #[test]
    fn initial_border_color_is_0() {
        let mem = make_spectrum16k();
        assert_eq!(mem.border_color(), 0);
    }

    #[test]
    fn initial_mic_state_is_false() {
        let mem = make_spectrum16k();
        assert!(!mem.mic_state());
    }

    #[test]
    fn initial_ear_state_is_false() {
        let mem = make_spectrum16k();
        assert!(!mem.ear_state());
    }

    #[test]
    fn initial_keyboard_state_all_released() {
        let mem = make_spectrum16k();
        for port in [
            0xFEFE, 0xFDFE, 0xFBFE, 0xF7FE, 0xEFFE, 0xDFFE, 0xBFFE, 0x7FFE,
        ] {
            assert_eq!(
                mem.port_read(port) & 0x1F,
                0x1F,
                "Port {port:#06X} not all released"
            );
        }
    }

    // -----------------------------------------------------------------
    // Edge cases
    // -----------------------------------------------------------------

    #[test]
    fn port_write_all_even_ports_in_small_range() {
        let mut mem = make_spectrum16k();
        for port in (0x00..=0xFE).step_by(2) {
            mem.port_write(port, 0x07);
            assert_eq!(mem.border_color(), 7, "Failed at port {port:#04X}");
        }
    }

    // #[test]
    // fn port_read_all_standard_keyboard_ports() {
    //     let mut mem = make_spectrum16k();
    //     let ports = [
    //         0xFEFE, 0xFDFE, 0xFBFE, 0xF7FE, 0xEFFE, 0xDFFE, 0xBFFE, 0x7FFE,
    //     ];

    //     for (row, &port) in ports.iter().enumerate() {
    //         mem.set_keyboard_row(row as u8, 0b01111);
    //         assert_eq!(
    //             mem.port_read(port) & 0x1F,
    //             0b01111,
    //             "Row {row} at port {port:#06X} failed"
    //         );
    //     }
    // }

    // #[test]
    // fn port_write_does_not_affect_keyboard_state() {
    //     let mut mem = make_spectrum16k();
    //     mem.set_keyboard_row(0, 0b11110);
    //     mem.port_write(0xFE, 0xFF);
    //     assert_eq!(mem.port_read(0xFEFE) & 0x1F, 0b11110);
    // }

    #[test]
    fn port_read_does_not_affect_ula_state() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x05);
        let _ = mem.port_read(0xFEFE);
        assert_eq!(mem.border_color(), 5);
    }
}
