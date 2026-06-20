# Model Detection

Several techniques can distinguish between ZX Spectrum models at runtime. Emulators may need to model these for software that queries the hardware.

## Port 0xFE Bit 6 (EAR Input)

On Issue 3 and 128K/+2 machines, bit 6 read from port `0xFE` depends on what was written to port `0xFE` bits 3 and 4. On **+2A/+3** machines, bit 6 **always returns 0** if there is no EAR input signal, regardless of the written value.

```
; returns 0 on +2A/+3, non-zero on other models
LD   A,0x10      ; bit 4 = 1, bit 3 = 0
OUT  (0xFE),A
IN   A,(0xFE)
AND  0x40        ; isolate bit 6
```

## Floating Bus (Unused Port Read)

On **48K, 128K, and +2** models, reading a non-existing port (e.g. `0xFF`) returns a mixture of `0xFF` and screen/attribute bytes during the active display period (~60% of frame). On **+2A/+3** machines, reading an unused port **always returns 255**.

```
; returns variable data on 48K/128K/+2, always 255 on +2A/+3
IN   A,(0xFF)
```

## I Register Snow Crash

On the **128K/+2**, setting the I register to point to contended memory (`0x40–0x7F`) not only causes screen snow (as on the 48K) but also **crashes the machine shortly after**. This can theoretically be used to distinguish 128K from 48K, though it is destructive.

## Memory Paging Port

Writing to port `0x7FFD` has no effect on 48K models (the port doesn't respond or is ignored). On 128K and later models, it controls bank switching. Reading back the system variable at `0x5B5C` (if it changes) can also indicate a 128K-class machine.

## CPU Speed

128K-derived machines run at ~3.5469 MHz vs 3.5 MHz on 48K models. This can be detected by timing a fixed-length software loop against the 50 Hz interrupt — but the difference is small (~1.3%) and requires careful measurement.
