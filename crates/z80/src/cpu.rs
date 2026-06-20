use crate::bus::Bus;

const FLAG_SIGN: u8 = 1 << 7;
const FLAG_ZERO: u8 = 1 << 6;
const FLAG_HALF_CARRY: u8 = 1 << 4;
const FLAG_PARITY: u8 = 1 << 2;
const FLAG_ADD_OR_SUBTRACT: u8 = 1 << 1;
const FLAG_CARRY: u8 = 1 << 0;

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
        4
    }

    fn read_byte<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let byte = bus.read(self.pc);
        self.pc += 1;
        byte
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

    // TODO: handle other cpu states
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
        // "i": 62,
        // "r": 0,
        // "ei": 0,
        // "wz": 6102,
        ix: u16,
        iy: u16,
        af_: u16,
        bc_: u16,
        de_: u16,
        hl_: u16,
        // "im": 0,
        // "p": 1,
        // "q": 0,
        // "iff1": 0,
        // "iff2": 1,
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
                "{}: CPU cycles produced doesn't match with the test case",
                case.name
            );
            assert_eq!(
                case.final_state.create_memory(),
                bus,
                "{}: RAM state doesn't match the expected final state",
                case.name
            );
            assert_eq!(
                case.final_state.create_cpu(),
                cpu,
                "{}: CPU state doesn't match the expected final state",
                case.name
            );
        }
    }

    z80_tests! {
        test_nop      => "00.json",
    }
}
