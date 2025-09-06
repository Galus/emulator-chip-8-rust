#![allow(dead_code)]
#![allow(unused_variables)]
// Contains the CPUs Registers, OpCodes, and their impls.
use super::{
    iset::{Chip8ISet, Nibbles, OpCode},
    timer::Timer,
};
use crate::emu::{gpu::Gpu, mem::Memory};

/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
use color_eyre::Result;
#[derive(Debug)]
#[allow(dead_code)] // REMOVE THIS WHEN DONE
pub struct Cpu {
    pub current_opcode: OpCode,
    pub registers: [u8; 16],  // general purpose reggies V0-VF
    pub program_counter: u16, // pts to the next instruction
    pub stack: [u16; 16],
    pub stack_pointer: usize,
    // IR: Address of the current instruction
    // even tho u16, can only go to 12-bit mem addys b/c chip8 MAX RAM is 4096
    // ex. 1111 1111 1111 -> 0xFFF -> 4095 -> memsize
    pub index_register: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            current_opcode: OpCode(0),
            registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    //pub fn memory(&mut self) -> &mut Memory {
    //    &mut self.mem;
    //}

    /// Retrieves opcode from memory into the cpu
    pub fn fetch_opcode(&mut self, memory: &Memory) {
        let opcode_high: u8 = memory.ram[self.program_counter as usize];
        let opcode_low: u8 = memory.ram[self.program_counter as usize + 1];
        let opcode: u16 = (opcode_high as u16) << 8 | opcode_low as u16;
        self.current_opcode = OpCode(opcode);
    }

    pub fn process(
        &mut self,
        memory: &mut Memory,
        gpu: &mut Gpu,
        timers: &mut Timer,
    ) -> Result<()> {
        // Map the current OpCode to an actual function.
        // DECODE and Process
        match &self.current_opcode.into_tuple() {
            (0, 0, 0xE, 0xE) => OpCode::_00ee(self),
            (0, 0, 0xE, 0) => OpCode::_00e0(gpu),
            (0, _, _, _) => OpCode::_0nnn(self),
            (1, _, _, _) => OpCode::_1nnn(self),
            (2, _, _, _) => OpCode::_2nnn(self),
            (3, _, _, _) => OpCode::_3xnn(self),
            (4, _, _, _) => OpCode::_4xnn(self),
            (5, _, _, 0) => OpCode::_5xy0(self),
            (6, _, _, _) => OpCode::_6xnn(self),
            (7, _, _, _) => OpCode::_7xnn(self),
            (8, _, _, 0) => OpCode::_8xy0(self),
            (8, _, _, 1) => OpCode::_8xy1(self),
            (8, _, _, 2) => OpCode::_8xy2(self),
            (8, _, _, 3) => OpCode::_8xy3(self),
            (8, _, _, 4) => OpCode::_8xy4(self),
            (8, _, _, 5) => OpCode::_8xy5(self),
            (8, _, _, 6) => OpCode::_8xy6(self),
            (8, _, _, 7) => OpCode::_8xy7(self),
            (8, _, _, 0xE) => OpCode::_8xye(self),
            (9, _, _, 0) => OpCode::_9xy0(self),
            (0xA, _, _, _) => OpCode::annn(self),
            (0xB, _, _, _) => OpCode::bnnn(self),
            (0xC, _, _, _) => OpCode::cxnn(self),
            (0xD, _, _, _) => OpCode::dxyn(self, memory, gpu),
            (0xE, _, 9, 0xE) => OpCode::ex9e(self),
            (0xE, _, 0xA, 1) => OpCode::exa1(self),
            (0xF, _, 0, 7) => OpCode::fx07(self, timers),
            (0xF, _, 0, 0xA) => OpCode::fx0a(self, gpu),
            (0xF, _, 1, 5) => OpCode::fx15(self, timers),
            (0xF, _, 1, 8) => OpCode::fx18(self, timers),
            (0xF, _, 1, 0xE) => OpCode::fx1e(self),
            (0xF, _, 2, 9) => OpCode::fx29(self),
            (0xF, _, 3, 3) => OpCode::fx33(self, memory),
            (0xF, _, 5, 5) => OpCode::fx55(self, memory),
            (0xF, _, 6, 5) => OpCode::fx65(self, memory),
            (a, b, c, d) => println!("Not implemented {:x?}", (a, b, c, d)),
        }
        Ok(())
    }

