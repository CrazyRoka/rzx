# 128K Memory Paging

The 128K and later models introduce bank-switched memory paging, controlled through dedicated I/O ports. The Z80's 64 KB address space is divided into fixed and paged regions.

## 128K / +2 Memory Map

```
0xFFFF +--------+--------+--------+--------+--------+--------+--------+--------+
       | Bank 0 | Bank 1 | Bank 2 | Bank 3 | Bank 4 | Bank 5 | Bank 6 | Bank 7 |
       |        |        |(also at|        |        |(also at|        |        |
       |        |        | 0x8000)|        |        | 0x4000)|        |        |
       |        |        |        |        |        | screen |        | screen |
0xC000 +--------+--------+--------+--------+--------+--------+--------+--------+
       | Bank 2 |        Any one of these pages may be switched in.
       |        |
       |        |
0x8000 +--------+
       | Bank 5 |
       |        |
       |        |
       | screen |
0x4000 +--------+--------+
       | ROM 0  | ROM 1  | Either ROM may be switched in.
       |        |        |
       |        |        |
0x0000 +--------+--------+
```

| Address range | Size | Content |
|---|---|---|
| `0x0000 – 0x3FFF` | 16 KB | ROM (bank 0 or 1, selectable) |
| `0x4000 – 0x7FFF` | 16 KB | Bank 5 (screen, always) |
| `0x8000 – 0xBFFF` | 16 KB | Bank 2 (fixed) |
| `0xC000 – 0xFFFF` | 16 KB | Paged bank (0–7, selectable) |

RAM banks 1, 3, 5 and 7 are contended. Banks 0, 2, 4, 6 are not.

## Port 0x7FFD (128K / +2)

Memory paging on 128K and +2 is controlled by writes to port `0x7FFD`. As is standard on Sinclair hardware, the port address is only partially decoded: the hardware responds to any port address with bits 1 and 15 reset. However, `0x7FFD` should be used to avoid conflicts.

The byte written is interpreted as follows:

| Bit | Function |
|---|---|
| 0–2 | RAM page (0–7) to map into `0xC000 – 0xFFFF` |
| 3 | Screen selection: 0 = normal (bank 5), 1 = shadow (bank 7). Does not affect `0x4000 – 0x7FFF`, which is always bank 5. |
| 4 | ROM select: 0 = ROM 0 (128K editor/menu), 1 = ROM 1 (48K BASIC) |
| 5 | Paging lock: if set, further writes to this port are ignored until reset |
| 6–7 | Unused |

When memory is being paged, interrupts should be disabled and the stack should be in an area that is not going to change. If normal interrupt code is to run, the system variable at `0x5B5C` (23388) must be kept updated with the last value sent to port `0x7FFD`.

Reading from `0x7FFD` returns floating bus values (no special result).

### Example: Switching to Bank 4

```
LD   A,(0x5B5C)       ; previous value of port
AND  0xF8
OR   4                 ; select bank 4
LD   BC,0x7FFD
DI
LD   (0x5B5C),A
OUT  (C),A
EI
```

## +2A / +2B / +3 Paging

The +2A and +3 share the same basic paging principle as the 128K/+2 but add a second paging port for extended control.

### Port 0x7FFD (+2A/+3)

Functions the same as on the 128K/+2 with two differences:

| Bit | Change |
|---|---|
| 4 | Low bit of ROM selection (used together with port `0x1FFD` bit 2) |
| — | Partial decoding: responds only to addresses with bit 1 reset, bit 14 set, bit 15 reset |

### Port 0x1FFD (+2A/+3)

Extended paging is controlled by port `0x1FFD`. Partial decoding: responds to addresses with bit 1 reset, bit 12 set, bits 13–15 reset. This port is write-only; its last value should be saved at `0x5B67` (23399).

| Bit | Function |
|---|---|
| 0 | Paging mode: 0 = normal, 1 = special |
| 1 | In normal mode: ignored. In special mode: see table below. |
| 2 | Normal mode: high bit of ROM selection. Special mode: see table below. |
| 3 | Disk motor: 1 = on, 0 = off (+3 only) |
| 4 | Printer port strobe (+3 only) |
| 5–7 | Unused |

The four ROMs are selected by bits 4 (port `0x7FFD`) and 2 (port `0x1FFD`):

| 0x7FFD bit 4 | 0x1FFD bit 2 | ROM |
|---|---|---|
| 0 | 0 | ROM 0: 128K editor, menu system, self-test |
| 1 | 0 | ROM 1: 128K syntax checker |
| 0 | 1 | ROM 2: +3DOS |
| 1 | 1 | ROM 3: 48K BASIC |

### Special Paging Mode (+2A/+3)

When bit 0 of port `0x1FFD` is set, the memory map changes to one of four special configurations determined by bits 1 and 2:

```
         Bit 2=0     Bit 2=0     Bit 2=1     Bit 2=1
         Bit 1=0     Bit 1=1     Bit 1=0     Bit 1=1
0xFFFF +--------+  +--------+  +--------+  +--------+
       | Bank 3 |  | Bank 7 |  | Bank 3 |  | Bank 3 |
       |        |  |        |  |        |  |        |
0xC000 +--------+  +--------+  +--------+  +--------+
       | Bank 2 |  | Bank 6 |  | Bank 6 |  | Bank 6 |
0x8000 +--------+  +--------+  +--------+  +--------+
       | Bank 1 |  | Bank 5 |  | Bank 5 |  | Bank 7 |
       |        |  |        |  |        |  |        |
       |        |  | screen |  | screen |  | screen |
0x4000 +--------+  +--------+  +--------+  +--------+
       | Bank 0 |  | Bank 4 |  | Bank 4 |  | Bank 4 |
0x0000 +--------+  +--------+  +--------+  +--------+
```

On the +2A/+3, RAM banks 4, 5, 6 and 7 are contended. Banks 0, 1, 2, 3 are not.

RAM banks 1, 3, 4 and 6 are used for the disc cache and RAMdisc. Bank 7 contains editor scratchpads and +3DOS workspace.
