# EAR/MIC Circuit Delays

## Issue 2 vs Issue 3 — Voltage Levels

The ULA uses the same pin (pin 28) for the MIC socket, EAR socket, and internal speaker. Bits 3 and 4 written to port `0xFE` control this pin, and bit 6 read from port `0xFE` reflects its voltage level.

The threshold voltage at pin 28 for bit 6 to read as 1 is **0.70 V** on both Issue 2 and Issue 3 machines, with no hysteresis. The difference between issues is the output voltage levels produced:

| OUT bit 4 | OUT bit 3 | Issue 2 V | Issue 3 V |
|---|---|---|---|
| 1 | 1 | 3.79 V | 3.70 V |
| 1 | 0 | 3.66 V | 3.56 V |
| 0 | 1 | 0.73 V | 0.66 V |
| 0 | 0 | 0.39 V | 0.34 V |

The `0 1` combination (bit 4 = 0, bit 3 = 1) produces 0.73 V on Issue 2 (just above threshold, bit 6 = 1) but 0.66 V on Issue 3 (below threshold, bit 6 = 0). This is the root cause of the Issue 2 vs 3 detection behaviour — see [Port 0xFE](../ula/port_fe.md).

## Capacitor Delay

Capacitors on the EAR and MIC lines introduce an analogue delay when bit 4 transitions from 1 to 0. The delay varies depending on how long bit 4 was held high:

| High-level duration | Approximate delay |
|---|---|
| Short (a few T-states) | ~180 T-states (~50 µs) |
| Long (20 ms, one frame) | ~2800 T-states (~800 µs) |

There is **no delay** when bit 4 changes from 0 to 1.

This delay affects tape-loading accuracy in emulation — the exact pulse widths received from tape depend on the preceding output state and duration.
