# Mass Storage

## ZX Microdrive

Announced in 1982 and launched in 1983, the ZX Microdrive is a tape-cartridge system used with the ZX Interface I, providing a faster alternative to cassette tape.

| Property | Value |
|---|---|
| Cartridge capacity | 85 KB |
| Transfer rate | ~15 KB/s |
| Max. drives per Interface I | 8 |
| Tape length | 6 m of 1.9 mm magnetic tape (endless loop) |

Later Microdrive improvements for the Sinclair QL allowed ~100 KB per cartridge, but the formats are incompatible.

## Rotronics Wafadrive

A combined dual-tape-drive system with RS-232 and Centronics ports, attaching to the Spectrum's expansion connector via a ribbon cable:

| Property | Value |
|---|---|
| Capacity per cartridge | 16K, 64K, or 128K |
| Transfer rate | 18 KBaud |
| Tape speed | 10 in/s (fast search: 15 in/s) |
| Format time (128K) | ~4 min 42 s |
| Catalogue time (128K) | up to 47 s |

The Wafadrive Operating System occupies addresses 23754–26046, which can cause compatibility problems with some software. Wafadrive and Microdrive cartridges are not interchangeable. The company (Rotronics) collapsed in 1986.

## Opus Discovery

A multi-function unit combining a 3.5" floppy disk drive with a joystick port, parallel printer port, pass-through expansion connector, and composite video / monitor output. Disk capacity: up to 250 KB (178 KB formatted). An upgraded model (Discovery 2) features two drives.

Operationally similar to the ZX Microdrive — the same commands are supported, easing adaptation of Microdrive-compatible software. The Microdrive is faster, however.

## Timex FDD / FDD-3000

Timex Portugal produced two disk systems for the TS2068, TC2068, and TC2048 (third-party interfaces enable use with the 128K Spectrum).

**FDD:** Three separate boxes, 16 KB RAM, own Z80 CPU.
**FDD-3000:** Integrated PSU with two 3" drives, 64 KB RAM, can serve as a CP/M terminal. Possible (but tricky) to upgrade an FDD to an FDD-3000.

Both support 3.5" and 5.25" drives in addition to 3". The interface pages in a 4 KB FDD ROM on calls to `0x0000` or `0x0008`, and pages it out on calls to `0x0604`. The ROM is mapped at `0x0000` and `0x1000`, with `0x2000–0x3FFF` holding eight copies of 1 KB RAM (or four copies of 2 KB). Disk I/O is controlled via port `0xEF`.

## Crescent Quick Disk

Three models: 128 (drive only), 128i (with interface), 256i (higher capacity). Each includes an RGB socket and RS-423 interface. Compatible with the ZX Spectrum and Spectrum+.

## Triton Quick Disk

Uses 2.8" double-sided disks with a **spiral** recording pattern (not concentric tracks). Access is sequential (like Microdrive/Wafadrive), not random-access.

| Property | Value |
|---|---|
| Capacity | up to 100 KB (50 KB per side) |
| Data rate | 101.6 Kbit/s |
| Density | 4410 BPI |
| Sectors | 20 per side, 2.5 KB each |
| Max seek | ~8 seconds |
| Max drives | 2 (multiple interfaces required) |

Includes a pass-through connector for a ZX Printer. Produced by Radofin Electronics.

## Omnitronix Pacer

A modular disk system. The interface connects to the Spectrum's edge connector via ribbon cable and supports 5.25", 3.5", and 3" drives. Complete systems with a drive were available at various capacities:

- Interface only: £79.95
- Interface + 5.25" single-sided 40-track (100 KB): £119.95
- Interface + 5.25" double-sided 40/80-track (400 KB): £189.95

## MGT Lifetime

Refer to the MGT Lifetime FAQ by Damien Guard for full technical details. Available from the usual Spectrum reference archives.

## Beta 128 (TR-DOS)

A floppy disk interface developed in Bulgaria, widely used in Eastern Europe and the Russian Spectrum scene. Runs the **TR-DOS** operating system.

| Port | Function |
|---|---|
| `0x1F` | Command (OUT) / State (IN) |
| `0x3F` | Track register |
| `0x5F` | Sector register |
| `0x7F` | Data register |
| `0xFF` | System register |

The Beta 128 response mask is `---- ---- 000- ----` for the command/state port, with the high nibble of the low byte selecting the function — see [I/O Port Map](../quirks/io_port_map.md).

## +D (Disciple)

A disk interface compatible with the Disciple and +D systems, supporting double-sided 3.5" drives and CP/M.

| Port | Function |
|---|---|
| `0xE3` | Command (OUT) / State (IN) |
| `0xE7` | Memory paging |
| `0xEB` | Track register |
| `0xEF` | System register |
| `0xF3` | Sector register |
| `0xF7` | Printer data / ready |
| `0xFB` | Data register |

The +D response mask uses bits 7–3 of the low byte for decoding — see [I/O Port Map](../quirks/io_port_map.md).

## Timex TS2020 Program Recorder

A standard cassette recorder for Timex systems:

| Property | Value |
|---|---|
| Output power | 500 mW |
| Speaker | 2 in (50 mm), 8 Ω |
| Tape speed | 1⅞ in/s (4.75 cm/s) |
| Frequency response | 200–6300 Hz |
| Power | 6 V DC (4×AA batteries) or 120 V AC adaptor |
