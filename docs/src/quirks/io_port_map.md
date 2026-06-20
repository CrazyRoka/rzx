# I/O Port Map

Devices respond to port addresses through **partial decoding** — only certain address bits are checked, so a device responds to a range of ports rather than a single address. In the tables below, `-` means "don't care", `0` means the bit must be reset, and `1` means the bit must be set.

Bit positions are shown as `15 14 13 12 | 11 10 9 8 | 7 6 5 4 | 3 2 1 0`.

## Systems

| Device | Decode mask | Canonical port | Ref |
|---|---|---|---|
| 48K ULA | `---- ---- ---- ---0` | `0xFE` | [Port 0xFE](../ula/port_fe.md) |
| TS2068 ULA | `---- ---- 1111 1110` | `0xFE` | [Timex Models](../appendix/timex_models.md) |
| TS2068 Display mode | `---- ---- 1111 1111` | `0xFF` | [Timex Models](../appendix/timex_models.md) |
| TS2068 Horizontal Select | `---- ---- 1111 0100` | `0xF4` | |

## Audio

| Device | Decode mask | Canonical port | Ref |
|---|---|---|---|
| 128K AY register | `11-- ---- ---- --0-` | `0xFFFD` | [AY-3-8912](../audio/ay38912.md) |
| 128K AY data | `10-- ---- ---- --0-` | `0xBFFD` | [AY-3-8912](../audio/ay38912.md) |
| TS2068 AY register | `---- ---- 1111 0101` | `0xF5` | [Timex Models](../appendix/timex_models.md) |
| TS2068 AY data | `---- ---- 1111 0110` | `0xF6` | [Timex Models](../appendix/timex_models.md) |
| Fuller AY control | `---- ---- 0011 1111` | `0x3F` | [Other Peripherals](../peripherals/misc.md) |
| Fuller AY data | `---- ---- 0101 1111` | `0x5F` | [Other Peripherals](../peripherals/misc.md) |
| Fuller joystick | `---- ---- 0111 1111` | `0x7F` | [Joysticks](../peripherals/joysticks.md) |

## Memory Paging

| Device | Decode mask | Canonical port | Ref |
|---|---|---|---|
| 128K / +2 memory control | `0--- ---- ---- --0-` | `0x7FFD` | [128K Paging](../memory/paging_128k.md) |
| +2A / +3 primary memory | `01-- ---- ---- --0-` | `0x7FFD` | [+2A/+3 Paging](../memory/paging_128k.md) |
| +2A / +3 secondary memory | `0001 ---- ---- --0-` | `0x1FFD` | [+2A/+3 Paging](../memory/paging_128k.md) |

## Disk Controllers

| Device | Decode mask | Canonical port | Ref |
|---|---|---|---|
| +3 FDC data | `0011 ---- ---- --0-` | `0x3FFD` | [+3 Disk](../tape/dsk_format.md) |
| +3 FDC status | `0010 ---- ---- --0-` | `0x2FFD` | [+3 Disk](../tape/dsk_format.md) |
| Beta 128 command/state | `---- ---- 0001 1111` | `0x1F` | [Mass Storage](../peripherals/mass_storage.md) |
| Beta 128 track | `---- ---- 0011 1111` | `0x3F` | [Mass Storage](../peripherals/mass_storage.md) |
| Beta 128 sector | `---- ---- 0101 1111` | `0x5F` | [Mass Storage](../peripherals/mass_storage.md) |
| Beta 128 data | `---- ---- 0111 1111` | `0x7F` | [Mass Storage](../peripherals/mass_storage.md) |
| Beta 128 system | `---- ---- 1111 1111` | `0xFF` | [Mass Storage](../peripherals/mass_storage.md) |
| +D command/state | `---- 1110 0011` | `0xE3` | [Mass Storage](../peripherals/mass_storage.md) |
| +D memory page | `---- 1110 0111` | `0xE7` | [Mass Storage](../peripherals/mass_storage.md) |
| +D track | `---- 1110 1011` | `0xEB` | [Mass Storage](../peripherals/mass_storage.md) |
| +D system | `---- 1110 1111` | `0xEF` | [Mass Storage](../peripherals/mass_storage.md) |
| +D sector | `---- 1111 0011` | `0xF3` | [Mass Storage](../peripherals/mass_storage.md) |
| +D printer | `---- 1111 0111` | `0xF7` | [Mass Storage](../peripherals/mass_storage.md) |
| +D data | `---- 1111 1011` | `0xFB` | [Mass Storage](../peripherals/mass_storage.md) |
| D80 command/state | `---- ---- 1000 0001` | `0x81` | |
| D80 track | `---- ---- 1000 0011` | `0x83` | |
| D80 sector | `---- ---- 1000 0101` | `0x85` | |
| D80 data | `---- ---- 1000 0111` | `0x87` | |
| D80 system | `---- ---- 1000 1001` | `0x89` | |
| JLO status/command | `---- ---- 1000 1111` | `0x8F` | |
| JLO track | `---- ---- 1001 1111` | `0x9F` | |
| JLO sector | `---- ---- 1010 1111` | `0xAF` | |
| JLO data | `---- ---- 1011 1111` | `0xBF` | |
| JLO select | `---- ---- 1011 0111` | `0xB7` | |

