# Other Peripherals

## 128K RS-232 Serial Port

The ZX Spectrum 128 provides a software-controlled RS-232 serial interface through the expansion edge connector. It is bit-banged via the ULA's general-purpose I/O lines rather than a dedicated UART, so timing is tied to the CPU clock rate. The port is configured through control registers in the ROM and accessed using BASIC's `OPEN #` and stream commands for serial I/O.

(TODO: document exact edge connector pin assignments for the serial signals)

## Multiface 1

The original model for 48K Spectrums:

| Function | Port |
|---|---|
| Read freeze state / ID | IN `0x9F` |
| Write control | OUT `0x1F` |

Includes a Kempston-compatible joystick port, 2 KB RAM, 8 KB EPROM. Freezes the running program, allows saving to tape/Microdrive/Wafadrive.

## Multiface 128

For 128K and +2 models:

| Function | Port |
|---|---|
| Read freeze state / ID | IN `0xBF` (or `0x9F` for Disciple compatibility) |
| Write control | OUT `0x3F` |

8 KB RAM + 8 KB EPROM. Handles the 128K paging system correctly.

## Multiface 3

For +3 models:

| Function | Port |
|---|---|
| Read freeze state / button | IN `0x3F` |
| Write control | OUT `0xBF` |
| Trap port `0x7FFD` | `0x7F3F` |
| Trap port `0x1FFD` | `0x1F3F` |

Supports the +3's dual paging ports and can intercept writes to pages being swapped.

## Currah µSpeech

A speech synthesiser attaching to the expansion port. It reproduces speech using **allophones** — phonetic descriptions of words or letters. Phrases can be composed with emphasis on hard sounds as required. A table of allophone sets is included in the µSpeech programming manual. Several commercial games support it.

## Fuller Audio Box

Provides an AY-3-8912 sound chip externally for pre-128K Spectrums, plus a joystick port and additional EAR/MIC sockets.

| Port | Function |
|---|---|
| `0x3F` | Select AY register (OUT) / read data (IN) |
| `0x5F` | Send data to AY register (OUT) |
| `0x7F` | Read joystick (see [Joystick Interfaces](./joysticks.md)) |

An optional speech chip was available. A pass-through connector allows daisy-chaining additional peripherals.

## AMX Mouse

A 3-button mouse package comprising the mouse, interface, and software suite. The interface includes a parallel printer port. The AMX Control Language adds 28 commands to BASIC for mouse control. Widely supported by art and design software.

## Kempston Mouse

A 2-button mouse readable from machine code or BASIC:

| Function | Port |
|---|---|
| Horizontal position | IN `64479` |
| Vertical position | IN `65503` |
| Buttons | IN `64223` — 255 = none, 254 = left, 253 = right, 252 = both |

## Timex TS1016

A RAM pack for the TS1000 (2K → 16K) or TS1500 (16K → 32K). Attaches to the expansion connector.

## Prism VTX-5000 Modem

Styled to sit under the Spectrum, this modem attaches to the expansion connector and operates at up to **1200 baud**. Popular for connecting to Micronet (British Telecom's online service) and other BBS systems.

## Timex TS2050 Modem (Westridge 2050)

A 300-baud, full-duplex modem for TS1000/1500/2068, produced by Westridge Communications after Timex left the market.

| Property | Value |
|---|---|
| Data rate | 0–300 bps, full duplex |
| Modulation | FSK |
| Power | 120 V AC, 60 Hz, 16 W → 9.75 V DC, 650 mA, centre +ve |

Supplied with MTERM/T software on a multi-format cassette.
