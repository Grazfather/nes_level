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

struct AbsoluteXAddressingMode;
impl AddressingMode for AbsoluteXAddressingMode {
    fn load(cpu: &mut CPU) -> u8 {
        let mut addr = cpu.loadw_move();
        addr += cpu.regs.x as u16;
        cpu.memory.loadb(addr)
    }
    fn store(cpu: &mut CPU, val: u8) {
        let mut addr = cpu.loadw_move();
        addr += cpu.regs.x as u16;
        cpu.memory.storeb(addr, val);
    }
}

struct AbsoluteYAddressingMode;
impl AddressingMode for AbsoluteYAddressingMode {
    fn load(cpu: &mut CPU) -> u8 {
        let mut addr = cpu.loadw_move();
        addr += cpu.regs.y as u16;
        cpu.memory.loadb(addr)
    }
    fn store(cpu: &mut CPU, val: u8) {
        let mut addr = cpu.loadw_move();
        addr += cpu.regs.y as u16;
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
        println!("{:?}", self.regs);
        println!("0x{:x}: Got opcode ${:x}", self.regs.pc - 1, opcode);
        // Process opcode
        match opcode {
            // Arithmetic
            // -- Adds
            0x69 => { self.adc::<ImmediateAddressingMode>(); },
            0x6d => { self.adc::<AbsoluteAddressingMode>(); },
            0x7d => { self.adc::<AbsoluteXAddressingMode>(); },
            0x79 => { self.adc::<AbsoluteYAddressingMode>(); },
            // -- Subs
            // Comparisons
            0xcd => { self.cmp::<AbsoluteAddressingMode>(); },
            0xdd => { self.cmp::<AbsoluteXAddressingMode>(); },
            0xd9 => { self.cmp::<AbsoluteYAddressingMode>(); },
            0xc9 => { self.cmp::<ImmediateAddressingMode>(); },
            0xec => { self.cpx::<AbsoluteAddressingMode>(); },
            0xe0 => { self.cpx::<ImmediateAddressingMode>(); },
            0xcc => { self.cpy::<AbsoluteAddressingMode>(); },
            0xc0 => { self.cpy::<ImmediateAddressingMode>(); },
            // Loads
            0xad => { self.lda::<AbsoluteAddressingMode>(); },
            0xbd => { self.lda::<AbsoluteXAddressingMode>(); },
            0xb9 => { self.lda::<AbsoluteYAddressingMode>(); },
            0xa9 => { self.lda::<ImmediateAddressingMode>(); },
            0xae => { self.ldx::<AbsoluteAddressingMode>(); },
            0xbe => { self.ldx::<AbsoluteYAddressingMode>(); },
            0xa2 => { self.ldx::<ImmediateAddressingMode>(); },
            0xac => { self.ldy::<AbsoluteAddressingMode>(); },
            0xbc => { self.ldy::<AbsoluteXAddressingMode>(); },
            0xa0 => { self.ldy::<ImmediateAddressingMode>(); },
            // Stores
            0x8d => { self.sta::<AbsoluteAddressingMode>(); },
            // Nop
            0xea => { self.nop(); },
            // Boolean
            // -- And
            0x29 => { self.and::<ImmediateAddressingMode>(); },
            0x2d => { self.and::<AbsoluteAddressingMode>(); },
            0x3d => { self.and::<AbsoluteXAddressingMode>(); },
            0x39 => { self.and::<AbsoluteYAddressingMode>(); },
            // -- Or
            0x09 => { self.ora::<ImmediateAddressingMode>(); },
            0x0d => { self.ora::<AbsoluteAddressingMode>(); },
            0x1d => { self.ora::<AbsoluteXAddressingMode>(); },
            0x19 => { self.ora::<AbsoluteYAddressingMode>(); },
            // -- Eor
            // -- Bit set
            // Branches
            0x10 => { self.bpl(); },
            0x30 => { self.bmi(); },
            0x50 => { self.bvc(); },
            0x70 => { self.bvs(); },
            0x90 => { self.bcc(); },
            0xb0 => { self.bcs(); },
            0xd0 => { self.bne(); },
            0xf0 => { self.beq(); },
            // Jumps
            0x4c => { self.jmp(); },
            // Increment and decrement
            0xca => { self.dex(); },
            0x88 => { self.dey(); },
            0xe8 => { self.inx(); },
            0xc8 => { self.iny(); },
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
        println!("OR-ing A 0x{:x} and 0x{:x}", self.regs.a, val);
        self.regs.a |= val;
    }

    fn and<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        println!("AND-ing A 0x{:x} and 0x{:x}", self.regs.a, val);
        self.regs.a &= val;
    }

