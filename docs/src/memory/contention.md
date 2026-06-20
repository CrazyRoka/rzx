# Contended Memory

When the ULA is drawing the active screen area, it needs to access video memory (`0x4000–0x7FFF`). The RAM cannot be read by two devices simultaneously, so the ULA is given higher priority (the electron beam cannot be interrupted). If the Z80 attempts to read or write the lower 16K of RAM during this period, the ULA halts the CPU via the `WAIT` pin until the ULA has finished.

This effect occurs **only** during the active display period (192 scanlines). During border and vertical retrace, the ULA does not access RAM and no contention occurs.

On 128K models, the contention sequence starts at a different cycle (14361 vs 14335 on 48K). The alternate screen buffer in bank 7 is also subject to contention when it is the currently displayed screen.

## Contention Cycle Table

At the start of each active scanline, contention follows an 8-cycle pattern that repeats every 8 T-states. The pattern begins at cycle 14335 (48K) or 14361 (128K) relative to the interrupt:

| Cycle # (mod 8) | Delay (T-states) |
|---|---|
| 0 | 6 |
| 1 | 5 |
| 2 | 4 |
| 3 | 3 |
| 4 | 2 |
| 5 | 1 |
| 6 | 0 (no delay) |
| 7 | 0 (no delay) |

Expanded for the first 16 cycles of the active display:

| Cycle # (48K) | Cycle # (128K) | Delay |
|---|---|---|
| 14335 | 14361 | 6 |
| 14336 | 14362 | 5 |
| 14337 | 14363 | 4 |
| 14338 | 14364 | 3 |
| 14339 | 14365 | 2 |
| 14340 | 14366 | 1 |
| 14341 | 14367 | 0 |
| 14342 | 14368 | 0 |
| 14343 | 14369 | 6 |
| ... | ... | ... (pattern repeats every 8 cycles) |

This pattern continues through all 192 active display lines (14335 to 14463 for the first line), then resets at the start of each subsequent line. During border lines no contention occurs.

## Wait State Insertion

When a contended access occurs, the ULA inserts wait states equal to the delay for the current cycle. The Z80 is stopped mid-instruction, and the cycle counter is advanced by the delay amount. The exact number of T-states an instruction takes in contended memory is therefore:

```
actual_time = base_time + sum_of_delays_at_each_contented_access_point
```

## Applying Contention to Instructions

Each instruction has specific points where memory or I/O access occurs. Only these points are subject to contention delays. The tables below use the following format:

```
instruction    point1:duration1, point2:duration2, ...
```

Where `point` is the address or register that determines whether contention applies, and `duration` is the number of uncontended T-states consumed at that point (before any potential delay).

- If `point` is within `0x4000–0x7FFF` (contended RAM), apply the delay for the current cycle before continuing.
- `IO` means the cycle is an I/O access, subject to [Contended I/O](../ula/contended_io.md) rules.
- `(write)` marks the cycle in which the value is written to memory — important for pixel-timed effects.

For conditional instructions, entries in `[square brackets]` apply only when the condition is met. For unconditional instructions they always apply.

`dd` = BC, DE, HL, SP — `qq` = BC, DE, HL, AF — `ss` = BC, DE, HL — `ii` = IX or IY — `cc` = condition (NZ, Z, NC, C, PO, PE, P, M) — `b` = bit number 0–7 — `r, r'` = A, B, C, D, E, H, L — `alo` = ADD, ADC, SUB, SBC, AND, XOR, OR, CP — `sro` = RLC, RRC, RL, RR, SLA, SRA, SRL, SLL

### 1-byte / register-only instructions

```
NOP                                      pc:4
LD r,r'                                  pc:4
alo A,r                                  pc:4
INC/DEC r                                pc:4
EXX                                      pc:4
EX AF,AF'                                pc:4
EX DE,HL                                 pc:4
DAA / CPL / CCF / SCF / DI / EI          pc:4
RLA / RRA / RLCA / RRCA                  pc:4
JP (HL)                                  pc:4
```

### 2-byte (CB, ED prefix) instructions

```
NOPD                                     pc:4, pc+1:4
sro r                                    pc:4, pc+1:4
BIT b,r / SET b,r / RES b,r              pc:4, pc+1:4
NEG                                      pc:4, pc+1:4
IM 0/1/2                                 pc:4, pc+1:4
```

### Special register access

```
LD A,I / LD A,R                          pc:4, pc+1:5
LD I,A / LD R,A                          pc:4, pc+1:5
```

### 16-bit register operations

```
INC/DEC dd                               pc:6
LD SP,HL                                 pc:6
ADD HL,dd                                pc:11
ADC HL,dd / SBC HL,dd                    pc:4, pc+1:11
```

### Immediate / direct addressing

```
LD r,n / alo A,n                         pc:4, pc+1:3
LD r,(ss) / LD (ss),r                    pc:4, ss:3
alo A,(HL)                               pc:4, hl:3
```

