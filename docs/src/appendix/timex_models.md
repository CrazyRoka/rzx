# Timex Models

## Models

### TS2068 (1985, Timex — USA)
- CPU: Z80A @ 3.528 MHz
- RAM: 48 KB (3 × 16 KB banks)
- ROM: 24 KB (3 × 8 KB banks)
- Sound: AY-3-8912
- Power: 15 V DC @ 1.0 A, centre -ve
- Expansion port: 46-pin edge connector, **not compatible** with Sinclair Spectrum peripherals
- Two built-in joystick ports (mapped to AY R14)
- TULA (Timex ULA) — modified ULA with extra video modes

### TC2048 (Timex — Portugal)
- 99% ZX Spectrum compatible
- Built-in Kempston joystick port
- Extra video modes from TS2068 retained
- AY-3-8912 removed (bare socket on some boards)
- Power: 9 V DC @ 800 mA, centre -ve

### TC2068 (Timex — Portugal)
- PAL variant of TS2068
- Edge connector compatible with standard Sinclair Spectrum peripherals
- BASIC 64 in ROM

## Video System

The Timex ULA (TULA) adds four video modes beyond the standard Spectrum mode, controlled via ports `0xFF` (write) and `0xF5` (write).

### Ports

| Port | Direction | Function |
|---|---|---|
| `0xFF` | Write | Video mode select, extra colour enable |
| `0xF5` | Write | High resolution / display mode control |

**Port 0xFF bits:**
| Bit | Function |
|---|---|
| 0 | Mode select (0 = modes 0/1, 1 = modes 2/3) |
| 1 | Extra colour set (1 = bright red/cyan enabled) |
| 2–7 | Unused |

### Mode Summary

| Mode | Resolution | Attributes | Display file | Colour table | Description |
|---|---|---|---|---|---|
| 0 | 256×192 | 2 colours per 8×8 cell | `0x4000` | `0x5800` | Standard Spectrum |
| 1 | 512×192 | 2 colours per 8×1 cell | `0x4000` | `0x5800` | High resolution monochrome |
| 2 | 256×192 | 16 colours per 8×1 cell | `0x4000` | `0x4400` | WRX (Wide Resolution eXtended) |
| 3 | Dual 256×192 | Independent per screen | `0x4000` / `0x5800` | — | Two 6 KB screens in 16 KB video RAM. |

#### Mode 0 — Standard Spectrum
Identical to ZX Spectrum display. 256×192 pixels, 2 colours per 8×8 attribute cell. Display file at `0x4000`, attributes at `0x5800`.

#### Mode 1 — 512×192 High Resolution
Horizontal resolution doubled to 512. Attribute file at `0x5800` uses only bit 7 to determine the colour of each 8×1 cell. Display file same layout as Mode 0.

#### Mode 2 — WRX (Wide Resolution eXtended)
256×192 pixels with full colour per 8×1 cell. Each pixel encoded as 4 bits (16 colours) from a second colour table at `0x4400`. The display file at `0x4000` acts as a monochrome bitmask; the colour table supplies the actual pixel colour.

#### Mode 3 — Dual Screen
Two independent screen areas of 6 KB each within the 16 KB video memory window. The display can switch between them, enabling double-buffering or separate layer rendering.

### TC2048 Mode

The TC2048 adds a further mode through an updated TULA:

| Mode | Resolution | Attributes |
|---|---|---|
| 4 | 512×192 | 8×1 attributes |

(TODO: confirm exact port control bits for Mode 4)

## Memory Map

| Start | End | Description |
|---|---|---|
| `0x0000` | `0x1FFF` | ROM bank 0 |
| `0x2000` | `0x3FFF` | ROM bank 1 |
| `0x4000` | `0x7FFF` | Video RAM / banked RAM |
| `0x8000` | `0xBFFF` | Banked RAM |
| `0xC000` | `0xFFFF` | ROM bank 2 / banked RAM |

### ROM Banks (TS2068)
| Bank | Content |
|---|---|
| 0 | 16K Spectrum BASIC (subset) |
| 1 | Editor / screen handling |
| 2 | Timex BASIC extensions, monitor |

### RAM Banking (TS2068)
Three 16 KB banks fill the `0x4000–0xBFFF` region. The `0xC000–0xFFFF` region is either ROM bank 2 or banked RAM, controlled by a paging register.

## I/O Map (TS2068 Additions)

| Port | Direction | Function |
|---|---|---|
| `0xFF` | Write | Video mode |
| `0xF5` | Write | High resolution / display control |
| `0xF5` | Read | Joystick (Timex Kempston) |
| `0xF6` | Write | Memory paging, sound |

### Joystick (Timex Kempston)
Read from port `0xF5`. Uses AY-3-8912 port A register (R14) for the joystick state:

| Bit | Direction |
|---|---|
| 0 | Up |
| 1 | Down |
| 2 | Left |
| 3 | Right |
| 4 | Fire |

The TC2048 uses the same mapping but reads the Kempston standard port `0x1F` instead.

## Expansion Port

| Model | Edge connector | Spectrum peripheral compatible |
|---|---|---|
| TS2068 | 46 pins, TS1000-derived pinout | **No** |
| TC2048 | Sinclair-compatible | **Yes** |
| TC2068 | Sinclair-compatible | **Yes** |

The TS2068 pinout differs significantly from the ZX Spectrum edge connector. Standard Spectrum peripherals will not work without an adapter.

## Keyboard

The TS2068 keyboard layout differs from the Spectrum:
- Full-travel keys (not membrane)
- 44 keys including dedicated function keys
- Key matrix mapped similarly to Spectrum but with additional rows for the function keys

The TC2048 and TC2068 use a membrane-style keyboard similar to the Spectrum+.

## Sound

- TS2068: AY-3-8912 accessed through Timex-specific port `0xF6` (not the standard Spectrum 128K ports `0xFFFD`/`0xBFFD`).
- TC2048: AY-3-8912 socket present but generally not populated.
- TC2068: AY-3-8912 present, accessed through Timex-specific ports.

(TODO: confirm exact AY register/data port addresses for Timex models)
