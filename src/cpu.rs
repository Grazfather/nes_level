use mem;
use mem::Addressable;
use rom;

use std::fmt;

#[derive(Debug)]
#[derive(Default)]
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
        let r: Registers = Default::default();
        return r
    }
}

// Bits for Registers::flags
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

// Vectors
const NMI_VECTOR: u16 = 0xFFFA;
const RESET_VECTOR: u16 = 0xFFFC;
const IRQ_VECTOR: u16 = 0xFFFE;

pub struct CPU {
    regs: Registers,
    memory: mem::Memory,
}

impl CPU {
    pub fn new(rom_file: &str) -> CPU {
        let rom = rom::ROM::from_file(rom_file);
        CPU {
            regs: Registers::new(),
            memory: mem::Memory::from_rom(rom),
        }
    }

    // Facade that calls the memory directly
    pub fn loadb(&self, addr: u16) -> u8 {
        self.memory.loadb(addr)
    }

    pub fn loadw(&self, addr: u16) -> u16 {
        self.memory.loadw(addr)
    }

    fn get_flag(&self, flag: u8) -> bool {
        (self.regs.flags & flag == 1)
    }

    fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.regs.flags |= flag;
        } else {
            self.regs.flags &= !flag;
        }
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch opcode
        let opcode = self.loadb(self.regs.pc);
        println!("Got opcode {:x}", opcode);
        // Process opcode
        match opcode {
            ADC_I => { self.adc_i(); },
            _ => {
                panic!("Illegal/unimplemented opcode 0x{:02x}", opcode);
            }
        }
    }

    pub fn reset(&mut self) {
        // Reset registers
        self.regs.pc = self.loadw(RESET_VECTOR);
    }
}

// Instructions implementation
impl CPU {
    fn adc_i(&mut self) {
        let mut result = self.regs.a as u16;
        let i = self.loadb(self.regs.pc);
        result += i as u16;
        if self.get_flag(CARRY_FLAG) { result += 1; }

        self.set_flag(CARRY_FLAG, (result & 0x100) != 0);

        self.regs.a = result as u8;
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
        for byte in self.memory.ram.data.into_iter() {
            if addr % 32 == 0 {
                memdump.push_str(&format!("\n{:04x}: ", addr));
            }
            memdump.push_str(&format!("{:02x} ", byte));
            addr += 1;
        }
        write!(f, "{}\n{:?}", memdump, self.regs)
    }
}
