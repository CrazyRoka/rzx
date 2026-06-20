# SNA Snapshot Format

The .sna format is one of the most widely supported snapshot formats. It stores a dump of the Z80 registers and RAM.

## 48K SNA Format

The 48K snapshot is 49179 bytes:

| Offset | Size | Description |
|---|---|---|
| 0 | 1 | I register |
| 1 | 8 | HL', DE', BC', AF' (4 words) |
| 9 | 10 | HL, DE, BC, IY, IX (5 words) |
| 19 | 1 | Interrupt flip-flops (bit 2 = IFF2, 1 = EI / 0 = DI) |
| 20 | 1 | R register |
| 21 | 4 | AF, SP (2 words) |
| 25 | 1 | Interrupt mode (0 = IM 0, 1 = IM 1, 2 = IM 2) |
| 26 | 1 | Border colour (bits 0–2) |
| 27 | 49152 | RAM dump `16384–65535` (49152 bytes) |

### PC-on-stack Issue

The program counter is **not** stored in the file. Instead, the PC value is pushed onto the stack before the snapshot is taken, and a `RETN` instruction is used to restore it when the snapshot is loaded. This overwrites 2 bytes of RAM at `SP-2` and `SP-1`.

If the stack pointer points into critical data, those 2 bytes are corrupted. This can cause crashes with snapshots saved at critical moments. A workaround: read the PC from the stack, replace the overwritten word with `0x0000`, and increment SP. This works with some titles (Batman, Bounder, others) but is not universally reliable.

## 128K SNA Format

The 128K extension keeps the same 49179-byte 48K header, then appends the extra banks and a proper PC field:

| Offset | Size | Description |
|---|---|---|
| 0 | 27 | SNA header (see above) |
| 27 | 16384 | RAM bank 5 |
| 16411 | 16384 | RAM bank 2 |
| 32795 | 16384 | RAM bank N (the currently paged bank) |
| 49179 | 2 | **PC** (program counter) — not pushed to stack |
| 49181 | 1 | Port `0x7FFD` setting |
| 49182 | 1 | TR-DOS ROM paged (1) or not (0) |
| 49183 | 16384 × N | Remaining RAM banks in ascending order |

The third RAM bank saved is always the one currently paged (even if that is bank 5 or 2, which are already saved). The remaining banks follow in ascending order.

**Example:** if bank 4 is paged in, the snapshot order is: bank 5, bank 2, bank 4, then banks 0, 1, 3, 6, 7. Total size = 131103 bytes (8 banks × 16384 + 49179 - 2×16384 + 2 + 1 + 1).

If bank 5 is paged in, the order is: bank 5, bank 2, bank 5 (duplicate), then banks 0, 1, 3, 4, 6, 7. Total size = 147487 bytes.

## Loading

To restore a 48K SNA:
1. Load registers from offsets 0–26.
2. Copy RAM from offset 27 to address `16384`.
3. Read PC from `(SP)`. Push PC onto the emulated stack at `SP-2`.
4. Execute `RETN` to jump to the saved PC with the correct interrupt state (IFF2 was restored from bit 2 of the interrupt byte).

For 128K SNA:
1. Load registers from offsets 0–26.
2. Copy RAM banks from their offsets.
3. Set PC directly from offset 49179.
4. Write port `0x7FFD` setting to restore paging state.
