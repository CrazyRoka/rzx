# The .TAP Format

The .TAP format is the simplest emulated tape format. It stores blocks exactly as they appear on tape, omitting only the leader and sync pulses. Each block is stored as a length prefix followed by the raw bytes (flag + data + checksum).

## Tape Header Structure

The header block on tape is 19 bytes: a flag byte (`0x00`), 17 bytes of header data, and a checksum byte. The 17-byte header has this structure:

| Byte offset | Length | Description |
|---|---|---|
| 0 | 1 | File type |
| 1 | 10 | Filename (padded with spaces, `0x20`) |
| 11 | 2 | Length of data block (little-endian) |
| 13 | 2 | Parameter 1 (little-endian) |
| 15 | 2 | Parameter 2 (little-endian) |

### File Types

| Type | Value | Description |
|---|---|---|
| PROGRAM | 0 | BASIC program |
| Number array | 1 | Numeric array (`DIM`) |
| Character array | 2 | String array (`DIM a$()`) |
| CODE | 3 | Machine code or screen$ |

### Parameters by Type

- **PROGRAM (type 0):** Parameter 1 = autostart line number (or a value ≥ 32768 if no `LINE` parameter was given). Parameter 2 = offset of the variable area relative to the program start.
- **CODE (type 3):** Parameter 1 = load address. Parameter 2 = 32768 (for code, or 0 for screen$). A `SCREEN$` file is a CODE file with start address 16384 and length 6912.
- **Data arrays (types 1, 2):** The byte at header offset 14 holds the variable name.

## Checksum

The checksum byte is calculated so that XORing together the flag byte, all data bytes, and the checksum byte produces `0x00`. In practice: XOR all bytes of the block except the checksum, then append the result.

## Block Sequence on Tape

A `SAVE` writes two blocks:

1. **Header block** — flag `0x00`, 17 header bytes, checksum
2. **Data block** — flag `0xFF`, data bytes, checksum

## Example

`SAVE "ROM" CODE 0,2` produces on tape (pipe separates the two blocks):

```
00 03 52 4f 4d 20 20 20 20 20 20 20 02 00 00 00 00 80 f1 | ff f3 af a3
```

Breakdown of the header block (first 19 bytes):

| Bytes | Value | Meaning |
|---|---|---|
| 00 | 0x00 | Flag byte (header) |
| 03 | 0x03 | File type: CODE |
| 52 4f 4d 20... | "ROM...." | Filename (padded to 10) |
| 02 00 | 0x0002 | Data length = 2 |
| 00 00 | 0x0000 | Parameter 1 = start address 0 |
| 80 00 | 0x0080 | Parameter 2 = 32768 |
| f1 | 0xf1 | Checksum |

Data block:

| Bytes | Value | Meaning |
|---|---|---|
| ff | 0xFF | Flag byte (data) |
| f3 af | ... | Two bytes of ROM data |
| a3 | 0xa3 | Checksum |
