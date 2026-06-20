# Attributes & Flashing

## Attribute Byte Format

TODO: Document the attribute byte format — INK (bits 0–2), PAPER (bits 3–5), BRIGHT (bit 6), FLASH (bit 7). Include the colour palette mapping.

## Colour Palette

TODO: Map the 3-bit colour values (0–7) to the actual RGB values produced by the Spectrum.

## Attribute File Area

The attribute map occupies `0x5800 – 0x5AFF` (768 bytes, one byte per 8×8 character cell). It uses the same three-thirds layout as the pixel area.

## FLASH Timing

The FLASH effect is produced by the ULA: every **16 frames** the ink and paper colours of all cells with FLASH bit set are swapped. A full flash cycle (normal → inverted → normal) therefore takes **32 frames** ≈ **0.64 seconds**.

### Flash Timing Bug

The FLASH inversion is not properly synchronised to pixel boundaries. When the ink and paper colours are swapped, the transition can occur mid-pixel, leaving a thin visible edge 1–2 pixels wide at the boundary between attribute cells where the FLASH state differs. This occurs because the inversion of the display bitmap data is not aligned with the attribute cell boundaries during the ULA's output stage. Emulators that render per-scanline or per-pixel should apply the FLASH inversion at the same point in the scanline as the real hardware to reproduce this artifact accurately.
