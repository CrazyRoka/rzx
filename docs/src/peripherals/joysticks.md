# Joystick Interfaces

## Kempston

The Kempston interface reads through I/O port `0x1F`. Data is active-high:

| Bit | Direction |
|---|---|
| 0 | Right |
| 1 | Left |
| 2 | Down |
| 3 | Up |
| 4 | Fire |
| 5–7 | 0 |

Format: `000FUDLR`. Support must be built into the software.

## Sinclair (Left and Right)

The two Sinclair joystick ports map directions and fire to keyboard keys:

| Sinclair left | Port `0xF7FE` | Key |
|---|---|---|
| Left | bit 4 | 1 |
| Right | bit 4 | 2 |
| Down | bit 4 | 3 |
| Up | bit 4 | 4 |
| Fire | bit 4 | 5 |

| Sinclair right | Port `0xEFFE` | Key |
|---|---|---|
| Left | bit 4 | 6 |
| Right | bit 4 | 7 |
| Down | bit 4 | 8 |
| Up | bit 4 | 9 |
| Fire | bit 4 | 0 |

## Cursor

Maps to keys 5 (left), 6 (down), 7 (up), 8 (right), 0 (fire). Read through a combination of bit 4 of port `0xF7FE` and bits 0, 2, 3, 4 of port `0xEFFE`. Common in interfaces by Protek and AGF.

## Programmable

Allows mapping joystick directions to any keyboard key, configured through software (DK'Tronics, AGF) or hardware jumpers (AGF). Typically a setup program is run before loading the target game.

## Fuller

The Fuller Audio Box includes a joystick interface readable via port `0x7F`:

| Bit | Direction |
|---|---|
| 0 | Up (active low) |
| 1 | Down (active low) |
| 2 | Left (active low) |
| 3 | Right (active low) |
| 4 | Fire (active low) |
| 5–7 | — |

Format: `F---RLDU`, active-low bits.

## Timex (TS2068 / TC2068)

Built-in joystick interface accessed through register 14 of the AY-3-8912 sound chip. Address bits A8 and A9 select the joystick (01 = left, 10 = right, 11 = both OR-ed).

When register 14 acts as an input port (bit 6 of register R7 = 0):

| Bit | Direction |
|---|---|
| 0 | Up (active low) |
| 1 | Down (active low) |
| 2 | Left (active low) |
| 3 | Right (active low) |
| 4 | Fire (active low) |
| 5–7 | Always 1 |

Typical read code:

```
LD   A,7
OUT  (0xF5),A       ; select R7
IN   A,(0xF6)
AND  0xBF           ; clear bit 6 (I/O port A = R14)
OUT  (0xF6),A
LD   A,14
OUT  (0xF5),A       ; select R14
LD   A,3            ; 3 = both, 2 = left, 1 = right
IN   A,(0xF6)       ; result: FxxxRLDU, active low
```

The TS2048 has a built-in Kempston interface instead.
