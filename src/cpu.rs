use mem;
use mem::Addressable;
use rom;

use std::fmt;

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
        let r: Registers = Registers::default();
        return r
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Registers a: 0x{:02x}, x: 0x{:02x}, y: 0x{:02x}, s: 0x{:02x}, flags: 0x{:02x}, pc: 0x{:04x}",
            self.a, self.x, self.y, self.s, self.flags, self.pc)
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
const ORA_I: u8 = 0x09; // Or A immediate
const ORA_A: u8 = 0x0d; // Or A absolute
const AND_I: u8 = 0x29; // And A immediate
const AND_A: u8 = 0x2d; // And A absolute
const ADC_I: u8 = 0x69; // Add immediate
const LDA_I: u8 = 0xa9; // Load A immediate
const LDA_AX: u8 = 0xbd; // Load A absolute,X
const LDX_I: u8 = 0xa2; // Load X immediate
const LDX_A: u8 = 0xae; // Load X absolute
const STA_A: u8 = 0x8d; // Store A absolute
const CMP_I: u8 = 0xc9; // Compare immediate

// Vectors
const NMI_VECTOR: u16 = 0xFFFA;
const RESET_VECTOR: u16 = 0xFFFC;
const IRQ_VECTOR: u16 = 0xFFFE;

pub struct CPU {
    regs: Registers,
    memory: mem::Memory,
}

trait AddressingMode {
    fn load(cpu: &mut CPU) -> u8;
    fn store(cpu: &mut CPU, val: u8);
}

struct ImmediateAddressingMode;
impl AddressingMode for ImmediateAddressingMode {
    fn load(cpu: &mut CPU) -> u8 {
        cpu.loadb_move()
    }
    fn store(_cpu: &mut CPU, _val: u8) { panic!("Can't store to an immediate"); }
}

struct AbsoluteAddressingMode;
impl AddressingMode for AbsoluteAddressingMode {
    fn load(cpu: &mut CPU) -> u8 {
        let addr = cpu.loadw_move();
        cpu.memory.loadb(addr)
    }
    fn store(cpu: &mut CPU, val: u8) {
        let addr = cpu.loadw_move();
        cpu.memory.storeb(addr, val);
    }
}

impl CPU {
    pub fn new(rom_file: &str) -> CPU {
        let rom = rom::ROM::from_file(rom_file);
        CPU {
            regs: Registers::new(),
            memory: mem::Memory::from_rom(rom),
        }
    }

    // Read a byte at the PC and increment it
    fn loadb_move(&mut self) -> u8 {
        let val = self.memory.loadb(self.regs.pc);
        self.regs.pc += 1;
        return val;
    }

    // Read a word at the PC and increment it by 2
    fn loadw_move(&mut self) -> u16 {
        let val = self.memory.loadw(self.regs.pc);
        self.regs.pc += 2;
        return val;
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
        let opcode = self.loadb_move();
        println!("Got opcode {:x}", opcode);
        // Process opcode
        match opcode {
            ORA_I => { self.ora::<ImmediateAddressingMode>(); },
            ORA_A => { self.ora::<AbsoluteAddressingMode>(); },
            AND_I => { self.and::<ImmediateAddressingMode>(); },
            AND_A => { self.and::<AbsoluteAddressingMode>(); },
            ADC_I => { self.adc::<ImmediateAddressingMode>(); },
            LDA_I => { self.lda::<ImmediateAddressingMode>(); },
            LDA_AX => { self.lda_ax(); },
            LDX_I => { self.ldx::<ImmediateAddressingMode>(); },
            LDX_A => { self.ldx::<AbsoluteAddressingMode>(); },
            STA_A => { self.sta::<AbsoluteAddressingMode>(); },
            CMP_I => { self.cmp::<ImmediateAddressingMode>(); },
            _ => {
                panic!("Illegal/unimplemented opcode 0x{:02x}", opcode);
            }
        }
    }

    pub fn reset(&mut self) {
        // Reset registers
        self.regs.pc = self.memory.loadw(RESET_VECTOR);
    }
}

// Instructions implementation
impl CPU {
    fn ora<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        self.regs.a |= val;
    }

    fn and<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        self.regs.a &= val;
    }

    fn adc<AM: AddressingMode>(&mut self) {
        let mut result = self.regs.a as u16;
        let val = AM::load(self);
        result += val as u16;
        if self.get_flag(CARRY_FLAG) { result += 1; }

        self.set_flag(CARRY_FLAG, (result & 0x100) != 0);

        self.regs.a = result as u8;
    }

    fn lda<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        self.regs.a = val;
    }

    fn lda_ax(&mut self) { // 0xbd
        let addr = self.loadw_move();
        let val = self.memory.loadb(addr + self.regs.x as u16);
        self.regs.a = val;
    }

    fn ldx<AM: AddressingMode>(&mut self) { // 0xa2
        let val = AM::load(self);
        self.regs.x = val;
    }

    fn sta<AM: AddressingMode>(&mut self) {
        let val = self.regs.a;
        AM::store(self, val);
    }

    fn cmp<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        println!("Comparing {} and {}", self.regs.a, val);
        let result = (self.regs.a as u16).wrapping_sub(val as u16);
        self.set_flag(CARRY_FLAG, (result & 0x100) != 0);
        self.set_flag(ZERO_FLAG, result == 0);
    }
}

// Formatting
impl CPU {
    pub fn print_memory(&self, start: u16, end: u16) {
        if end == 0 {
            print!("{}", hexdump(&self.memory.ram.data[..], 0x0));
        } else {
            print!("{}", hexdump(&self.memory.ram.data[start as usize..end as usize], 0x0));
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
        // This is stupid. Why is the width the ROM length? Write a proper dumping method
        let len = match f.width() {
            Some(x) => { x },
            None => { self.memory.rom.prg.len() },
        };

        memdump.push_str(&hexdump(&self.memory.rom.prg[0..len], 0x8000));
        try!(write!(f, "{}", memdump));
        try!(write!(f, "{:?}", self.regs));
        Result::Ok(())
    }
}

fn hexdump(hex: &[u8], start: u32) -> String {
    let mut addr = start;
    let mut dump = String::new();
    for byte in hex.iter() {
        if addr % 32 == 0 {
            dump.push_str(&format!("\n{:04x}: ", addr));
        }
        dump.push_str(&format!("{:02x} ", byte));
        addr += 1;
    }
    dump.push_str(&"\n");

    return dump;
}
