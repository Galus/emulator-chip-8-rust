use super::{
    cpu::Cpu,
    gpu::{Gpu, SCREEN_HEIGHT, SCREEN_WIDTH},
    mem::Memory,
    timer::Timer,
};

#[derive(Debug, Copy, Clone)]
pub struct OpCode(pub u16);

pub trait Nibbles {
    fn into_tuple(&self) -> (u8, u8, u8, u8);
    // fn into_vec(&self) -> Vec<u8>;
}

impl Nibbles for OpCode {
    fn into_tuple(&self) -> (u8, u8, u8, u8) {
        (
            ((0xF000 & self.0) >> 12) as u8,
            ((0x0F00 & self.0) >> 8) as u8,
            ((0x00F0 & self.0) >> 4) as u8,
            (0x000F & self.0) as u8,
        )
    }

    //fn into_vec(&self) -> Vec<u8> {
    //    let nibbles: Vec<u8> = vec![
    //        ((0xF000 & self.0) >> 12) as u8,
    //        ((0x0F00 & self.0) >> 8) as u8,
    //        ((0x00F0 & self.0) >> 4) as u8,
    //        (0x000F & self.0) as u8,
    //    ];
    //    nibbles
    //}
}

pub enum ExecutionResult {
    Jumped,
    Advanced,
    Skipped,
}

pub trait Chip8ISet {
    /// Returns current opcodes 2nd nibble
    fn get_x(cpu: &Cpu) -> u8;

    /// Returns current opcodes 3rd nibble
    fn get_y(cpu: &Cpu) -> u8;

    /// Clear the screen
    fn _00e0(gpu: &mut Gpu) -> ExecutionResult;

    /// Return from a subroutine
    fn _00ee(_emu: &mut Cpu) -> ExecutionResult;

    /// Execute machine language subroutine at address NNN
    fn _0nnn(cpu: &mut Cpu) -> ExecutionResult;

    /// Jump to address NNN
    fn _1nnn(cpu: &mut Cpu) -> ExecutionResult;

    /// Execute subroutine starting at address NNN
    fn _2nnn(cpu: &mut Cpu) -> ExecutionResult;

    /// Skip the following instruction if the value of register vX is equal to NN
    fn _3xnn(cpu: &mut Cpu) -> ExecutionResult;

    /// Skip the following instruction if the value of register vX is NOT equal to NN
    fn _4xnn(cpu: &mut Cpu) -> ExecutionResult;

    /// Skip the following instruction if the value of register vX is equal to the value of
    /// register vY.
    fn _5xy0(cpu: &mut Cpu) -> ExecutionResult;

    /// Store the number NN in register vX
    fn _6xnn(cpu: &mut Cpu) -> ExecutionResult;

    /// Add the value NN to register vX
    fn _7xnn(cpu: &mut Cpu) -> ExecutionResult;

    /// Store the value of register vY in register vX
    fn _8xy0(cpu: &mut Cpu) -> ExecutionResult;

    /// Set vX to vX OR vY
    fn _8xy1(cpu: &mut Cpu) -> ExecutionResult;

    /// Set vX to vX AND vY
    fn _8xy2(cpu: &mut Cpu) -> ExecutionResult;

    /// Set vX to vX XOR vY
    fn _8xy3(cpu: &mut Cpu) -> ExecutionResult;

    /// Set vX to vX + vY
    /// Add the value of register VY to register VX
    /// Set VF to 01 if a carry occurs
    /// Set VF to 00 if a carry does not occur
    //#[feature(bigint_helper_methods)]
    fn _8xy4(cpu: &mut Cpu) -> ExecutionResult;

    /// Set vX to Vx - Vy
    /// Subtract the value of register VY from register VX
    /// ... Vx = Vx - Vy, set VF = NOT borrow
    /// ... Set VF to 00 if a borrow occurs
    /// ... Set VF to 01 if a borrow does not occur
    fn _8xy5(cpu: &mut Cpu) -> ExecutionResult;

    /// Set vX to vY>>
    /// Store the value of register VY shifted right one bit in register VX¹
    /// Set register VF to the least significant bit prior to the shift
    /// VY is unchanged
    fn _8xy6(cpu: &mut Cpu) -> ExecutionResult;

