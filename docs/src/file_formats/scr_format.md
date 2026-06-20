# SCR Screen Format

The .scr format stores a raw dump of the Spectrum's video memory: 6144 bytes of pixel data followed by 768 bytes of attribute data, for a total of 6912 bytes.

## Standard SCR (6912 bytes)

| Offset | Size | Content |
|---|---|---|
| 0 | 6144 | Pixel data (as stored at address `16384`) |
| 6144 | 768 | Attribute data (as stored at address `22528`) |

The pixel data is structured in the Spectrum's unusual three-thirds layout. Each third covers one-third of the screen height (8 character rows = 64 pixel rows). Within each third, consecutive 32-byte blocks represent the top pixel row of successive character cells, then the second pixel row, and so on.

```
Offset 0x0000  +-----------------+
               | Top third       |  2048 bytes = 64 rows × 32 bytes/row
               | (character rows |  8 character rows × 8 pixel rows
               |  0–7)           |
Offset 0x0800  +-----------------+
               | Middle third    |  2048 bytes
               | (character rows |
               |  8–15)          |
Offset 0x1000  +-----------------+
               | Bottom third    |  2048 bytes
               | (character rows |
               |  16–23)         |
Offset 0x1800  +-----------------+
               | Attributes      |  768 bytes = 24 rows × 32 bytes/row
               | (24×32 grid)    |
Offset 0x1B00  +-----------------+
```

### Address Calculation

To find the byte in the pixel area for a given (x, y) coordinate:

```
row_lo  = y & 0x07       ; pixel row within character cell (0–7)
third   = y & 0x18       ; character row within third (0, 8, 16)
char_y  = y & 0xC0       ; which third (0, 64, 128)
address = 16384 + char_y + (third << 2) + row_lo + (x >> 3)
```

For the attribute byte at (x, y):

```
attr_addr = 22528 + ((y >> 3) << 5) + (x >> 3)
```

## Timex Hi-Colour SCR (12288 bytes)

An extended .scr file used by vbSpec, BMP2SCR Pro, and Fuse:

| Offset | Size | Content |
|---|---|---|
| 0 | 6144 | Pixel data (from address `16384`) |
| 6144 | 6144 | Colour data (from address `22528`) |

The colour data is a full 6144-byte dump starting at `22528` — the same size as the pixel area. This stores the extended colour information used by Timex video modes.

## Timex Hi-Res SCR (12289 bytes)

| Offset | Size | Content |
|---|---|---|
| 0 | 6144 | Pixel data (from address `16384`) |
| 6144 | 6144 | Colour data (from address `22528`) |
| 12288 | 1 | Hi-res colour configuration (value written to port `0xFF`) |
