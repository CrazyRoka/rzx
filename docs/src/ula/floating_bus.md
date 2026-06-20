# The Floating Bus

When the Z80 reads from an I/O port that no device drives, nothing places data on the bus. The value read depends on what the ULA was last doing:

- **During active display (~60% of frame):** The ULA is reading screen memory. The bus carries a mixture of pixel data (from `0x4000–0x57FF`) and attribute data (from `0x5800–0x5AFF`), interleaved with idle-high bytes (`0xFF`).
- **During border or vertical retrace (~40% of frame):** The ULA is idle and the bus floats high, returning `0xFF`.

This behaviour is used intentionally by some programs. For example, **Arkanoid** reads from unused ports to obtain pseudo-random data derived from the screen content.

See also [Contended I/O](./contended_io.md) for how reading from even ports interacts with contention.

## Fetch Cycle Timing

During the active display, each scanline fetches pixel and attribute bytes in a repeating 8-cycle sequence. The first 4 cycles transfer two pairs of (bitmap, attribute) bytes; the remaining 4 cycles are idle (bus floats high to `0xFF`).

| Cycle | 48K T-state | 128K T-state | ULA bus content |
|---|---|---|---|
| 1 | 14338 | 14364 | Pixel byte from `0x4000` |
| 2 | 14339 | 14365 | Attribute byte from `0x5800` |
| 3 | 14340 | 14366 | Next pixel byte (`0x4001`) |
| 4 | 14341 | 14367 | Next attribute byte (`0x5801`) |
| 5–8 | 14342–14345 | 14368–14371 | IDLE (`0xFF`) |

On late-timing machines all values shift by +1 T-state.

The Z80 samples the data bus during the **final T-state** of the I/O machine cycle. All timings are relative to the ULA asserting `/INT`. Since the Z80 samples `/INT` during the final T-state of opcode execution, a minimum one-cycle delay exists before the interrupt is acknowledged.

The same floating-bus effect is observed when reading **unattached memory**, e.g. reading from the upper 32 KB address range on a 16K machine.

## +2A / +2B / +3 Exception

On the +2A, +2B, and +3 models, reading from a non-existing port (e.g. `0xFF`) will **always return 255** and never return screen or attribute bytes. This is a definitive way to distinguish the +2A/+3 from earlier models at runtime.

## Commercial Games Using the Floating Bus

The following games read from unattached I/O ports to synchronise with the display or obtain pseudo-random data:

- **Arkanoid** (original release only — the Hit Squad re-release does not use the floating bus)
- **Cobra** (original release only — the Hit Squad re-release does not use the floating bus)
- **Sidewize**
- **Short Circuit**

TODO: Document how the floating bus can be used to detect 48K vs 128K models, and the exact T-state conditions under which specific data values are returned.
