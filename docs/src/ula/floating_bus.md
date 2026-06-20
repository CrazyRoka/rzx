# The Floating Bus

When the Z80 reads from an I/O port that no device drives, nothing places data on the bus. The value read depends on what the ULA was last doing:

- **During active display (~60% of frame):** The ULA is reading screen memory. The bus carries a mixture of pixel data (from `0x4000–0x57FF`) and attribute data (from `0x5800–0x5AFF`), interleaved with idle-high bytes (`0xFF`).
- **During border or vertical retrace (~40% of frame):** The ULA is idle and the bus floats high, returning `0xFF`.

This behaviour is used intentionally by some programs. For example, **Arkanoid** reads from unused ports to obtain pseudo-random data derived from the screen content.

See also [Contended I/O](./contended_io.md) for how reading from even ports interacts with contention.

TODO: Document how the floating bus can be used to detect 48K vs 128K models, and the exact T-state conditions under which specific data values are returned.
