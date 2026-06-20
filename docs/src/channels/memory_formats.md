# Channel & Stream Memory Formats

## Channel Information Area (CHANS)

Channel records are stored in the channel information area starting at `CHANS` and ending at `PROG` – 2. Each record has the format:

| Offset | Length | Description |
|---|---|---|
| 0 | 2 | Address of output routine |
| 2 | 2 | Address of input routine |
| 4 | 1 | Channel code letter (`'K'`, `'S'`, `'P'`, etc.) |

The output routine receives character codes in register `A`. The input routine returns a character code in `A` with the carry flag set if data is available; if no data is available, both carry and zero are reset. A channel that does not support input or output should provide a stub (e.g. `RST 8` with an appropriate error code).

## Stream Table (STRMS)

The stream table is a 38-byte area starting at `STRMS`. It contains 16-bit offsets from `CHANS` for each stream:

- Streams 0–15: 16 entries × 2 bytes = 32 bytes
- Streams 253, 254, 255 (used internally by BASIC): 3 entries × 2 bytes = 6 bytes

A zero entry indicates the stream is not open.

A stream offset value of 1 means the channel record starts at `CHANS`, and so on. To redirect a channel, modify its record in `CHANS` to point to custom I/O routines. The `"P"` channel is the safest to redirect; the `"K"` channel is constantly restored by BASIC and should not be modified.

## Extended Format (with ZX Interface I)

When the ZX Interface I is present, channel records use an extended format:

| Offset | Length | Description |
|---|---|---|
| 0 | 2 | `0x0008` (address of Spectrum error routine) |
| 2 | 2 | `0x0008` (ditto) |
| 4 | 1 | Channel code letter |
| 5 | 2 | Address of output routine in shadow ROM |
| 7 | 2 | Address of input routine in shadow ROM |
| 9 | 2 | Total length of channel information |
| 11 | * | Any additional data needed by the channel |

The first two words being `0x0008` signals that the routines are in the shadow ROM. If either is a different address, the shadow ROM will not be paged in and the entry at offset 5 or 7 may contain arbitrary data.

## Creating a New Channel from Machine Code

It is not possible to create a new channel from BASIC. The following assembler routine creates a channel `"U"` (or any ASCII character) and associates it with stream 4:

```
LD HL,(PROG)        ; new channel starts below PROG
DEC HL
LD BC,0x0005        ; make space
CALL 0x1655         ; MAKE-ROOM
INC HL              ; HL → 1st byte of new channel data
LD A,0xfd           ; LSB of output routine
LD (HL),A
INC HL
PUSH HL             ; save address of 2nd byte
LD A,0xfd           ; MSB of output routine
LD (HL),A
INC HL
LD A,0xc4           ; LSB of input routine
LD (HL),A
INC HL
LD A,0x15           ; MSB of input routine
LD (HL),A
INC HL
LD A,0x55           ; channel name 'U'
LD (HL),A
POP HL              ; HL → 2nd byte of output routine addr
LD DE,(CHANS)
AND A
SBC HL,DE           ; compute offset from CHANS
EX DE,HL
LD HL,STRMS         ; find entry for stream 4
LD A,0x04
ADD A,0x03          ; streams 0–3 are at offsets 0–2
ADD A,A             ; each entry is 2 bytes
LD B,0x00
LD C,A
ADD HL,BC
LD (HL),E           ; store offset LSB
INC HL
LD (HL),D           ; store offset MSB
RET
```

This creates a channel with output routine at 65021 and input routine at 5572 (which generates error `'J'`). The stream number can be from −3 to 15, but it is safest to use 4–15 and leave the system streams (−3 to −1) and standard streams (0–3) alone.
