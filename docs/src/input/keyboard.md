# Keyboard Matrix

The Spectrum keyboard is a membrane-switch matrix arranged as 8 rows (driven by address lines A15–A8 through diodes) by 5 columns (read back on data lines D0–D4). The ULA drives one address line low at a time, and the returned nibble on D0–D4 indicates which keys in that row are pressed (0 = pressed).

## Physical Matrix Layout

The keys are arranged on the membrane in the following physical layout:

```
  OUTER SIDE    A15  A14  A8  A13  A12  A9  A10  A11
               D0   BR   EN   CS   P    0   A    Q    1
               D1   SS   L    Z    O    9   S    W    2
               D2   M    K    X    I    8   D    E    3
               D3   N    J    C    U    7   F    R    4
               D4   B    H    V    Y    6   G    T    5
  INNER SIDE
```

| Abbreviation | Key |
|---|---|
| BR | BREAK (CAPS SHIFT + SPACE) |
| EN | ENTER |
| CS | CAPS SHIFT |
| SS | SYMBOL SHIFT |

In reality, the matrix connections are in one row on the top side of the membrane.

## Row Decoding by Port Address

Each of the 8 rows is selected by reading from a specific port address (high byte selects the row):

| Port Address | Row select | Keys (D0–D4) |
|---|---|---|
| `0xFEFE` | A0 low | SHIFT, Z, X, C, V |
| `0xFDFE` | A1 low | A, S, D, F, G |
| `0xFBFE` | A2 low | Q, W, E, R, T |
| `0xF7FE` | A3 low | 1, 2, 3, 4, 5 |
| `0xEFFE` | A4 low | 0, 9, 8, 7, 6 |
| `0xDFFE` | A5 low | P, O, I, U, Y |
| `0xBFFE` | A6 low | ENTER, L, K, J, H |
| `0x7FFE` | A7 low | SPACE, SYM SHIFT, M, N, B |

## Reading the Keyboard

To read a specific row, perform `IN A,(port)` where the high byte selects the row and the low byte is `0xFE`. The result in A has:

- Bits 0–4: key states (0 = pressed) for that row
- Bit 5: always 1
- Bit 6: EAR input (see [Port 0xFE](../ula/port_fe.md))
- Bit 7: always 1

If no key is pressed in the row, all five low bits are 1. Reading `0x00FE` (by `XOR A; IN A,(0xFE)`) selects all rows simultaneously via partial decoding; if all five bits of the result are 1, no key is pressed anywhere.