    /// Set register VX to the value of VY minus VX
    /// ... Vx = Vy - Vx, VF = NOT borrow
    /// ... Set VF to 00 if a borrow occurs
    /// ... Set VF to 01 if a borrow does not occur
    fn _8xy7(cpu: &mut Cpu) -> ExecutionResult;

    /// Set vX to vY<<
    /// Store the value of register vY shifted left one bit in register vX
    /// Set register vF to the most significant bit prior to the shift
    /// vY is unchanged
    fn _8xye(cpu: &mut Cpu) -> ExecutionResult;

    /// Skip the following instruction if the value of register vX is not equal to the value of
    /// register vY.
    fn _9xy0(cpu: &mut Cpu) -> ExecutionResult;

    /// LD I, addr
    /// Store memory address NNN in register I
    fn annn(cpu: &mut Cpu) -> ExecutionResult;

    /// JP V0, addr
    /// Jump to address NNN + v0
    fn bnnn(cpu: &mut Cpu) -> ExecutionResult;

    /// RND vX, byte
    /// Set vX to a random number with a mask of NN
    /// Set Vx = random byte AND kk.
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    fn cxnn(cpu: &mut Cpu) -> ExecutionResult;

    /// DRW vX, vY, nibble
    /// Draw a sprite at position vX, vY with N bytes of sprite data starting at the address
    /// stored in I. Set vF to 01 if any set pixels are changed to unset, and 00 otherwise.
    fn dxyn(cpu: &mut Cpu, mem: &Memory, gpu: &mut Gpu) -> ExecutionResult;

    /// SKP vX
    /// Skip the following instruction if the key corresponding to
    /// the hex value currently stored in register vX is pressed
    fn ex9e(cpu: &mut Cpu) -> ExecutionResult;

    /// SKNP vX
    /// Skip the following instruction if the key corresponding to
    /// the hex value currently stored in register vX is NOT pressed
    fn exa1(cpu: &mut Cpu) -> ExecutionResult;

    /// LD vX, DT
    /// Store the current value of the delay timer in register vX
    fn fx07(cpu: &mut Cpu, timers: &Timer) -> ExecutionResult;

    /// LD vX, K
    /// Wait for a keypress and store the result in register vX
    fn fx0a(cpu: &mut Cpu, _gpu: &Gpu) -> ExecutionResult;

    ///// fx0a but presses the 'x' key
    //pub fn fx0a_test(cpu: &mut Cpu) {
    //    // TODO: lets impl later
    //    let x = OpCode::get_x(cpu);
    //
    //    use ratatui::crossterm::event::KeyCode;
    //
    //    let k = KeyCode::Char('x').into();
    //    let whatisit = handle_key_event(k).unwrap();
    //    cpu.registers[x as usize] = whatisit;
    //    assert_eq!(13, whatisit); // make sure our [1-4,q-r,a-f,z-v] maps to [0 - 16]
    //}

    /// LD DT, vX
    /// Set the delay timer to the value of register vX
    fn fx15(cpu: &mut Cpu, timers: &mut Timer) -> ExecutionResult;

    /// LD ST, vX
    /// Set the sound timer to value of register vX
    fn fx18(cpu: &mut Cpu, timers: &mut Timer) -> ExecutionResult;

    /// ADD I, vX
    /// Add the value stored in register vX to register I
    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in I.
    fn fx1e(cpu: &mut Cpu) -> ExecutionResult;

    /// LD F, vX
    /// Set I to memory address of the sprite data corresponding to hex digit stored in register vX
    fn fx29(cpu: &mut Cpu) -> ExecutionResult;

    /// LD B, vX
    /// Store BCD of value in vX at addresses I, I+1, I+2
    ///
    /// Stores the binary-coded decimal representation of VX, with the hundreds digit
    /// in memory at location in I, the tens digit at location I+1,
    /// and the ones digit at location I+2.[24]
    fn fx33(cpu: &mut Cpu, mem: &mut Memory) -> ExecutionResult;

    /// LD [I], vX
    /// Store register vals v0 to vX inclusive in memory starting at address I.
    /// Sets I = I + X + 1
    /// Basically fx65 but instead of putting memory into registers, puts registers into memory.
    fn fx55(cpu: &mut Cpu, mem: &mut Memory) -> ExecutionResult;

    /// LD vX, [I]
    /// Fill registers v0 to vX inclusive.
    /// Sets I = I + X + 1
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn fx65(cpu: &mut Cpu, mem: &Memory) -> ExecutionResult;
}

