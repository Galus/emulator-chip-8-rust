# CHIP-8 Instruction Set Test Plan

This document outlines a series of tests to be implemented for each CHIP-8 opcode. The tests should cover successful execution, edge cases, and proper state manipulation of the CPU and memory.

## 00E0 - CLS
- [X] Test: CPU state, video memory.
- [X] Steps: Initialize a CPU state and a non-empty video buffer. Execute the `CLS` instruction.
- [X] Expected Result: The entire video buffer is cleared to zeros, and the program counter (PC) is incremented by 2.

## 00EE - RET
- [X] Test: Stack and PC.
- [X] Steps: Initialize a CPU with a value pushed onto the stack. Execute the `RET` instruction.
- [X] Expected Result: The PC is set to the address popped from the stack, and the stack pointer is decremented.

## 0nnn - SYS addr
- [ ] Test: SYS is ignored.
- [ ] Steps: Initialize a CPU with a program that includes a `SYS` instruction.
- [ ] Expected Result: The instruction is ignored, and the PC is incremented by 2.

## 1nnn - JP addr
- [ ] Test: PC jump.
- [ ] Steps: Initialize a CPU. Execute a `JP` instruction with a target address.
- [ ] Expected Result: The PC is set to the target address.

## 2nnn - CALL addr
- [ ] Test: Stack and PC.
- [ ] Steps: Initialize a CPU state. Execute a `CALL` instruction with a target address.
- [ ] Expected Result: The current PC is pushed onto the stack, the stack pointer is incremented, and the PC is set to the target address.

## 3xkk - SE Vx, byte
- [ ] Test: Skip on equal.
- [ ] Steps: Initialize a CPU with `V[x]` equal to `kk`. Execute `SE Vx, kk`.
- [ ] Expected Result: PC is incremented by 4 (skips the next instruction).
- [ ] Test: No skip on not equal.
- [ ] Steps: Initialize a CPU with `V[x]` not equal to `kk`. Execute `SE Vx, kk`.
- [ ] Expected Result: PC is incremented by 2.

## 4xkk - SNE Vx, byte
- [ ] Test: Skip on not equal.
- [ ] Steps: Initialize a CPU with `V[x]` not equal to `kk`. Execute `SNE Vx, kk`.
- [ ] Expected Result: PC is incremented by 4.
- [ ] Test: No skip on equal.
- [ ] Steps: Initialize a CPU with `V[x]` equal to `kk`. Execute `SNE Vx, kk`.
- [ ] Expected Result: PC is incremented by 2.

## 5xy0 - SE Vx, Vy
- [ ] Test: Skip on equal registers.
- [ ] Steps: Initialize a CPU with `V[x]` equal to `V[y]`. Execute `SE Vx, Vy`.
- [ ] Expected Result: PC is incremented by 4.
- [ ] Test: No skip on unequal registers.
- [ ] Steps: Initialize a CPU with `V[x]` not equal to `V[y]`. Execute `SE Vx, Vy`.
- [ ] Expected Result: PC is incremented by 2.

## 6xkk - LD Vx, byte
- [ ] Test: Load immediate byte.
- [ ] Steps: Initialize a CPU. Execute `LD Vx, kk`.
- [ ] Expected Result: `V[x]` is set to `kk`, and PC is incremented by 2.

## 7xkk - ADD Vx, byte
- [ ] Test: Add immediate byte.
- [ ] Steps: Initialize a CPU with a value in `V[x]`. Execute `ADD Vx, kk`.
- [ ] Expected Result: `V[x]` is set to `V[x] + kk`, and PC is incremented by 2. Test for overflow where the result wraps around.

## 8xy0 - LD Vx, Vy
- [ ] Test: Load register.
- [ ] Steps: Initialize a CPU with a value in `V[y]`. Execute `LD Vx, Vy`.
- [ ] Expected Result: `V[x]` is set to the value of `V[y]`, and PC is incremented by 2.

## 8xy1 - OR Vx, Vy
- [ ] Test: Logical OR.
- [ ] Steps: Initialize a CPU with values in `V[x]` and `V[y]`. Execute `OR Vx, Vy`.
- [ ] Expected Result: `V[x]` is set to the bitwise OR of `V[x]` and `V[y]`, and PC is incremented by 2.

## 8xy2 - AND Vx, Vy
- [ ] Test: Logical AND.
- [ ] Steps: Initialize a CPU with values in `V[x]` and `V[y]`. Execute `AND Vx, Vy`.
- [ ] Expected Result: `V[x]` is set to the bitwise AND of `V[x]` and `V[y]`, and PC is incremented by 2.

