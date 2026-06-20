# Tape Data Structure (Pulses)

Each tape block produced by the `SAVE` command consists of a leader tone, sync pulses, and the encoded data.

## Block Structure

| Segment | Header block | Data block |
|---|---|---|
| Leader pulses | 8063 pulses @ 2168 T-states each | 3223 pulses @ 2168 T-states each |
| First sync pulse | 1 pulse @ 667 T-states | 1 pulse @ 667 T-states |
| Second sync pulse | 1 pulse @ 735 T-states | 1 pulse @ 735 T-states |
| Data | Flag byte + header (19 bytes) | Flag byte + data + checksum |

Note on leader count: the .TZX format specification incorrectly states 8064 leader pulses for header blocks and 3220 for data blocks.

A `SAVE` produces two blocks consecutively: the 19-byte header block followed by the variable-length data block.

## Bit Encoding

Each bit is encoded as a pair of pulses:

| Bit | Pulse pair |
|---|---|
| 0 | 2 pulses @ 855 T-states each |
| 1 | 2 pulses @ 1710 T-states each |

Within each byte, the most significant bit is transmitted first. Memory is transmitted starting from the lowest address.

## Byte and Block Framing

Each block's data section has the following structure:

1. **Flag byte:** `0x00` for header blocks, `0xFF` for data blocks.
2. **Data payload:** the actual header (17 bytes) or program data.
3. **Checksum byte:** the XOR of all preceding bytes (including the flag byte) produces `0x00`. Calculated as: XOR all data bytes together with the flag byte; the result is the checksum.

## Timing Notes

All pulse timings above are nominal — they assume no output contention. In practice, contention during active display can stretch pulse lengths slightly. See [Contended I/O](../ula/contended_io.md) and [Contended Memory](../memory/contention.md).
