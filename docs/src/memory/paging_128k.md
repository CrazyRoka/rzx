# Memory Paging

## General Principles

The Z80 has 16 address lines and no built-in MMU, so it can address a maximum of 64 KB (`0x0000–0xFFFF`). To use more than 64 KB of memory, additional logic selects which physical memory chip (or chip region) responds to which CPU address range.

- **Bank** — a 16 KB slot in the CPU's address space (bank 0 = `0x0000–0x3FFF`, bank 1 = `0x4000–0x7FFF`, etc.)
- **Page** — a 16 KB region of physical RAM or ROM (page 0 = `0x00000–0x03FFF`, page 1 = `0x04000–0x07FFF`, etc.)

Two mechanisms are used:

1. **/ROMCS method** — peripherals disable the internal ROM by pulling the `/ROMCS` pad on the edge connector high, mapping their own ROM or RAM into `0x0000–0x3FFF`.
2. **Built-in MMU** — a dedicated chip (PAL, gate array, or discrete logic) controls which physical pages map to each CPU bank.

## /ROMCS Method

The `/ROMCS` signal (edge connector lower pin 50) is connected to the ROM's chip-select pin via a **680 Ω resistor**. Pulling this line high (+5 V) disables the internal ROM, allowing a peripheral to supply its own ROM or RAM in the `0x0000–0x3FFF` region.

Limitations:
- Only the ROM address space (`0x0000–0x3FFF`) can be remapped.
- `/ROMCS` is **not present** on the +2A, +2B, +3, or +3B edge connector — these models cannot use ROMCS-based peripherals.

## 16K/48K MMU

The 16K and 48K models have no true MMU, only a fixed split of the address space:

- `0x0000–0x3FFF` — ROM (always)
- `0x4000–0x7FFF` — 16 KB RAM, managed by the ULA (contended zone)
- `0x8000–0xFFFF` — 32 KB RAM, addressed directly by the CPU

The ULA monitors the CPU's address bus; if the CPU accesses `0x4000–0x7FFF` while the ULA is reading screen data, the ULA halts the CPU clock (see [Contended Memory](./contention.md)).

## 128K / +2 Memory Map

The 128K and later models introduce bank-switched memory paging, controlled through dedicated I/O ports. The Z80's 64 KB address space is divided into fixed and paged regions.

### 128K / +2 Memory Map

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

### 128K MMU Implementation

The 128K MMU is built from several components:

- **HAL10H8** (PAL10H8) — decodes address and control signals to generate the bank register clock and contention select.
- **74LS174** (hex D flip-flop) — holds the 6-bit bank register.
- **ZX8401** (Mullard ULA) — generates DRAM row/column addresses for the RAM chips.
- **ULA** (5C/6C/7K010E) — handles video RAM addressing and CPU contention.

The 128 KB DRAM is organised as two banks of eight **4164** (64K×1 bit) chips:
- **"Upper" RAM** — pages 0–3, *uncontended* (confusingly named; these are the lower-numbered pages).
- **"Lower" RAM** — pages 4–7, *contended*.

The 74LS174's data inputs D1–D6 connect to the CPU data bus D0–D5 respectively. Its CLK input is driven by the HAL10H8, which asserts the clock when an I/O write to port `0x7FFD` (decoded from `A15=0`, `A1=0`, `/IOREQ=0`) occurs.

The six flip-flop outputs map as follows:

| Output | Name | Connected to |
|---|---|---|
| Q1 | B0 | HAL10H8 B0 input |
| Q2 | B1 | HAL10H8 B1 input |
| Q3 | B2 | HAL10H8 B2 input |
| Q4 | B3 (VB) | ULA video bank select |
| Q5 | B4 | ROM A14 (selects ROM 0 or ROM 1) |
| Q6 | B5 (lock) | 74LS174 CLK (via diode + 470 Ω pull-up) |

When Q6 (B5) is high, the CLK line is held high and no further writes to the bank register can occur until the next reset.

### HAL10H8 Read Crash Bug

On the original 128K (and some early +2s), reading from `0x7FFD` **crashes the machine**. The HAL10H8 chip does not distinguish between I/O reads and writes to this port — a read corrupts the paging registers with whatever value is on the data bus (typically floating bus data). The machine must be reset.

Later grey +2s shipped with an updated HAL10H8 that distinguishes reads from writes. On these machines (and on +2A/+3), reading `0x7FFD` returns floating bus values without side effects.

### HAL10H8 Contention Bug

The HAL10H8 also decides which RAM banks are contended. Due to a bug — reversed B0 and B2 inputs either on the PCB or within the HAL itself — the original 128K contends banks **1, 3, 5, and 7** instead of the intended banks 4, 5, 6, and 7 (as documented in the service manual). The grey +2 inherits this same contention pattern. The +2A and +3 use a different gate array that contends banks 4, 5, 6, and 7 as originally intended.

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

Special paging mode was introduced for **CP/M** compatibility — CP/M requires RAM (not ROM) at `0x0000` for its interrupt vector table and system workspace. When bit 0 of port `0x1FFD` is set, the memory map changes to one of four configurations determined by bits 1 and 2:

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
