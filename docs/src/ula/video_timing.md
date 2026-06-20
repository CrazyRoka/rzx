# Video Timing (T-states per frame)

## Frame Structure

The ULA generates a 50 Hz interrupt synchronised to the video signal. Each frame consists of 312 scanlines at 224 T-states per line:

```
Frame T-states = (64 + 192 + 56) × 224 = 69888
```

| Region | Scanlines | T-states |
|---|---|---|
| Vertical retrace + top border | 64 | 14336 |
| Active display (192 screen lines) | 192 | 43008 |
| Bottom border | 56 | 12544 |
| **Total** | **312** | **69888** |

The interrupt frequency is therefore 3.5 MHz / 69888 ≈ **50.08 Hz**. A software clock running for an hour will be approximately 6 seconds fast due to this discrepancy. On real hardware the frequency also varies slightly as the ULA heats up.

## Scanline Timing

Each scanline lasts 224 T-states. Every half T-state a pixel is written, so the ULA reads a byte every 4 T-states (one pixel byte and one attribute byte per read). The active display line is 256 pixels (32 bytes) wide, bordered by 48 pixels on each side:

| Segment | Pixels | T-states |
|---|---|---|
| Left border | 48 | 24 |
| Active screen | 256 | 128 |
| Right border | 48 | 24 |
| Horizontal retrace | — | 48 |
| **Total** | | **224** |

## Screen Start Timing

After an interrupt, the first byte of screen memory (address 16384 / `0x4000`) is displayed at **14336 T-states** (64 scanlines) into the frame. Contention begins one T-state earlier at **cycle 14335** — see [Contended Memory](../memory/contention.md).

The border change position can be computed relative to this. An OUT to port `0xFE` ending at T-state 14339–14342 changes the border at exactly the position of byte 16384. Other positions follow from the line structure (8 pixels = 4 T-states, 1 line = 224 T-states).

## Border Colour Change Timing

To change the border at a specific horizontal position within a scanline, the OUT instruction must end at the correct T-state within the line. Each 8-pixel byte occupies 4 T-states. The first pixel byte of active screen (byte 16384) spans T-state 14339–14342 relative to the interrupt; border changes at other positions can be derived from this.

**Caveat:** Port 0xFE is subject to I/O contention during the active display period, which can delay the OUT instruction — see [Contended I/O](./contended_io.md).

## Machine Variance

Investigations have revealed that video timings vary between machines even with the same ULA model. Some Spectrums use the timings given here, while others are **one T-state later** for all timing events. The reason is not currently understood.

## FLASH Timing

TODO: FLASH timing documentation moved to [Attributes & Flashing](./attributes.md).
