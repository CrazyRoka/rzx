# ZX Interface I & II

## ZX Interface I

The ZX Interface I is an add-on peripheral providing microdrive storage, RS-232 serial, and a local area network. It contains an 8 KB shadow ROM paged in at ROM addresses `0x0008` and `0x1708` (error and `CLOSE#`) and paged out at the `RET` at `0x0700`.

### I/O Ports

#### Port 0xE7 — Microdrive Data

Used to send or receive data to or from the microdrive. Accessing this port halts the Z80 until the Interface I has collected 8 bits from the microdrive head. If no cartridge is present, the Spectrum hangs ("IN 0 crash").

#### Port 0xEF — RS-232 and Microdrive Control

| Bit | Read | Write |
|---|---|---|
| 7–5 | (unused) | (unused) |
| 4 | BUSY (hardware only) | CTS (RS-232) |
| 3 | DTR (RS-232) | ERASE (microdrive) |
| 2 | GAP (microdrive) | R/W (microdrive) |
| 1 | SYNC (microdrive) | COMMS CLK |
| 0 | WRITE PROTECT | COMMS DATA |

#### Port 0xF7 — RS-232 and Network Data

COMMS DATA (port 0xEF bit 0) selects bit 0 function: RS-232 output or network output.

| Bit | Read | Write |
|---|---|---|
| 7 | TXDATA (RS-232 input) | (unused) |
| 6–1 | (unused) | (unused) |
| 0 | NET INPUT | NET OUTPUT / RXDATA |

### RS-232 Connector (9-pin D-sub)

| Pin | Signal |
|---|---|
| 1 | Not connected |
| 2 | TX Data (input) |
| 3 | RX Data (output) |
| 4 | DTR (input, high = ready) |
| 5 | CTS (output, high = ready) |
| 6 | Not connected |
| 7 | Ground |
| 8 | Not connected |
| 9 | +9 V (pull-up) |

To connect to standard RS-232 (25-pin): 2→2, 3→3, 5→5, 6→6 (DSR), 7→7, 4→20 (DTR).

### System Variables

| Address | Name | Contents |
|---|---|---|
| 23734 | FLAGS3 | Flags |
| 23735 | VECTOR | Address for BASIC interpreter extension |
| 23737 | SBRT | ROM paging subroutine |
| 23747 | BAUD | Baud rate: `3500000 / (26 × baud) - 2` |
| 23749 | NTSTAT | Own network station number |
| 23750 | IOBORD | Border colour during I/O |
| 23751 | SER_FL | RS-232 workspace (2 bytes) |
| 23753 | SECTOR | Microdrive workspace (2 bytes) |
| 23755 | CHADD_ | Temporary store for CH_ADD |
| 23757 | NTRESP | Network response code |
| 23758 | NTDEST | Network destination station (0–64) |
| 23759 | NTSRCE | Source station number |
| 23760 | NTNUMB | Network block number (0–65535) |
| 23762 | NTTYPE | Header type code |
| 23763 | NTLEN | Data block length (0–255) |
| 23764 | NTDCS | Data block checksum |
| 23765 | NTHCS | Header block checksum |
| 23766 | D_STR1 | Drive number (1–8, first file specifier) |
| 23768 | S_STR1 | Stream number (0–15) |
| 23769 | L_STR1 | Device type: `"M"`, `"N"`, `"T"`, `"B"` |
| 23770 | N_STR1 | Filename length |
| 23772 | D_STR2 | Second file specifier (MOVE/LOAD) |
| 23782 | HD_00 | SAVE/LOAD/VERIFY/MERGE data type code |
| 23783 | HD_0B | Data length (0–65535) |
| 23785 | HD_0D | Data start address (0–65535) |
| 23787 | HD_0F | Program length (0–65535) |
| 23789 | HD_11 | Line number |
| 23791 | COPIES | Number of copies by SAVE |

**Note:** The original manual specifies a stream range of 1–15, but `CAT #0, 7` puts 0 in address 23768, as do some `OPEN` and `MOVE` commands.

## ZX Interface II

The ZX Interface II provides two Sinclair-style joystick ports and a ROM cartridge socket for instant game loading. Only 10 titles were released on cartridge; the interface was withdrawn within a year of release.

The joystick ports map to the keyboard matrix as described in [Joystick Interfaces](../peripherals/joysticks.md).

## Documentation

- ZX Interface I User Manual and Service Manual
- ZX Interface II User Manual and Service Manual
