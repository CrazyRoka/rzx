# Architecture Overview

ZX Spectrum is a one-man band playing aggressively on a single CPU. 

To emulate the Spectrum accurately, you must understand the tight coupling between its few components. On the Spectrum, CPU and ULA are fighting over the same physical memory bus.

## The Core Components

The original 48K Spectrum consists of only four active components from an emulation standpoint. Later models extend this base with additional chips, but the core remains the same:

1. **The Z80A CPU**: The brain. Running at either 3.5 MHz (48K/+) or approximately 3.5469 MHz (128K/+2/+2A/+2B/+3), it executes instructions, handles interrupts, and drives all I/O.
2. **The 16K ROM**: A mask ROM containing the Sinclair BASIC interpreter and core I/O routines. It occupies the lowest address space (`0x0000 - 0x3FFF`). Later models may page in alternative ROM banks.
3. **The RAM**: 48K of dynamic RAM on original models (`0x4000 - 0xFFFF`), expanded to 128K on later models via bank switching. The lower 16K (`0x4000 - 0x7FFF`) is always shared with the video hardware.
4. **The ULA (Uncommitted Logic Array)**: The custom silicon that generates the video signal, reads the keyboard, outputs tape audio, and triggers interrupts. It is *not* a programmable GPU; it is a hardwired state machine.

### Additional Components in Later Models

| Model | Added Components |
|---|---|
| 128K | AY-3-8912 sound chip, 128K RAM paging controller, keypad |
| +3 | WD1770 floppy disk controller, +3-specific ROM, additional paging port |

## The Bus and "Contention"

The most important architectural concept to internalize is **Memory Contention**.

The ULA must constantly read from the lower 16K of RAM to figure out what pixels and colors to display on the TV. However, there is only one data bus connecting the RAM to the rest of the system. 

- **When the ULA needs a byte**, it takes priority. If the Z80 tries to read or write to the lower 16K of RAM at the exact same time, the ULA forces the Z80 to wait (halting it via the `WAIT` pin) until the ULA is done. 
- **When the ULA is drawing the border or in vertical blank**, it doesn't need the RAM, so the Z80 has uninterrupted access.

This means the Z80 runs slightly slower when executing code or reading data from the lower 16K of RAM during the active screen drawing period. This phenomenon is called *contended memory*, and emulating it accurately is the #1 challenge of Spectrum emulation. Games relied on these exact slowdowns to time their routines.

On 128K models, the contention pattern also applies to the alternate screen buffer when it is displayed, and the paging logic introduces minor differences in the exact timing of ULA memory access.

## The I/O Philosophy

The Spectrum uses the Z80's dedicated I/O space. You communicate with the ULA, keyboard, and audio using `IN` and `OUT` instructions targeting port addresses. To make things even more quirkier, the Spectrum only partially decodes port addresses. For example, the ULA responds to *any* even port number, though by convention, only Port `0xFE` is used. Later models introduce additional I/O ports for memory paging (`0x7FFD`, `0x1FFD`) and the AY-3-8912 sound chip (`0xFFFD`, `0xBFFD`).

## Emulation Strategy Implications

Because the ULA and CPU share the RAM bus, you cannot emulate the Z80 and the Video independently. Many Spectrum games change colors or draw pixels mid-scanline, relying on the exact T-state (clock cycle) the Z80 is currently on. 

**Your emulator must be cycle-stepped.** You must know exactly which T-state the CPU is on at all times, pause the CPU when the ULA contends it, and allow the ULA to read the screen data on the exact correct T-state. If your timing is off by even one T-state, effects like multicolor graphics will glitch, and some games will crash.