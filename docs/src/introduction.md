# Introduction

Welcome to the ZX Spectrum Hardware Documentation. The documentation is scattered across decades of newsgroup posts, individual emulator source code, hardware manuals, and fragmented wikis. This book is an attempt to consolidate that information into a single, locally hosted, emulator-focused reference.

### Why the Spectrum?

The ZX Spectrum is a fascinating family of machines to emulate. Released in 1982, the original Spectrum relies on absolute minimalism. There is no graphics chip. There is no sound chip. There is barely an I/O controller. The machine is essentially a Z80 CPU, a block of RAM, and a custom gate array called the ULA that sneaks reads from the RAM behind the CPU's back to generate a video signal.

This minimalist core remained at the heart of the family even as later models added memory paging, a dedicated sound chip, floppy disk interfaces, and other enhancements. Across every model, the tight coupling between the CPU and video hardware remains the defining challenge of accurate emulation.

### Models Covered

This book covers the following ZX Spectrum models:

| Model | Year | CPU | ROM | RAM | Key Differences |
|---|---|---|---|---|---|
| 16K | 1982 | 3.5 MHz | 16 KB | 16 KB | Original |
| 48K | 1982 | 3.5 MHz | 16 KB | 48 KB | Baseline reference model |
| Spectrum+ | 1984 | 3.5 MHz | 16 KB | 48 KB | New keyboard case, reset button |
| 128K | 1985 | ~3.5469 MHz | 32 KB | 128 KB | AY-3-8912 sound, keypad, MIDI, RGB |
| +2 | 1986 | ~3.5469 MHz | 32 KB | 128 KB | Built-in tape deck, 2 joystick ports |
| +3 | 1987 | ~3.5469 MHz | 64 KB | 128 KB | 3" floppy drive, CP/M, +3DOS |
| +2A/+2B | 1987–88 | ~3.5469 MHz | 64 KB | 128 KB | +3 motherboard in +2 case, no disk |

Timex also produced several Spectrum-compatible or Spectrum-derived models (TS2068, TC2048, TC2068) with additional video modes, built-in joystick ports, and other differences — see [Timex Models](../appendix/timex_models.md).

Where a feature is model-specific, it is explicitly called out. Sections that do not specify a model describe the baseline behaviour shared across the entire family.

### Scope of this Book

This book focuses primarily on the **ZX Spectrum**.