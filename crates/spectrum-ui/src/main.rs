use std::{cell::RefCell, env::args, fs, io::Error, process::exit, rc::Rc, time::Instant};

use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use spectrum::{Keyboard, Spectrum16k, SpectrumKey, TapePlayer, ULA, WINDOW_HEIGHT, WINDOW_WIDTH};
use z80::{Bus, Z80};

fn main() -> Result<(), Error> {
    if args().len() != 3 {
        eprintln!("Expected ROM path and TAP path as argument.");
        exit(-1);
    }

    let rom_path = args().nth(1).expect("Argument should be present");
    println!("Loading ROM from path: {}", rom_path);
    let rom_bytes = fs::read(rom_path)?;

    let tap_path = args().nth(2).expect("Argument should be present");
    println!("Loading TAP from path: {}", tap_path);
    let tap_bytes = fs::read(tap_path)?;
    let tape_player = Rc::new(RefCell::new(TapePlayer::from_tape(&tap_bytes)));

    let keyboard = Rc::new(RefCell::new(Keyboard::new()));
    let mut bus = Spectrum16k::new(&rom_bytes, Rc::clone(&keyboard), Rc::clone(&tape_player));
    let mut ula = ULA::new();
    let mut cpu = Z80::new();

    let mut buffer: Vec<u32> = vec![0; WINDOW_HEIGHT * WINDOW_WIDTH];
    let mut window = match Window::new(
        "ZX Spectrum Emulator",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => {
            panic!("Failed to create a window: {}", err);
        }
    };
    window.set_target_fps(50);
    let mut unlimited_fps = false;
    let mut last_fps_update = Instant::now();
    let mut frame_count = 0;

    while window.is_open() {
        keyboard.borrow_mut().reset();
        for key in window.get_keys() {
            if let Some(spectrum_key) = convert_to_spectrum_key(key) {
                keyboard.borrow_mut().press_key(&spectrum_key);
            }
        }

        if window.is_key_pressed(Key::F1, KeyRepeat::No) {
            unlimited_fps = !unlimited_fps;
            if unlimited_fps {
                window.set_target_fps(0);
            } else {
                window.set_target_fps(50);
            }
        }

        if window.is_key_pressed(Key::F2, KeyRepeat::No) {
            if tape_player.borrow().is_playing() {
                tape_player.borrow_mut().stop();
            } else {
                tape_player.borrow_mut().play();
            }
        }

        loop {
            let cycles = cpu.execute(&mut bus);
            tape_player.borrow_mut().advance(cycles);
            if ula.render(&mut buffer, cycles, &bus) {
                cpu.request_int(0xFF);
                break;
            }
        }

        if let Err(err) = window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT) {
            panic!("Failed to update window: {}", err);
        }

        frame_count += 1;
        let elapsed = last_fps_update.elapsed();
        if elapsed.as_secs_f32() >= 0.5 {
            let fps = frame_count as f32 / elapsed.as_secs_f32();
            let mode_str = if unlimited_fps {
                "Unlimited"
            } else {
                "Locked (50Hz)"
            };

            window.set_title(&format!(
                "ZX Spectrum Emulator | FPS: {:.1} | Mode: {} [Press F1 to Toggle]",
                fps, mode_str
            ));

            frame_count = 0;
            last_fps_update = Instant::now();
        }
    }

    Ok(())
}

fn convert_to_spectrum_key(key: Key) -> Option<SpectrumKey> {
    match key {
        Key::LeftShift => Some(SpectrumKey::CapsShift),
        Key::Z => Some(SpectrumKey::Z),
        Key::X => Some(SpectrumKey::X),
        Key::C => Some(SpectrumKey::C),
        Key::V => Some(SpectrumKey::V),
        Key::A => Some(SpectrumKey::A),
        Key::S => Some(SpectrumKey::S),
        Key::D => Some(SpectrumKey::D),
        Key::F => Some(SpectrumKey::F),
        Key::G => Some(SpectrumKey::G),
        Key::Q => Some(SpectrumKey::Q),
        Key::W => Some(SpectrumKey::W),
        Key::E => Some(SpectrumKey::E),
        Key::R => Some(SpectrumKey::R),
        Key::T => Some(SpectrumKey::T),
        Key::Key1 => Some(SpectrumKey::Key1),
        Key::Key2 => Some(SpectrumKey::Key2),
        Key::Key3 => Some(SpectrumKey::Key3),
        Key::Key4 => Some(SpectrumKey::Key4),
        Key::Key5 => Some(SpectrumKey::Key5),
        Key::Key0 => Some(SpectrumKey::Key0),
        Key::Key9 => Some(SpectrumKey::Key9),
        Key::Key8 => Some(SpectrumKey::Key8),
        Key::Key7 => Some(SpectrumKey::Key7),
        Key::Key6 => Some(SpectrumKey::Key6),
        Key::P => Some(SpectrumKey::P),
        Key::O => Some(SpectrumKey::O),
        Key::I => Some(SpectrumKey::I),
        Key::U => Some(SpectrumKey::U),
        Key::Y => Some(SpectrumKey::Y),
        Key::Enter => Some(SpectrumKey::Enter),
        Key::L => Some(SpectrumKey::L),
        Key::K => Some(SpectrumKey::K),
        Key::J => Some(SpectrumKey::J),
        Key::H => Some(SpectrumKey::H),
        Key::Space => Some(SpectrumKey::Space),
        Key::LeftCtrl => Some(SpectrumKey::SymbolShift),
        Key::M => Some(SpectrumKey::M),
        Key::N => Some(SpectrumKey::N),
        Key::B => Some(SpectrumKey::B),
        _ => None,
    }
}
