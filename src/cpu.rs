use mem;
use rom;

use std::fmt;

#[derive(Debug)]
struct Registers {
    A: u8,
    X: u8,
    Y: u8,
    S: u8, // Stack pointer
    P: u8, // Status register
    PC: u16,
}

impl Registers {
    fn new() -> Registers {
        Registers {
             A: 0, X: 0, Y: 0, S: 0, P: 0, PC: 0
        }
    }
}

// Bits for Registers::S
const CARRY_FLAG: u8 = 1 << 0;
const ZERO_FLAG: u8 = 1 << 1;
const INT_FLAG: u8 = 1 << 2;
const DEC_FLAG: u8 = 1 << 3;
const S1_FLAG: u8 = 1 << 4;
const S2_FLAG: u8 = 1 << 5;
const OVERFLOW_FLAG: u8 = 1 << 6;
const NEG_FLAG: u8 = 1 << 7;

// Opcodes
const ADC_I: u8 = 0x69; // Add immediate

pub struct CPU {
    regs: Registers,
    memory: mem::RAM,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            regs: Registers::new(),
            memory: mem::RAM::new(),
        }
    }

    pub fn load_rom(&mut self, filename: &str) {
        // Load ROM and copy its code into the CPU
        let r = rom::ROM::from_file(filename);
        self.memory.data[0..0x100].clone_from_slice(&r.code[..]);
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch opcode
        let opcode = self.memory.loadb(self.regs.PC);
        self.regs.PC += 1; // Obviously this wont work with variable length opcodes.
        println!("Got opcode {:x}", opcode);
        // Process opcode
        match opcode {
            ADC_I => {
                let i = self.memory.loadb(self.regs.PC);
                self.regs.PC += 1;
                self.regs.A += i;
            },
            _ => {
                println!("Illegal opcode {:x}", opcode);
            }
        }

        // Increment PC
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.regs)
    }
}
