# Interrupts (IM 0, 1, 2)

The Z80 has three maskable interrupt modes and one non-maskable interrupt (NMI). On the Spectrum, the ULA pulls the /INT pin low for **32 T-states** every frame (50 Hz). This pin is sampled during the last M-cycle of every instruction (excluding repeated IX/IY prefixes — DD and FD).

The /INT pin must be held low for at least **23 T-states** (the length of the longest IX/IY instruction) to guarantee detection.

## Interrupt Enable / Disable

- `EI` enables interrupts but with a **one-instruction delay** — an interrupt cannot occur until the instruction **after** EI completes.
- `DI` disables interrupts immediately.
- When an interrupt is accepted, IFF1 and IFF2 are both reset. `LD A,I` and `LD A,R` copy IFF2 to the P/V flag — if the instruction is interrupted, P/V is reset even if interrupts were enabled beforehand.

## IM 1 (Default)

On reset, the Spectrum initialises to IM 1. The processor executes `RST 38` when an interrupt is requested. Timing:

| Cycle | T-states | Action |
|---|---|---|
| M1 | 7 | Acknowledge interrupt, decrement SP |
| M2 | 3 | Write high byte of PC to stack, decrement SP |
| M3 | 3 | Write low byte of PC to stack, set PC = 0x0038 |
| **Total** | **13** | |

## IM 2

The processor builds a 16-bit address from the I register (high byte) and the byte placed on the data bus by the interrupting device (low byte). On the Spectrum, no device places a byte on the bus, so the data bus reads `0xFF` (bus floats high). The resulting vector address is `256 × I + 255`.

Timing:

| Cycle | T-states | Action |
|---|---|---|
| M1 | 7 | Acknowledge interrupt, decrement SP |
| M2 | 3 | Write high byte, decrement SP |
| M3 | 3 | Write low byte |
| M4 | 3 | Read low byte of vector |
| M5 | 3 | Read high byte of vector, jump |
| **Total** | **19** | |

A common trick: place a 257-byte table of equal bytes at `256 × I` and put the interrupt routine at an address that is a multiple of 257. Alternatively, fill the table with `0xFF` and place a `JR` opcode (`0x18`) at `0xFFFF`, which jumps to `0xFFF4` where a long jump to the actual handler resides.

## IM 0

The interrupting device places an instruction on the data bus. On a standard Spectrum this is `0xFF` (`RST 38`), identical to IM 1 behaviour in practice but less reliable.

| Opcode on bus | T-states | Notes |
|---|---|---|
| `RST n` (or `0xFF`) | 12 | 6 (M1) + 3 (M2) + 3 (M3) |
| `CALL nnnn` | 19 | 6 + 3 + 4 + 3 + 3 |

## Double Interrupts

If the interrupt service routine starts with `EI / NOP`, a second interrupt can occur immediately because /INT is sampled 27 T-states after the first sample (19 for IM 2 entry + 4 for EI + 4 for NOP) — still within the 32 T-state /INT pulse. This can cause re-entrant interrupts.

## NMI (Non-Maskable Interrupt)

The /NMI pin is sampled at the end of every instruction (except DD/FD prefixes and possibly EI/DI). When triggered:

- IFF1 is reset (maskable interrupts disabled).
- IFF2 is left unchanged (so the NMI handler can check the previous interrupt state).
- PC is set to `0x0066`.

`RETN` restores IFF1 from IFF2, returning the maskable interrupt state to what it was before the NMI.

| Cycle | T-states | Action |
|---|---|---|
| M1 | 5 | Opcode read, decrement SP |
| M2 | 3 | Write high byte, decrement SP |
| M3 | 3 | Write low byte, jump to 0x0066 |
| **Total** | **11** | |

## ULA /INT Pulse

The ULA holds /INT low for exactly **32 T-states** on the 48K Spectrum. The pin is sampled during the last M-cycle of each instruction. If it goes high before sampling, no interrupt occurs. For accurate emulation, the interrupt line should be asserted for precisely this window every frame.
