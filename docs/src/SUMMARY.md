# Summary

- [Introduction](introduction.md)
- [Architecture Overview](architecture.md)

# CPU

- [The Z80 CPU](./z80/README.md)
  - [Registers & Flags](./z80/registers.md)
  - [Instruction Set](./z80/instructions.md)
  - [Undocumented Opcodes & Flags](./z80/undocumented.md)
  - [Interrupts (IM 0, 1, 2)](./z80/interrupts.md)

# Memory

- [Memory](./memory/README.md)
  - [Memory Map (16K/48K Baseline)](./memory/map.md)
  - [ROM Layout](./memory/rom.md)
  - [Contended Memory](./memory/contention.md)
  - [128K Memory Paging](./memory/paging_128k.md)

# Video

- [The ULA](./ula/README.md)
  - [Port 0xFE (Border, Sound, Keys)](./ula/port_fe.md)
  - [Video Timing (T-states per frame)](./ula/video_timing.md)
  - [Video Memory Layout](./ula/vram_layout.md)
  - [Attributes & Flashing](./ula/attributes.md)
  - [The Floating Bus](./ula/floating_bus.md)
  - [Contended I/O](./ula/contended_io.md)

# Input

- [Input](./input/README.md)
  - [Keyboard Matrix](./input/keyboard.md)
  - [Keyboard Ghosting & Quirks](./input/ghosting.md)
  - [128K Keypad](./input/keypad_128k.md)

# Audio

- [Audio](./audio/README.md)
  - [The 1-Bit Beeper](./audio/beeper.md)
  - [Timing Audio to T-states](./audio/timing.md)
  - [AY-3-8912 Sound Chip (128K)](./audio/ay38912.md)

# Storage

- [Storage & Tape](./tape/README.md)
  - [Tape Data Structure (Pulses)](./tape/structure.md)
  - [The .TAP Format](./tape/tap_format.md)
  - [The .TZX Format](./tape/tzx_format.md)
  - [LOAD/SAVE ROM Routines](./tape/rom_routines.md)
  - [+3 Disk Drive & .DSK Format](./tape/dsk_format.md)

# Hardware Reference

- [Hardware Revisions & Quirks](./quirks/README.md)
  - [Model Variants](./quirks/model_variants.md)
  - [The I Register Snow Bug](./quirks/i_register_snow.md)
  - [EAR/MIC Circuit Delays](./quirks/ear_mic_delays.md)
  - [Model Detection](./quirks/model_detection.md)

# Appendix

- [Testing & Validation](./testing/README.md)
  - [Z80 CPU Tests (ZEXALL/ZEXDOC)](./testing/z80_tests.md)
  - [ULA & Timing Tests (Fusetest)](./testing/fusetest.md)
  - [Real-World Game Tests](./testing/game_tests.md)
- [Appendix: FAQ & References](./appendix/faq_reference.md)
