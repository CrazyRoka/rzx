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

TODO: Document known differences between Z80 manufacturers (Mostek, Sharp, SGS, Zilog) and their effect on undocumented opcodes.