## 8xy3 - XOR Vx, Vy
- [ ] Test: Logical XOR.
- [ ] Steps: Initialize a CPU with values in `V[x]` and `V[y]`. Execute `XOR Vx, Vy`.
- [ ] Expected Result: `V[x]` is set to the bitwise XOR of `V[x]` and `V[y]`, and PC is incremented by 2.

## 8xy4 - ADD Vx, Vy
- [ ] Test: Addition with carry flag.
- [ ] Steps: Initialize a CPU with `V[x]` and `V[y]` where the sum is less than 256. Execute `ADD Vx, Vy`.
- [ ] Expected Result: `V[x]` is set to `V[x] + V[y]`, `Vf` is set to 0, and PC is incremented by 2.
- [ ] Test: Addition with carry flag (overflow).
- [ ] Steps: Initialize a CPU with `V[x]` and `V[y]` where the sum is greater than 255. Execute `ADD Vx, Vy`.
- [ ] Expected Result: `V[x]` is set to the lowest 8 bits of the sum, `Vf` is set to 1, and PC is incremented by 2.

## 8xy5 - SUB Vx, Vy
- [ ] Test: Subtraction with borrow flag (no borrow).
- [ ] Steps: Initialize a CPU with `V[x] >= V[y]`. Execute `SUB Vx, Vy`.
- [ ] Expected Result: `V[x]` is set to `V[x] - V[y]`, `Vf` is set to 1, and PC is incremented by 2.
- [ ] Test: Subtraction with borrow flag (borrow).
- [ ] Steps: Initialize a CPU with `V[x]` < `V[y]`. Execute `SUB Vx, Vy`.
- [ ] Expected Result: `V[x]` is set to `V[x] - V[y]` (wraps around), `Vf` is set to 0, and PC is incremented by 2.

## 8xy6 - SHR Vx {, Vy}
- [ ] Test: Shift right.
- [ ] Steps: Initialize a CPU with a value in `V[x]`. Execute `SHR Vx`.
- [ ] Expected Result: `Vf` is set to the least significant bit of `V[x]` before the shift, `V[x]` is divided by 2, and PC is incremented by 2.

## 8xy7 - SUBN Vx, Vy
- [ ] Test: Subtraction with borrow flag (`Vy - Vx`).
- [ ] Steps: Initialize a CPU with `V[y]` >= `V[x]`. Execute `SUBN Vx, Vy`.
- [ ] Expected Result: `V[x]` is set to `V[y] - V[x]`, `Vf` is set to 1, and PC is incremented by 2.
- [ ] Test: Subtraction with borrow flag (`Vy - Vx`, borrow).
- [ ] Steps: Initialize a CPU with `V[y]` < `V[x]`. Execute `SUBN Vx, Vy`.
- [ ] Expected Result: `V[x]` is set to `V[y] - V[x]` (wraps around), `Vf` is set to 0, and PC is incremented by 2.

## 8xyE - SHL Vx {, Vy}
- [ ] Test: Shift left.
- [ ] Steps: Initialize a CPU with a value in `V[x]`. Execute `SHL Vx`.
- [ ] Expected Result: `Vf` is set to the most significant bit of `V[x]` before the shift, `V[x]` is multiplied by 2, and PC is incremented by 2.

## 9xy0 - SNE Vx, Vy
- [ ] Test: Skip on not equal registers.
- [ ] Steps: Initialize a CPU with `V[x]` not equal to `V[y]`. Execute `SNE Vx, Vy`.
- [ ] Expected Result: PC is incremented by 4.
- [ ] Test: No skip on equal registers.
- [ ] Steps: Initialize a CPU with `V[x]` equal to `V[y]`. Execute `SNE Vx, Vy`.
- [ ] Expected Result: PC is incremented by 2.

## Annn - LD I, addr
- [ ] Test: Load I register.
- [ ] Steps: Initialize a CPU. Execute `LD I, addr`.
- [ ] Expected Result: The I register is set to `addr`, and PC is incremented by 2.

## Bnnn - JP V0, addr
- [ ] Test: Jump with offset.
- [ ] Steps: Initialize a CPU with a value in `V0`. Execute `JP V0, addr`.
- [ ] Expected Result: PC is set to `addr + V0`, and PC is incremented by 2.

## Cxkk - RND Vx, byte
- [ ] Test: Random number generation.
- [ ] Steps: Initialize a CPU. Execute `RND Vx, kk`. The test should be deterministic (e.g., mock the random number generator).
- [ ] Expected Result: `V[x]` is set to a random number bitwise ANDed with `kk`, and PC is incremented by 2.

