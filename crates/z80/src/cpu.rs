use crate::bus::Bus;

const FLAG_SIGN: u8 = 1 << 7;
const FLAG_ZERO: u8 = 1 << 6;
const FLAG_HALF_CARRY: u8 = 1 << 4;
const FLAG_PARITY: u8 = 1 << 2;
const FLAG_ADD_OR_SUBTRACT: u8 = 1 << 1;
const FLAG_CARRY: u8 = 1 << 0;

#[derive(Default, PartialEq, Eq, Debug)]
enum InterruptMode {
    #[default]
    IM1,
    IM2,
    IM0,
}

#[derive(Default, PartialEq, Eq, Debug)]
struct Z80 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    i: u8,
    r: u8,
    p: bool,
    w: u8,
    z: u8,
    q: u8,
    a_shadow: u8,
    b_shadow: u8,
    c_shadow: u8,
    d_shadow: u8,
    e_shadow: u8,
    f_shadow: u8,
    h_shadow: u8,
    l_shadow: u8,
    w_shadow: u8,
    z_shadow: u8,
    pc: u16,
    sp: u16,
    ix: u16,
    iy: u16,
    ei: bool,
    im: InterruptMode,
    iff1: bool,
    iff2: bool,
}

impl Z80 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute<B: Bus>(&mut self, bus: &mut B) -> u64 {
        let opcode = self.read_byte(bus);
        self.r = (((self.r & 0x7F) + 1) & 0x7F) | (self.r & 0x80);
        self.ei = false;
        self.p = false;
        let old_f = self.f;
        let old_q = self.q;
        let cycles = self.step(bus, opcode);
        self.q = if (old_f != self.f || old_q != self.q) && opcode != 0x08 {
            self.f
        } else {
            0
        };
        cycles
    }

    fn read_byte<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let byte = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    fn read_word<B: Bus>(&mut self, bus: &mut B) -> u16 {
        let lo = self.read_byte(bus) as u16;
        let hi = self.read_byte(bus) as u16;
        (hi << 8) | lo
    }

    fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    fn wz(&self) -> u16 {
        ((self.w as u16) << 8) | (self.z as u16)
    }

    fn set_wz(&mut self, value: u16) {
        self.w = (value >> 8) as u8;
        self.z = (value & 0xFF) as u8;
    }

    fn get_rp(&self, idx: u8) -> u16 {
        match idx {
            0 => self.bc(),
            1 => self.de(),
            2 => self.hl(),
            3 => self.sp,
            _ => unreachable!(),
        }
    }

    fn set_rp(&mut self, idx: u8, value: u16) {
        match idx {
            0 => self.set_bc(value),
            1 => self.set_de(value),
            2 => self.set_hl(value),
            3 => self.sp = value,
            _ => unreachable!(),
        }
    }

    fn get_reg<B: Bus>(&self, bus: &B, idx: u8) -> u8 {
        match idx {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => bus.read(self.hl()),
            7 => self.a,
            _ => unreachable!(),
        }
    }

    fn set_reg<B: Bus>(&mut self, bus: &mut B, idx: u8, value: u8) {
        match idx {
            0 => self.b = value,
            1 => self.c = value,
            2 => self.d = value,
            3 => self.e = value,
            4 => self.h = value,
            5 => self.l = value,
            6 => bus.write(self.hl(), value),
            7 => self.a = value,
            _ => unreachable!(),
        }
    }

    fn parity(value: u8) -> bool {
        value.count_ones() % 2 == 0
    }

    fn inc8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.f = if result == 0 { FLAG_ZERO } else { 0 }
            | if result & 0x80 != 0 { FLAG_SIGN } else { 0 }
            | (result & 0x28) // Mirror bits 3 and 5 from the result
            | if value & 0x0F == 0x0F {
                FLAG_HALF_CARRY
            } else {
                0
            }
            | if value == 0x7F { FLAG_PARITY } else { 0 }
            | (self.f & FLAG_CARRY);
        self.q = !self.q;
        result
    }

    fn dec8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.f = if result == 0 { FLAG_ZERO } else { 0 }
            | if result & 0x80 != 0 { FLAG_SIGN } else { 0 }
            | (result & 0x28) // Mirror bits 3 and 5 from the result
            | if value & 0x0F == 0x00 {
                FLAG_HALF_CARRY
            } else {
                0
            }
            | if value == 0x80 { FLAG_PARITY } else { 0 }
            | FLAG_ADD_OR_SUBTRACT
            | (self.f & FLAG_CARRY);
        self.q = !self.q;
        result
    }

    fn add_hl(&mut self, value: u16) {
        let hl = self.hl();
        let (result, carry) = hl.overflowing_add(value);
        let half_carry = (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF;
        self.f = (self.f & (FLAG_SIGN | FLAG_ZERO | FLAG_PARITY))
            | if half_carry { FLAG_HALF_CARRY } else { 0 }
            | if carry { FLAG_CARRY } else { 0 }
            | ((result >> 8) & 0x28) as u8;
        self.q = !self.q;
        self.set_wz(hl.wrapping_add(1));
        self.set_hl(result);
    }

    fn add_a(&mut self, value: u8, with_carry: bool) {
        let c = if with_carry && self.f & FLAG_CARRY != 0 {
            1
        } else {
            0
        };
        let result = self.a as u16 + value as u16 + c as u16;
        let half_carry = (self.a & 0x0F) + (value & 0x0F) + c > 0x0F;
        let result8 = result as u8;
        let overflow = ((self.a ^ result8) & (value ^ result8) & 0x80) != 0;
        self.f = if result8 == 0 { FLAG_ZERO } else { 0 }
            | if result8 & 0x80 != 0 { FLAG_SIGN } else { 0 }
            | if half_carry { FLAG_HALF_CARRY } else { 0 }
            | if overflow { FLAG_PARITY } else { 0 }
            | if result > 0xFF { FLAG_CARRY } else { 0 }
            | (result8 & 0x28);
        self.q = !self.q;
        self.a = result8;
    }

    fn sub_a(&mut self, value: u8, with_carry: bool, store: bool) {
        let c = if with_carry && self.f & FLAG_CARRY != 0 {
            1
        } else {
            0
        };
        let result = self.a.wrapping_sub(value).wrapping_sub(c);
        let carry = (self.a as u16) < value as u16 + c as u16;
        let half_carry = ((self.a & 0x0F) as u16) < (value & 0x0F) as u16 + c as u16;
        let overflow = ((self.a ^ value) & 0x80) != 0 && ((self.a ^ result) & 0x80) != 0;
        self.f = if result == 0 { FLAG_ZERO } else { 0 }
            | if result & 0x80 != 0 { FLAG_SIGN } else { 0 }
            | if half_carry { FLAG_HALF_CARRY } else { 0 }
            | if overflow { FLAG_PARITY } else { 0 }
            | if carry { FLAG_CARRY } else { 0 }
            | FLAG_ADD_OR_SUBTRACT
            | if store { result & 0x28 } else { value & 0x28 };
        self.q = !self.q;
        if store {
            self.a = result;
        }
    }

    fn and_a(&mut self, value: u8) {
        self.a &= value;
        self.f = if self.a == 0 { FLAG_ZERO } else { 0 }
            | if self.a & 0x80 != 0 { FLAG_SIGN } else { 0 }
            | FLAG_HALF_CARRY
            | if Self::parity(self.a) { FLAG_PARITY } else { 0 }
            | (self.a & 0x28);
        self.q = !self.q;
    }

    fn xor_a(&mut self, value: u8) {
        self.a ^= value;
        self.f = if self.a == 0 { FLAG_ZERO } else { 0 }
            | if self.a & 0x80 != 0 { FLAG_SIGN } else { 0 }
            | if Self::parity(self.a) { FLAG_PARITY } else { 0 }
            | (self.a & 0x28);
        self.q = !self.q;
    }

    fn or_a(&mut self, value: u8) {
        self.a |= value;
        self.f = if self.a == 0 { FLAG_ZERO } else { 0 }
            | if self.a & 0x80 != 0 { FLAG_SIGN } else { 0 }
            | if Self::parity(self.a) { FLAG_PARITY } else { 0 }
            | (self.a & 0x28);
        self.q = !self.q;
    }

    fn daa(&mut self) {
        let a = self.a;
        let carry = self.f & FLAG_CARRY != 0;
        let half = self.f & FLAG_HALF_CARRY != 0;
        let sub = self.f & FLAG_ADD_OR_SUBTRACT != 0;
        let mut add = 0u8;
        let mut new_carry = carry;

        if half || (a & 0x0F) > 9 {
            add |= 0x06;
        }
        if carry || (a & 0xF0) > 0x90 || ((a & 0x0F) > 9 && (a & 0xF0) > 0x80) {
            add |= 0x60;
            new_carry = true;
        }

        self.a = if sub {
            a.wrapping_sub(add)
        } else {
            a.wrapping_add(add)
        };
        self.f = if new_carry { FLAG_CARRY } else { 0 }
            | if self.a == 0 { FLAG_ZERO } else { 0 }
            | if self.a & 0x80 != 0 { FLAG_SIGN } else { 0 }
            | if Self::parity(self.a) { FLAG_PARITY } else { 0 }
            | if sub { FLAG_ADD_OR_SUBTRACT } else { 0 }
            | (self.a & 0x28)
            | if sub && half && (a & 0x0F) < 6 {
                FLAG_HALF_CARRY
            } else {
                0
            }
            | if !sub && (a & 0x0F) > 9 {
                FLAG_HALF_CARRY
            } else {
                0
            };
        self.q = !self.q;
    }

    fn step<B: Bus>(&mut self, bus: &mut B, opcode: u8) -> u64 {
        match opcode {
            // NOP
            0x00 => 4, // NOP
            0x01 => {
                let word = self.read_word(bus);
                self.set_bc(word);
                10
            } // LD BC,nn
            0x02 => {
                bus.write(self.bc(), self.a);
                self.w = self.a;
                self.z = self.c.wrapping_add(1);
                7
            } // LD (BC),A
            0x03 => {
                self.set_bc(self.bc().wrapping_add(1));
                6
            } // INC BC
            0x04 => {
                self.b = self.inc8(self.b);
                4
            } // INC B
            0x05 => {
                self.b = self.dec8(self.b);
                4
            } // DEC B
            0x06 => {
                self.b = self.read_byte(bus);
                7
            } // LD B,n
            0x07 => {
                // RLCA
                let c = self.a >> 7;
                self.a = (self.a << 1) | c;
                self.f = (self.f & (FLAG_SIGN | FLAG_ZERO | FLAG_PARITY)) | c | (self.a & 0x28);
                self.q = !self.q;
                4
            }
            0x08 => {
                // EX AF,AF'
                std::mem::swap(&mut self.a, &mut self.a_shadow);
                std::mem::swap(&mut self.f, &mut self.f_shadow);
                4
            }
            0x09 => {
                self.add_hl(self.bc());
                11
            } // ADD HL,BC
            0x0A => {
                self.a = bus.read(self.bc());
                self.set_wz(self.bc().wrapping_add(1));
                7
            } // LD A,(BC)
            0x0B => {
                self.set_bc(self.bc().wrapping_sub(1));
                6
            } // DEC BC
            0x0C => {
                self.c = self.inc8(self.c);
                4
            } // INC C
            0x0D => {
                self.c = self.dec8(self.c);
                4
            } // DEC C
            0x0E => {
                self.c = self.read_byte(bus);
                7
            } // LD C,n
            0x0F => {
                // RRCA
                let c = self.a & 1;
                self.a = (self.a >> 1) | (c << 7);
                self.f = (self.f & (FLAG_SIGN | FLAG_ZERO | FLAG_PARITY)) | c | (self.a & 0x28);
                self.q = !self.q; // reset inside execute
                4
            }
            0x10 => {
                // DJNZ d
                self.b = self.b.wrapping_sub(1);
                let displacement = self.read_byte(bus) as i8 as u16;
                if self.b != 0 {
                    self.pc = self.pc.wrapping_add(displacement);
                    self.set_wz(self.pc);
                    13
                } else {
                    8
                }
            }
            0x11 => {
                let word = self.read_word(bus);
                self.set_de(word);
                10
            } // LD DE,nn
            0x12 => {
                bus.write(self.de(), self.a);
                self.w = self.a;
                self.z = self.e.wrapping_add(1);
                7
            } // LD (DE),A
            0x13 => {
                self.set_de(self.de().wrapping_add(1));
                6
            } // INC DE
            0x14 => {
                self.d = self.inc8(self.d);
                4
            } // INC D
            0x15 => {
                self.d = self.dec8(self.d);
                4
            } // DEC D
            0x16 => {
                self.d = self.read_byte(bus);
                7
            } // LD D,n
            0x17 => {
                // RLA
                let old_c = if self.f & FLAG_CARRY != 0 { 1 } else { 0 };
                let new_c = self.a >> 7;
                self.a = (self.a << 1) | old_c;
                self.f = (self.f & (FLAG_SIGN | FLAG_ZERO | FLAG_PARITY)) | new_c | (self.a & 0x28);
                self.q = !self.q; // reset inside execute
                4
            }
            0x18 => {
                // JR d
                let displacement = self.read_byte(bus) as i8 as u16;
                self.pc = self.pc.wrapping_add(displacement);
                self.set_wz(self.pc);
                12
            }
            0x19 => {
                self.add_hl(self.de());
                11
            } // ADD HL,DE
            0x1A => {
                self.a = bus.read(self.de());
                self.set_wz(self.de().wrapping_add(1));
                7
            } // LD A,(DE)
            0x1B => {
                self.set_de(self.de().wrapping_sub(1));
                6
            } // DEC DE
            0x1C => {
                self.e = self.inc8(self.e);
                4
            } // INC E
            0x1D => {
                self.e = self.dec8(self.e);
                4
            } // DEC E
            0x1E => {
                self.e = self.read_byte(bus);
                7
            } // LD E,n
            0x1F => {
                // RRA
                let old_c = if self.f & FLAG_CARRY != 0 { 0x80 } else { 0 };
                let new_c = self.a & 1;
                self.a = (self.a >> 1) | old_c;
                self.f = (self.f & (FLAG_SIGN | FLAG_ZERO | FLAG_PARITY)) | new_c | (self.a & 0x28);
                self.q = !self.q;
                4
            }
            0x20 => {
                // JR NZ,d
                let displacement = self.read_byte(bus) as i8 as u16;
                if self.f & FLAG_ZERO == 0 {
                    self.pc = self.pc.wrapping_add(displacement);
                    self.set_wz(self.pc);
                    12
                } else {
                    7
                }
            }
            0x21 => {
                let word = self.read_word(bus);
                self.set_hl(word);
                10
            } // LD HL,nn
            0x22 => {
                // LD (nn),HL
                let addr = self.read_word(bus);
                bus.write(addr, self.l);
                bus.write(addr.wrapping_add(1), self.h);
                self.set_wz(addr.wrapping_add(1));
                16
            }
            0x23 => {
                self.set_hl(self.hl().wrapping_add(1));
                6
            } // INC HL
            0x24 => {
                self.h = self.inc8(self.h);
                4
            } // INC H
            0x25 => {
                self.h = self.dec8(self.h);
                4
            } // DEC H
            0x26 => {
                self.h = self.read_byte(bus);
                7
            } // LD H,n
            0x27 => {
                self.daa();
                4
            } // DAA
            0x28 => {
                // JR Z,d
                let displacement = self.read_byte(bus) as i8 as u16;
                if self.f & FLAG_ZERO != 0 {
                    self.pc = self.pc.wrapping_add(displacement);
                    self.set_wz(self.pc);
                    12
                } else {
                    7
                }
            }
            0x29 => {
                self.add_hl(self.hl());
                11
            } // ADD HL,HL
            0x2A => {
                // LD HL,(nn)
                let addr = self.read_word(bus);
                self.l = bus.read(addr);
                self.h = bus.read(addr.wrapping_add(1));
                self.set_wz(addr.wrapping_add(1));
                16
            }
            0x2B => {
                self.set_hl(self.hl().wrapping_sub(1));
                6
            } // DEC HL
            0x2C => {
                self.l = self.inc8(self.l);
                4
            } // INC L
            0x2D => {
                self.l = self.dec8(self.l);
                4
            } // DEC L
            0x2E => {
                self.l = self.read_byte(bus);
                7
            } // LD L,n
            0x2F => {
                // CPL
                self.a = !self.a;
                self.f = (self.f & (FLAG_SIGN | FLAG_ZERO | FLAG_PARITY | FLAG_CARRY))
                    | FLAG_HALF_CARRY
                    | FLAG_ADD_OR_SUBTRACT
                    | (self.a & 0x28); // Mirror bits 3 and 5 from the new A
                self.q = !self.q;
                4
            }
            0x30 => {
                // JR NC,d
                let displacement = self.read_byte(bus) as i8 as u16;
                if self.f & FLAG_CARRY == 0 {
                    self.pc = self.pc.wrapping_add(displacement);
                    self.set_wz(self.pc);
                    12
                } else {
                    7
                }
            }
            0x31 => {
                self.sp = self.read_word(bus);
                10
            } // LD SP,nn
            0x32 => {
                // LD (nn),A
                let addr = self.read_word(bus);
                bus.write(addr, self.a);
                self.w = self.a;
                self.z = (addr.wrapping_add(1) & 0xFF) as u8;
                13
            }
            0x33 => {
                self.sp = self.sp.wrapping_add(1);
                6
            } // INC SP
            0x34 => {
                // INC (HL)
                let addr = self.hl();
                let value = bus.read(addr);
                let result = self.inc8(value);
                bus.write(addr, result);
                11
            }
            0x35 => {
                // DEC (HL)
                let addr = self.hl();
                let value = bus.read(addr);
                let result = self.dec8(value);
                bus.write(addr, result);
                11
            }
            0x36 => {
                // LD (HL),n
                let value = self.read_byte(bus);
                bus.write(self.hl(), value);
                10
            }
            0x37 => {
                // SCF
                self.f = (self.f & (FLAG_SIGN | FLAG_ZERO | FLAG_PARITY))
                    | FLAG_CARRY
                    | (((self.q ^ self.f) | self.a) & 0x28);
                self.q = !self.q;
                4
            }
            0x38 => {
                // JR C,d
                let displacement = self.read_byte(bus) as i8 as u16;
                if self.f & FLAG_CARRY != 0 {
                    self.pc = self.pc.wrapping_add(displacement);
                    self.set_wz(self.pc);
                    12
                } else {
                    7
                }
            }
            0x39 => {
                self.add_hl(self.sp);
                11
            } // ADD HL,SP
            0x3A => {
                // LD A,(nn)
                let addr = self.read_word(bus);
                self.a = bus.read(addr);
                self.set_wz(addr.wrapping_add(1));
                13
            }
            0x3B => {
                self.sp = self.sp.wrapping_sub(1);
                6
            } // DEC SP
            0x3C => {
                self.a = self.inc8(self.a);
                4
            } // INC A
            0x3D => {
                self.a = self.dec8(self.a);
                4
            } // DEC A
            0x3E => {
                self.a = self.read_byte(bus);
                7
            } // LD A,n
            0x3F => {
                // CCF
                let old_carry = self.f & FLAG_CARRY;
                self.f = (self.f & (FLAG_SIGN | FLAG_ZERO | FLAG_PARITY))
                    | if old_carry != 0 { FLAG_HALF_CARRY } else { 0 }
                    | if old_carry == 0 { FLAG_CARRY } else { 0 }
                    | (((self.q ^ self.f) | self.a) & 0x28);
                self.q = !self.q;
                4
            }
            0x40..=0x7F if opcode != 0x76 => {
                let dst = (opcode >> 3) & 7;
                let src = opcode & 7;
                let val = self.get_reg(bus, src);
                self.set_reg(bus, dst, val);
                if dst == 6 || src == 6 { 7 } else { 4 }
            }
            0x76 => {
                // HALT
                // TODO: handle halt
                dbg!("HALT CALLED");
                4
            }
            0x80..=0xBF => {
                let op = (opcode >> 3) & 7;
                let src = opcode & 7;
                let val = self.get_reg(bus, src);
                match op {
                    0 => self.add_a(val, false),        // ADD
                    1 => self.add_a(val, true),         // ADC
                    2 => self.sub_a(val, false, true),  // SUB
                    3 => self.sub_a(val, true, true),   // SBC
                    4 => self.and_a(val),               // AND
                    5 => self.xor_a(val),               // XOR
                    6 => self.or_a(val),                // OR
                    7 => self.sub_a(val, false, false), // CP
                    _ => unreachable!(),
                }
                if src == 6 { 7 } else { 4 }
            }

            _ => panic!("Unexpected opcode {opcode:02X}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs, path::Path};

    use serde::Deserialize;

    use crate::{bus::Bus, cpu::Z80};

    #[derive(Default, PartialEq, Eq, Debug)]
    struct TestBus {
        memory: HashMap<u16, u8>,
    }

    impl Bus for TestBus {
        fn read(&self, addr: u16) -> u8 {
            *self.memory.get(&addr).unwrap_or(&0xFF)
        }

        fn write(&mut self, addr: u16, value: u8) {
            self.memory.insert(addr, value);
        }
    }

    #[derive(Deserialize)]
    struct TestCaseState {
        pc: u16,
        sp: u16,
        a: u8,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        f: u8,
        h: u8,
        l: u8,
        i: u8,
        r: u8,
        ei: u8,
        wz: u16,
        ix: u16,
        iy: u16,
        af_: u16,
        bc_: u16,
        de_: u16,
        hl_: u16,
        im: u8,
        p: u8,
        q: u8,
        iff1: u8,
        iff2: u8,
        ram: Vec<(u16, u8)>,
    }

    impl TestCaseState {
        fn create_cpu(&self) -> Z80 {
            let mut cpu = Z80::default();

            cpu.a = self.a;
            cpu.b = self.b;
            cpu.c = self.c;
            cpu.d = self.d;
            cpu.e = self.e;
            cpu.f = self.f;
            cpu.h = self.h;
            cpu.l = self.l;
            cpu.i = self.i;
            cpu.r = self.r;
            cpu.p = match self.p {
                0 => false,
                1 => true,
                _ => panic!("Unexpected P value {}", self.ei),
            };
            cpu.w = (self.wz >> 8) as u8;
            cpu.z = (self.wz & 0xFF) as u8;
            cpu.q = self.q;
            cpu.ix = self.ix;
            cpu.iy = self.iy;
            cpu.a_shadow = (self.af_ >> 8) as u8;
            cpu.f_shadow = (self.af_ & 0xFF) as u8;
            cpu.b_shadow = (self.bc_ >> 8) as u8;
            cpu.c_shadow = (self.bc_ & 0xFF) as u8;
            cpu.d_shadow = (self.de_ >> 8) as u8;
            cpu.e_shadow = (self.de_ & 0xFF) as u8;
            cpu.h_shadow = (self.hl_ >> 8) as u8;
            cpu.l_shadow = (self.hl_ & 0xFF) as u8;
            cpu.pc = self.pc;
            cpu.sp = self.sp;
            cpu.ei = match self.ei {
                0 => false,
                1 => true,
                _ => panic!("Unexpected EI value {}", self.ei),
            };
            cpu.im = match self.im {
                0 => super::InterruptMode::IM0,
                1 => super::InterruptMode::IM1,
                2 => super::InterruptMode::IM2,
                _ => panic!("Unexpected interrupt mode {}", self.im),
            };
            cpu.iff1 = match self.iff1 {
                0 => false,
                1 => true,
                _ => panic!("Unexpected IFF1 value {}", self.iff1),
            };
            cpu.iff2 = match self.iff2 {
                0 => false,
                1 => true,
                _ => panic!("Unexpected IFF2 value {}", self.iff2),
            };

            cpu
        }

        fn create_memory(&self) -> TestBus {
            let mut bus = TestBus::default();

            for (addr, value) in self.ram.iter() {
                bus.write(*addr, *value);
            }

            bus
        }
    }

    // TODO: handle ports
    #[derive(Deserialize)]
    struct TestCase {
        name: String,
        #[serde(rename = "initial")]
        initial_state: TestCaseState,
        #[serde(rename = "final")]
        final_state: TestCaseState,
        cycles: Vec<(u16, Option<u8>, String)>,
        // ports: Vec<(u16, Option<u8>, String)>,
    }

    macro_rules! z80_tests {
    ($($name:ident => $file:expr),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                execute_test_cases($file);
            }
        )*
    };
}

    fn execute_test_cases(test_file_name: &str) {
        let test_path = "tests/z80/v1";
        let full_test_path = Path::new(test_path).join(test_file_name);
        let test_cases_string = fs::read_to_string(&full_test_path)
            .expect(&format!("Failed to read test file: {full_test_path:?}"));
        let test_cases: Vec<TestCase> =
            serde_json::from_str(&test_cases_string).expect("Test case file failed to deserialize");

        for case in test_cases {
            let mut bus = case.initial_state.create_memory();
            let mut cpu = case.initial_state.create_cpu();

            let cycles = cpu.execute(&mut bus);

            assert_eq!(
                case.cycles.len() as u64,
                cycles,
                "Test Case '{}': CPU cycles produced doesn't match with the test case",
                case.name
            );
            assert_eq!(
                case.final_state.create_memory(),
                bus,
                "Test Case '{}': RAM state doesn't match the expected final state",
                case.name
            );
            assert_eq!(
                case.final_state.create_cpu(),
                cpu,
                "Test Case '{}': CPU state doesn't match the expected final state",
                case.name
            );
        }
    }

    z80_tests! {
         test_nop      => "00.json",
         test_ld_bc_nn => "01.json",
         test_ld_bcp_a => "02.json",
         test_inc_bc   => "03.json",
         test_inc_b    => "04.json",
         test_dec_b    => "05.json",
         test_ld_b_n   => "06.json",
         test_rlca     => "07.json",
         test_ex_af_af => "08.json",
         test_add_hl_bc=> "09.json",
         test_ld_a_bcp => "0a.json",
         test_dec_bc   => "0b.json",
         test_inc_c    => "0c.json",
         test_dec_c    => "0d.json",
         test_ld_c_n   => "0e.json",
         test_rrca     => "0f.json",
         test_djnz_d   => "10.json",
         test_ld_de_nn => "11.json",
         test_ld_dep_a => "12.json",
         test_inc_de   => "13.json",
         test_inc_d    => "14.json",
         test_dec_d    => "15.json",
         test_ld_d_n   => "16.json",
         test_rla      => "17.json",
         test_jr_d     => "18.json",
         test_add_hl_de=> "19.json",
         test_ld_a_dep => "1a.json",
         test_dec_de   => "1b.json",
         test_inc_e    => "1c.json",
         test_dec_e    => "1d.json",
         test_ld_e_n   => "1e.json",
         test_rra      => "1f.json",
         test_jr_nz_d  => "20.json",
         test_ld_hl_nn => "21.json",
         test_ld_nnp_hl=> "22.json",
         test_inc_hl   => "23.json",
         test_inc_h    => "24.json",
         test_dec_h    => "25.json",
         test_ld_h_n   => "26.json",
         test_daa      => "27.json",
         test_jr_z_d   => "28.json",
         test_add_hl_hl=> "29.json",
         test_ld_hl_nnp=> "2a.json",
         test_dec_hl   => "2b.json",
         test_inc_l    => "2c.json",
         test_dec_l    => "2d.json",
         test_ld_l_n   => "2e.json",
         test_cpl      => "2f.json",
         test_jr_nc_d  => "30.json",
         test_ld_sp_nn => "31.json",
         test_ld_nnp_a => "32.json",
         test_inc_sp   => "33.json",
         test_inc_hlp  => "34.json",
         test_dec_hlp  => "35.json",
         test_ld_hlp_n => "36.json",
         test_scf      => "37.json",
         test_jr_c_d   => "38.json",
         test_add_hl_sp=> "39.json",
         test_ld_a_nnp => "3a.json",
         test_dec_sp   => "3b.json",
         test_inc_a    => "3c.json",
         test_dec_a    => "3d.json",
         test_ld_a_n   => "3e.json",
         test_ccf      => "3f.json",
         test_ld_b_b   => "40.json",
         test_ld_b_c   => "41.json",
         test_ld_b_d   => "42.json",
         test_ld_b_e   => "43.json",
         test_ld_b_h   => "44.json",
         test_ld_b_l   => "45.json",
         test_ld_b_hlp => "46.json",
         test_ld_b_a   => "47.json",
         test_ld_c_b   => "48.json",
         test_ld_c_c   => "49.json",
         test_ld_c_d   => "4a.json",
         test_ld_c_e   => "4b.json",
         test_ld_c_h   => "4c.json",
         test_ld_c_l   => "4d.json",
         test_ld_c_hlp => "4e.json",
         test_ld_c_a   => "4f.json",
         test_ld_d_b   => "50.json",
         test_ld_d_c   => "51.json",
         test_ld_d_d   => "52.json",
         test_ld_d_e   => "53.json",
         test_ld_d_h   => "54.json",
         test_ld_d_l   => "55.json",
         test_ld_d_hlp => "56.json",
         test_ld_d_a   => "57.json",
         test_ld_e_b   => "58.json",
         test_ld_e_c   => "59.json",
         test_ld_e_d   => "5a.json",
         test_ld_e_e   => "5b.json",
         test_ld_e_h   => "5c.json",
         test_ld_e_l   => "5d.json",
         test_ld_e_hlp => "5e.json",
         test_ld_e_a   => "5f.json",
         test_ld_h_b   => "60.json",
         test_ld_h_c   => "61.json",
         test_ld_h_d   => "62.json",
         test_ld_h_e   => "63.json",
         test_ld_h_h   => "64.json",
         test_ld_h_l   => "65.json",
         test_ld_h_hlp => "66.json",
         test_ld_h_a   => "67.json",
         test_ld_l_b   => "68.json",
         test_ld_l_c   => "69.json",
         test_ld_l_d   => "6a.json",
         test_ld_l_e   => "6b.json",
         test_ld_l_h   => "6c.json",
         test_ld_l_l   => "6d.json",
         test_ld_l_hlp => "6e.json",
         test_ld_l_a   => "6f.json",
         test_ld_hlp_b => "70.json",
         test_ld_hlp_c => "71.json",
         test_ld_hlp_d => "72.json",
         test_ld_hlp_e => "73.json",
         test_ld_hlp_h => "74.json",
         test_ld_hlp_l => "75.json",
         test_halt     => "76.json",
         test_ld_hlp_a => "77.json",
         test_ld_a_b   => "78.json",
         test_ld_a_c   => "79.json",
         test_ld_a_d   => "7a.json",
         test_ld_a_e   => "7b.json",
         test_ld_a_h   => "7c.json",
         test_ld_a_l   => "7d.json",
         test_ld_a_hlp => "7e.json",
         test_ld_a_a   => "7f.json",
         test_add_a_b   => "80.json",
         test_add_a_c   => "81.json",
         test_add_a_d   => "82.json",
         test_add_a_e   => "83.json",
         test_add_a_h   => "84.json",
         test_add_a_l   => "85.json",
         test_add_a_hlp => "86.json",
         test_add_a_a   => "87.json",
         test_adc_a_b   => "88.json",
         test_adc_a_c   => "89.json",
         test_adc_a_d   => "8a.json",
         test_adc_a_e   => "8b.json",
         test_adc_a_h   => "8c.json",
         test_adc_a_l   => "8d.json",
         test_adc_a_hlp => "8e.json",
         test_adc_a_a   => "8f.json",
         test_sub_b     => "90.json",
         test_sub_c     => "91.json",
         test_sub_d     => "92.json",
         test_sub_e     => "93.json",
         test_sub_h     => "94.json",
         test_sub_l     => "95.json",
         test_sub_hlp   => "96.json",
         test_sub_a     => "97.json",
         test_sbc_a_b   => "98.json",
         test_sbc_a_c   => "99.json",
         test_sbc_a_d   => "9a.json",
         test_sbc_a_e   => "9b.json",
         test_sbc_a_h   => "9c.json",
         test_sbc_a_l   => "9d.json",
         test_sbc_a_hlp => "9e.json",
         test_sbc_a_a   => "9f.json",
         test_and_b     => "a0.json",
         test_and_c     => "a1.json",
         test_and_d     => "a2.json",
         test_and_e     => "a3.json",
         test_and_h     => "a4.json",
         test_and_l     => "a5.json",
         test_and_hlp   => "a6.json",
         test_and_a     => "a7.json",
         test_xor_b     => "a8.json",
         test_xor_c     => "a9.json",
         test_xor_d     => "aa.json",
         test_xor_e     => "ab.json",
         test_xor_h     => "ac.json",
         test_xor_l     => "ad.json",
         test_xor_hlp   => "ae.json",
         test_xor_a     => "af.json",
         test_or_b      => "b0.json",
         test_or_c      => "b1.json",
         test_or_d      => "b2.json",
         test_or_e      => "b3.json",
         test_or_h      => "b4.json",
         test_or_l      => "b5.json",
         test_or_hlp    => "b6.json",
         test_or_a      => "b7.json",
         test_cp_b      => "b8.json",
         test_cp_c      => "b9.json",
         test_cp_d      => "ba.json",
         test_cp_e      => "bb.json",
         test_cp_h      => "bc.json",
         test_cp_l      => "bd.json",
         test_cp_hlp    => "be.json",
         test_cp_a      => "bf.json",
    }
}
