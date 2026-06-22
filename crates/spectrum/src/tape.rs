use std::collections::VecDeque;

// ZX Spectrum Tape Constants
const PILOT_DURATION: u64 = 2168;
const SYNC1_DURATION: u64 = 667;
const SYNC2_DURATION: u64 = 735;
const BIT0_DURATION: u64 = 855;
const BIT1_DURATION: u64 = 1710;
const PAUSE_DURATION: u64 = 3_500_000; // 1 second in T-states

#[derive(Debug)]
pub struct TapePlayer {
    durations: VecDeque<u64>,
    ear: bool,
    playing: bool,
}

impl TapePlayer {
    // TODO: consider returning ERROR
    pub fn from_tape(data: &[u8]) -> Self {
        let mut durations = VecDeque::new();

        let mut idx = 0;
        while idx < data.len() {
            let low = data[idx] as usize;
            let high = data[idx + 1] as usize;
            idx += 2;

            let bytes_to_follow = (high << 8) | low;
            let block = &data[idx..idx + bytes_to_follow];
            idx += bytes_to_follow;

            let flag = block[0];
            let pulses = match flag {
                0x00 => 8063,
                0xFF => 3223,
                _ => panic!("Unexpected TAP header flag {:#02X}", flag),
            };

            for _ in 0..pulses {
                durations.push_back(PILOT_DURATION);
            }
            durations.push_back(SYNC1_DURATION);
            durations.push_back(SYNC2_DURATION);

            for byte in block {
                for bit in (0..8).rev() {
                    let p = if ((byte >> bit) & 1) != 0 {
                        BIT1_DURATION
                    } else {
                        BIT0_DURATION
                    };
                    durations.push_back(p);
                    durations.push_back(p);
                }
            }

            durations.push_back(PAUSE_DURATION);
        }

        Self {
            durations,
            ear: false,
            playing: false,
        }
    }

    pub fn advance(&mut self, mut cycles: u64) {
        if !self.is_playing() {
            return;
        }

        while cycles > 0 && !self.durations.is_empty() {
            let duration = self.durations.front_mut().expect("Data should be there");
            if *duration > cycles {
                *duration -= cycles;
                break;
            } else {
                cycles -= *duration;
                self.durations.pop_front();
                self.ear = !self.ear;
            }
        }
    }

    pub fn ear(&self) -> bool {
        self.ear
    }

    pub fn is_playing(&self) -> bool {
        self.playing && !self.durations.is_empty()
    }

