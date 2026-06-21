use std::{cell::RefCell, env::args, fs, io::Error, process::exit, rc::Rc};

use minifb::{Key, Scale, Window, WindowOptions};
use spectrum::{Keyboard, Spectrum16k, SpectrumKey, ULA, WINDOW_HEIGHT, WINDOW_WIDTH};
use z80::{Bus, Z80};

fn main() -> Result<(), Error> {
    if args().len() != 2 {
        eprintln!("Expected ROM path as argument.");
        exit(-1);
    }

    let rom_path = args().last().expect("Argument should be present");
    println!("Loading ROM from path: {}", rom_path);

    let rom_bytes = fs::read(rom_path)?;
    let keyboard = Rc::new(RefCell::new(Keyboard::new()));
    let mut bus = Spectrum16k::new(&rom_bytes, Rc::clone(&keyboard));
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

    while window.is_open() {
        keyboard.borrow_mut().reset();
        for key in window.get_keys_pressed(minifb::KeyRepeat::Yes) {
            if let Some(spectrum_key) = convert_to_spectrum_key(key) {
                keyboard.borrow_mut().press_key(&spectrum_key);
            }
        }

        loop {
            let cycles = cpu.execute(&mut bus);
            if ula.render(&mut buffer, cycles, &bus) {
                cpu.request_int(0xFF);
                break;
            }
        }

        if let Err(err) = window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT) {
            panic!("Failed to update window: {}", err);
        }
    }

    Ok(())
}

fn convert_to_spectrum_key(key: Key) -> Option<SpectrumKey> {
    match key {
        Key::CapsLock => Some(SpectrumKey::CapsShift),
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
        Key::LeftShift | Key::RightShift => Some(SpectrumKey::SymbolShift),
        Key::M => Some(SpectrumKey::M),
        Key::N => Some(SpectrumKey::N),
        Key::B => Some(SpectrumKey::B),
        _ => None,
    }
}
