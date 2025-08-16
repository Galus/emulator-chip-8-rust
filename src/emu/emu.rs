use crate::{cpu::Cpu, gpu::Gpu}



#[derive(Debug)]
#[allow(dead_code)] // REMOVE THIS WHEN DONE
pub struct Emulator {
    cpu: Cpu,
    gpu: Gpu,

    pub current_opcode: OpCode,
    memory: [u8; 4096],

    registers: [u8; 16], // general purpose
    /// Address of the current instruction
    index_register: u16, // can only load 12-bit mem address due to range of mem accessible
    //                      1111 1111 1111 -> 0xFFF -> 4095 -> memsize
    program_counter: u16,

    pub screen: [bool; 64 * 32],

    pub stack: [u16; 16],
    stack_pointer: usize,

    delay_timer: u8,
    sound_timer: u8,

    pub keypad: [bool; 16],
    pub rom_buffer: Vec<u8>,
    pub running: bool,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            current_opcode: OpCode(0),
            memory: [0; 4096],
            registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            screen: [false; 64 * 32],
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            rom_buffer: Vec::new(),
            running: false,
        }
    }

    pub fn print_memory(&self) {
        for (i, byte) in self.memory.iter().enumerate() {
            if i % 16 == 0 {
                println!("\n{:04X}: ", i);
            }
            print!("{:02X} ", byte);
        }
        println!();
    }
}
