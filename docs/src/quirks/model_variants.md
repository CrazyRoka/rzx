# Model Variants

## Timing Comparison

| Model | CPU speed | T-states/frame | Scanlines | T-states/line | Lines before picture | Border change at |
|---|---|---|---|---|---|---|
| 16K / 48K / + | 3.5 MHz | 69888 | 312 | 224 | 64 | 14339–14342 |
| 128K / +2 | ~3.5469 MHz | 70908 | 311 | 228 | 63 | 14365–14368 |
| +2A / +2B / +3 | ~3.5469 MHz | 70908 | 311 | 228 | 63 | 14365–14368 |

## Contention Summary

| Model | Contention start | Pattern | Repeat | Contended banks |
|---|---|---|---|---|
| 16K / 48K / + | 14335 | 6,5,4,3,2,1,0,0 | 224 | `0x4000–0x7FFF` |
| 128K / +2 | 14361 | 6,5,4,3,2,1,0,0 | 228 | 1, 3, 5, 7 |
| +2A / +2B / +3 | 14365 | 1,0,7,6,5,4,3,2,1,0 | 228 | 4, 5, 6, 7 |

## Per-Model Details

### 16K (1982)
- Original Sinclair model, 16 KB RAM (only `0x4000–0x7FFF` populated)
- Upper 32 KB may mirror lower RAM on some boards
- Identical timing to 48K
- Power supply: 9 V DC @ 1.4 A, centre -ve
- Membrane keyboard, 40 keys

### 48K (1982)
- The baseline reference model. 48 KB RAM.
- Most documentation assumes this model unless noted.
- Power supply: 9 V DC @ 1.4 A, centre -ve
- Membrane keyboard, 40 keys (later + model: 58 keys)

### Spectrum+ (1984)
- 48K internals in a new keyboard case with a reset button.
- Identical to 48K for emulation purposes.
- 58-key keyboard.

### 128K (1985, Sinclair — "Toastrack")
- First model with 128 KB RAM and bank switching.
- AY-3-8912 sound chip.
- Numeric keypad with editing functions (58 keys total).
- Faster CPU clock (~3.5469 MHz) and altered video timing.
- RGB output via standard monitor connector.
- Built-in MIDI interface.
- The I register snow bug also exists on this model, and additionally crashes the machine shortly after I is set to point to contended memory.
- Port 0xFE bit 6 behaviour matches Issue 3.
- **HAL10H8 read bug:** reading port `0x7FFD` crashes the machine (HAL does not distinguish reads from writes). Later +2s fix this.
- **HAL10H8 contention bug:** contends banks 1, 3, 5, 7 (not 4, 5, 6, 7 as documented in the service manual) due to swapped inputs in the HAL.
- Power supply: 9 V DC @ 1.85 A, centre -ve.
- Nicknamed **"Toastrack"** for the large external heatsink on the right side.

### +2 (1986, Amstrad — Grey)
- 128K hardware in a grey case with built-in cassette deck.
- Two built-in Sinclair-style joystick ports (non-standard pinout).
- Timing and memory identical to 128K. No MIDI interface.
- Keyboard: 58 plastic keys with metal springs over a plastic membrane.
- PCB revisions: **Z70500** (Taiwan, earliest), **Z70700** (UK, later), **0500** (fixed HAL10H8 — reading port `0x7FFD` does not crash).
- Power supply: 9 V DC @ 2.1 A, centre -ve.

### +3 (1987, Amstrad)
- 128K with a 3" floppy disk drive, WD1770 FDC.
- Four ROM banks totalling 64 KB (editor, syntax checker, +3DOS, 48K BASIC).
- Additional paging port `0x1FFD` with special memory modes.
- Port 0xFE bit 6 always returns 0 (no EAR/MIC dependency).
- No floating bus — unused ports always return 255.
- Port 0xFE is not contended.
- Different contention pattern and combined instruction entries.
- Motherboard: **Z70830** (ISSUE 1 & 2, ©1987), common with +2A.
- Built-in Centronics parallel printer port (strobe via `0x1FFD` bit 4).
- Software-controlled RS-232 serial port and AUX port on expansion connector.
- Power supply: +5 V @ 2 A, +12 V @ 700 mA, -12 V @ 50 mA.

### +2A (1987, Amstrad — Black)
- +3 motherboard in a +2-style case, no disk drive.
- Motherboard: **Z70830** (ISSUE 1 & 2, ©1987), common with +3.
- The three disk controller signals from the gate array are present on the expansion port for an external FDC add-on (SI-1, planned but never released).
- Same memory paging, timing, and contention as the +3.
- Same port 0xFE and floating bus behaviour as the +3.
- **Audio distortion:** the AY-3-8912 output circuit has a design flaw causing serious distortion (fixed in +2B).
- Black case, +12 V rail: 200 mA (reduced from +3).

### +2B (1988, Amstrad — Black)
- Redesigned audio circuit fixing the +2A's AY-3-8912 distortion.
- Motherboard: **Z70833** (ISSUE 1, 2 ©1988, ISSUE 4 ©1990). Stepped right edge for paneling.
- Z70833 ISSUE 4 uses a different FM modulator circuit for audio output.
- Many +2A cases actually contain a Z70833 board (Z70830 in +2A cases is rare).

### +3B (1988, Amstrad)
- Redesigned audio circuit fixing the +3's AY-3-8912 distortion.
- Motherboard: **Z70835** (ISSUE 1, ©1988). No datacorder provision, external FDC signals remain on expansion port.

### NTSC Spectrum (1982, Sinclair — Chile)
- Standard 48K Spectrum hardware sold in Chile.
- Uses **5C114E** or **6C011E-3** ULA (NTSC variants).
- Did not meet FCC standards, so was never sold in the USA — the Timex TS2068 was produced for the US market instead.
- Very few machines known to exist.

### TS2068 (1985, Timex — USA)
- Enhanced Spectrum, 48 KB RAM, 24 KB ROM (3 banks of 8 KB).
- CPU: Z80A @ 3.528 MHz.
- AY-3-8912 sound chip.
- Four video modes (see [Timex Models](../appendix/timex_models.md)).
- Two built-in joystick ports.
- Edge connector incompatible with Sinclair Spectrum peripherals.
- Power supply: 15 V DC @ 1.0 A, centre -ve.

### TC2048 (Timex — Portugal)
- 99% Spectrum-compatible.
- Kempston joystick port built in.
- Extra display modes retained from TS2068. No AY-3-8912.
- Power supply: 9 V DC @ 800 mA, centre -ve.

### TC2068 (Timex — Portugal)
- PAL variant of TS2068.
- Edge connector compatible with Sinclair Spectrum peripherals.
- BASIC 64 in ROM.
- Power supply: 9 V DC @ 1.0 A, centre -ve.
