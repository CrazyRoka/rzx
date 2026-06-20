# Undocumented Opcodes & Flags

## Undocumented Flag Bits (3 and 5)

Bits 3 and 5 of the F register are not used by any documented Z80 instruction. However, they are **not** random — they contain predictable values derived from the last operation. These bits affect real programs; for example, **Sabre Wulf**, **Ghosts'n'Goblins**, and the **Speedlock** loaders rely on their specific behaviour.

### General Rule

After most instructions, bits 5 and 3 reflect the corresponding bits of the **last 8-bit result** that updated the standard flags.

### Per-Instruction Behaviour

| Instruction | Bits 3 and 5 |
|---|---|
| `CP n` | Copied from the argument, not the result |
| `ADD HL,xx` / `ADC HL,xx` / `SBC HL,xx` | Set as for the second (high-byte) step of the 16-bit addition. Z is set only if the full 16-bit result is zero. (S and Z unchanged by `ADD HL,xx`.) |
| `BIT n,r` | P/V = Z. S is reset unless `BIT 7,r` with bit 7 of r set. |
| `BIT n,(HL)` / `BIT n,(IX/IY+d)` | 3 and 5 copied from internal Z80 state. Set by: `ADD HL,xx` (H before add), `LD r,(IX/IY+d)` (high byte of IX/IY+d), `JR d` (high byte of jump target). |
| `SCF` / `CCF` / `CPL` | Copied from A. `CCF` sets H to the previous carry value. |
| `LDD/LDDR/LDI/LDIR` | 3 = bit 3 of (value + A); 5 = bit 1 of that value |
| `CPD/CPDR/CPI/CPIR` | 3 = bit 3 of (A − (HL) − H); 5 = bit 1 of that value |
| `IND/INDR/INI/INIR` / `OUTD/OTDR/OUTI/OTIR` | S, 5, 3 as `DEC B`; N = bit 7 of value written/read; C and H computed from (C ± 1) + value; P/V via the formula below |

### P/V for Block I/O Instructions

P/V for `IND/INDR/OUTD/OTDR` (first value) and `INI/INIR/OUTI/OTIR` (second value) depends on C, the I/O data byte `inp`, and B:

1. Compute `Temp1` from C and `inp` bits 0–1 (see reference table in Sean Young's document).
2. Compute `Temp2`:
   ```
   If (B & 0x0F) == 0:
       Temp2 = Parity(B) xor (B.4 or (B.6 and not B.5))
   else:
       Temp2 = Parity(B) xor (B.0 or (B.2 and not B.1))
   ```
3. `P/V = Temp1 xor Temp2 xor C.2 xor inp.2`

## Undocumented IX/IY Low-Byte Instructions

`DD 66 nn` = `LD H,(IX+nn)` reads from memory and writes to the high byte of IX. Similarly `DD 6E nn` = `LD L,(IX+nn)`, and so on for all instructions that normally use H and L. These are widely used in commercial software.

## Real-World Examples

- **Sabre Wulf:** The rhino walks backward or runs in small circles in a corner if the S flag behaviour of `BIT 7,(IX+6)` is not emulated correctly. The code at `0xAD86` does `BIT 7,(IX+6)` followed by `JP P,0xAD8F`.
- **Ghosts'n'Goblins:** Uses undocumented flag behaviour due to a programming error.
- **Speedlock:** Relies on precise undocumented flag emulation for correct operation.
- **128K ROM:** Uses the AF register to temporarily hold a return address, relying on undocumented flag preservation.

## Manufacturer Differences

### NMOS vs CMOS Z80

The documented instruction set is identical across all Z80 variants, but NMOS and CMOS implementations differ on several undocumented instructions:

| Feature | NMOS (Zilog, NEC, Mostek) | CMOS (Zilog, Toshiba) |
|---|---|---|
| `OUT (C),0` | Writes 0 to the port | Writes **255** (`OUT (C),255`) |
| `LD A,I` / `LD A,R` | IFF2 bug: records state after interrupt reset during the instruction | Bug fixed — IFF2 state is captured correctly |
| HALT during refresh | R increments, memory refreshed | R **not** incremented, **no refresh** during HALT |
| `SCF`/`CCF` bits 3/5 | OR of previous F bits 3/5 with A bits 3/5 | AND with unknown value (unreliable) |

**LD A,I / LD A,R bug:** On NMOS Z80s, if an interrupt arrives during `LD A,I` or `LD A,R`, IFF2 is reset before the instruction reads the flag state, so the instruction records the **wrong** interrupt state. This can be used to **detect** the CPU type: an NMOS Z80 will show IFF2 = 0 after the instruction, while a CMOS Z80 shows the correct state.

**OUT (C),0 vs OUT (C),255:** This is the most reliable runtime detection method — write to a harmless port and read it back. NMOS returns 0, CMOS returns 255.

### U880 and East European Clones

The **U880** (East German), **Т34ВМ1** / **T34VM1** (Russian, Angstrem), and **КР1858ВМ1** / **KR1858VM1** (Russian) are Z80 clones used in machines like the Didaktik Gama. They differ from Zilog Z80s in setting the **carry flag after OUTI** differently. The **UA880D** may run at 4 MHz; the **UB880D** is rated for 2.5 MHz.

A CMOS clone, the **КР1858ВМ3** / **KR1858VM3**, exhibits the same CMOS HALT behaviour (no R increment, no refresh) and additionally permits HALT during DI (interrupts remain disabled afterwards) and interrupts HALT immediately rather than after a delay.

### SCF/CCF Timing Dependence

On genuine Zilog NMOS Z80s, the way bits 5 and 3 are affected by `SCF`/`CCF` depends on the **previous instruction**:
- If the previous instruction **modified** the flags: bits 5/3 are **copied** from A (move).
- If the previous instruction **did not modify** the flags (or after an interrupt): bits 5/3 are **ORed** from A.
- On NEC and other clones: the OR becomes an **AND** with an unknown value, making the result unreliable.
