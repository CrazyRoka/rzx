# Edge Connector

The ZX Spectrum expansion connector is a double-sided card edge connector with 0.1" (2.54 mm) pin spacing. The two rows are numbered from right to left looking into the rear of the computer. An indexing slot removes two opposing pins.

## Pinout

| Upper pin | Signal | | Lower pin | Signal |
|---|---|---|---|---|
| 1 | A15 | | 2 | A14 |
| 3 | A13 | | 4 | A12 |
| 5 | D7 | | 6 | +5 V |
| 7 | /OE | | 8 | NC |
| 9 | SLOT | | 10 | SLOT |
| 11 | D0 | | 12 | 0 V (GND) |
| 13 | D1 | | 14 | 0 V (GND) |
| 15 | D2 | | 16 | /CK (CPU clock) |
| 17 | D6 | | 18 | A0 |
| 19 | D5 | | 20 | A1 |
| 21 | D3 | | 22 | A2 |
| 23 | D4 | | 24 | A3 |
| 25 | /INT | | 26 | NC |
| 27 | /NMI | | 28 | 0 V (GND) |
| 29 | /HALT | | 30 | /OE (ROM output enable) |
| 31 | /MREQ | | 32 | NC |
| 33 | /IORQ | | 34 | NC |
| 35 | /RD | | 36 | NC |
| 37 | /WR | | 38 | /BUSRQ |
| 39 | NC | | 40 | /RESET |
| 41 | /WAIT | | 42 | A7 |
| 43 | +12 V | | 44 | A6 |
| 45 | NC | | 46 | A5 |
| 47 | /M1 | | 48 | A4 |
| 49 | /RFSH | | 50 | /ROMCS |
| 51 | A8 | | 52 | /BUSACK |
| 53 | A10 | | 54 | A9 |
| 55 | NC | | 56 | A11 |

### Model-Specific Differences

| Signal | 16K/48K/Spectrum+ | 128K/+2 | +2A/+3/+2B/+3B |
|---|---|---|---|
| `/ROMCS` (lower 50) | Connected — held high to disable internal ROM (connected via 680 Ω resistor on the motherboard) | Connected | **Not connected** |
| `/OE` (upper 7) | Not connected | Not connected | **Connected** — ROM output enable |
| `/OE` (lower 30) | Composite video out (16K/48K) | Not used | **Connected** — ROM output enable |
| `/CK` (lower 16) | CPU clock | **Not connected** (Spanish 128K only) | CPU clock |

The +3 and its derivatives have **two physical ROM chips**, each with its output enable (`/OE`) routed to the expansion port (upper pin 7 and lower pin 30). Applying ROM disable signals to lower pin 30 on a 16K/48K causes brief display patterning but no permanent damage.

### ZX80/ZX81 Compatibility

The ZX Spectrum edge connector is related to the earlier ZX80/ZX81 connector. The data bus, low address lines (A0–A7), and a subset of the control bus remain in the same locations relative to the index slot. Other signals were moved or added.
