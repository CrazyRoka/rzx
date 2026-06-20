# The I Register Snow Bug ("Rain" Effect)

After each instruction fetch cycle, the Z80 places the I-R register pair (the interrupt vector register and memory refresh register) on the address bus. The lowest 7 bits (the R register) provide dynamic RAM refresh.

The ULA monitors the address bus to detect when the Z80 accesses contended memory. However, if the I register holds a value in the range **64–127** (`0x40–0x7F`), the ULA becomes confused: it interprets every instruction fetch's refresh cycle as an access to the contended region, even when the actual instruction is executing from uncontended memory.

The ULA cannot service these phantom accesses at the rate they occur, and periodically **misses reading a screen byte**. Instead of the actual byte, the ULA repeats the byte it read previously, producing a visual "snow" effect on the display. The program continues to run normally.

One title that deliberately uses this effect for visual interest is **Vectron**.
