use crate::bus::Bus;

const FLAG_SIGN: u8 = 1 << 7;
const FLAG_ZERO: u8 = 1 << 6;
const FLAG_HALF_CARRY: u8 = 1 << 4;
const FLAG_PARITY: u8 = 1 << 2;
const FLAG_ADD_OR_SUBTRACT: u8 = 1 << 1;
const FLAG_CARRY: u8 = 1 << 0;

#[derive(Default)]
struct Z80 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    // w: u8
    // z: u8
    a_shadow: u8,
    b_shadow: u8,
    c_shadow: u8,
    d_shadow: u8,
    e_shadow: u8,
    f_shadow: u8,
    h_shadow: u8,
    l_shadow: u8,
    // w_shadow: u8
    // z_shadow: u8
    pc: u16,
    sp: u16,
    ix: u16,
    iy: u16,
}

impl Z80 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute<B: Bus>(&mut self, bus: &mut B) -> u64 {
        let opcode = self.read_byte(bus);
        0
    }

    fn read_byte<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let byte = bus.read(self.pc);
        self.pc += 1;
        byte
    }
}