### Index register (IX/IY + n) operations

```
LD r,(ii+n) / LD (ii+n),r / alo A,(ii+n)  pc:4, pc+1:4, pc+2:3, pc+2:1×5, ii+n:3
```

### Bit operations on (HL) and (II+n)

```
BIT b,(HL)                               pc:4, pc+1:4, hl:3, hl:1
BIT b,(ii+n)                             pc+1:4, pc+2:3, pc+3:3, pc+3:1×2, ii+n:3, ii+n:1
```

### 16-bit load / jump

```
LD dd,nn / JP nn / JP cc,nn              pc:4, pc+1:3, pc+2:3
```

### Load immediate to memory

```
LD (HL),n                                pc:4, pc+1:3, hl:3
LD (ii+n),n                              pc:4, pc+1:4, pc+2:3, pc+3:3, pc+3:1×2, ii+n:3
```

### Absolute address loads (unprefixed: 22, 2A)

```
LD A,(nn) / LD (nn),A                    pc:4, pc+1:3, pc+2:3, nn:3
```

### Absolute address loads (prefixed: ED)

```
LD HL,(nn) / LD (nn),HL                  pc:4, pc+1:3, pc+2:3, nn:3, nn+1:3
LD dd,(nn) / LD (nn),dd                  pc:4, pc+1:4, pc+2:3, pc+3:3, nn:3, nn+1:3
```

### Read-modify-write on (HL) and (ii+n)

```
INC/DEC (HL)                             pc:4, hl:3, hl:1, hl(write):3
SET b,(HL) / RES b,(HL) / sro (HL)       pc:4, pc+1:4, hl:3, hl:1, hl(write):3
INC/DEC (ii+n)                           pc:4, pc+1:4, pc+2:3, pc+2:1×5, ii+n:3, ii+n:1, ii+n(write):3
SET b,(ii+n) / RES b,(ii+n) / sro (ii+n) pc:4, pc+1:4, pc+2:3, pc+3:3, pc+3:1×2, ii+n:3, ii+n:1, ii+n(write):3
```

### Stack operations

```
POP dd / RET                             pc:4, sp:3, sp+1:3
RETI / RETN                              pc:4, pc+1:4, sp:3, sp+1:3
RET cc                                   pc:5, [sp:3, sp+1:3]
PUSH dd / RST n                          pc:5, sp-1:3, sp-2:3
CALL nn / CALL cc,nn                     pc:4, pc+1:3, pc+2:3, [pc+2:1, sp-1:3, sp-2:3]
```

### Conditional jumps

```
JR n                                     pc:4, pc+1:3, [pc+1:1×5]
JR cc,n                                  pc:4, pc+1:3, [pc+1:1×5]
DJNZ n                                   pc:5, pc+1:3, [pc+1:1×5]
```

### Block instructions

```
RLD / RRD                                pc:4, pc+1:4, hl:3, hl:1×4, hl(write):3
```

### I/O instructions

```
IN A,(n) / OUT (n),A                     pc:4, pc+1:3, IO
IN r,(C) / OUT (C),r                     pc:4, pc+1:4, IO
```

### Exchange

```
EX (SP),HL                               pc:4, sp:3, sp+1:4, sp(write):3, sp+1(write):3, sp+1(write):1×2
```

### Block copy / search

```
LDI/LDIR / LDD/LDDR                      pc:4, pc+1:4, hl:3, de:3, de:1×2, [de:1×5]
CPI/CPIR / CPD/CPDR                      pc:4, pc+1:4, hl:3, hl:1×5, [hl:1×5]
```

### Block I/O

```
INI/INIR / IND/INDR                      pc:4, pc+1:5, IO, hl:3, [hl:1×5]
OUTI/OTIR / OUTD/OTDR                    pc:4, pc+1:5, hl:3, IO, [hl:1×5]
```

### Notes

- Replacing HL with IX or IY does not affect timings except for adding an initial `pc:4` for the DD or FD prefix.
- Undocumented DDCB and FDCB variants share the same timings as documented CB-prefixed versions.
- DD/FD prefixes on instructions that do not involve HL add only an initial `pc:4`.

## Example

Instruction `LD (HL),A` with `pc:4, hl:3`:

1. If PC is in `0x4000–0x7FFF`, apply contention delay for current cycle, then wait 4 T-states (opcode fetch).
2. If HL is in `0x4000–0x7FFF`, apply contention delay for the new current cycle, then write A to (HL) in 3 T-states.

Starting at cycle 14335 with PC=25000, HL=26000:
1. Cycle 14335 → 6 T-states delay (now at 14341)
2. Opcode fetch: 4 T-states (now at 14345)
3. Cycle 14345 → 4 T-states delay (now at 14349)  
4. Write A to (HL): 3 T-states (now at 14352)

Next opcode fetch at cycle 14352 (PC=25001) → 1 T-state delay.
