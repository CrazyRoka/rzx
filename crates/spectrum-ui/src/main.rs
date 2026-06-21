use std::{cell::RefCell, env::args, fs, io::Error, process::exit, rc::Rc};

use minifb::{Scale, Window, WindowOptions};
use spectrum::{Keyboard, Spectrum16k, ULA, WINDOW_HEIGHT, WINDOW_WIDTH};
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
        for key in window.get_keys_pressed(minifb::KeyRepeat::No) {
            match key {
                _ => {}
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
