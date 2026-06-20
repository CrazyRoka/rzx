# Contended I/O

## Spider Modification

Early ULA revisions contended **all** I/O access regardless of port address, even though the ULA only responds to even ports (A0 = 0). The "spider" modification gates the ULA's /IORQ input with A0, ensuring only even ports trigger the ULA's contention circuit. This was added as a bodge on Issue 1 and Issue 2 boards and incorporated into the PCB from Issue 3 onward.

Without this fix, reading odd ports during the active display could cause unnecessary delays and return stale data from the ULA's internal bus. Emulators that handle the "spider" correctly apply the contention pattern **only** when A0 = 0, regardless of whether the port is actually `0xFE`.

## Contention Patterns

It takes four T-states for the Z80 to read from or write to an I/O port. This can be lengthened by the ULA through two independent effects:

1. **Low bit of port address is reset (even ports):** The ULA must supply the result, causing a delay if the ULA is busy handling the screen.
2. **High byte of port address is in the range `0x40–0x7F`:** The ULA treats this as an attempted access to contended memory and introduces a delay, regardless of the actual memory being accessed. This effect does not apply on 128K models even if a contended RAM bank is paged into `0xC000–0xFFFF`.

These two effects combine into four possible contention patterns:

| High byte in 0x40–0x7F? | Low bit set? | Contention pattern |
|---|---|---|
| No | Reset (even port) | `N:1, C:3` |
| No | Set (odd port) | `N:4` |
| Yes | Reset (even port) | `C:1, C:3` |
| Yes | Set (odd port) | `C:1, C:1, C:1, C:1` |

The pattern is read left to right:
- **`N:n`** — no delay, Z80 continues uninterrupted for `n` T-states.
- **`C:n`** — ULA halts the Z80 for the same delay as a contended memory access at this cycle (e.g. 6 T-states at cycle 14335, 5 at 14336, etc. on the 48K). After the delay, the Z80 continues for `n` cycles.

See [Contended Memory](../memory/contention.md) for the per-cycle delay table.

## Model Exceptions

- **+2A, +2B, +3:** Port 0xFE is **not** contended. Reading or writing even ports does not trigger a ULA delay on these models.
- **Port 0x7FFD (all 128K models):** Not contended in itself, but the high byte being `0x7F` (which falls in `0x40–0x7F`) causes the second effect to apply.
