# The Z80 CPU

The ZX Spectrum uses a Z80A-compatible CPU clocked at approximately 3.5 MHz (48K) or 3.5469 MHz (128K and later). The majority of Spectrums use the **NEC D780C-1**, an unlicensed reverse-engineered Z80 clone. Other manufacturers (SGS, Mostek, Sharp, Toshiba) also produced Z80 variants that may be found in different machines.

## Subsections

- [Registers & Flags](./registers.md) — standard and shadow register set, flag bits
- [Instruction Set](./instructions.md) — documented opcode map and execution times
- [Undocumented Opcodes & Flags](./undocumented.md) — illegal opcodes and flag behaviour
- [Interrupts (IM 0, 1, 2)](./interrupts.md) — interrupt modes and the Spectrum's wiring