    // main emulation loop tick - fetches & processes a single opcode
    pub fn tick(&mut self, memory: &mut Memory, gpu: &mut Gpu, timers: &mut Timer) -> Result<()> {
        let _ = self.fetch_opcode(memory);
        if let Err(err) = self.process(memory, gpu, timers) {
            eprintln!("failed to process.: {}", err);
        }
        Ok(())
    }
}

#[cfg(test)]
mod cputests {
    use crate::emu::cpu::Cpu;
    use crate::emu::iset::{Chip8ISet, OpCode};
    use crate::emu::Gpu;
    use crate::emu::Memory;
    use crate::emu::Timer;

    fn test_init_mem() -> Memory {
        let mut mem = Memory::default();

        // create some fake memory
        mem.ram[0x200] = 1;
        mem.ram[0x201] = 2;
        mem.ram[0x202] = 3;
        mem.ram[0x203] = 4;
        mem.ram[0x204] = 5;
        mem
    }

    fn test_init_cpu() -> Cpu {
        let mut cpu = Cpu::new();

        // random registers populated
        cpu.registers[0] = 105;
        cpu.registers[1] = 5;
        cpu.registers[2] = 14;
        cpu.registers[7] = 33;
        cpu.registers[12] = 0x11;
        cpu.index_register = 0x200;

        for i in 0..cpu.stack.len() {
            cpu.stack[i] = (i + 1) as u16;
        }

        cpu
    }

    fn test_init_gpu() -> Gpu {
        let gpu = Gpu::new();

        gpu
    }

    #[test]
    fn test_00e0() {
        let mut gpu = test_init_gpu();
        OpCode::_00e0(&mut gpu);
        assert!(gpu.screen.iter().all(|&pixel| !pixel));
    }

    #[test]
    fn test_00ee() {
        let mut cpu = test_init_cpu();
        assert!(
            cpu.stack_pointer == 0,
            "before operation sp: {:?}",
            cpu.stack_pointer
        );
        assert!(
            cpu.program_counter == 0x200,
            "before operation pc: {:?}",
            cpu.program_counter
        );
        assert!(
            cpu.stack[cpu.stack_pointer] == 1,
            "before operation stack[sp]: {:?}",
            cpu.stack[cpu.stack_pointer],
        );
        // artifically set the stack pointer to be at second index
        cpu.stack_pointer = 1;
        OpCode::_00ee(&mut cpu);
        assert!(cpu.stack_pointer == 0, "sp: {:?}", cpu.stack_pointer);
        // Remember init cpu method will create our stack such that:
        // stack: [1, 2, 3, ..., 16]
        assert!(cpu.program_counter == 1, "pc: {:?}", cpu.program_counter);
        assert!(
            cpu.stack[cpu.stack_pointer] == 1,
            "stack[sp]: {:?}",
            cpu.stack[cpu.stack_pointer],
        );
    }

    #[test]
    #[should_panic(expected = "Stack underflow!")]
    fn test_00ee_underflow() {
        let mut cpu = test_init_cpu();
        cpu.stack_pointer = 0;
        OpCode::_00ee(&mut cpu);
    }

    #[test]
    fn test_fx33() {
        let mut cpu = test_init_cpu();
        let mut mem = test_init_mem();
        cpu.current_opcode = OpCode(0xF533);
        cpu.registers[5] = 105;
        // unnecessary but oh well...
        cpu.index_register = 0x200;
        // Test init wierd mishaps
        //// init'd by test_init_cpu
        assert_eq!(mem.ram[cpu.index_register as usize], 1);
        assert_eq!(mem.ram[(cpu.index_register + 1) as usize], 2);
        assert_eq!(mem.ram[(cpu.index_register + 2) as usize], 3);
        println!("index_register: {:?}", cpu.index_register);
        let idxr: usize = cpu.index_register as usize;
        println!("memory.data[ir..ir+3]: {:x?}", &mem.ram[(idxr)..(idxr + 3)]);
        // Test fx33
        OpCode::fx33(&mut cpu, &mut mem);
        println!("memory.data[ir..ir+3]: {:x?}", &mem.ram[(idxr)..(idxr + 3)]);
        assert_eq!(mem.ram[cpu.index_register as usize], 1);
        assert_eq!(mem.ram[(cpu.index_register + 1) as usize], 0);
        assert_eq!(mem.ram[(cpu.index_register + 2) as usize], 5);
        //println!("{:x?}", &emu.memory);
        //assert_eq!(mem.ram[(cpu.index_register + 2) as usize], 8);
    }

