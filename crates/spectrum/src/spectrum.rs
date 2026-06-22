use crate::{Keyboard, TapePlayer, memory::SpectrumMemory};
use std::{cell::RefCell, rc::Rc};
use z80::Bus;

const CPU_FREQUENCY: usize = 3_500_000;
pub const AUDIO_RATE: usize = 48_000;

pub struct Spectrum {
    memory: SpectrumMemory,
    //IO
    keyboard: Rc<RefCell<Keyboard>>,
    tape_player: Rc<RefCell<TapePlayer>>,
    // ULA
    border_color: u8,
    mic: bool,
    ear: bool,
    // Audio
    cycles: usize,
    audio_idx: u16,
    audio_buffer: [f32; 2000],
}

impl Spectrum {
    pub fn new(
        memory: SpectrumMemory,
        keyboard: Rc<RefCell<Keyboard>>,
        tape_player: Rc<RefCell<TapePlayer>>,
    ) -> Self {
        Self {
            memory,
            keyboard,
            tape_player,
            border_color: 0,
            mic: false,
            ear: false,
            cycles: 0,
            audio_idx: 0,
            audio_buffer: [0.0; 2000],
        }
    }

    pub fn border_color(&self) -> u8 {
        self.border_color
    }

    pub fn ear_state(&self) -> bool {
        self.ear ^ self.tape_player.borrow().ear()
    }

    pub fn mic_state(&self) -> bool {
        self.mic
    }

    pub fn step(&mut self, cycles: u64) {
        self.cycles += AUDIO_RATE * cycles as usize;
        while self.cycles >= CPU_FREQUENCY {
            self.cycles -= CPU_FREQUENCY;
            if self.audio_idx < self.audio_buffer.len() as u16 {
                self.audio_buffer[self.audio_idx as usize] =
                    if self.ear_state() { 0.2 } else { 0.0 };
                self.audio_idx += 1;
            }
        }
    }

    pub fn consume_audio(&mut self) -> &[f32] {
        let idx = self.audio_idx as usize;
        self.audio_idx = 0;
        &self.audio_buffer[0..idx]
    }
}

impl Bus for Spectrum {
    fn read(&self, addr: u16) -> u8 {
        self.memory.read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.memory.write(addr, value);
    }

    fn port_read(&self, port: u16) -> u8 {
        if (port & 0xFF) != 0xFE {
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
        keyboard_state |= (self.ear_state() as u8) << 6;

        keyboard_state
    }

    fn port_write(&mut self, port: u16, value: u8) {
        if (port & 0x01) == 1 {
            return;
        }

        self.border_color = value & 0x07;
        self.mic = (value & 0x08) == 0x08;
        self.ear = (value & 0x10) == 0x10;
    }
}

#[cfg(test)]
mod tests {
    use crate::SpectrumKey;

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

    fn make_tape() -> Rc<RefCell<TapePlayer>> {
        let mut block = Vec::new();
        let payload_len = 2;

        block.push((payload_len & 0xFF) as u8);
        block.push(((payload_len >> 8) & 0xFF) as u8);
        block.push(0x00);

        let checksum = 0x00;
        block.push(checksum);

        Rc::new(RefCell::new(TapePlayer::from_tape(&block)))
    }

    fn make_spectrum16k() -> Spectrum {
        let memory = SpectrumMemory::new_16k(&make_rom());
        Spectrum::new(memory, make_keyboard(), make_tape())
    }