## Interfaces

| Device | Decode mask | Canonical port | Ref |
|---|---|---|---|
| +3 Centronics printer | `0000 ---- ---- --0-` | — | [Printers](../peripherals/printers.md) |
| Aerco Centronics | `---- ---- 0111 1111` | `0x7F` | |
| ZX Interface I (RS-232/Net) | `---- ---- ---1 0---` | `0xF7` | [ZX Interface I](./zx_interface1.md) |
| ZX Interface I (control) | `---- ---- ---0 1---` | `0xEF` | [ZX Interface I](./zx_interface1.md) |
| ZX Interface I (Microdrive) | `---- ---- ---0 0---` | `0xE7` | [ZX Interface I](./zx_interface1.md) |
| Kempston joystick | `---- ---- 000- ----` | `0x1F` | [Joysticks](../peripherals/joysticks.md) |
| Sinclair Interface II (left) | `---0 ---- ---- ----` | `0xEFFE` | [Joysticks](../peripherals/joysticks.md) |
| Sinclair Interface II (right) | `---- 0--- ---- ----` | `0xF7FE` | [Joysticks](../peripherals/joysticks.md) |
| Multiface 1 (read) | `---- ---- 1001 1111` | `0x9F` | [Other Peripherals](../peripherals/misc.md) |
| Multiface 1 (write) | `---- ---- 0001 1111` | `0x1F` | [Other Peripherals](../peripherals/misc.md) |
| Multiface 128 (read) | `---- ---- 1011 1111` | `0xBF` | [Other Peripherals](../peripherals/misc.md) |
| Multiface 128 (read alt) | `---- ---- 1001 1111` | `0x9F` | [Other Peripherals](../peripherals/misc.md) |
| Multiface 128 (write) | `---- ---- 0011 1111` | `0x3F` | [Other Peripherals](../peripherals/misc.md) |
| Multiface 3 (read/button) | `---- ---- 0011 1111` | `0x3F` | |
| Multiface 3 (write) | `---- ---- 1011 1111` | `0xBF` | |
| Multiface 3 (0x7FFD trap) | `0111 1111 0011 1111` | `0x7F3F` | |
| Multiface 3 (0x1FFD trap) | `0001 1111 0011 1111` | `0x1F3F` | |

## Other

| Device | Decode mask | Canonical port | Ref |
|---|---|---|---|
| ZX Printer | `---- ---- ---- -0--` | all odd ports | [Printers](../peripherals/printers.md) |
| Timex TS2040 / Alphacom 32 | `---- ---- 1111 1011` | `0xFB` | [Printers](../peripherals/printers.md) |
| ZX LPrint III (page in ROM) | `---- ---- 1--- -0--` | `0xFB` | |
| ZX LPrint III (page out ROM) | `---- ---- 0--- -0--` | `0x7B` | |
| Grafpad pen up/down | `1111 1111 0011 1111` | `0xFF3F` | |
| Grafpad X coordinate | `1111 1111 1011 1111` | `0xFFBF` | |
| Grafpad Y coordinate | `1111 1111 0111 1111` | `0xFF7F` | |
| Kempston Mouse buttons | `1111 1010 1101 1111` | `0xFADF` | [Other Peripherals](../peripherals/misc.md) |
| Kempston Mouse X | `1111 1011 1101 1111` | `0xFBDF` | [Other Peripherals](../peripherals/misc.md) |
| Kempston Mouse Y | `1111 1111 1101 1111` | `0xFFDF` | [Other Peripherals](../peripherals/misc.md) |
