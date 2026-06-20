# Video Timing (T-states per frame)

Video timing differs between the 48K family and the later 128K-derived machines. Both share the same principle of a ULA-generated interrupt synchronised to the video signal, but the exact counts vary.

## 48K / Spectrum+ Timing

| Property | Value |
|---|---|
| CPU speed | 3.5 MHz |
| Scanlines per frame | 312 |
| T-states per scanline | 224 |
| Scanlines before picture | 64 |
| Frame T-states | (64 + 192 + 56) × 224 = **69888** |
| Interrupt frequency | 3.5 MHz / 69888 ≈ **50.08 Hz** |

| Region | Scanlines | T-states |
|---|---|---|
| Vertical retrace + top border | 64 | 14336 |
| Active display (192 screen lines) | 192 | 43008 |
| Bottom border | 56 | 12544 |
| **Total** | **312** | **69888** |

### Scanline Timing (48K)

Each scanline lasts 224 T-states. Every half T-state a pixel is written, so the ULA reads a byte every 4 T-states (one pixel byte and one attribute byte per read):

| Segment | Pixels | T-states |
|---|---|---|
| Left border | 48 | 24 |
| Active screen | 256 | 128 |
| Right border | 48 | 24 |
| Horizontal retrace | — | 48 |
| **Total** | | **224** |

### Screen Start Timing (48K)

After an interrupt, the first byte of screen memory (address 16384 / `0x4000`) is displayed at **14336 T-states** (64 scanlines) into the frame. Contention begins one T-state earlier at **cycle 14335**.

An OUT to port `0xFE` ending at T-state 14339–14342 changes the border at exactly the position of byte 16384. Other positions follow from the line structure (8 pixels = 4 T-states, 1 line = 224 T-states).

## 128K / +2 / +2A / +2B / +3 Timing

| Property | Value |
|---|---|
| CPU speed | ~3.5469 MHz |
| Scanlines per frame | 311 |
| T-states per scanline | 228 |
| Scanlines before picture | 63 |
| Frame T-states | (63 + 192 + 56) × 228 = **70908** |
| Interrupt frequency | 3.5469 MHz / 70908 ≈ **50.01 Hz** |

| Region | Scanlines | T-states |
|---|---|---|
| Vertical retrace + top border | 63 | 14364 |
| Active display (192 screen lines) | 192 | 43776 |
| Bottom border | 56 | 12768 |
| **Total** | **311** | **70908** |

### Scanline Timing (128K/+3)

Each scanline lasts 228 T-states. The horizontal segment sizes differ from the 48K:

TODO: Determine exact pixel/T-state breakdown for 128K scanline segments.

### Screen Start Timing (128K/+2)

To modify the border at the position of the first byte of the screen, the OUT must finish after **14365, 14366, 14367 or 14368** T-states have passed since interrupt.

### Screen Start Timing (+2A/+3)

The top-left pixel of the screen is displayed **14364** T-states after the 50 Hz interrupt occurs. Contention follows a different pattern — see [Contended Memory](../memory/contention.md).

## Border Colour Change Timing

To change the border at a specific horizontal position within a scanline, the OUT instruction must end at the correct T-state within the line. Each 8-pixel byte occupies 4 T-states.

**Caveat:** On 48K and 128K models, port 0xFE is subject to I/O contention during the active display period, which can delay the OUT instruction — see [Contended I/O](./contended_io.md). On +2A/+3 models, port 0xFE is **not** contended.

## Machine Variance

Investigations have revealed that video timings vary between machines even with the same ULA model. Some Spectrums use the timings given here, while others are **one T-state later** for all timing events. The reason is not currently understood.

## FLASH Timing

Every 16 frames the ink and paper colours of all flashing cells are swapped. A full flash cycle (normal → inverted → normal) takes 32 frames ≈ 0.64 seconds.
