# Instruction Set

For a complete Z80 opcode table, refer to external references such as:
- [clrhome Z80 Opcode Table](https://clrhome.org/table/) — interactive opcode map with timings
- [The Undocumented Z80 Documented](https://www.myquest.nl/z80undocumented/) (Sean Young)
- [Z80 CPU Users Manual](https://www.zilog.com/) (Zilog)
- [z80.info](https://www.z80.info/) — opcode tables, timing, and documentation

This section covers structural notes, undocumented instructions, and algorithms that are particularly relevant to Spectrum emulation.

## CB Opcodes

The block `CB 30 – CB 37` is missing from the official Zilog documentation. These instructions perform **SLL** (Shift Left Logical) — a left shift that sets bit 0 to 1. Despite being undocumented, they are commonly used (e.g. by Bounder and Enduro Racer).

## ED Opcodes

A complete documented and undocumented ED opcode list is available in Sean Young's reference. Key undocumented notes:

- `ED 70` (`IN (C)`) reads from port (C) and discards the result, but updates flags as per other IN instructions.
- `ED 71` (`OUT (C),0`) outputs a zero byte to port (C).
- `ED 4E` and `ED 6E` behave as **IM 0** equivalents (not IM 0/1 as sometimes claimed).
- All `ED xx RET?` instructions (`RETI`, `RETN`, and undocumented variants) copy IFF2 to IFF1. The only difference between `RETI` (`ED 4D`) and `RETN` is that daisy-chain interrupt peripherals (e.g. Z80 PIO) recognise the `ED 4D` sequence as end-of-interrupt.
- Unlisted ED opcodes in range `00–3F` and `80–FF` (excluding block instructions) do nothing — they take 8 T-states and increment R by 2.

## DD and FD Opcodes (IX/IY)

A DD or FD prefix changes the meaning of HL in the following instruction to IX or IY respectively. If a memory byte is addressed indirectly via HL (e.g. `LD A,(HL)`), a displacement byte is added (e.g. `LD A,(IX+d)`). Instructions that access H or L instead access the high/low halves of IX or IY.

For example:
- `DD 66 01` = `LD H,(IX+01)` — reads from `(IX+1)`, stores in the high byte of IX.
- `DD 2A nn` = `LD IX,(nn)` — note HL replaced by IX.
- `JP (HL)` → `JP (IX)` — a notational awkwardness for assembler/disassembler writers.

Multiple DD or FD prefixes in a row act as NOPs (4 T-states each), toggling the "treat HL as IX/IY" flag.

### Doubly-Shifted Opcodes (DD CB / FD CB)

When CB is preceded by DD or FD, the instruction operates on `(IX+nn)` or `(IY+nn)`, but also copies the result to the register encoded in the CB byte — unless that register is (HL). The offset byte comes **before** the opcode byte (third byte overall: `DD CB nn opcode`).

Example: `DD CB nn CE` = `SET 0,(IX+nn)`, while `DD CB nn C0` = `SET 0,(IX+nn)` with the result copied to B.

## DAA (Decimal Adjust Accumulator)

DAA corrects the accumulator after BCD arithmetic. The algorithm:

1. If A > 0x99 or Carry is set: upper correction = 0x60, set Carry. Else: upper correction = 0x00, clear Carry.
2. If lower nibble of A > 9 or Half-Carry is set: lower correction = 0x06. Else: lower correction = 0x00.
3. Correction factor = upper OR lower (one of 0x00, 0x06, 0x60, 0x66).
4. If N flag is clear: add correction to A. If N is set: subtract correction.
5. Flags: Carry set per step 1; Half-Carry set if correction caused bit 3→4 carry/borrow; S, Z, P/V from result; N unchanged.

## Timing and Contention

For per-instruction T-state breakdowns and contention patterns, see [Contended Memory](../memory/contention.md).
