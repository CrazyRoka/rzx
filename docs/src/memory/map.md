# Memory Map (16K/48K Baseline)

On 16K and 48K models, the Z80's 64 KB address space is mapped as follows:

| Address Range | Size | Description |
|---|---|---|
| `0x0000 – 0x3FFF` | 16 KB | ROM (Sinclair BASIC) |
| `0x4000 – 0x7FFF` | 16 KB | Screen area (contended) |
| `0x5B00 – 0x5BFF` | 256 B | Printer buffer (48K only) |
| `0x8000 – 0xFFFF` | 32 KB | General-purpose RAM (upper 32K, not contended) |

On the 16K model, only the lower 16 KB of RAM is populated (`0x4000 – 0x7FFF`). The upper 32 KB address range (`0x8000 – 0xFFFF`) does not respond and the RAM will wrap, mirroring the lower 16 KB on some board revisions.

On 128K models, the upper 32 KB is replaced by a paged window — see [128K Memory Paging](./paging_128k.md).

### Contended Regions

Contended memory regions: `0x4000 – 0x7FFF` (video RAM + attributes).
On 128K models, the alternate screen buffer (bank 7, when mapped to `0xC000 – 0xFFFF`) is also contended if it is the currently displayed screen.