    #[test]
    fn test_fx55() {
        let mut cpu = test_init_cpu();
        let mut mem = test_init_mem();
        cpu.current_opcode = OpCode(0xF555);
        // memory should be 1-7 at 0x200-206
        mem.ram[0x205] = 6;
        mem.ram[0x206] = 7;
        assert_eq!(mem.ram[0x200], 1);
        assert_eq!(mem.ram[0x201], 2);
        assert_eq!(mem.ram[0x202], 3);
        assert_eq!(mem.ram[0x203], 4);
        assert_eq!(mem.ram[0x204], 5);
        assert_eq!(mem.ram[0x205], 6);
        assert_eq!(mem.ram[0x206], 7);
        OpCode::fx55(&mut cpu, &mut mem);
        // our x was 5, v0..vx needs to get set with I..I+x
        assert_eq!(mem.ram[0x200], cpu.registers[0]);
        assert_eq!(mem.ram[0x201], cpu.registers[1]);
        assert_eq!(mem.ram[0x202], cpu.registers[2]);
        assert_eq!(mem.ram[0x203], cpu.registers[3]);
        assert_eq!(mem.ram[0x204], cpu.registers[4]);
        assert_eq!(mem.ram[0x205], cpu.registers[5]);
        // this next cpu.memoryory address shouldnt have been affected by 0xF555 b/c x=5
        assert_ne!(mem.ram[0x206], cpu.registers[6]);
        assert_eq!(mem.ram[0x206], 7);
    }

    #[test]
    fn test_fx65() {
        let mut cpu = test_init_cpu();
        let mut mem = test_init_mem();
        cpu.current_opcode = OpCode(0xF565);
        // setting up data to check for out of bounds bugs
        (mem.ram[0x206], cpu.registers[6]) = (0xDE, 0xAD);
        OpCode::fx65(&mut cpu, &mut mem);
        // our x was 5, v0..vx needs to get set with I..I+x
        assert_eq!(mem.ram[0x200], cpu.registers[0]);
        assert_eq!(mem.ram[0x201], cpu.registers[1]);
        assert_eq!(mem.ram[0x202], cpu.registers[2]);
        assert_eq!(mem.ram[0x203], cpu.registers[3]);
        assert_eq!(mem.ram[0x204], cpu.registers[4]);
        assert_eq!(mem.ram[0x205], cpu.registers[5]);
        assert_ne!(mem.ram[0x206], cpu.registers[6]);
    }

    #[test]
    fn test_fx1e() {
        // init
        let mut cpu = test_init_cpu();
        cpu.index_register = 0x200; // unnecessary but oh well...
                                    // save before
        cpu.current_opcode = OpCode(0xF51E);
        let old_i = cpu.index_register.clone();
        println!("old_i: {:?}", old_i);
        // test fx1e to see if vX = 0 works
        // b/c x=5 -> v[5] and b/c all registers are 0'd out now -> 0
        OpCode::fx1e(&mut cpu); // add 5 to i
        assert_eq!(cpu.index_register, old_i);
        cpu.registers[5] = 3;
        OpCode::fx1e(&mut cpu); // add 5 to i
        assert_eq!(cpu.index_register, old_i + 3);
    }

    //#[test]
    //fn test_fx0a_test() {
    //    let mut cpu = test_init_cpu();
    //    let old = cpu.registers[7].clone();
    //    cpu.current_opcode = OpCode(0xF70A);
    //    // presses x, == 13 in our keymap
    //    OpCode::fx0a_test(&mut cpu);
    //    // This opcode fx0a_test should have mutated our '7' register b/c fx0a -> x = 7 => f70a
    //    let new = cpu.registers[7].clone();
    //    assert_eq!(13, new);
    //    assert_ne!(old, new);
    //}

