# Other Peripherals

## Multiface 1 / 128 / +3

Developed by Romantic Robot, the Multiface connects to the Spectrum's edge connector and provides:

- A Kempston-compatible joystick port
- 8 KB EPROM + 8 KB RAM (Multiface 1: 2 KB RAM)
- The ability to freeze a running program and save it to tape, Microdrive, disk, or Wafadrive
- In-memory inspection and patching

Saved programs can be compressed and reloaded without a Multiface attached.

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
