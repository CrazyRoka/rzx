# +3 Disk Drive & .DSK Format

## Disk Controller

The +3 uses a **NEC uPD765A** floppy disk controller (FDC). Later models may use a **WD1770** instead. The FDC controls the internal 3-inch disk drive (single-sided, 180 KB formatted) and supports external drives.

### I/O Ports

| Port | Direction | Function |
|---|---|---|
| `0x1FFD` | OUT | Bit 3: disk motor on (1) / off (0). Also used for memory paging — see [128K Memory Paging](../memory/paging_128k.md). |
| `0x2FFD` | IN | Main status register of the uPD765A |
| `0x3FFD` | IN/OUT | Read data from FDC / write data to FDC |

### Drive Modification

An Amiga 3.5" drive can be attached by wiring the pins by name and providing external power. A public-domain formatter can format the disk to 720 KB.

## .DSK Format

TODO: Document the .DSK disk image format — standard CPC-style format (track/sector/header layout), extended PC/MFM format, and the +3-specific formatting details.

## Further Reading

The uPD765A is the same FDC used in the Amstrad CPC range. Relevant documentation: the first two entries in the "Disks" section at the Unofficial Amstrad WWW Resource.
