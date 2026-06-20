# Channels & Streams

The Spectrum's I/O system is based on channels and streams — a software abstraction layer in the ROM that decouples program I/O from the specific hardware device being addressed.

A **channel** represents a hardware device (keyboard, screen, printer). A **stream** is a numbered conduit (0–15) that can be associated with any channel. Programs write to stream numbers; the ROM routes the data to the channel's handler routines.

| Channel | Device | Supports Input | Supports Output |
|---|---|---|---|
| `"K"` | Keyboard | Yes | Yes |
| `"S"` | Screen | No | Yes |
| `"P"` | Printer | No | Yes |
| `"R"` | Edit buffer (internal) | No | Yes |

On power-up, four streams are opened automatically: stream 0 → `"K"`, stream 1 → `"K"`, stream 2 → `"S"`, stream 3 → `"P"`. The BASIC statements `INPUT` and `PRINT` are shorthand for `INPUT #0` and `PRINT #2` respectively; `LPRINT` is shorthand for `PRINT #3`.

## Subsections

- [Stream Concepts](./concepts.md) — opening, closing, device independence, and stream commands
- [Channel & Stream Memory Formats](./memory_formats.md) — CHANS and STRMS structures in memory
