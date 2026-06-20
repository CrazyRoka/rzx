# ZX Interface I

The ZX Interface I is an add-on peripheral that provides microdrive storage, an RS-232 serial port, and a local area network. It contains an 8 KB shadow ROM that is automatically paged in when certain ROM addresses are executed (`0x0008`, `0x1708` — the error and `CLOSE#` routines) and paged out when the Z80 executes `RET` at address `0x0700`.

The Interface I uses three I/O ports.

## Port 0xE7 — Microdrive Data

Used to send or receive data to or from the microdrive. Accessing this port halts the Z80 until the Interface I has collected 8 bits from the microdrive head. If the microdrive motor is not running or no formatted cartridge is present, the Spectrum hangs — this is the well-known "IN 0 crash".

## Port 0xEF — RS-232 and Microdrive Control

| Bit | Read | Write |
|---|---|---|
| 7 | (unused) | (unused) |
| 6 | (unused) | (unused) |
| 5 | (unused) | WAIT (network sync) |
| 4 | BUSY | CTS (RS-232) |
| 3 | DTR (RS-232) | ERASE (microdrive) |
| 2 | GAP (microdrive) | R/W (microdrive) |
| 1 | SYNC (microdrive) | COMMS CLK |
| 0 | WRITE PROTECT | COMMS DATA |

Note: the BUSY signal (bit 4 read) is used by hardware, not software.

## Port 0xF7 — RS-232 and Network Data

When the microdrive is not in use, the COMMS DATA signal (from port 0xEF bit 0) selects the function of port 0xF7 bit 0:

| Bit | Read | Write |
|---|---|---|
| 7 | TXDATA (RS-232 input) | (unused) |
| 6–1 | (unused) | (unused) |
| 0 | NET INPUT | NET OUTPUT / RXDATA (RS-232 output) |

TXDATA and RXDATA are the RS-232 serial data lines. COMMS DATA (port 0xEF bit 0 write) determines whether port 0xF7 bit 0 is used for the RS-232 output (RXDATA) or the network output.

## ROM Listings

Geoff Wearmouth provides complete assembly listings of Interface I ROM versions 1 and 2 (version 2 used after serial number 87315).