    pub fn play(&mut self) {
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to generate a minimal valid TAP block.
    fn make_tap_block(flag: u8, data: &[u8]) -> Vec<u8> {
        let mut block = Vec::new();
        let payload_len = data.len() + 2; // +1 for flag, +1 for checksum

        block.push((payload_len & 0xFF) as u8);
        block.push(((payload_len >> 8) & 0xFF) as u8);
        block.push(flag);
        block.extend_from_slice(data);

        let mut checksum = flag;
        for &byte in data {
            checksum ^= byte;
        }
        block.push(checksum);
        block
    }

    #[test]
    fn test_initial_state() {
        let tape_data = make_tap_block(0x00, &[]);
        let player = TapePlayer::from_tape(&tape_data);

        assert!(
            !player.is_playing(),
            "Player should not start playing automatically upon loading"
        );
        assert!(!player.ear, "EAR should be FALSE initially");
    }

    #[test]
    fn test_play_and_stop_state_transitions() {
        let tape_data = make_tap_block(0x00, &[]);
        let mut player = TapePlayer::from_tape(&tape_data);

        player.play();
        assert!(
            player.is_playing(),
            "Player should  start playing when requested"
        );

        player.stop();
        assert!(
            !player.is_playing(),
            "Player should stop playing when requested"
        );
    }

    #[test]
    fn test_empty_tape_is_never_playing() {
        let tape_data = make_tap_block(0x00, &[]);
        let mut player = TapePlayer::from_tape(&tape_data);

        player.play();
        assert!(
            player.is_playing(),
            "Player should start playing when requested"
        );

        player.durations.clear();
        assert!(
            !player.is_playing(),
            "Player should stop playing when empty"
        );
    }

    #[test]
    fn test_advance_before_play() {
        let tape_data = make_tap_block(0x00, &[]);
        let mut player = TapePlayer::from_tape(&tape_data);
        let initial_ear = player.ear();

        // Advancing before playing should have no effect
        player.advance(10_000_000);

        assert!(!player.is_playing(), "Player should remain stopped");
        assert_eq!(
            player.ear(),
            initial_ear,
            "EAR should not change while stopped"
        );

        // Now start playing and verify it still functions from the beginning
        player.play();
        assert!(
            player.is_playing(),
            "Player should start playing after play() is called"
        );

        player.advance(PILOT_DURATION);
        assert_ne!(
            player.ear(),
            initial_ear,
            "EAR should toggle now that play has started"
        );
    }

    #[test]
    fn test_empty_tape_does_not_play() {
        let data: [u8; 0] = [];
        let mut player = TapePlayer::from_tape(&data);

        player.play();
        assert!(
            !player.is_playing(),
            "An empty tape should not transition to playing"
        );
    }

    #[test]
    fn test_header_block_pilot_pulses() {
        let tape_data = make_tap_block(0x00, &[]);
        let mut player = TapePlayer::from_tape(&tape_data);

        player.play();
        assert!(player.is_playing());
        let initial_ear = player.ear();

        player.advance(PILOT_DURATION - 1);
        assert_eq!(
            player.ear(),
            initial_ear,
            "EAR should not change before duration expires"
        );

        player.advance(1);
        assert_ne!(
            player.ear(),
            initial_ear,
            "EAR should toggle after the first pilot pulse"
        );
    }

    #[test]
    fn test_data_block_pilot_pulses() {
        let tape_data = make_tap_block(0xFF, &[]);
        let mut player = TapePlayer::from_tape(&tape_data);

        player.play();
        assert!(player.is_playing());

        let mut transitions = 0;
        let mut current_ear = player.ear();

        for _ in 0..3223 {
            player.advance(PILOT_DURATION);
            if player.ear() != current_ear {
                transitions += 1;
                current_ear = player.ear();
            }
        }

        assert_eq!(
            transitions, 3223,
            "Expected exactly 3223 pilot pulse transitions for a data block"
        );
    }

    #[test]
    fn test_sync_pulses() {
        let tape_data = make_tap_block(0x00, &[]);
        let mut player = TapePlayer::from_tape(&tape_data);

        player.play();
        player.advance(8063 * PILOT_DURATION);

        let ear_before_sync = player.ear();

        player.advance(SYNC1_DURATION);
        assert_ne!(
            player.ear(),
            ear_before_sync,
            "EAR should toggle after Sync 1"
        );

        let ear_after_sync1 = player.ear();

        player.advance(SYNC2_DURATION);
        assert_ne!(
            player.ear(),
            ear_after_sync1,
            "EAR should toggle after Sync 2"
        );
    }

    #[test]
    fn test_data_bits_encoding() {
        let tape_data = make_tap_block(0x00, &[0x40]);
        let mut player = TapePlayer::from_tape(&tape_data);

        player.play();
        player.advance(8063 * PILOT_DURATION + SYNC1_DURATION + SYNC2_DURATION);

        let mut ear_state = player.ear();
        // Flag byte (0x00) -> 16 pulses of BIT0_DURATION
        for _ in 0..16 {
            player.advance(BIT0_DURATION);
            assert_ne!(player.ear(), ear_state);
            ear_state = player.ear();
        }

        // Data byte (0x40) -> '0' bit (2 pulses of 855), '1' bit (2 pulses of 1710)
        // Bit 7: '0'
        for _ in 0..2 {
            player.advance(BIT0_DURATION);
            assert_ne!(player.ear(), ear_state);
            ear_state = player.ear();
        }

        // Bit 6: '1'
        for _ in 0..2 {
            player.advance(BIT1_DURATION);
            assert_ne!(player.ear(), ear_state);
            ear_state = player.ear();
        }

        // Bits 5 to 0: '0' (should be BIT0_DURATION)
        for _ in 0..6 {
            for _ in 0..2 {
                player.advance(BIT0_DURATION);
                assert_ne!(player.ear(), ear_state);
                ear_state = player.ear();
            }
        }
    }

    #[test]
    fn test_block_pause_and_stop() {
        let tape_data = make_tap_block(0x00, &[]);
        let mut player = TapePlayer::from_tape(&tape_data);

        player.play();

        let total_active_cycles =
            (8063 * PILOT_DURATION) + SYNC1_DURATION + SYNC2_DURATION + (16 * 2 * BIT0_DURATION);

        player.advance(total_active_cycles);
        assert!(
            player.is_playing(),
            "Player should still be in the pause period"
        );

        player.advance(PAUSE_DURATION - 1);
        assert!(
            player.is_playing(),
            "Player should still be playing at the last cycle of pause"
        );

        player.advance(1);
        assert!(
            !player.is_playing(),
            "Player should stop playing once the tape is exhausted"
        );
    }

    #[test]
    fn test_multiple_blocks() {
        let block1 = make_tap_block(0x00, &[]);
        let block2 = make_tap_block(0xFF, &[]);

        let mut tape_data = Vec::new();
        tape_data.extend(block1);
        tape_data.extend(block2);

        let mut player = TapePlayer::from_tape(&tape_data);
        player.play();
        assert!(player.is_playing());

        let block1_cycles = (8063 * PILOT_DURATION)
            + SYNC1_DURATION
            + SYNC2_DURATION
            + (16 * 2 * BIT0_DURATION)
            + PAUSE_DURATION;

        player.advance(block1_cycles);
        assert!(
            player.is_playing(),
            "Player should transition immediately to block 2"
        );

        let ear_start = player.ear();
        player.advance(3223 * PILOT_DURATION);
        assert_ne!(
            player.ear(),
            ear_start,
            "EAR should have toggled through the second pilot phase"
        );
    }
}
