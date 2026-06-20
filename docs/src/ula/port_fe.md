# Port 0xFE (Border, Sound, Keys)

Every even I/O address reaches the ULA, but only port `0xFE` should be used to avoid conflicts with other devices.

## OUT — Writing to Port 0xFE

```
Bit     7   6   5   4   3   2   1   0
      +-------------------------------+
      |       |   |   | E | M | Border |
      +-------------------------------+
```

| Bits | Function |
|---|---|
| 0–2 | Border colour (0–7, matching the standard colour palette) |
| 3 | MIC output (0 = activate) |
| 4 | EAR output and internal speaker (1 = activate) |
| 5–7 | Unused |

Bits 3 and 4 are connected through resistors — activating one partially activates the other. The EAR output is generally used as it produces a louder sound.

## IN — Reading from Port 0xFE

Reading from port `0xFE` returns the keyboard state and the EAR input. The high byte of the port address selects which half-row of keys is read:

| Port Address | Keys (bits 0–4) |
|---|---|
| `0xFEFE` | SHIFT, Z, X, C, V |
| `0xFDFE` | A, S, D, F, G |
| `0xFBFE` | Q, W, E, R, T |
| `0xF7FE` | 1, 2, 3, 4, 5 |
| `0xEFFE` | 0, 9, 8, 7, 6 |
| `0xDFFE` | P, O, I, U, Y |
| `0xBFFE` | ENTER, L, K, J, H |
| `0x7FFE` | SPACE, SYM SHIFT, M, N, B |

A zero in bits 0–4 means the corresponding key is pressed. If multiple address lines are low simultaneously, the result is the logical AND of all single-row inputs — a zero in a bit means at least one of the corresponding keys is pressed. If all five lowest bits are 1, no key in that row is pressed.

| Bit | Function |
|---|---|
| 0–4 | Keyboard row data (see table above) |
| 5 | Always 1 |
| 6 | EAR input (see Issue 2 vs 3 below) |
| 7 | Always 1 |

### Issue 2 vs Issue 3 EAR Bit Behaviour

Bit 6 read from port `0xFE` depends on the ULA output on bits 3 and 4, and differs between Issue 2 and Issue 3 motherboards:

| OUT bit 4 | OUT bit 3 | Issue 2 (bit 6) | Issue 3 (bit 6) | Issue 2 V | Issue 3 V |
|---|---|---|---|---|---|
| 1 | 1 | 1 | 1 | 3.79 V | 3.70 V |
| 1 | 0 | 1 | 1 | 3.66 V | 3.56 V |
| 0 | 1 | 1 | 0 | 0.73 V | 0.66 V |
| 0 | 0 | 0 | 0 | 0.39 V | 0.34 V |

The threshold voltage at pin 28 of the ULA for bit 6 to read as 1 is exactly **0.70 V** on both issues, with no hysteresis. The difference is that Issue 2 machines produce slightly higher output voltages — the `0 1` input combination (0.73 V) just barely exceeds the threshold on Issue 2 but falls below it on Issue 3.

This means:
- **Issue 3:** outputting with bit 4 = 0 is sufficient to reset bit 6 on a subsequent read.
- **Issue 2:** both bits 3 and 4 must be 0 for the same effect.

The commonly cited BASIC detection method (`PRINT IN 254`) gives differing results because the ROM beep routine always sets bit 3 before `OUT (0xFE),A`, so an Issue 2 machine always returns bit 6 = 1.

### Capacitor Delay on Bit 4 Transitions

There is an analogue delay when bit 4 changes from 1 to 0, caused by capacitors on the EAR and MIC lines. The delay ranges from approximately **180 T-states (~50 µs)** to **2800 T-states (~800 µs)**, depending on how long bit 4 was held high. There is no delay when bit 4 changes from 0 to 1.

### Keyboard Matrix Quirks

The keyboard is arranged as an 8×5 matrix. Any two simultaneously pressed keys can be uniquely decoded. However, with three or more keys, decoding may produce phantom results. For example, pressing CAPS SHIFT, B, and V simultaneously causes the Spectrum to also detect SPACE as pressed, producing a "Break into Program" report. Some games rely on this matrix behaviour (e.g., Zynaps requires pressing 5, 6, 7, 8, and 0 simultaneously to pause).

### Floating Bus on Port Reads

Reading from a port with the low bit set (odd ports) when the ULA is not driving the bus returns a mixture of `0xFF` and screen/attribute data — see [The Floating Bus](./floating_bus.md).
