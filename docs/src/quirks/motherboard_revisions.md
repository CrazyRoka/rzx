# Motherboard Revisions

## Issue 1 (1982)
- Earliest production revision.
- Upper 32 KB RAM provided as a daughterboard expansion.
- Requires the "dead cockroach" bodge (two capacitors near ULA) for correct I/O contention timing.
- Requires the "spider" bodge (gate ULA /IORQ with A0) to prevent stray contention on odd ports.
- Speaker: 200 Ω, driven directly by the ULA.

## Issue 2 (1982–83)
- 16 KB variant can be upgraded to 48 KB by adding four logic ICs plus eight DRAMs.
- Same bodge requirements as Issue 1.
- Rubber key mat changed to bluish-grey colour.
- Speaker: 200 Ω, driven directly by the ULA.

## Issue 3 (1984)
- "Spider" fix incorporated into the PCB layout.
- Switch to 6C series ULA (lower power, lower temperature).
- EAR socket handling changed — causes compatibility differences with tape loading (see [EAR/MIC Delays](./ear_mic_delays.md)).
- **New speaker circuit:** impedance changed to 40 Ω, driven through a dedicated transistor (TR7, ZTX450) instead of directly from the ULA. Significantly louder output.
- First revision sold as the Spectrum+.

## Issue 4 (A and B) (1984)
- Improved ULA memory signal timing by delaying RAS through two previously unused gates in IC24.
- Requires ULA 6C001E-7.

## Issue 5
- Described in the service manual but **no known units exist** in the wild.
- Exact changes are undocumented.

## Issue 6 (1985)
- Includes changes required for the **AMI SAGA** gate array (a custom ASIC replacing the Ferranti ULA).
- Only **one machine is known to exist**: Issue 6A board in a Spectrum+ case, marked with a sticker requesting return to Sinclair's QA manager for repair.
- AMI SAGA differences: runs significantly cooler, no chroma bias circuitry needed, clock clean enough to drive Z80 directly without amplification.
- Introduces the **ZX8401** (also marked PCF1306P) — a single IC that consolidates the six 74LS logic ICs previously used for memory access control.

## Summary Table

| Issue | ULA series | Spider integrated | Speaker | RAM config | Key notes |
|---|---|---|---|---|---|
| 1 | 5C | No (bodge) | 200 Ω, direct | 16K + daughterboard | Dead cockroach bodge needed |
| 2 | 5C | No (bodge) | 200 Ω, direct | 16K or 48K | |
| 3 | 6C | Yes | 40 Ω, TR7 | 16K or 48K | EAR socket change, first Spectrum+ |
| 4A/B | 6C001E-7 | Yes | 40 Ω, TR7 | 16K or 48K | RAS timing improvement |
| 5 | — | — | — | — | Service manual only, no known units |
| 6 | — | Yes | 40 Ω, TR7 | 48K | ZX8401 gate array, AMI SAGA |
