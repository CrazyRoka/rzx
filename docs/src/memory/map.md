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

### Contended Regions by Model

| Model | Contended memory |
|---|---|
| 16K / 48K / Spectrum+ | `0x4000 – 0x7FFF` (video RAM + attributes) |
| 128K / +2 | Banks 1, 3, 5, 7 (banks 5 and 7 are the two screen buffers) |
| +2A / +2B / +3 | Banks 4, 5, 6, 7 |

On 128K machines, contended banks are slowed regardless of whether they are the currently displayed screen.
