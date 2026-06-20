# Model Variants

## Timing Comparison

| Model | CPU speed | T-states/frame | Scanlines | T-states/line | Lines before picture | Border change at |
|---|---|---|---|---|---|---|
| 16K / 48K / + | 3.5 MHz | 69888 | 312 | 224 | 64 | 14339–14342 |
| 128K / +2 | ~3.5469 MHz | 70908 | 311 | 228 | 63 | 14365–14368 |
| +2A / +2B / +3 | ~3.5469 MHz | 70908 | 311 | 228 | 63 | 14365–14368 |

## Contention Summary

| Model | Contention start | Pattern | Repeat | Contended banks |
|---|---|---|---|---|
| 16K / 48K / + | 14335 | 6,5,4,3,2,1,0,0 | 224 | `0x4000–0x7FFF` |
| 128K / +2 | 14361 | 6,5,4,3,2,1,0,0 | 228 | 1, 3, 5, 7 |
| +2A / +2B / +3 | 14365 | 1,0,7,6,5,4,3,2,1,0 | 228 | 4, 5, 6, 7 |

## Per-Model Details

### 16K (1982)
- Original Sinclair model, 16 KB RAM (only `0x4000–0x7FFF` populated)
- Upper 32 KB may mirror lower RAM on some boards
- Identical timing to 48K

### 48K (1982)
- The baseline reference model. 48 KB RAM.
- Most documentation assumes this model unless noted.

### Spectrum+ (1984)
- 48K internals in a new keyboard case with a reset button.
- Identical to 48K for emulation purposes.

### 128K (1985, Sinclair)
- First model with 128 KB RAM and bank switching.
- AY-3-8912 sound chip.
- Numeric keypad with editing functions.
- Faster CPU clock (~3.5469 MHz) and altered video timing.
- The I register snow bug also exists on this model, and additionally crashes the machine shortly after I is set to point to contended memory.
- Port 0xFE bit 6 behaviour matches Issue 3.

### +2 (1986, Amstrad)
- 128K hardware in a grey case with built-in CASSETTE deck.
- Two built-in Sinclair-style joystick ports (non-standard pinout).
- Timing and memory identical to 128K.

### +3 (1987, Amstrad)
- 128K with a 3" floppy disk drive, WD1770 FDC.
- Four ROM banks (editor, syntax checker, +3DOS, 48K BASIC).
- Additional paging port `0x1FFD` with special memory modes.
- Port 0xFE bit 6 always returns 0 (no EAR/MIC dependency).
- No floating bus — unused ports always return 255.
- Port 0xFE is not contended.
- Different contention pattern and combined instruction entries.

### +2A / +2B (1987–88, Amstrad)
- +3 motherboard in a +2-style case, no disk drive.
- Same memory paging, timing, and contention as the +3.
- Same port 0xFE and floating bus behaviour as the +3.
