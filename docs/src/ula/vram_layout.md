# Video Memory Layout

The Spectrum's screen memory has 6912 bytes: 6144 bytes of pixel data at `0x4000` (16384) and 768 bytes of attribute data at `0x5800` (22528).

## Three-Thirds Structure

The pixel area is divided into three thirds, each covering 8 character rows (64 pixel rows). This unusual layout means consecutive pixel rows within a character cell are separated by 256 bytes, not 32.

```
0x4000 +-----------------+  ^
       | Top third       |  |  2048 bytes (character rows 0–7)
       |                 |  |  each row = 32 bytes
0x4800 +-----------------+  v
       | Middle third    |  |  2048 bytes (character rows 8–15)
0x5000 +-----------------+  v
       | Bottom third    |  |  2048 bytes (character rows 16–23)
0x5800 +-----------------+  v
       | Attributes      |  |  768 bytes (24 rows × 32 columns)
0x5AFF +-----------------+  v
```

## Address Calculation

### Pixel Address from (x, y)

```
row_lo  = y & 0x07       ; pixel row within character cell (0–7)
third   = y & 0x18       ; character row within third (0, 8, 16)
char_y  = y & 0xC0       ; which third (0, 64, 128)
addr    = 0x4000 + char_y + (third << 2) + row_lo + (x >> 3)
```

In practice:
- Bits 0–2 of y: pixel row within the character cell (0–7)
- Bits 3–4 of y: character row within the third (0, 1, 2)
- Bits 6–7 of y: third selector (0, 64, 128)
- Bits 3–7 of x: column within the row (0–31)

Resulting address breakdown:

| Bit | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| | 0 | 1 | Y7 | Y6 | Y2 | Y1 | Y0 | Y3 | Y4 | X7 | X6 | X5 | X4 | X3 | X2 | X1 |

Where Y7–Y0 is the y-coordinate and X7–X0 is the x-coordinate.

### Attribute Address from (x, y)

```
attr_addr = 0x5800 + ((y >> 3) << 5) + (x >> 3)
```

This maps each 8×8 character cell to a single attribute byte. There are 32 columns × 24 rows = 768 attribute bytes.

## Internal Structure Detail

Within each third, the bytes are arranged so that the 32 bytes of the top pixel row of character row N come first, then the 32 bytes of the top pixel row of character row N+1, and so on for 8 iterations (one per character row in the third), followed by the second pixel row of character row N, etc.

```
Offset 0x0000  row 0 of char row 0  (32 bytes)
Offset 0x0020  row 0 of char row 1  (32 bytes)
...
Offset 0x00E0  row 0 of char row 7  (32 bytes)
Offset 0x0100  row 1 of char row 0  (32 bytes)
Offset 0x0120  row 1 of char row 1  (32 bytes)
...
```

This repeats for all 8 pixel rows within the 8 character rows of the third.
