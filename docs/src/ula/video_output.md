# Video Output

## 128K / +2 Composite Video

The 128K and +2 provide composite PAL video via an 8-pin DIN socket. Pin 1 carries composite video, pin 2 is ground. There is **no audio** on any pin of this connector.

| Pin | Signal | Level |
|---|---|---|
| 1 | Composite PAL | 75 Ω, 1.2 V pk-pk |
| 2 | GND | 0 V |

The picture may appear dull unless the TV's video input is high impedance. If the input is 75 Ω (typical), the built-in 68 Ω series resistor on pin 1 attenuates the signal. To compensate, short the 68 Ω resistor inside the Spectrum (follow the track on the PCB), or connect the signal directly to the RF modulator input.

**Audio:** The +3's audio can be taken from the MIC socket. For a better balance between 48K (beeper) and 128K (AY-3-8912) sound, take the signal directly from **pin 5 of IC38**.

## 128K / +2 RGB Video

The 8-pin DIN socket on the 128K provides separate RGB output at **TTL levels**:

| Pin | Signal | Level |
|---|---|---|
| 1 | Composite PAL | 75 Ω, 1.2 V pk-pk |
| 2 | GND | 0 V |
| 3 | Bright | TTL |
| 4 | Composite sync | TTL |
| 5 | Vertical sync | TTL |
| 6 | Green | TTL |
| 7 | Red | TTL |
| 8 | Blue | TTL |

## +2A / +3 RGB Video

The +2A and +3 use the same 8-pin DIN connector but with different signals — RGB is **analogue** rather than TTL, and audio is available:

| Pin | Signal | Level |
|---|---|---|
| 1 | +12 V | |
| 2 | GND | 0 V |
| 3 | Audio out | |
| 4 | /Composite sync | TTL |
| 5 | +12 V | |
| 6 | Green | Analogue, 1.67 V p-p |
| 7 | Red | Analogue, 1.67 V p-p |
| 8 | Blue | Analogue, 1.67 V p-p |

## Connector Pinout Reference

```
   --7--   --6--
         |
    3--  8  --1
  --     |     --
      /     \
     5   |   4
    /    2    \
         |
```
