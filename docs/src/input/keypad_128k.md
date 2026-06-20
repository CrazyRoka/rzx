# 128K Keypad

The 128K model introduced editing functions accessible via key combinations using **Extended Mode** (pressing CAPS SHIFT and SYMBOL SHIFT simultaneously, labelled `[E]`) and **Symbol Shift** (`[S]`). The dedicated numeric keypad on the 128K keyboard provides the same functions.

## Keypad Function Table

| Function | Keys |
|---|---|
| Beginning of next word | `[E]` `[S]` J |
| Beginning of previous word | `[E]` I |
| Up ten lines | `[E]` P |
| Down ten lines | `[S]` I |
| Start of line | `[E]` `[S]` 2 |
| End of line | `[E]` M |
| First line | `[E]` N |
| Last line | `[E]` T |
| Screen (top/bottom toggle) | `[E]` `[S]` 8 |
| Delete this character | `[E]` `[S]` K |
| Delete word left | `[E]` E |
| Delete word right | `[E]` W |
| Delete to start of line | `[E]` K |
| Delete to end of line | `[E]` J |

Where `[E]` = Extended Mode and `[S]` = Symbol Shift.

## Hardware Port

The keypad connects to the 128K's edge connector. Two motherboard revisions differ in keypad compatibility:

| Issue | Behaviour |
|---|---|
| **6K** | Provides incorrect voltage to the keypad — some UK keypads are incompatible. |
| **6U** | Corrected by replacing resistor **R137**, providing the proper voltage. |

Keypad signals map to the same keyboard matrix rows as the numeric key functions — the keypad hardware appears to the software as standard keypresses distinguishable from the main keyboard only by position.
