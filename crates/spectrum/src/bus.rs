use z80::Bus;

struct Spectrum16k {
    rom: [u8; 0x4000], // 0x0000 – 0x3FFF
    ram: [u8; 0x4000], // 0x4000 - 0x7FFF
}

impl Spectrum16k {
    fn new(rom: &[u8]) -> Self {
        assert_eq!(rom.len(), 0x4000, "16K model requires a 16KB ROM");
        let mut rom_clone = [0; 0x4000];
        rom_clone.copy_from_slice(rom);

        Spectrum16k {
            rom: rom_clone,
            ram: [0xFF; 0x4000],
        }
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
        todo!()
    }

    fn port_write(&mut self, port: u16, value: u8) {
        todo!()
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

    fn make_spectrum16k() -> Spectrum16k {
        Spectrum16k::new(&make_rom())
    }

    // -----------------------------------------------------------------
    // Construction / validation
    // -----------------------------------------------------------------

    #[test]
    fn new_accepts_exactly_16kb_rom() {
        // Should not panic
        let _mem = Spectrum16k::new(&[0u8; 0x4000]);
    }

    #[test]
    #[should_panic(expected = "16K model requires a 16KB ROM")]
    fn new_rejects_rom_shorter_than_16kb() {
        let _mem = Spectrum16k::new(&[0u8; 0x3FFF]);
    }

    #[test]
    #[should_panic(expected = "16K model requires a 16KB ROM")]
    fn new_rejects_rom_longer_than_16kb() {
        let _mem = Spectrum16k::new(&[0u8; 0x4001]);
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
}
