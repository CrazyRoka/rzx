use cpal::{
    BufferSize, OutputCallbackInfo, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use iced::{
    Element, Length, Subscription, Task as Command,
    keyboard::{
        self,
        key::{Code, Physical},
    },
    time,
    widget::{Space, button, column, container, image as iced_image, pick_list, row, text},
};
use ringbuf::{
    HeapRb,
    traits::{Consumer, Producer, Split},
};
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use spectrum::{
    AUDIO_RATE, Keyboard, Spectrum, SpectrumKey, SpectrumMemory, TapePlayer, ULA, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};
use z80::Z80;

const ROM_BYTES_48K_MODEL: &[u8] = include_bytes!("../../../roms/boot-16k-48k.rom");

pub fn main() -> iced::Result {
    iced::application(EmulatorApp::new, EmulatorApp::update, EmulatorApp::view)
        .subscription(EmulatorApp::subscription)
        .window_size((
            WINDOW_WIDTH as f32 * 2.0 + 200.0,
            WINDOW_HEIGHT as f32 * 2.0,
        ))
        .title(EmulatorApp::title)
        .run()
}

// -----------------------------------------------------------------------------
// Shared State between Emulator Thread and UI
// -----------------------------------------------------------------------------
struct SharedState {
    frame_buffer: Vec<u8>, // RGBA8 buffer for Iced Image
    current_fps: f32,
    tape_playing: bool,
}

// Commands sent from UI -> Emulator Thread
enum EmuCommand {
    LoadTape(PathBuf),
    ToggleTape,
    ReloadTape,
    SetModel(String),
    Restart,
    ToggleFpsLimit,
    KeyDown(SpectrumKey),
    KeyUp(SpectrumKey),
}

// -----------------------------------------------------------------------------
// Iced UI Application
// -----------------------------------------------------------------------------
struct EmulatorApp {
    shared_state: Arc<Mutex<SharedState>>,
    command_tx: std::sync::mpsc::Sender<EmuCommand>,

    // UI Local State
    selected_model: String,
    unlimited_fps: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    LoadTapePressed,
    TapeLoaded(Option<PathBuf>),
    ToggleTapePressed,
    ReloadTapePressed,
    ModelSelected(String),
    RestartPressed,
    ToggleFpsPressed,
    EventProcessed(iced::Event),
}

impl EmulatorApp {
    fn new() -> (Self, Command<Message>) {
        let (command_tx, command_rx) = std::sync::mpsc::channel();
        let shared_state = Arc::new(Mutex::new(SharedState {
            frame_buffer: vec![0; WINDOW_WIDTH * WINDOW_HEIGHT * 4],
            current_fps: 0.0,
            tape_playing: false,
        }));

        // Spawn Emulator Thread
        std::thread::spawn({
            let state_clone = Arc::clone(&shared_state);
            move || run_emulator_thread(command_rx, state_clone)
        });

        (
            Self {
                shared_state,
                command_tx,
                selected_model: "48k".to_string(),
                unlimited_fps: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let state = self.shared_state.lock().unwrap();
        format!("ZX Spectrum Emulator - FPS: {:.1}", state.current_fps)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadTapePressed => {
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .add_filter("Tape Files", &["tap"])
                            .pick_file()
                            .await
                            .map(|handle| handle.path().to_path_buf())
                    },
                    Message::TapeLoaded,
                );
            }
            Message::TapeLoaded(Some(path)) => {
                let _ = self.command_tx.send(EmuCommand::LoadTape(path));
            }
            Message::TapeLoaded(None) => {} // User canceled file picker
            Message::ToggleTapePressed => {
                let _ = self.command_tx.send(EmuCommand::ToggleTape);
            }
            Message::ReloadTapePressed => {
                let _ = self.command_tx.send(EmuCommand::ReloadTape);
            }
            Message::ModelSelected(model) => {
                self.selected_model = model.clone();
                let _ = self.command_tx.send(EmuCommand::SetModel(model));
            }
            Message::RestartPressed => {
                let _ = self.command_tx.send(EmuCommand::Restart);
            }
            Message::ToggleFpsPressed => {
                self.unlimited_fps = !self.unlimited_fps;
                let _ = self.command_tx.send(EmuCommand::ToggleFpsLimit);
            }
            Message::EventProcessed(iced::Event::Keyboard(event)) => match event {
                keyboard::Event::KeyPressed { physical_key, .. } => {
                    if let Physical::Code(code) = physical_key {
                        if let Some(s_key) = map_keycode(code) {
                            let _ = self.command_tx.send(EmuCommand::KeyDown(s_key));
                        }
                    }
                }
                keyboard::Event::KeyReleased { physical_key, .. } => {
                    if let Physical::Code(code) = physical_key {
                        if let Some(s_key) = map_keycode(code) {
                            let _ = self.command_tx.send(EmuCommand::KeyUp(s_key));
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let (current_fps, tape_playing, frame_data) = {
            let state = self.shared_state.lock().unwrap();
            (
                state.current_fps,
                state.tape_playing,
                state.frame_buffer.clone(),
            )
        };

        // Left Sidebar Control Panel
        let sidebar = column![
            text("ZX Spectrum").size(24),
            Space::new().height(Length::Fixed(20.0)),
            button(text("Load TAP File..."))
                .width(Length::Fill)
                .on_press(Message::LoadTapePressed),
            button(text(if tape_playing {
                "Stop Tape"
            } else {
                "Play Tape"
            }))
            .width(Length::Fill)
            .on_press(Message::ToggleTapePressed),
            button(text("Rewind Tape"))
                .width(Length::Fill)
                .on_press(Message::ReloadTapePressed),
            Space::new().height(Length::Fixed(20.0)),
            text("Model:"),
            pick_list(
                vec!["16k".to_string(), "48k".to_string()],
                Some(self.selected_model.clone()),
                Message::ModelSelected
            )
            .width(Length::Fill),
            button(text("Restart"))
                .width(Length::Fill)
                .on_press(Message::RestartPressed),
            Space::new().height(Length::Fixed(20.0)),
            button(text(if self.unlimited_fps {
                "Lock FPS (50Hz)"
            } else {
                "Unlock FPS"
            }))
            .width(Length::Fill)
            .on_press(Message::ToggleFpsPressed),
            Space::new().height(Length::Fill),
            text(format!("FPS: {:.1}", current_fps)).size(14),
        ]
        .width(Length::Fixed(200.0))
        .padding(15)
        .spacing(10);

        // Emulator Screen
        let handle =
            iced_image::Handle::from_rgba(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, frame_data);

        let screen = container(
            iced_image::Image::new(handle)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::alignment::Alignment::Center)
        .align_y(iced::alignment::Alignment::Center);

        row![sidebar, screen].into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            time::every(Duration::from_millis(16)).map(|_| Message::Tick), // 60Hz screen refresh
            iced::event::listen().map(Message::EventProcessed),
        ])
    }
}

// -----------------------------------------------------------------------------
// Core Emulator Background Thread
// -----------------------------------------------------------------------------
fn run_emulator_thread(
    rx: std::sync::mpsc::Receiver<EmuCommand>,
    shared_state: Arc<Mutex<SharedState>>,
) {
    // Audio Setup
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Default Audio device can't be found");
    let config = StreamConfig {
        channels: 1,
        sample_rate: AUDIO_RATE as u32,
        buffer_size: BufferSize::Default,
    };
    let (mut prod, mut cons) = HeapRb::<f32>::new(4096).split();
    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [f32], _: &OutputCallbackInfo| {
                for sample in data {
                    *sample = cons.try_pop().unwrap_or(0.0);
                }
            },
            |err| eprintln!("Audio stream error: {err}"),
            None,
        )
        .expect("Failed to create Audio stream");
    stream.play().expect("Failed to play Audio stream");

    // Spectrum Setup
    let keyboard = Rc::new(RefCell::new(Keyboard::new()));
    let mut tape_bytes = Vec::new();
    let tape_player = Rc::new(RefCell::new(TapePlayer::from_tape(&tape_bytes)));

    let mut current_model = "48k".to_string();
    let mut bus = Spectrum::new(
        SpectrumMemory::new_48k(ROM_BYTES_48K_MODEL),
        Rc::clone(&keyboard),
        Rc::clone(&tape_player),
    );
    let mut ula = ULA::new();
    let mut cpu = Z80::new();
    let mut local_buffer: Vec<u32> = vec![0; WINDOW_HEIGHT * WINDOW_WIDTH];

    let mut unlimited_fps = false;
    let mut frame_count = 0;
    let mut last_fps_update = Instant::now();
    let mut last_frame_time = Instant::now();

    loop {
        // Handle UI Commands
        while let Ok(cmd) = rx.try_recv() {
            match cmd {
                EmuCommand::LoadTape(path) => {
                    if let Ok(bytes) = std::fs::read(&path) {
                        tape_bytes = bytes;
                        *tape_player.borrow_mut() = TapePlayer::from_tape(&tape_bytes);
                    }
                }
                EmuCommand::ReloadTape => {
                    *tape_player.borrow_mut() = TapePlayer::from_tape(&tape_bytes);
                }
                EmuCommand::ToggleTape => {
                    let mut tp = tape_player.borrow_mut();
                    if tp.is_playing() {
                        tp.stop();
                    } else {
                        tp.play();
                    }
                }
                EmuCommand::SetModel(model) => {
                    current_model = model;
                    rebuild_machine(
                        &current_model,
                        &keyboard,
                        &tape_player,
                        &mut bus,
                        &mut ula,
                        &mut cpu,
                    );
                }
                EmuCommand::Restart => {
                    rebuild_machine(
                        &current_model,
                        &keyboard,
                        &tape_player,
                        &mut bus,
                        &mut ula,
                        &mut cpu,
                    );
                }
                EmuCommand::ToggleFpsLimit => unlimited_fps = !unlimited_fps,
                EmuCommand::KeyDown(k) => keyboard.borrow_mut().press_key(&k),
                EmuCommand::KeyUp(k) => keyboard.borrow_mut().release_key(&k),
            }
        }

        // Execute 1 frame worth of cycles
        loop {
            let cycles = cpu.execute(&mut bus);
            bus.step(cycles);
            tape_player.borrow_mut().advance(cycles);
            if ula.render(&mut local_buffer, cycles, &bus) {
                cpu.request_int(0xFF);
                break;
            }
        }

        // Process Audio
        let audio_samples = bus.consume_audio();
        if !unlimited_fps {
            let _ = prod.push_slice(audio_samples);
        }

        // Update Shared Framebuffer
        {
            let mut state = shared_state.lock().unwrap();
            state.tape_playing = tape_player.borrow().is_playing();
            state.frame_buffer = local_buffer
                .iter()
                .flat_map(|rgba| rgba.to_be_bytes())
                .collect();
        }

        frame_count += 1;
        let elapsed = last_fps_update.elapsed();
        if elapsed.as_secs_f32() >= 0.5 {
            let fps = frame_count as f32 / elapsed.as_secs_f32();
            shared_state.lock().unwrap().current_fps = fps;
            frame_count = 0;
            last_fps_update = Instant::now();
        }

        // Limit FPS to 50Hz if requested
        if !unlimited_fps {
            let target_frame_time = Duration::from_millis(20); // 50 FPS
            let time_taken = last_frame_time.elapsed();
            if time_taken < target_frame_time {
                std::thread::sleep(target_frame_time - time_taken);
            }
            last_frame_time = Instant::now();
        } else {
            last_frame_time = Instant::now();
        }
    }
}

fn rebuild_machine(
    model: &str,
    keyboard: &Rc<RefCell<Keyboard>>,
    tape_player: &Rc<RefCell<TapePlayer>>,
    bus: &mut Spectrum,
    ula: &mut ULA,
    cpu: &mut Z80,
) {
    *bus = Spectrum::new(
        if model == "16k" {
            SpectrumMemory::new_16k(ROM_BYTES_48K_MODEL)
        } else {
            SpectrumMemory::new_48k(ROM_BYTES_48K_MODEL)
        },
        Rc::clone(keyboard),
        Rc::clone(tape_player),
    );
    *cpu = Z80::new();
    *ula = ULA::new();
}

// -----------------------------------------------------------------------------
// Keyboard Mapping
// -----------------------------------------------------------------------------
fn map_keycode(code: Code) -> Option<SpectrumKey> {
    match code {
        Code::ShiftLeft | Code::ShiftRight => Some(SpectrumKey::CapsShift),
        Code::ControlLeft | Code::ControlRight => Some(SpectrumKey::SymbolShift),
        Code::Enter => Some(SpectrumKey::Enter),
        Code::Space => Some(SpectrumKey::Space),

        Code::KeyZ => Some(SpectrumKey::Z),
        Code::KeyX => Some(SpectrumKey::X),
        Code::KeyC => Some(SpectrumKey::C),
        Code::KeyV => Some(SpectrumKey::V),

        Code::KeyA => Some(SpectrumKey::A),
        Code::KeyS => Some(SpectrumKey::S),
        Code::KeyD => Some(SpectrumKey::D),
        Code::KeyF => Some(SpectrumKey::F),
        Code::KeyG => Some(SpectrumKey::G),

        Code::KeyQ => Some(SpectrumKey::Q),
        Code::KeyW => Some(SpectrumKey::W),
        Code::KeyE => Some(SpectrumKey::E),
        Code::KeyR => Some(SpectrumKey::R),
        Code::KeyT => Some(SpectrumKey::T),

        Code::Digit1 => Some(SpectrumKey::Key1),
        Code::Digit2 => Some(SpectrumKey::Key2),
        Code::Digit3 => Some(SpectrumKey::Key3),
        Code::Digit4 => Some(SpectrumKey::Key4),
        Code::Digit5 => Some(SpectrumKey::Key5),
        Code::Digit0 => Some(SpectrumKey::Key0),
        Code::Digit9 => Some(SpectrumKey::Key9),
        Code::Digit8 => Some(SpectrumKey::Key8),
        Code::Digit7 => Some(SpectrumKey::Key7),
        Code::Digit6 => Some(SpectrumKey::Key6),

        Code::KeyP => Some(SpectrumKey::P),
        Code::KeyO => Some(SpectrumKey::O),
        Code::KeyI => Some(SpectrumKey::I),
        Code::KeyU => Some(SpectrumKey::U),
        Code::KeyY => Some(SpectrumKey::Y),

        Code::KeyL => Some(SpectrumKey::L),
        Code::KeyK => Some(SpectrumKey::K),
        Code::KeyJ => Some(SpectrumKey::J),
        Code::KeyH => Some(SpectrumKey::H),

        Code::KeyM => Some(SpectrumKey::M),
        Code::KeyN => Some(SpectrumKey::N),
        Code::KeyB => Some(SpectrumKey::B),
        _ => None,
    }
}