## Dxyn - DRW Vx, Vy, nibble
- [ ] Test: Draw sprite without collision.
- [ ] Steps: Initialize a CPU with a clear video buffer. Execute `DRW Vx, Vy, nibble`.
- [ ] Expected Result: The sprite is drawn correctly at the coordinates `(V[x], V[y])`. `Vf` is set to 0. PC is incremented by 2.
- [ ] Test: Draw sprite with collision.
- [ ] Steps: Initialize a CPU with a pre-drawn pixel at a location where the new sprite will overlap. Execute `DRW Vx, Vy, nibble`.
- [ ] Expected Result: The overlapping pixel is XORed correctly. `Vf` is set to 1. PC is incremented by 2.

## Ex9E - SKP Vx
- [ ] Test: Skip on key press.
- [ ] Steps: Initialize a CPU with a key press stored in `V[x]`. Execute `SKP Vx`.
- [ ] Expected Result: PC is incremented by 4.
- [ ] Test: No skip on no key press.
- [ ] Steps: Initialize a CPU with a key press not stored in `V[x]`. Execute `SKP Vx`.
- [ ] Expected Result: PC is incremented by 2.

## ExA1 - SKNP Vx
- [ ] Test: Skip on no key press.
- [ ] Steps: Initialize a CPU with a key press not stored in `V[x]`. Execute `SKNP Vx`.
- [ ] Expected Result: PC is incremented by 4.
- [ ] Test: No skip on key press.
- [ ] Steps: Initialize a CPU with a key press stored in `V[x]`. Execute `SKNP Vx`.
- [ ] Expected Result: PC is incremented by 2.

## Fx07 - LD Vx, DT
- [ ] Test: Load delay timer.
- [ ] Steps: Initialize a CPU with a value in the delay timer. Execute `LD Vx, DT`.
- [ ] Expected Result: `V[x]` is set to the value of the delay timer. PC is incremented by 2.

## Fx0A - LD Vx, K
- [ ] Test: Wait for key press.
- [ ] Steps: Initialize a CPU. Execute `LD Vx, K`.
- [ ] Expected Result: The emulator should halt until a key is pressed. Once a key is pressed, `V[x]` is set to the key's value, and PC is incremented by 2.

## Fx15 - LD DT, Vx
- [ ] Test: Load delay timer.
- [ ] Steps: Initialize a CPU with a value in `V[x]`. Execute `LD DT, Vx`.
- [ ] Expected Result: The delay timer is set to the value of `V[x]`. PC is incremented by 2.

## Fx18 - LD ST, Vx
- [ ] Test: Load sound timer.
- [ ] Steps: Initialize a CPU with a value in `V[x]`. Execute `LD ST, Vx`.
- [ ] Expected Result: The sound timer is set to the value of `V[x]`. PC is incremented by 2.

## Fx1E - ADD I, Vx
- [ ] Test: Add to I register.
- [ ] Steps: Initialize a CPU with values in `I` and `V[x]`. Execute `ADD I, Vx`.
- [ ] Expected Result: `I` is set to `I + V[x]`, and PC is incremented by 2.

## Fx29 - LD F, Vx
- [ ] Test: Load font sprite.
- [ ] Steps: Initialize a CPU with a value `0-F` in `V[x]`. Execute `LD F, Vx`.
- [ ] Expected Result: The I register is set to the memory address of the sprite for the character in `V[x]`. PC is incremented by 2.

## Fx33 - LD B, Vx
- [ ] Test: BCD conversion.
- [ ] Steps: Initialize a CPU with a decimal value in `V[x]`. Execute `LD B, Vx`.
- [ ] Expected Result: The three decimal digits of `V[x]` are stored in memory starting at address `I`. PC is incremented by 2.

## Fx55 - LD [I], Vx
- [ ] Test: Store registers in memory.
- [ ] Steps: Initialize a CPU with values in registers `V0` through `V[x]`. Execute `LD [I], Vx`.
- [ ] Expected Result: The values from `V0` to `V[x]` are stored in memory starting at address `I`. PC is incremented by 2.

## Fx65 - LD Vx, [I]
- [ ] Test: Read registers from memory.
- [ ] Steps: Initialize a CPU with values in memory starting at address `I`. Execute `LD Vx, [I]`.
- [ ] Expected Result: The values from memory starting at address `I` are loaded into registers `V0` through `V[x]`. PC is incremented by 2.


