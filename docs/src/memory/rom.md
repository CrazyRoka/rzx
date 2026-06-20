# ROM Layout

The 16 KB ROM at `0x0000 – 0x3FFF` contains the Sinclair BASIC interpreter and the system's I/O service routines. Different models shipped with different ROM versions.

| Model | ROM Label | Notes |
|---|---|---|
| 16K / 48K | Original 48K ROM | 16 KB, single bank |
| Spectrum+ | Original 48K ROM | Same as 48K |
| 128K | 128K ROM (v0/v1) | 32 KB, two 16 KB banks paged via port `0x7FFD` |
| +2 | +2 ROM | 32 KB, derived from 128K ROM with minor changes |
| +3 | +3 ROM | 64 KB, four 16 KB banks, includes +3 DOS for CP/M |
| +2A / +2B | +3-derived ROM | Same ROM hardware as +3, but OS behaves as +2 |

On models with paged ROM (128K and later), the lower ROM bank (`0x0000 – 0x3FFF`) is selected via bit 4 of port `0x7FFD`. The +3 additionally uses port `0x1FFD` to select between four 16 KB ROM banks.

TODO: Document the ROM entry points — CHRAM, KEYBOARD, BEEPER, TAPE routines, error handler, etc. Include the well-known addresses used by emulators and disassembly tools.
