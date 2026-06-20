# Registers & Flags

## Standard Register Set

| Register | Size | Purpose |
|---|---|---|
| AF | 16-bit | Accumulator + Flags |
| BC | 16-bit | General purpose / counter |
| DE | 16-bit | General purpose / data pointer |
| HL | 16-bit | General purpose / memory pointer |
| IX | 16-bit | Index register X |
| IY | 16-bit | Index register Y |
| SP | 16-bit | Stack pointer |
| PC | 16-bit | Program counter |

## Shadow Registers

`EX AF,AF'` and `EXX` swap between the main and shadow sets:

| Main | Shadow |
|---|---|
| AF | AF' |
| BC | BC' |
| DE | DE' |
| HL | HL' |

## Special-Purpose Registers

| Register | Size | Purpose |
|---|---|---|
| I | 8-bit | Interrupt vector base address (high byte of interrupt table pointer) |
| R | 7-bit | DRAM refresh counter |

### The R Register

R is incremented during every M1 cycle (approximately every instruction). Prefix bytes (DD, FD, ED, CB) each increment R by one. Doubly-shifted opcodes (DD CB, FD CB) also increment R by two total. Block instructions like LDI increment R by 2; LDIR increments it by 2 × BC (same for LDDR, etc.). R is reset to zero when the Z80 is reset.

`LD A,R` and `LD R,A` use the value of R **after** it has been incremented. For example, `XOR A / LD R,A` sets R to zero; after a reset, `DI / LD A,R` sets A to `0x03`.

Only the lowest 7 bits of R are used for refresh (historical: 16 Kbit DRAM chips used a 128×128 matrix requiring a 7-bit row address). The highest bit of R is never changed.

The R register is crucial to memory refresh. Example: running a tight loop that continuously loads R without accessing upper RAM will cause the upper 32K to fade, as only the ULA refreshes the lower 16K:

```
ORG 32768
DI
LD B,0
L1: XOR A
    LD R,A
    DEC HL
    LD A,H
    OR L
    JR NZ,L1
    DJNZ L1
    EI
    RET
```

This takes about three minutes. Afterward, only the first few bytes of each 256-byte block in the upper 32K remain valid. (This will not work on an emulator.)

R is also incremented by 1 during interrupt or NMI acknowledge.

## Flag Register (F)

| Bit | Flag | Meaning |
|---|---|---|
| 7 | S | Sign (set if result negative) |
| 6 | Z | Zero (set if result zero) |
| 5 | — | Undocumented — see [Undocumented Opcodes & Flags](./undocumented.md) |
| 4 | H | Half-carry (BCD auxiliary carry) |
| 3 | — | Undocumented — see [Undocumented Opcodes & Flags](./undocumented.md) |
| 2 | P/V | Parity/Overflow |
| 1 | N | Add/Subtract (BCD) |
| 0 | C | Carry |