    fn adc<AM: AddressingMode>(&mut self) {
        let mut result = self.regs.a as u16;
        let val = AM::load(self);
        println!("Adding {} to {}", result, val);
        result += val as u16;
        if self.get_flag(CARRY_FLAG) { result += 1; }

        self.set_flag(CARRY_FLAG, (result & 0x100) != 0);

        self.regs.a = result as u8;
    }

    fn lda<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        println!("Loading {} into A", val);
        self.regs.a = val;
    }

    fn ldx<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        println!("Loading {} into X", val);
        self.regs.x = val;
    }

    fn ldy<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        println!("Loading {} into Y", val);
        self.regs.y = val;
    }

    fn nop(&mut self) {}

    fn sta<AM: AddressingMode>(&mut self) {
        let val = self.regs.a;
        println!("Storing 0x{:x} from A", val);
        AM::store(self, val);
    }

    fn compare(&mut self, first: u8, second: u8) {
        let result = (first as u16).wrapping_sub(second as u16);
        println!("Comparing 0x{:x} and 0x{:x}", first, second);
        self.set_flag(CARRY_FLAG, (result & 0x100) != 0);
        self.set_flag(ZERO_FLAG, result == 0);
    }

    fn cmp<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        let a = self.regs.a;
        self.compare(a, val);
    }

    fn cpx<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        let x = self.regs.x;
        self.compare(x, val);
    }

    fn cpy<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        let y = self.regs.y;
        self.compare(y, val);
    }

    fn bpl(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BPL might branch to +0x{:x}", offset);
        if (self.regs.flags & NEG_FLAG) == 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn bmi(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BMI might branch to +0x{:x}", offset);
        if (self.regs.flags & NEG_FLAG) != 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn bvc(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BVC might branch to +0x{:x}", offset);
        if (self.regs.flags & OVERFLOW_FLAG) == 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn bvs(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BVS might branch to +0x{:x}", offset);
        if (self.regs.flags & OVERFLOW_FLAG) != 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn bcc(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BCC might branch to +0x{:x}", offset);
        if (self.regs.flags & CARRY_FLAG) == 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn bcs(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BCS might branch to +0x{:x}", offset);
        if (self.regs.flags & CARRY_FLAG) != 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn bne(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BNE might branch to +0x{:x}", offset);
        if (self.regs.flags & ZERO_FLAG) == 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn beq(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BEQ might branch to +0x{:x}", offset);
        if (self.regs.flags & ZERO_FLAG) != 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn dex(&mut self) {
        println!("Decrementing X");
        self.regs.x.wrapping_sub(1);
    }

    fn dey(&mut self) {
        println!("Decrementing Y");
        self.regs.y.wrapping_sub(1);
    }

    fn inx(&mut self) {
        println!("Incrementing X");
        self.regs.x.wrapping_add(1);
    }

    fn iny(&mut self) {
        println!("Incrementing Y");
        self.regs.y.wrapping_add(1);
    }

    fn jmp(&mut self) {
        let addr = self.loadw_move();
        println!("Jumping to 0x{:x}", addr);
        self.regs.pc = addr;
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