    //#[test]
    //fn test_dxyn() {
    //    let mut cpu = test_init_cpu();
    //    let mut mem = test_init_mem();
    //    let mut gpu = test_init_gpu();
    //    let old_vf = cpu.registers[0xF];
    //    // assert old_vf is not set
    //    assert_eq!(old_vf, 0);
    //    assert_eq!(old_vf, cpu.registers[0xF]);
    //
    //    // assert blank screen
    //    const W: usize = 64;
    //    const H: usize = 32;
    //    assert_eq!(gpu.screen, [false; W * H]);
    //
    //    // Setup some existing screen data
    //    // lets draw '1111 0001' in the middle of second row
    //    //    Calc the offset
    //    let offset = W + (W / 2);
    //    gpu.screen[offset..(offset + 4)].fill(true);
    //    gpu.screen[offset + 7] = true;
    //    println!("Second row filled with '1111 0001' somewhere...");
    //    println!("{:x?}", gpu.screen.map(|bool| bool as u32));
    //    assert_eq!(
    //        gpu.screen[offset..(offset + 8)],
    //        [true, true, true, true, false, false, false, true]
    //    );
    //
    //    // Setup some new pixels to draw!
    //    //   lets test 2 bytes worth of pixels
    //    let pixel_byte1 = [true, false, true, false, true, false, true, false];
    //    let pixel_byte1_u8 = pixel_byte1
    //        .iter()
    //        .enumerate()
    //        .fold(0u8, |acc, (i, &b)| acc | ((b as u8) << (7 - i)));
    //    // We will use pixel_byte2 one to make sure dxyn's register vF doesnt get set
    //    let pixel_byte2 = [false, false, false, false, false, false, false, false];
    //    let pixel_byte2_u8 = pixel_byte2
    //        .iter()
    //        .enumerate()
    //        .fold(0u8, |acc, (i, &b)| acc | ((b as u8) << (7 - i)));
    //
    //    // Put these bytes into the instruction memory somewhere, lets say 1337 :)
    //    // 1337 = 0b10100111001, this requires 11 bits, index_register holds up to 12
    //    cpu.index_register = 1337;
    //    mem.ram[cpu.index_register as usize] = pixel_byte1_u8;
    //    mem.ram[(cpu.index_register as usize) + 1] = pixel_byte2_u8;
    //    // This actuall happens to show up as '0xaa' t,f,t,f,t,f,t,f = 1010 1010 = 0xa 0xa
    //    println!("ram:");
    //    println!("{:x?}", mem.ram.map(|u| u as u8));
    //
    //    // Lets draw into an unset, blank, area and make sure vF is 0
    //    // ...draw at the bottom-right of the screen (64x32) -> 48,30
    //    // ...put these in two random registers, register 8 and 10
    //    cpu.current_opcode = OpCode(0xD432);
    //    const VX: u8 = 48; // max is 64
    //    const VY: u8 = 30; // max is 32
    //    cpu.registers[4] = VX;
    //    cpu.registers[3] = VY;
    //    println!("screen (before writing to bottom-right of screen):");
    //    println!("{:x?}", gpu.screen.map(|bool| bool as u8));
    //    OpCode::dxyn(&mut cpu, &mem, &gpu);
    //    println!("screen (after writing to bottom-right of screen):");
    //    println!("{:x?}", gpu.screen.map(|bool| bool as u8));
    //    assert_eq!(cpu.registers[0xF], 0); // see if the unset flag in vF remained at 0
    //
    //    // calculate offset in screen for this bottom-right test
    //    let offset = W.wrapping_mul(VY as usize) + VX as usize;
    //    assert_eq!(gpu.screen[offset..offset + 8], pixel_byte1);
    //    assert_eq!(gpu.screen[offset + 8..offset + 16], pixel_byte2);
    //
    //    // Lets draw into an already populated set portion of the screen
    //    // ... At position 96 we have our first set pixel.
    //    // ... This is because earlier we populated '1111 0001' in the 'middle of second row'
    //    // ... 'middle of 2nd row => 64 + (64/2) => 64 + 32 = 96'
    //    let offset = W + (W / 2);
    //
    //    // convert to a vX, vY
    //    let v_x = W / 2;
    //    let v_y = 1;
    //
    //    // .. set these to register 8 and 10
    //    cpu.registers[8] = v_x as u8;
    //    cpu.registers[0xA] = v_y as u8;
    //    cpu.current_opcode = OpCode(0xD8A2);
    //
    //    OpCode::dxyn(&mut cpu, &mem, &gpu);
    //    println!("screen (after overwriting the second-rows set pixels):");
    //    println!("{:x?}", gpu.screen.map(|bool| bool as u8));
    //
    //    // Remember, pixels are xor'd, you cant assume the screen will have the exact pixel bytes
    //    // ...                 if existing pixels = 1111 0001
    //    // ...                   and pixel_byte1 = '1010 1010'
    //    // ... the xor'd output on the screen is = '0101 1011'
    //    let expected_screen_after_xor_pixel_byte1 =
    //        [false, true, false, true, true, false, true, true];
    //    assert_eq!(
    //        gpu.screen[offset..offset + 8],
    //        expected_screen_after_xor_pixel_byte1
    //    );
    //
    //    // ... similarly screen: 0000 0000
    //    // ...           pix b2: 0000 0000
    //    // ...              xor: 0000 0000
    //    let expected_screen_after_xor_pixel_byte2 =
    //        [false, false, false, false, false, false, false, false];
    //    assert_eq!(
    //        gpu.screen[offset + 8..offset + 16],
    //        expected_screen_after_xor_pixel_byte2
    //    );
    //
    //    // ... Last but not least, make sure that the vF unset flag got set to 1
    //    assert_eq!(cpu.registers[0xF], 1);
    //}

