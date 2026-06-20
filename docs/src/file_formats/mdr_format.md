# MDR Microdrive Format

The .mdr format stores the contents of a ZX Microdrive cartridge. A cartridge contains 254 sectors of 543 bytes each, plus a final write-protect flag byte, for a total file size of **137923 bytes**.

## File Layout

| Offset | Size | Description |
|---|---|---|
| 0 | 543 × 254 | 254 sector blocks |
| 137922 | 1 | Write-protect flag (non-zero = write-protected) |

## Sector Format (543 bytes)

Each sector on the Microdrive tape stores a 15-byte header block followed by a 528-byte record block. The ZX Interface I hardware strips the preamble (10 zero bytes + 2 `0xFF` sync bytes) before storing the data, so the .mdr file contains only the header and record data.

### Header Block (bytes 0–14)

| Offset | Size | Name | Description |
|---|---|---|---|
| 0 | 1 | HDFLAG | Always 0x01 (header block marker) |
| 1 | 1 | HDNUMB | Sector number (254 down to 1) |
| 2 | 2 | — | Unused (value indeterminate) |
| 4 | 10 | HDNAME | Cartridge name (space-padded) |
| 14 | 1 | HDCHK | Header checksum |

### Record Block (bytes 15–542)

| Offset | Size | Name | Description |
|---|---|---|---|
| 15 | 1 | RECFLG | Record flags (see below) |
| 16 | 1 | RECNUM | Data block sequence number (starts at 0) |
| 17 | 2 | RECLEN | Data block length (≤ 512, little-endian) |
| 19 | 10 | RECNAM | Filename (space-padded) |
| 29 | 1 | DESCHK | Record descriptor checksum |
| 30 | 512 | DATA | Data block |
| 542 | 1 | DCHK | Data block checksum |

### Record Flags (RECFLG)

| Bit | Function |
|---|---|
| 0 | Always 0 (record block) |
| 1 | Set for EOF block |
| 2 | Reset for a PRINT file |
| 3–7 | Unused (0) |

### Block States

| State | RECFLG | RECLEN |
|---|---|---|
| Used data | bit 1 = 0 | 512 |
| EOF | bit 1 = 1 | any |
| Empty | bit 1 = 0 | 0 |
| Unusable (FORMAT) | bit 1 = 1 | 0 |

### Checksum Calculation

All three checksums (HDCHK, DESCHK, DCHK) are calculated by summing all preceding bytes in the block modulo 255. The result is never 255 — if the sum modulo 255 is 0, the checksum is 0 (not 255).

- **HDCHK:** checksum of bytes 0–13 (14 bytes).
- **DESCHK:** checksum of bytes 15–28 (14 bytes).
- **DCHK:** checksum of bytes 30–541 (512 bytes of data).

If a checksum is incorrect, the Interface I treats the block as a GAP. Writing `OUT 239,0` erases a cartridge completely in approximately 7 seconds; subsequent `CAT 1` reports "microdrive not ready".

## Emulator Notes

- The .mdr file is written sequentially as the Interface I OUTs data: alternating 15-byte and 528-byte blocks.
- An emulator should verify checksums when reading. Warajevo ignores the write-protect flag (using file attributes instead) and does not require all 254 sectors to be present.