impl Chip8ISet for OpCode {
    /// Returns current opcodes 2nd nibble
    fn get_x(cpu: &Cpu) -> u8 {
        //let op = cpu.current_opcode;
        //let (_, x, _, _) = op.into_tuple();
        //x
        cpu.current_opcode.into_tuple().1
    }

    /// Returns current opcodes 3rd nibble
    fn get_y(cpu: &Cpu) -> u8 {
        cpu.current_opcode.into_tuple().2
    }

    /// Clear the screen
    fn _00e0(gpu: &mut Gpu) -> ExecutionResult {
        gpu.screen = [false; 64 * 32];
        ExecutionResult::Advanced
    }

    /// Return from a subroutine
    /// decrements the stack pointer and sets the program counter
    /// to the current address on the top of the stack
    fn _00ee(cpu: &mut Cpu) -> ExecutionResult {
        if cpu.stack_pointer == 0 {
            panic!("Stack underflow!")
        } else {
            cpu.stack_pointer -= 1;

            cpu.program_counter = cpu.stack[cpu.stack_pointer];
        }
        ExecutionResult::Advanced
    }

    /// Execute machine language subroutine at address NNN
    /// Tobias Langhoff says to skip impl of this func
    /// https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
    /// Tobias lied, because the ibm chip8 logo program uses this
    /// 00000050: 0f02 0202 0202 0000 1f3f 71e0 e5e0 e8a0  .........?q.....
    ///                ^^^^
    fn _0nnn(cpu: &mut Cpu) -> ExecutionResult {
        // let (_, n1, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        // let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        // Figure out if this NNN is BCD'd or if its the bits sequentially
        // where 0000 1111     0000 1011     0000 0111 implies -> 1111 1011 0111
        //              15            11             6         ->    E    B    6
        // otherwise BCD wouldnt allow for 1111, as 9 is the highest bcd.
        //
        // 9/7/25 - b/c memory for chip8 goes to 0xFFF, I assume its not BCD.
        // cpu.program_counter = address;
        // DO NOTHING
        ExecutionResult::Advanced
    }