    //#[test]
    //fn test_fx0a() {
    //    let mut cpu = test_init_cpu();
    //    let old = cpu.registers[7].clone();
    //    cpu.current_opcode = OpCode(0xF70A);
    //
    //    //if poll(Duration::from_millis(100))? {
    //    OpCode::fx0a(&mut cpu);
    //    let new = cpu.registers[7].clone();
    //    // This opcode fx0a_test should have mutated our '7' register b/c fx0a -> x = 7 => f70a
    //    assert_eq!(13, new);
    //    assert_ne!(old, new);
    //    //} else {
    //    //    println!("hubbababa");
    //    //}
    //}

    mod mock {
        use crate::emu::{gpu::Gpu, mem::Memory, timer::Timer};
        use std::mem::size_of_val;
        pub struct MockMemory {
            pub ram: [u8; 4096],
        }
        impl MockMemory {
            pub fn new() -> Self {
                MockMemory { ram: [0; 4096] }
            }
        }
        pub struct MockGpu {
            pub vram: [u8; 64 * 32],
            pub clear_screen_called: bool,
        }
        impl MockGpu {
            pub fn new() -> Self {
                MockGpu {
                    vram: [0; 64 * 32],
                    clear_screen_called: false,
                }
            }
            pub fn clear_screen(&mut self) {
                self.clear_screen_called = true;
            }
        }
        pub struct MockTimer {
            pub value: u8,
        }
        impl MockTimer {
            pub fn new() -> Self {
                MockTimer { value: 0 }
            }
        }
    }

    #[test]
    fn test_cpu_new() {
        let cpu = Cpu::new();
        assert_eq!(cpu.program_counter, 0x200);
        assert_eq!(cpu.registers, [0; 16]);
        assert_eq!(cpu.index_register, 0);
        assert_eq!(cpu.stack_pointer, 0);
    }

    #[test]
    fn test_fetch_opcode() {
        let mut cpu = test_init_cpu();
        let mut memory = Memory::new();
        memory.ram[0x200] = 0x12;
        memory.ram[0x201] = 0x34;
        cpu.fetch_opcode(&memory);
        assert_eq!(cpu.current_opcode.0, 0x1234);
    }

    #[test]
    fn test_process_00e0() {
        let mut cpu = test_init_cpu();
        let mut memory = Memory::new();
        let mut gpu = Gpu::new();
        let mut timers = Timer::new(1);
        cpu.current_opcode = OpCode(0x00E0);
        let result = cpu.process(&mut memory, &mut gpu, &mut timers);
        assert!(result.is_ok());
        assert!(gpu.screen.iter().all(|&pixel| !pixel));
    }
}
