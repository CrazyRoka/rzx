use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SpectrumKey {
    // Row 1
    CapsShift,
    Z,
    X,
    C,
    V,
    // Row 2
    A,
    S,
    D,
    F,
    G,
    // Row 3
    Q,
    W,
    E,
    R,
    T,
    // Row 4
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    // Row 5
    Key0,
    Key9,
    Key8,
    Key7,
    Key6,
    // Row 6
    P,
    O,
    I,
    U,
    Y,
    // Row 7
    Enter,
    L,
    K,
    J,
    H,
    // Row 8
    Space,
    SymbolShift,
    M,
    N,
    B,
}

impl SpectrumKey {
    const ROW1: [SpectrumKey; 5] = [
        SpectrumKey::CapsShift,
        SpectrumKey::Z,
        SpectrumKey::X,
        SpectrumKey::C,
        SpectrumKey::V,
    ];
    const ROW2: [SpectrumKey; 5] = [
        SpectrumKey::A,
        SpectrumKey::S,
        SpectrumKey::D,
        SpectrumKey::F,
        SpectrumKey::G,
    ];
    const ROW3: [SpectrumKey; 5] = [
        SpectrumKey::Q,
        SpectrumKey::W,
        SpectrumKey::E,
        SpectrumKey::R,
        SpectrumKey::T,
    ];
    const ROW4: [SpectrumKey; 5] = [
        SpectrumKey::Key1,
        SpectrumKey::Key2,
        SpectrumKey::Key3,
        SpectrumKey::Key4,
        SpectrumKey::Key5,
    ];
    const ROW5: [SpectrumKey; 5] = [
        SpectrumKey::Key0,
        SpectrumKey::Key9,
        SpectrumKey::Key8,
        SpectrumKey::Key7,
        SpectrumKey::Key6,
    ];
    const ROW6: [SpectrumKey; 5] = [
        SpectrumKey::P,
        SpectrumKey::O,
        SpectrumKey::I,
        SpectrumKey::U,
        SpectrumKey::Y,
    ];
    const ROW7: [SpectrumKey; 5] = [
        SpectrumKey::Enter,
        SpectrumKey::L,
        SpectrumKey::K,
        SpectrumKey::J,
        SpectrumKey::H,
    ];
    const ROW8: [SpectrumKey; 5] = [
        SpectrumKey::Space,
        SpectrumKey::SymbolShift,
        SpectrumKey::M,
        SpectrumKey::N,
        SpectrumKey::B,
    ];
    pub const ALL_KEYS: [SpectrumKey; 40] = {
        let mut flat = [SpectrumKey::CapsShift; 40];

        let matrix = [
            SpectrumKey::ROW1,
            SpectrumKey::ROW2,
            SpectrumKey::ROW3,
            SpectrumKey::ROW4,
            SpectrumKey::ROW5,
            SpectrumKey::ROW6,
            SpectrumKey::ROW7,
            SpectrumKey::ROW8,
        ];

        let mut idx = 0;
        let mut row = 0;
        let mut col = 0;
        while row < matrix.len() {
            col = 0;

            while col < matrix[row].len() {
                flat[idx] = matrix[row][col];
                idx += 1;
                col += 1;
            }

            row += 1;
        }

        flat
    };
}

