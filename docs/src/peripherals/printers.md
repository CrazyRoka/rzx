# Printers

## ZX Printer

The ZX Printer (1981) is compatible with the ZX80 (with ROM upgrade), ZX81 and ZX Spectrum. It is a compact 32-column printer using aluminium-coated paper. The image is "burned" onto the paper by two metal pins — a voltage passed through the pins produces a spark that leaves a black dot.

The ZX Printer is addressed identically to the Alphacom 32 and Timex TS2040 (I/O port `0xFB`). Notable quirks:

- Data lines D0 and D7 are latched and remain high until the computer writes something new to the printer. Even if the data read in is unused, an output instruction should be sent to reset the latches.
- The paper-detect signal is used internally to stop the styli off the paper. If power is applied to the stylus, the paper signal goes high even between scans — the stylus must be turned off before attempting to detect the paper edge.

## Alphacom 32

A thermal 32-column printer, slightly larger than the ZX Printer, with a print speed of approximately 2 lines per second. Uses the same I/O mapping as the ZX Printer and Timex TS2040. Requires a 240 V adaptor.

## Timex TS2040

A branded version of the Alphacom 32 distributed by Timex for the US market. Uses thermal paper (4.33 in / 110 mm wide rolls). Powered by 120 V AC, 60 Hz, 35 W input, 24 V AC 1.2 A output.

I/O is via port `0xFB` (selected by A2 low, A7 high):

| Bit | OUT (write) | IN (read) |
|---|---|---|
| 0 | — | High when ready for next bit |
| 1–5 | — | — |
| 6 | — | Low if printer is connected |
| 7 | Apply power to print head | High at start of a new line |
| 2 | Low = start motor, high = stop | — |

All output lines retain their state until new data is sent. At switch-on or after pressing the feed button, D7 is low and D2 is high (motor stopped).

A self-test mode is triggered by pressing OFF while holding ON/ADVANCE — the printer repeatedly prints lines of `8`'s and `1`'s.

## QL-800 Printer

A 9-pin dot-matrix printer styled to match the Sinclair QL, attaching to the SER1 port. Uses RS-232 serial.

| Switch 1-1 | Switch 1-2 | Baud rate |
|---|---|---|
| OFF | OFF | 9600 |
| ON | OFF | 4800 |
| OFF | ON | 2400 |
| ON | ON | 1200 |

RS-232 25-pin connector: pin 1 = frame ground, pin 3 = RX data, pin 7 = signal ground, pin 20 = DTR (ready when +3 to +25 V).

## Seikosha GP-50s

A 46-column dot-matrix printer using plain paper (tractor feed optional). Maximum paper width 5 in, print speed up to 40 characters per second. Shipped with a Spectrum-compatible cable.
