# Attributes & Flashing

## Attribute Byte Format

Each attribute byte controls an 8×8 pixel cell. The byte is laid out as follows:

| Bit | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|
| Field | FLASH | BRIGHT | PAPER G | PAPER R | PAPER B | INK G | INK R | INK B |

- **INK** (bits 0–2) — pixel colour (0–7)
- **PAPER** (bits 3–5) — background colour (0–7)
- **BRIGHT** (bit 6) — 0 = normal intensity, 1 = bright
- **FLASH** (bit 7) — 0 = static, 1 = alternating

Both INK and PAPER use the same 3-bit colour encoding:

| Value | Colour | Normal | Bright |
|---|---|---|---|
| 0 | Black | #000000 | #000000 |
| 1 | Blue | #0000CD | #0000FF |
| 2 | Red | #CD0000 | #FF0000 |
| 3 | Magenta | #CD00CD | #FF00FF |
| 4 | Green | #00CD00 | #00FF00 |
| 5 | Cyan | #00CDCD | #00FFFF |
| 6 | Yellow | #CDCD00 | #FFFF00 |
| 7 | White | #CDCDCD | #FFFFFF |

BRIGHT black is still black — no visible difference.

## Attribute File Area

The attribute map occupies `0x5800 – 0x5AFF` (768 bytes, one byte per 8×8 character cell). It uses the same three-thirds layout as the pixel area.

## FLASH Timing

The FLASH effect is produced by the ULA: every **16 frames** the ink and paper colours of all cells with FLASH bit set are swapped. A full flash cycle (normal → inverted → normal) therefore takes **32 frames** ≈ **0.64 seconds**.

> **Discrepancy note:** Some sources report the swap occurring every **32 frames** (full cycle 64 frames ≈ 1.28 s). The difference may stem from variations between ULA revisions or from how the frame counter is defined. Emulators should verify against real hardware. The 32-frame toggle (1.28 s cycle) appears in multiple independent references.

### Flash Timing Bug

The FLASH inversion is not properly synchronised to pixel boundaries. When the ink and paper colours are swapped, the transition can occur mid-pixel, leaving a thin visible edge 1–2 pixels wide at the boundary between attribute cells where the FLASH state differs. This occurs because the inversion of the display bitmap data is not aligned with the attribute cell boundaries during the ULA's output stage. Emulators that render per-scanline or per-pixel should apply the FLASH inversion at the same point in the scanline as the real hardware to reproduce this artifact accurately.