pub struct Keyboard {
    pressed_keys: HashSet<SpectrumKey>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            pressed_keys: HashSet::with_capacity(SpectrumKey::ALL_KEYS.len()),
        }
    }

    pub fn is_pressed(&self, key: &SpectrumKey) -> bool {
        self.pressed_keys.contains(key)
    }

    pub fn press_key(&mut self, key: &SpectrumKey) {
        self.pressed_keys.insert(*key);
    }

    pub fn release_key(&mut self, key: &SpectrumKey) {
        self.pressed_keys.remove(key);
    }

    pub fn any_key_pressed(&self) -> bool {
        !self.pressed_keys.is_empty()
    }

    pub fn pressed_keys_count(&self) -> usize {
        self.pressed_keys.len()
    }

    pub fn reset(&mut self) {
        self.pressed_keys.clear();
    }

    fn select_row(row: u8) -> &'static [SpectrumKey] {
        match row {
            0 => &SpectrumKey::ROW1,
            1 => &SpectrumKey::ROW2,
            2 => &SpectrumKey::ROW3,
            3 => &SpectrumKey::ROW4,
            4 => &SpectrumKey::ROW5,
            5 => &SpectrumKey::ROW6,
            6 => &SpectrumKey::ROW7,
            7 => &SpectrumKey::ROW8,
            _ => panic!("Unexpected row {row}"),
        }
    }

    pub fn read_row(&self, row: u8) -> u8 {
        let mut result = 0xFF;
        let row = Self::select_row(row);

        for (idx, key) in row.iter().enumerate() {
            if self.is_pressed(key) {
                result &= !(1 << idx);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Construction and Initialization Tests
    // =========================================================================

    #[test]
    fn new_keyboard_has_all_keys_unpressed() {
        let keyboard = Keyboard::new();

        for key in SpectrumKey::ALL_KEYS {
            assert!(
                !keyboard.is_pressed(&key),
                "Key {:?} should not be pressed on new keyboard",
                key
            );
        }
    }

    #[test]
    fn new_keyboard_all_rows_return_all_bits_set() {
        let keyboard = Keyboard::new();

        // On ZX Spectrum, unpressed keys return all 1s (0b11111 = 0x1F for 5 bits)
        // or often extended to 8 bits (0xFF)
        for row in 0..8 {
            let row_value = keyboard.read_row(row);
            assert_eq!(
                row_value, 0xFF,
                "Row {} should return 0xFF when no keys pressed",
                row
            );
        }
    }

    // =========================================================================
    // Single Key Press/Release Tests
    // =========================================================================

    #[test]
    fn press_key_reports_as_pressed() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A);
        assert!(keyboard.is_pressed(&SpectrumKey::A));
    }

    #[test]
    fn press_key_only_affects_that_key() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A);

        assert!(keyboard.is_pressed(&SpectrumKey::A));
        assert!(!keyboard.is_pressed(&SpectrumKey::S));
        assert!(!keyboard.is_pressed(&SpectrumKey::D));
        assert!(!keyboard.is_pressed(&SpectrumKey::Q));
    }

    #[test]
    fn release_key_reports_as_unpressed() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A);
        keyboard.release_key(&SpectrumKey::A);

        assert!(!keyboard.is_pressed(&SpectrumKey::A));
    }

    #[test]
    fn release_unpressed_key_does_nothing() {
        let mut keyboard = Keyboard::new();

        // Should not panic or cause issues
        keyboard.release_key(&SpectrumKey::A);
        assert!(!keyboard.is_pressed(&SpectrumKey::A));
    }

    #[test]
    fn press_already_pressed_key_stays_pressed() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A);
        keyboard.press_key(&SpectrumKey::A);

        assert!(keyboard.is_pressed(&SpectrumKey::A));
    }

    #[test]
    fn press_then_release_different_keys() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A);
        keyboard.press_key(&SpectrumKey::S);

        keyboard.release_key(&SpectrumKey::A);

        assert!(!keyboard.is_pressed(&SpectrumKey::A));
        assert!(keyboard.is_pressed(&SpectrumKey::S));
    }

    // =========================================================================
    // Row Reading Tests - Bit Position Verification
    // =========================================================================

    // Row 1: Caps Shift (bit 0), Z (bit 1), X (bit 2), C (bit 3), V (bit 4)
    #[test]
    fn row_1_caps_shift_is_bit_0() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::CapsShift);
        assert_eq!(keyboard.read_row(0), 0xFE); // bit 0 cleared
    }

    #[test]
    fn row_1_z_is_bit_1() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Z);
        assert_eq!(keyboard.read_row(0), 0xFD); // bit 1 cleared
    }

    #[test]
    fn row_1_x_is_bit_2() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::X);
        assert_eq!(keyboard.read_row(0), 0xFB); // bit 2 cleared
    }

    #[test]
    fn row_1_c_is_bit_3() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::C);
        assert_eq!(keyboard.read_row(0), 0xF7); // bit 3 cleared
    }

    #[test]
    fn row_1_v_is_bit_4() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::V);
        assert_eq!(keyboard.read_row(0), 0xEF); // bit 4 cleared
    }

    // Row 2: A (bit 0), S (bit 1), D (bit 2), F (bit 3), G (bit 4)
    #[test]
    fn row_2_a_is_bit_0() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::A);
        assert_eq!(keyboard.read_row(1), 0xFE);
    }

    #[test]
    fn row_2_s_is_bit_1() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::S);
        assert_eq!(keyboard.read_row(1), 0xFD);
    }

    #[test]
    fn row_2_d_is_bit_2() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::D);
        assert_eq!(keyboard.read_row(1), 0xFB);
    }

    #[test]
    fn row_2_f_is_bit_3() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::F);
        assert_eq!(keyboard.read_row(1), 0xF7);
    }

    #[test]
    fn row_2_g_is_bit_4() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::G);
        assert_eq!(keyboard.read_row(1), 0xEF);
    }

    // Row 3: Q (bit 0), W (bit 1), E (bit 2), R (bit 3), T (bit 4)
    #[test]
    fn row_3_q_is_bit_0() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Q);
        assert_eq!(keyboard.read_row(2), 0xFE);
    }

    #[test]
    fn row_3_w_is_bit_1() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::W);
        assert_eq!(keyboard.read_row(2), 0xFD);
    }

    #[test]
    fn row_3_e_is_bit_2() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::E);
        assert_eq!(keyboard.read_row(2), 0xFB);
    }

    #[test]
    fn row_3_r_is_bit_3() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::R);
        assert_eq!(keyboard.read_row(2), 0xF7);
    }

    #[test]
    fn row_3_t_is_bit_4() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::T);
        assert_eq!(keyboard.read_row(2), 0xEF);
    }

    // Row 4: 1 (bit 0), 2 (bit 1), 3 (bit 2), 4 (bit 3), 5 (bit 4)
    #[test]
    fn row_4_key1_is_bit_0() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Key1);
        assert_eq!(keyboard.read_row(3), 0xFE);
    }

    #[test]
    fn row_4_key2_is_bit_1() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Key2);
        assert_eq!(keyboard.read_row(3), 0xFD);
    }

    #[test]
    fn row_4_key3_is_bit_2() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Key3);
        assert_eq!(keyboard.read_row(3), 0xFB);
    }

    #[test]
    fn row_4_key4_is_bit_3() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Key4);
        assert_eq!(keyboard.read_row(3), 0xF7);
    }

    #[test]
    fn row_4_key5_is_bit_4() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Key5);
        assert_eq!(keyboard.read_row(3), 0xEF);
    }

    // Row 5: 0 (bit 0), 9 (bit 1), 8 (bit 2), 7 (bit 3), 6 (bit 4)
    #[test]
    fn row_5_key0_is_bit_0() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Key0);
        assert_eq!(keyboard.read_row(4), 0xFE);
    }

    #[test]
    fn row_5_key9_is_bit_1() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Key9);
        assert_eq!(keyboard.read_row(4), 0xFD);
    }

    #[test]
    fn row_5_key8_is_bit_2() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Key8);
        assert_eq!(keyboard.read_row(4), 0xFB);
    }

    #[test]
    fn row_5_key7_is_bit_3() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Key7);
        assert_eq!(keyboard.read_row(4), 0xF7);
    }

    #[test]
    fn row_5_key6_is_bit_4() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Key6);
        assert_eq!(keyboard.read_row(4), 0xEF);
    }

    // Row 6: P (bit 0), O (bit 1), I (bit 2), U (bit 3), Y (bit 4)
    #[test]
    fn row_6_p_is_bit_0() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::P);
        assert_eq!(keyboard.read_row(5), 0xFE);
    }

    #[test]
    fn row_6_o_is_bit_1() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::O);
        assert_eq!(keyboard.read_row(5), 0xFD);
    }

    #[test]
    fn row_6_i_is_bit_2() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::I);
        assert_eq!(keyboard.read_row(5), 0xFB);
    }

    #[test]
    fn row_6_u_is_bit_3() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::U);
        assert_eq!(keyboard.read_row(5), 0xF7);
    }

    #[test]
    fn row_6_y_is_bit_4() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Y);
        assert_eq!(keyboard.read_row(5), 0xEF);
    }

    // Row 7: Enter (bit 0), L (bit 1), K (bit 2), J (bit 3), H (bit 4)
    #[test]
    fn row_7_enter_is_bit_0() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Enter);
        assert_eq!(keyboard.read_row(6), 0xFE);
    }

    #[test]
    fn row_7_l_is_bit_1() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::L);
        assert_eq!(keyboard.read_row(6), 0xFD);
    }

    #[test]
    fn row_7_k_is_bit_2() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::K);
        assert_eq!(keyboard.read_row(6), 0xFB);
    }

    #[test]
    fn row_7_j_is_bit_3() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::J);
        assert_eq!(keyboard.read_row(6), 0xF7);
    }

    #[test]
    fn row_7_h_is_bit_4() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::H);
        assert_eq!(keyboard.read_row(6), 0xEF);
    }

    // Row 8: Space (bit 0), Symbol Shift (bit 1), M (bit 2), N (bit 3), B (bit 4)
    #[test]
    fn row_8_space_is_bit_0() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Space);
        assert_eq!(keyboard.read_row(7), 0xFE);
    }

    #[test]
    fn row_8_symbol_shift_is_bit_1() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::SymbolShift);
        assert_eq!(keyboard.read_row(7), 0xFD);
    }

    #[test]
    fn row_8_m_is_bit_2() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::M);
        assert_eq!(keyboard.read_row(7), 0xFB);
    }

    #[test]
    fn row_8_n_is_bit_3() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::N);
        assert_eq!(keyboard.read_row(7), 0xF7);
    }

    #[test]
    fn row_8_b_is_bit_4() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::B);
        assert_eq!(keyboard.read_row(7), 0xEF);
    }

    // =========================================================================
    // Multiple Key Press Tests
    // =========================================================================

    #[test]
    fn two_keys_in_same_row() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A); // bit 0
        keyboard.press_key(&SpectrumKey::S); // bit 1

        assert_eq!(keyboard.read_row(1), 0xFC); // bits 0 and 1 cleared
    }

    #[test]
    fn three_keys_in_same_row() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A); // bit 0
        keyboard.press_key(&SpectrumKey::D); // bit 2
        keyboard.press_key(&SpectrumKey::G); // bit 4

        assert_eq!(keyboard.read_row(1), 0xEA); // bits 0, 2, 4 cleared
    }

    #[test]
    fn all_keys_in_row_pressed() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A);
        keyboard.press_key(&SpectrumKey::S);
        keyboard.press_key(&SpectrumKey::D);
        keyboard.press_key(&SpectrumKey::F);
        keyboard.press_key(&SpectrumKey::G);

        assert_eq!(keyboard.read_row(1), 0xE0); // lower 5 bits cleared
    }

    #[test]
    fn keys_in_different_rows_dont_interfere() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A); // Row 2, bit 0
        keyboard.press_key(&SpectrumKey::Q); // Row 3, bit 0

        assert_eq!(keyboard.read_row(1), 0xFE); // Only A's bit cleared
        assert_eq!(keyboard.read_row(2), 0xFE); // Only Q's bit cleared
        assert_eq!(keyboard.read_row(0), 0xFF); // Unaffected
    }

    #[test]
    fn release_one_of_multiple_pressed_keys() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A);
        keyboard.press_key(&SpectrumKey::S);

        keyboard.release_key(&SpectrumKey::A);

        assert_eq!(keyboard.read_row(1), 0xFD); // Only S's bit cleared
    }

    #[test]
    fn shift_combinations_caps_and_letter() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::CapsShift); // Row 1, bit 0
        keyboard.press_key(&SpectrumKey::A); // Row 2, bit 0

        assert_eq!(keyboard.read_row(0), 0xFE);
        assert_eq!(keyboard.read_row(1), 0xFE);
    }

    #[test]
    fn shift_combinations_symbol_and_number() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::SymbolShift); // Row 8, bit 1
        keyboard.press_key(&SpectrumKey::Key1); // Row 4, bit 0

        assert_eq!(keyboard.read_row(7), 0xFD);
        assert_eq!(keyboard.read_row(3), 0xFE);
    }

    #[test]
    fn cursor_keys_simulation() {
        // Cursor keys on Spectrum are achieved with Caps Shift + 5,6,7,8
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::CapsShift); // Row 1
        keyboard.press_key(&SpectrumKey::Key8); // Row 5, bit 2 (cursor up)

        assert!(keyboard.is_pressed(&SpectrumKey::CapsShift));
        assert!(keyboard.is_pressed(&SpectrumKey::Key8));
    }

    #[test]
    fn break_key_simulation() {
        // Break is Caps Shift + Space
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::CapsShift); // Row 1, bit 0
        keyboard.press_key(&SpectrumKey::Space); // Row 8, bit 0

        assert_eq!(keyboard.read_row(0) & 0x01, 0x00);
        assert_eq!(keyboard.read_row(7) & 0x01, 0x00);
    }

    // =========================================================================
    // Reset Tests
    // =========================================================================

    #[test]
    fn reset_clears_all_pressed_keys() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A);
        keyboard.press_key(&SpectrumKey::Q);
        keyboard.press_key(&SpectrumKey::Space);
        keyboard.press_key(&SpectrumKey::Enter);

        keyboard.reset();

        assert!(!keyboard.is_pressed(&SpectrumKey::A));
        assert!(!keyboard.is_pressed(&SpectrumKey::Q));
        assert!(!keyboard.is_pressed(&SpectrumKey::Space));
        assert!(!keyboard.is_pressed(&SpectrumKey::Enter));
    }

    #[test]
    fn reset_returns_all_rows_to_unpressed() {
        let mut keyboard = Keyboard::new();

        // Press at least one key in each row
        keyboard.press_key(&SpectrumKey::CapsShift); // Row 1
        keyboard.press_key(&SpectrumKey::A); // Row 2
        keyboard.press_key(&SpectrumKey::Q); // Row 3
        keyboard.press_key(&SpectrumKey::Key1); // Row 4
        keyboard.press_key(&SpectrumKey::Key0); // Row 5
        keyboard.press_key(&SpectrumKey::P); // Row 6
        keyboard.press_key(&SpectrumKey::Enter); // Row 7
        keyboard.press_key(&SpectrumKey::Space); // Row 8

        keyboard.reset();

        for row in 0..8 {
            assert_eq!(keyboard.read_row(row), 0xFF);
        }
    }

    #[test]
    fn reset_on_empty_keyboard_does_nothing() {
        let mut keyboard = Keyboard::new();

        keyboard.reset();

        for row in 0..8 {
            assert_eq!(keyboard.read_row(row), 0xFF);
        }
    }

    // =========================================================================
    // Boundary and Edge Case Tests
    // =========================================================================

    #[test]
    fn rapid_press_release_cycles() {
        let mut keyboard = Keyboard::new();

        for _ in 0..1000 {
            keyboard.press_key(&SpectrumKey::A);
            assert!(keyboard.is_pressed(&SpectrumKey::A));

            keyboard.release_key(&SpectrumKey::A);
            assert!(!keyboard.is_pressed(&SpectrumKey::A));
        }
    }

    #[test]
    fn all_keys_pressed_simultaneously() {
        let mut keyboard = Keyboard::new();

        for key in SpectrumKey::ALL_KEYS {
            keyboard.press_key(&key);
        }

        for key in SpectrumKey::ALL_KEYS {
            assert!(keyboard.is_pressed(&key), "Key {:?} should be pressed", key);
        }

        // All rows should have lower 5 bits cleared
        for row in 0..8 {
            assert_eq!(keyboard.read_row(row), 0xE0);
        }
    }

    #[test]
    fn release_all_keys_after_all_pressed() {
        let mut keyboard = Keyboard::new();

        for key in SpectrumKey::ALL_KEYS {
            keyboard.press_key(&key);
        }

        for key in SpectrumKey::ALL_KEYS {
            keyboard.release_key(&key);
        }

        for row in 0..8 {
            assert_eq!(keyboard.read_row(row), 0xFF);
        }
    }

    #[test]
    fn pressing_same_key_multiple_times_is_idempotent() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A);
        keyboard.press_key(&SpectrumKey::A);
        keyboard.press_key(&SpectrumKey::A);

        assert_eq!(keyboard.read_row(1), 0xFE); // Only bit 0 cleared
    }

    #[test]
    fn releasing_same_key_multiple_times_is_idempotent() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::A);
        keyboard.release_key(&SpectrumKey::A);
        keyboard.release_key(&SpectrumKey::A);
        keyboard.release_key(&SpectrumKey::A);

        assert_eq!(keyboard.read_row(1), 0xFF);
    }

    #[test]
    fn keyboard_state_is_independent_between_instances() {
        let mut keyboard1 = Keyboard::new();
        let mut keyboard2 = Keyboard::new();

        keyboard1.press_key(&SpectrumKey::A);
        keyboard2.press_key(&SpectrumKey::Q);

        assert_eq!(keyboard1.read_row(1), 0xFE); // A pressed
        assert_eq!(keyboard1.read_row(2), 0xFF); // Q not pressed

        assert_eq!(keyboard2.read_row(1), 0xFF); // A not pressed
        assert_eq!(keyboard2.read_row(2), 0xFE); // Q pressed
    }

    // =========================================================================
    // Common Key Combination Tests (for BASIC keywords)
    // =========================================================================

    #[test]
    fn typing_hello() {
        let mut keyboard = Keyboard::new();

        // H - Row 7, bit 4
        keyboard.press_key(&SpectrumKey::H);
        assert_eq!(keyboard.read_row(6), 0xEF);
        keyboard.release_key(&SpectrumKey::H);

        // E - Row 3, bit 2
        keyboard.press_key(&SpectrumKey::E);
        assert_eq!(keyboard.read_row(2), 0xFB);
        keyboard.release_key(&SpectrumKey::E);

        // L - Row 7, bit 1
        keyboard.press_key(&SpectrumKey::L);
        assert_eq!(keyboard.read_row(6), 0xFD);
        keyboard.release_key(&SpectrumKey::L);

        // L - Row 7, bit 1
        keyboard.press_key(&SpectrumKey::L);
        assert_eq!(keyboard.read_row(6), 0xFD);
        keyboard.release_key(&SpectrumKey::L);

        // O - Row 6, bit 1
        keyboard.press_key(&SpectrumKey::O);
        assert_eq!(keyboard.read_row(5), 0xFD);
        keyboard.release_key(&SpectrumKey::O);
    }

    #[test]
    fn print_keyword_simulation() {
        let mut keyboard = Keyboard::new();

        // P key alone gives PRINT keyword on Spectrum
        keyboard.press_key(&SpectrumKey::P);
        assert_eq!(keyboard.read_row(5), 0xFE);
        keyboard.release_key(&SpectrumKey::P);
    }

    #[test]
    fn new_keyword_with_shift() {
        let mut keyboard = Keyboard::new();

        // N + Caps Shift gives NEW on some models, or just N
        keyboard.press_key(&SpectrumKey::CapsShift);
        keyboard.press_key(&SpectrumKey::N);

        assert_eq!(keyboard.read_row(0), 0xFE); // Caps Shift
        assert_eq!(keyboard.read_row(7), 0xF7); // N

        keyboard.release_key(&SpectrumKey::N);
        keyboard.release_key(&SpectrumKey::CapsShift);
    }

    #[test]
    fn number_input_sequence() {
        let mut keyboard = Keyboard::new();

        // Type "123"
        keyboard.press_key(&SpectrumKey::Key1);
        assert_eq!(keyboard.read_row(3), 0xFE);
        keyboard.release_key(&SpectrumKey::Key1);

        keyboard.press_key(&SpectrumKey::Key2);
        assert_eq!(keyboard.read_row(3), 0xFD);
        keyboard.release_key(&SpectrumKey::Key2);

        keyboard.press_key(&SpectrumKey::Key3);
        assert_eq!(keyboard.read_row(3), 0xFB);
        keyboard.release_key(&SpectrumKey::Key3);
    }

    #[test]
    fn symbol_shift_with_number_for_special_chars() {
        let mut keyboard = Keyboard::new();

        // Symbol Shift + 1 gives ! on Spectrum
        keyboard.press_key(&SpectrumKey::SymbolShift);
        keyboard.press_key(&SpectrumKey::Key1);

        assert_eq!(keyboard.read_row(7), 0xFD); // Symbol Shift bit 1
        assert_eq!(keyboard.read_row(3), 0xFE); // Key1 bit 0

        keyboard.release_key(&SpectrumKey::Key1);
        keyboard.release_key(&SpectrumKey::SymbolShift);
    }

    // =========================================================================
    // Row Isolation Tests
    // =========================================================================

    #[test]
    fn pressing_key_in_row_1_does_not_affect_other_rows() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::Z);

        assert_ne!(keyboard.read_row(0), 0xFF); // Row 1 affected
        assert_eq!(keyboard.read_row(1), 0xFF); // Row 2 unaffected
        assert_eq!(keyboard.read_row(2), 0xFF); // Row 3 unaffected
        assert_eq!(keyboard.read_row(3), 0xFF); // Row 4 unaffected
        assert_eq!(keyboard.read_row(4), 0xFF); // Row 5 unaffected
        assert_eq!(keyboard.read_row(5), 0xFF); // Row 6 unaffected
        assert_eq!(keyboard.read_row(6), 0xFF); // Row 7 unaffected
        assert_eq!(keyboard.read_row(7), 0xFF); // Row 8 unaffected
    }

    #[test]
    fn pressing_key_in_row_5_does_not_affect_other_rows() {
        let mut keyboard = Keyboard::new();

        keyboard.press_key(&SpectrumKey::Key7);

        assert_eq!(keyboard.read_row(0), 0xFF); // Row 1 unaffected
        assert_eq!(keyboard.read_row(1), 0xFF); // Row 2 unaffected
        assert_eq!(keyboard.read_row(2), 0xFF); // Row 3 unaffected
        assert_eq!(keyboard.read_row(3), 0xFF); // Row 4 unaffected
        assert_ne!(keyboard.read_row(4), 0xFF); // Row 5 affected
        assert_eq!(keyboard.read_row(5), 0xFF); // Row 6 unaffected
        assert_eq!(keyboard.read_row(6), 0xFF); // Row 7 unaffected
        assert_eq!(keyboard.read_row(7), 0xFF); // Row 8 unaffected
    }

    #[test]
    fn one_key_pressed_per_row() {
        let mut keyboard = Keyboard::new();

        // Press first key in each row
        keyboard.press_key(&SpectrumKey::CapsShift); // Row 1
        keyboard.press_key(&SpectrumKey::A); // Row 2
        keyboard.press_key(&SpectrumKey::Q); // Row 3
        keyboard.press_key(&SpectrumKey::Key1); // Row 4
        keyboard.press_key(&SpectrumKey::Key0); // Row 5
        keyboard.press_key(&SpectrumKey::P); // Row 6
        keyboard.press_key(&SpectrumKey::Enter); // Row 7
        keyboard.press_key(&SpectrumKey::Space); // Row 8

        // Each row should have exactly bit 0 cleared
        for row in 0..8 {
            assert_eq!(keyboard.read_row(row), 0xFE);
        }
    }

    // =========================================================================
    // State Query Tests
    // =========================================================================

    #[test]
    fn get_state_of_pressed_key() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::A);

        assert_eq!(keyboard.is_pressed(&SpectrumKey::A), true);
    }

    #[test]
    fn get_state_of_released_key() {
        let keyboard = Keyboard::new();

        assert_eq!(keyboard.is_pressed(&SpectrumKey::A), false);
    }

    #[test]
    fn any_key_pressed_returns_false_when_none_pressed() {
        let keyboard = Keyboard::new();
        assert!(!keyboard.any_key_pressed());
    }

    #[test]
    fn any_key_pressed_returns_true_when_any_pressed() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::Z);
        assert!(keyboard.any_key_pressed());
    }

    #[test]
    fn get_pressed_keys_count_zero() {
        let keyboard = Keyboard::new();
        assert_eq!(keyboard.pressed_keys_count(), 0);
    }

    #[test]
    fn get_pressed_keys_count_multiple() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::A);
        keyboard.press_key(&SpectrumKey::S);
        keyboard.press_key(&SpectrumKey::D);
        assert_eq!(keyboard.pressed_keys_count(), 3);
    }

    #[test]
    fn get_pressed_keys_count_after_release() {
        let mut keyboard = Keyboard::new();
        keyboard.press_key(&SpectrumKey::A);
        keyboard.press_key(&SpectrumKey::S);
        keyboard.release_key(&SpectrumKey::A);
        assert_eq!(keyboard.pressed_keys_count(), 1);
    }
}
