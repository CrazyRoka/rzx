# The ULA

TODO: Write overview of the Spectrum's ULA (Uncommitted Logic Array) — its role as video generator, keyboard interface, audio output, cassette interface, and interrupt source. Explain the hardwired state machine design and how it differs from a programmable GPU.

## Subsections

- [Port 0xFE (Border, Sound, Keys)](./port_fe.md) — the ULA's main I/O port
- [Video Timing (T-states per frame)](./video_timing.md) — scanline timings and frame structure
- [Video Memory Layout](./vram_layout.md) — pixel and attribute memory mapping
- [Attributes & Flashing](./attributes.md) — colour attributes and the flash mechanism
- [The Floating Bus](./floating_bus.md) — reading undefined data from the ULA bus
- [Contended I/O](./contended_io.md) — contention during IN/OUT instructions
