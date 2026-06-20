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

## RS-232 Connector (9-pin D-sub)

The RS-232 port uses a 9-pin D-sub connector (DE-9) with the following pinout:

```
\ 5 o   o   o   o   o 1/
 \  9 o   o   o   o 6 /
  \------------------/
```

| Pin | Signal | Direction |
|---|---|---|
| 1 | Not connected | |
| 2 | TX Data | Input (to Spectrum) |
| 3 | RX Data | Output (from Spectrum) |
| 4 | DTR | Input — should be high when ready |
| 5 | CTS | Output — high when ready |
| 6 | Not connected | |
| 7 | Ground | |
| 8 | Not connected | |
| 9 | +9 V | Pull-up |

### RS-232 Cable Wiring

To connect to standard RS-232 equipment (25-pin D-sub), wire as follows:

| Interface I (9-pin) | Standard RS-232 (25-pin) |
|---|---|
| Pin 2 (TX Data) | Pin 2 (TX Data) |
| Pin 3 (RX Data) | Pin 3 (RX Data) |
| Pin 5 (CTS) | Pin 5 (CTS) |
| Pin 6 (+9 V) | Pin 6 (DSR) |
| Pin 7 (Ground) | Pin 7 (Ground) |
| Pin 4 (DTR) | Pin 20 (DTR) |

## ROM Listings

Geoff Wearmouth provides complete assembly listings of Interface I ROM versions 1 and 2 (version 2 used after serial number 87315).