    /// Jump to address NNN
    fn _1nnn(cpu: &mut Cpu) -> ExecutionResult {
        let (_, n1, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        debug!(target: "iset", "1nnn: Jumping to address {:x?} (setting pc)", address);
        cpu.program_counter = address;
        ExecutionResult::Jumped
    }

    /// Execute subroutine starting at address NNN
    fn _2nnn(cpu: &mut Cpu) -> ExecutionResult {
        if cpu.stack_pointer >= cpu.stack.len() {
            panic!("Stack overflow!");
        }

        // push current address onto the stack
        cpu.stack[cpu.stack_pointer] = cpu.program_counter;
        cpu.stack_pointer += 1;

        let (_, n1, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        cpu.program_counter = address;
        ExecutionResult::Jumped
    }

    /// Skip the following instruction if the value of register vX is equal to NN
    fn _3xnn(cpu: &mut Cpu) -> ExecutionResult {
        let (_, x, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let value = (n2 as u8) << 4 | n3 as u8;
        let vx = cpu.registers[x as usize];
        if vx == value {
            ExecutionResult::Skipped
        } else {
            ExecutionResult::Advanced
        }
    }

    /// Skip the following instruction if the value of register vX is NOT equal to NN
    fn _4xnn(cpu: &mut Cpu) -> ExecutionResult {
        let (_, x, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let value = (n2 as u8) << 4 | n3 as u8;
        let vx = cpu.registers[x as usize];
        if vx != value {
            ExecutionResult::Skipped
        } else {
            ExecutionResult::Advanced
        }
    }

    /// Skip the following instruction if the value of register vX is equal to the value of
    /// register vY.
    fn _5xy0(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(cpu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        if vx == vy {
            // Not sure if I should increment program counter by two or increment index_register ?
            // who knows, future galus
            // future galus: we need to handle the execution w/ the program_counter
            // ... the index_register is for interacting with memory and other things
            // ... and +1 will go to next instruction, so we need to +2 instead
            ExecutionResult::Skipped
        } else {
            ExecutionResult::Advanced
        }
    }

    /// Store the number NN in register vX
    fn _6xnn(cpu: &mut Cpu) -> ExecutionResult {
        let (_, x, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let value = (n2 as u8) << 4 | n3 as u8;
        cpu.registers[x as usize] = value;
        ExecutionResult::Advanced
    }

    /// Add the value NN to register vX
    fn _7xnn(cpu: &mut Cpu) -> ExecutionResult {
        let (_, x, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let value = (n2 as u8) << 4 | n3 as u8;
        let temp = cpu.registers[x as usize] + value;
        cpu.registers[x as usize] = temp;
        ExecutionResult::Advanced
    }

    /// Store the value of register vY in register vX
    fn _8xy0(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(cpu);
        let vy = cpu.registers[y as usize];
        cpu.registers[x as usize] = vy;
        ExecutionResult::Advanced
    }

    /// Set vX to vX OR vY
    fn _8xy1(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(cpu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        cpu.registers[x as usize] = vx | vy;
        ExecutionResult::Advanced
    }

    /// Set vX to vX AND vY
    fn _8xy2(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(cpu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        cpu.registers[x as usize] = vx & vy;
        ExecutionResult::Advanced
    }

    /// 11 + 11 =>  3 + 3 = 6 = 110 , 111 + 111 = 7+7 = 14 = 1110 , overflow means lsb of larger
    ///    type

    /// Set vX to vX XOR vY
    fn _8xy3(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(cpu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        cpu.registers[x as usize] = vx ^ vy;
        ExecutionResult::Advanced
    }

    /// Add the value of register VY to register VX
    /// Set VF to 01 if a carry occurs
    /// Set VF to 00 if a carry does not occur
    //#[feature(bigint_helper_methods)]
    fn _8xy4(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(cpu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        let (sum, carry) = {
            let this = vx;
            let rhs = vy;
            let carry = false;
            let (a, b) = this.overflowing_add(rhs);
            let (c, d) = a.overflowing_add(carry as u8);
            (c, b || d)
        };
        cpu.registers[x as usize] = sum;
        cpu.registers[0xF] = carry as u8;
        ExecutionResult::Advanced
    }

    /// Subtract the value of register VY from register VX
    /// ... Vx = Vx - Vy, set VF = NOT borrow
    /// ... Set VF to 00 if a borrow occurs
    /// ... Set VF to 01 if a borrow does not occur
    fn _8xy5(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(cpu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        let (diff, borrow) = {
            let this = vx;
            let rhs = vy;
            let borrow = false;
            let (a, b) = this.overflowing_sub(rhs);
            let (c, d) = a.overflowing_sub(borrow as u8);
            (c, b || d)
        };
        cpu.registers[x as usize] = diff;
        if borrow {
            cpu.registers[0xF as usize] = 0x00;
        } else {
            cpu.registers[0xF as usize] = 0x01;
        }
        ExecutionResult::Advanced
    }

    /// Store the value of register VY shifted right one bit in register VX¹
    /// Set register VF to the least significant bit prior to the shift
    /// VY is unchanged
    fn _8xy6(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(cpu);
        let vy = cpu.registers[y as usize];
        let lsb_vy = vy & 0b00000001;
        cpu.registers[0xF as usize] = lsb_vy;
        let shifted_vy = vy >> 1;
        cpu.registers[x as usize] = shifted_vy;
        ExecutionResult::Advanced
    }

    /// Set register VX to the value of VY minus VX
    /// ... Vx = Vy - Vx, VF = NOT borrow
    /// ... Set VF to 00 if a borrow occurs
    /// ... Set VF to 01 if a borrow does not occur
    fn _8xy7(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(cpu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        let (diff, borrow) = {
            let this = vy;
            let rhs = vx;
            let borrow = false;
            let (a, b) = this.overflowing_sub(rhs);
            let (c, d) = a.overflowing_sub(borrow as u8);
            (c, b || d)
        };
        cpu.registers[x as usize] = diff;
        if borrow {
            cpu.registers[0xF as usize] = 0x00;
        } else {
            cpu.registers[0xF as usize] = 0x01;
        }
        ExecutionResult::Advanced
    }

    /// Store the value of register vY shifted left one bit in register vX
    /// Set register vF to the most significant bit prior to the shift
    /// vY is unchanged
    fn _8xye(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(cpu);
        let vy = cpu.registers[y as usize];
        let msb_vy = (vy & 0b10000000) >> 7;
        cpu.registers[0xF as usize] = msb_vy;
        let shifted_vy = vy << 1;
        cpu.registers[x as usize] = shifted_vy;
        ExecutionResult::Advanced
    }

    /// Skip the following instruction if the value of register vX is not equal to the value of
    /// register vY.
    fn _9xy0(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(cpu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        if vx != vy {
            ExecutionResult::Skipped
        } else {
            ExecutionResult::Advanced
        }
    }

    /// Store memory address NNN in register I
    fn annn(cpu: &mut Cpu) -> ExecutionResult {
        let (_, n1, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        cpu.index_register = address;
        ExecutionResult::Advanced
    }

    /// Jump to address NNN + v0
    fn bnnn(cpu: &mut Cpu) -> ExecutionResult {
        let (_, n1, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        let added_address = cpu.registers[0] as u16 + address;
        cpu.program_counter = added_address;
        ExecutionResult::Jumped
    }

    /// Set vX to a random number with a mask of NN
    /// Set Vx = random byte AND kk.
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    fn cxnn(cpu: &mut Cpu) -> ExecutionResult {
        let (_, x, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let rng = rand::random::<u8>();
        let masked_rng = (n2 << 4 | n3) & rng;
        cpu.registers[x as usize] = masked_rng;
        ExecutionResult::Advanced
    }

    /// Draw a sprite at position vX, vY with n height.
    /// bytes of sprite data start at the address stored in I.
    /// Set vF to 01 if any set pixels are changed to unset, and 00 otherwise.
    fn dxyn(cpu: &mut Cpu, mem: &Memory, gpu: &mut Gpu) -> ExecutionResult {
        let (_, x, y, n) = OpCode::into_tuple(&cpu.current_opcode);
        let start = cpu.index_register as usize;
        let end = start + (n as usize);
        let sprite_data = &mem.ram[start..end];
        // Too many people online say that we should wrap around w/ modulus
        let (vx, vy) = (
            cpu.registers[x as usize] as usize % SCREEN_WIDTH,
            cpu.registers[y as usize] as usize % SCREEN_HEIGHT,
        );
        let mut collision_detected = false;

        for row in 0..n {
            let sprite_byte = sprite_data[row as usize];
            let current_y = vy.wrapping_add(row as usize) % SCREEN_HEIGHT;

            for bit_index in 0..8 {
                let current_x = vx.wrapping_add(bit_index) % SCREEN_WIDTH;
                let screen_index = current_y * SCREEN_WIDTH + current_x;

                if screen_index >= SCREEN_WIDTH * SCREEN_HEIGHT {
                    continue;
                }
                let old_pixel = gpu.screen[screen_index];
                let new_pixel = (sprite_byte >> (7 - bit_index)) & 0x1 == 1;
                gpu.screen[screen_index] = old_pixel ^ new_pixel;

                if old_pixel && !gpu.screen[screen_index] {
                    collision_detected = true;
                }
            }
        }
        if collision_detected {
            cpu.registers[0xF] = 1;
        } else {
            cpu.registers[0xF] = 0;
        }
        ExecutionResult::Advanced
    }

    /// Skip the following instruction if the key corresponding to
    /// the hex value currently stored in register vX is pressed
    fn ex9e(cpu: &mut Cpu) -> ExecutionResult {
        todo!("impl later");
        //let x = OpCode::get_x(cpu);
        //let vx = cpu.registers[x as usize];
        //let pressed_value = cpu.memory.gpu.handle_events().unwrap();
        //if pressed_value == vx {
        //    // skip instruction
        //    cpu.program_counter += 2;
        //} else {
        //    // dont skip
        //    // galus note: I think that program counter being automatically incremented may
        //    // start to cause problems... future galus will find out soonTm.
        //}
        //ExecutionResult::Skipped
    }

    /// Skip the following instruction if the key corresponding to
    /// the hex value currently stored in register vX is NOT pressed
    fn exa1(cpu: &mut Cpu) -> ExecutionResult {
        todo!("impl later");
        //let x = OpCode::get_x(cpu);
        //let vx = cpu.registers[x as usize];
        //let pressed_value = cpu.memory.gpu.handle_events().unwrap();
        //if pressed_value != vx {
        //    // skip instruction
        //    cpu.program_counter += 2;
        //} else {
        //    // dont skip
        //}
        //ExecutionResult::Advanced
    }

    /// Store the current value of the delay timer in register vX
    fn fx07(cpu: &mut Cpu, timers: &Timer) -> ExecutionResult {
        let delay_timer = timers.delay_timer;
        let x = OpCode::get_x(cpu);
        cpu.registers[x as usize] = delay_timer;
        ExecutionResult::Advanced
    }

    /// Wait for a keypress and store the result in register vX
    fn fx0a(cpu: &mut Cpu, _gpu: &Gpu) -> ExecutionResult {
        // use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode};
        let x = OpCode::get_x(cpu);
        //let _ = enable_raw_mode();
        //TODO: figure it out
        //let pressed_value = gpu.handle_events().unwrap();
        let pressed_value = 111;
        //let _ = disable_raw_mode();
        cpu.registers[x as usize] = pressed_value;
        ExecutionResult::Advanced
    }

    ///// fx0a but presses the 'x' key
    //pub fn fx0a_test(cpu: &mut Cpu) {
    //    // TODO: lets impl later
    //    let x = OpCode::get_x(cpu);
    //
    //    use ratatui::crossterm::event::KeyCode;
    //
    //    let k = KeyCode::Char('x').into();
    //    let whatisit = handle_key_event(k).unwrap();
    //    cpu.registers[x as usize] = whatisit;
    //    assert_eq!(13, whatisit); // make sure our [1-4,q-r,a-f,z-v] maps to [0 - 16]
    //}

    /// Set the delay timer to the value of register vX
    fn fx15(cpu: &mut Cpu, timers: &mut Timer) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let vx = cpu.registers[x as usize];
        timers.delay_timer = vx;
        ExecutionResult::Advanced
    }

    /// Set the sound timer to value of register vX
    fn fx18(cpu: &mut Cpu, timers: &mut Timer) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let vx = cpu.registers[x as usize];
        timers.sound_timer = vx;
        ExecutionResult::Advanced
    }

    /// Add the value stored in register vX to register I
    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in I.
    fn fx1e(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let vx = &cpu.registers[x as usize];
        let i = &cpu.index_register;
        let new_i = (*vx) as u16 + i;
        cpu.index_register = new_i;
        ExecutionResult::Advanced
    }

    /// Set I to memory address of the sprite data corresponding to hex digit stored in register vX
    fn fx29(cpu: &mut Cpu) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let vx = &cpu.registers[x as usize];
        cpu.index_register = *vx as u16;
        ExecutionResult::Advanced
    }

    /// Store BCD of value in vX at addresses I, I+1, I+2
    ///
    /// Stores the binary-coded decimal representation of VX, with the hundreds digit
    /// in memory at location in I, the tens digit at location I+1,
    /// and the ones digit at location I+2.[24]
    fn fx33(cpu: &mut Cpu, mem: &mut Memory) -> ExecutionResult {
        let x = OpCode::get_x(cpu);
        let register = cpu.registers[x as usize];
        let padded = format!("{:0>3}", register);
        let a: u8 = padded.chars().nth(0).unwrap() as u8 - 48; // ascii '0' starts at decimal 48
        let b: u8 = padded.chars().nth(1).unwrap() as u8 - 48;
        let c: u8 = padded.chars().nth(2).unwrap() as u8 - 48;
        let index = cpu.index_register as usize;
        mem.ram[index] = a;
        mem.ram[index + 1] = b;
        mem.ram[index + 2] = c;
        ExecutionResult::Advanced
    }

    /// Store register vals v0 to vX inclusive in memory starting at address I.
    /// Sets I = I + X + 1
    /// Basically fx65 but instead of putting memory into registers, puts registers into memory.
    fn fx55(cpu: &mut Cpu, mem: &mut Memory) -> ExecutionResult {
        let num_registers = OpCode::get_x(&cpu);
        for x in 0..=num_registers {
            let load_index = cpu.index_register + (x as u16);
            mem.ram[load_index as usize] = cpu.registers[x as usize];
        }
        cpu.index_register += (num_registers + 1) as u16;
        ExecutionResult::Advanced
    }

    /// Fill registers v0 to vX inclusive.
    /// Sets I = I + X + 1
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn fx65(cpu: &mut Cpu, mem: &Memory) -> ExecutionResult {
        let num_registers = OpCode::get_x(&cpu);
        for x in 0..=num_registers {
            let load_index = cpu.index_register + (x as u16);
            cpu.registers[x as usize] = mem.ram[load_index as usize]
        }
        cpu.index_register += (num_registers + 1) as u16;
        ExecutionResult::Advanced
    }
}
