pub enum SpectrumMemory {
    Model16K {
        rom: [u8; 0x4000], // 0x0000 – 0x3FFF
        ram: [u8; 0x4000], // 0x4000 - 0x7FFF
    },
    Model48K {
        rom: [u8; 0x4000], // 0x0000 - 0x3FFF
        ram: [u8; 0xC000], // 0x4000 - 0xFFFF
    },
}

impl SpectrumMemory {
    pub fn new_16k(rom: &[u8]) -> Self {
        assert_eq!(rom.len(), 0x4000, "16K model requires a 16KB ROM");
        let mut rom_clone = [0; 0x4000];
        rom_clone.copy_from_slice(rom);

        Self::Model16K {
            rom: rom_clone,
            ram: [0xFF; 0x4000],
        }
    }

    pub fn new_48k(rom: &[u8]) -> Self {
        assert_eq!(rom.len(), 0x4000, "48K model requires a 16KB ROM");
        let mut rom_clone = [0; 0x4000];
        rom_clone.copy_from_slice(rom);

        Self::Model48K {
            rom: rom_clone,
            ram: [0xFF; 0xC000],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match self {
            Self::Model16K { rom, ram } => match addr {
                0x0000..=0x3FFF => rom[addr as usize],
                0x4000..=0x7FFF => ram[addr as usize - 0x4000],
                0x8000..=0xFFFF => 0xFF,
            },
            Self::Model48K { rom, ram } => match addr {
                0x0000..=0x3FFF => rom[addr as usize],
                0x4000..=0xFFFF => ram[addr as usize - 0x4000],
            },
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match self {
            Self::Model16K { rom: _, ram } => match addr {
                0x0000..=0x3FFF => {}
                0x4000..=0x7FFF => ram[addr as usize - 0x4000] = value,
                0x8000..=0xFFFF => {}
            },
            Self::Model48K { rom: _, ram } => match addr {
                0x0000..=0x3FFF => {}
                0x4000..=0xFFFF => ram[addr as usize - 0x4000] = value,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    // -----------------------------------------------------------------
    // Construction / validation
    // -----------------------------------------------------------------

    use crate::memory::SpectrumMemory;

    #[test]
    fn test_16k_model_accepts_exactly_16kb_rom() {
        // Should not panic
        let _mem = SpectrumMemory::new_16k(&[0u8; 0x4000]);
    }

    #[test]
    #[should_panic(expected = "16K model requires a 16KB ROM")]
    fn test_16k_model_rejects_rom_shorter_than_16kb() {
        let _mem = SpectrumMemory::new_16k(&[0u8; 0x3FFF]);
    }

    #[test]
    #[should_panic(expected = "16K model requires a 16KB ROM")]
    fn test_16k_model_rejects_rom_longer_than_16kb() {
        let _mem = SpectrumMemory::new_16k(&[0u8; 0x4001]);
    }

    #[test]
    fn test_48k_model_accepts_exactly_16kb_rom() {
        let _mem = SpectrumMemory::new_48k(&[0u8; 0x4000]);
    }

    #[test]
    #[should_panic(expected = "48K model requires a 16KB ROM")]
    fn test_48k_model_rejects_rom_shorter_than_16kb() {
        let _mem = SpectrumMemory::new_48k(&[0u8; 0x3FFF]);
    }

    #[test]
    #[should_panic(expected = "48K model requires a 16KB ROM")]
    fn test_48k_model_rejects_rom_longer_than_16kb() {
        let _mem = SpectrumMemory::new_48k(&[0u8; 0x4001]);
    }
}
