# The ULA

The ULA (Uncommitted Logic Array) is the Spectrum's custom silicon — a hardwired state machine that generates the video signal, reads the keyboard, outputs audio to the beeper and tape, and triggers interrupts. It is **not** a programmable GPU; every function is baked into the gate array.

## Pinout (40-pin DIP)

```
            +------+
  /CAS   1  |      | 40  GND
  /WR    2  |      | 39  Q (14 MHz crystal)
  /RD    3  |      | 38  /MREQ
  /WE    4  |      | 37  A15
  A0     5  |      | 36  A14
  A1     6  |      | 35  /RAS
  A2     7  |      | 34  /ROM CS
  A3     8  |      | 33  /IO-ULA
  A4     9  |      | 32  CLOCK (to CPU)
  A5    10  |      | 31  D7
  A6    11  |      | 30  D6
  /INT  12  |      | 29  D5
  +5V   13  |      | 28  SOUND (EAR/MIC/beeper)
  +5V   14  |      | 27  D4
  U     15  |      | 26  T4 (keyboard column 4)
  V     16  |      | 25  D3
  /Y    17  |      | 24  T3 (keyboard column 3)
  D0    18  |      | 23  T2 (keyboard column 2)
  T0    19  |      | 22  D2
  T1    20  |      | 21  D1
            +------+
```

## Signal Descriptions

| Pin(s) | Name | Function |
|---|---|---|
| 1 | /CAS | Column address strobe for DRAM |
| 2 | /WR | Write strobe to DRAM |
| 3 | /RD | Read strobe from DRAM |
| 4 | /WE | Write enable for DRAM |
| 5–11 | A0–A6 | Address lines to DRAM |
| 12 | /INT | Interrupt to Z80 (50 Hz) |
| 13–14 | +5V | Power supply, decoupled through RC low-pass |
| 15 | U | Colour-difference signal (blue minus luminance) |
| 16 | V | Colour-difference signal (red minus luminance) |
| 17 | /Y | Composite video including sync (inverted) |
| 18, 21–22, 25, 27, 29–31 | D0–D7 | Data lines, decoupled from the CPU by resistors |
| 19–20, 23–24, 26 | T0–T4 | Keyboard column read lines (via diodes from address lines) |
| 28 | SOUND | Analogue I/O for beeper, tape save and load (EAR/MIC) |
| 32 | CLOCK | Clock output to CPU, including inhibited T-states during contention |
| 33 | /IO-ULA | Generated as `(A0(CPU) OR /IORQ)` — selects I/O port 0xFE |
| 34 | /ROM CS | Chip select for the 16K ROM |
| 35 | /RAS | Row address strobe for DRAM |
| 36–37 | A14–A15 | High address lines to DRAM and ROM |
| 38 | /MREQ | Memory request (from Z80, monitored by ULA) |
| 39 | Q | 14 MHz crystal input (other side grounded through a capacitor) |
| 40 | GND | Ground |

## Functional Overview

The ULA is a state machine that cycles through the video frame, reading screen and attribute bytes from the lower 16K of RAM every 4 T-states during the active display period. Between these reads it handles:

- **Video generation:** produces the U, V colour-difference signals and the composite sync /Y signal.
- **Memory contention:** monitors /MREQ and address lines; if the CPU tries to access contended RAM during a ULA read cycle, the ULA pulls the CLOCK line (pin 32) to insert wait states.
- **Keyboard scanning:** drives address lines through diodes to select rows, reads back column state via T0–T4.
- **Audio output:** pin 28 (SOUND) is shared between the internal speaker, EAR output, and MIC input, with the voltage level determined by bits 3 and 4 of port 0xFE.
- **Interrupt generation:** pulls /INT low at the start of each video frame (50 Hz on PAL models).

## Subsections

- [Port 0xFE (Border, Sound, Keys)](./port_fe.md) — the ULA's main I/O port
- [Video Timing (T-states per frame)](./video_timing.md) — scanline timings and frame structure
- [Video Memory Layout](./vram_layout.md) — pixel and attribute memory mapping
- [Attributes & Flashing](./attributes.md) — colour attributes and the flash mechanism
- [The Floating Bus](./floating_bus.md) — reading undefined data from the ULA bus
- [Contended I/O](./contended_io.md) — contention during IN/OUT instructions
- [Video Output](./video_output.md) — composite and RGB video connectors
