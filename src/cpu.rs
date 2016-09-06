use mem;
use rom;

use std;
use std::fmt;

#[derive(Debug)]
struct Registers {
    a: u8,
    x: u8,
    y: u8,
    s: u8, // Stack pointer
    flags: u8, // Status register
    pc: u16,
}

impl Registers {
    fn new() -> Registers {
        let r: Registers = unsafe{ std::mem::zeroed() };
        return r
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
        let opcode = self.memory.loadb(self.regs.pc);
        self.regs.pc += 1; // Obviously this wont work with variable length opcodes.
        println!("Got opcode {:x}", opcode);
        // Process opcode
        match opcode {
            ADC_I => {
                let i = self.memory.loadb(self.regs.pc);
                self.regs.pc += 1;
                self.regs.a += i;
            },
            _ => {
                panic!("Illegal/unimplemented opcode 0x{:02x}", opcode);
            }
        }
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.regs)
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut memdump = String::new();
        let mut addr = 0;
        for byte in self.memory.data.into_iter() {
            if addr % 32 == 0 {
                memdump.push_str(&format!("\n{:04x}: ", addr));
            }
            memdump.push_str(&format!("{:02x} ", byte));
            addr += 1;
        }
        write!(f, "{}\n{:?}", memdump, self.regs)
    }
}