    fn make_spectrum48k() -> Spectrum {
        let memory = SpectrumMemory::new_48k(&make_rom());
        Spectrum::new(memory, make_keyboard(), make_tape())
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
    fn test_16k_unmapped_read_returns_0xff() {
        let mem = make_spectrum16k();
        assert_eq!(mem.read(0x8000), 0xFF);
        assert_eq!(mem.read(0xC000), 0xFF);
        assert_eq!(mem.read(0xFFFF), 0xFF);
    }

    #[test]
    fn test_16k_unmapped_write_does_not_crash_and_is_ignored() {
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
    // 48k Model Upper RAM (0x8000 ..= 0xFFFF)
    // -----------------------------------------------------------------

    #[test]
    fn test_48k_upper_banks_are_valid_ram() {
        let mut mem = make_spectrum48k();

        // 0x8000..=0xFFFF should behave like standard RAM
        assert_eq!(mem.read(0x8000), 0xFF);
        assert_eq!(mem.read(0xFFFF), 0xFF);

        mem.write(0x8000, 0x55);
        mem.write(0xFFFF, 0xAA);
        assert_eq!(mem.read(0x8000), 0x55);
        assert_eq!(mem.read(0xFFFF), 0xAA);
    }

    #[test]
    fn test_48k_full_ram_range_independent() {
        let mut mem = make_spectrum48k();
        mem.write(0x4000, 0x11);
        mem.write(0x8000, 0x22);
        mem.write(0xC000, 0x33);
        assert_eq!(mem.read(0x4000), 0x11);
        assert_eq!(mem.read(0x8000), 0x22);
        assert_eq!(mem.read(0xC000), 0x33);
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

    #[test]
    fn port_fe_read_port_ear_from_tape() {
        let mem = make_spectrum16k();
        mem.tape_player.borrow_mut().play();
        assert!((mem.port_read(0xFFFE) & 0x40) == 0);

        mem.tape_player.borrow_mut().advance(8063);
        assert!((mem.port_read(0xFFFE) & 0x40) != 0);
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

    #[test]
    fn port_fe_read_ear_bit_6_independent_of_keyboard() {
        let mut mem = make_spectrum16k();
        SpectrumKey::ALL_KEYS
            .iter()
            .for_each(|k| mem.keyboard.borrow_mut().press_key(k));
        mem.port_write(0xFE, 0x10); // EAR high

        let result = mem.port_read(0xFEFE);
        assert_ne!(result & 0x40, 0); // EAR bit still set
        assert_eq!(result & 0x1F, 0b00000); // Keyboard still works
    }

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

    #[test]
    fn port_write_does_not_affect_keyboard_state() {
        let mut mem: Spectrum = make_spectrum16k();
        mem.keyboard
            .borrow_mut()
            .press_key(&crate::SpectrumKey::CapsShift);
        mem.port_write(0xFE, 0xFF);
        assert_eq!(mem.port_read(0xFEFE) & 0x1F, 0b11110);
    }

    #[test]
    fn port_read_does_not_affect_ula_state() {
        let mut mem = make_spectrum16k();
        mem.port_write(0xFE, 0x05);
        let _ = mem.port_read(0xFEFE);
        assert_eq!(mem.border_color(), 5);
    }

    // -----------------------------------------------------------------
    // Audio Generation Tests
    // -----------------------------------------------------------------

    #[test]
    fn test_audio_generation_exact_rate() {
        let mut mem = make_spectrum16k();

        // 3.5 MHz CPU for exactly 1 second
        // At exactly 48,000Hz, we should produce EXACTLY 48,000 samples.
        let mut total_samples = 0;
        let chunk_size = 1000;

        for _ in 0..(3_500_000 / chunk_size) {
            mem.step(chunk_size);
            total_samples += mem.consume_audio().len();
        }

        assert_eq!(
            total_samples, 48_000,
            "Audio generation drifted! Produced {} instead of exactly 48000 samples.",
            total_samples
        );
    }

    #[test]
    fn test_audio_buffer_overflow_prevention() {
        let mut mem = make_spectrum16k();

        // Simulating a massive frame skip/stall (e.g., stepping 200,000 cycles at once)
        mem.step(200_000);

        let samples = mem.consume_audio();
        assert!(
            samples.len() <= 2000,
            "Audio buffer should cap safely without panicking, got len: {}",
            samples.len()
        );
    }

    #[test]
    fn test_audio_amplitude_on_ear_toggle() {
        let mut mem = make_spectrum16k();

        // Default state (EAR off)
        mem.step(100); // Step enough for at least 1 sample
        let samples = mem.consume_audio();
        assert_eq!(samples[0], 0.0, "EAR default should output 0.0");

        // Toggle EAR on via port 0xFE
        mem.port_write(0xFE, 0x10);
        mem.step(100);
        let samples_on = mem.consume_audio();
        assert_eq!(samples_on[0], 0.2, "EAR active should output 0.2");
    }
}
