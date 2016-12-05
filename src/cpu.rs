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
        write!(f, "Registers a: {:#02x}, x: {:#02x}, y: {:#02x}, s: {:#02x}, flags: {:#02x}, pc: {:#04x}",
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

struct AccumulatorAddressingMode;
impl AddressingMode for AccumulatorAddressingMode {
    fn load(cpu: &mut CPU) -> u8 { cpu.regs.a }
    fn store(cpu: &mut CPU, val: u8) { cpu.regs.a = val; }
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

struct AbsoluteWBAddressingMode;
impl AddressingMode for AbsoluteWBAddressingMode {
    fn load(cpu: &mut CPU) -> u8 {
        let addr = cpu.memory.loadb(cpu.regs.pc) as u16;
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

struct AbsoluteXWBAddressingMode;
impl AddressingMode for AbsoluteXWBAddressingMode {
    fn load(cpu: &mut CPU) -> u8 {
        let mut addr = cpu.memory.loadb(cpu.regs.pc) as u16;
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

struct ZeroPageAddressingMode;
impl AddressingMode for ZeroPageAddressingMode {
    fn load(cpu: &mut CPU) -> u8 {
        let addr = cpu.loadb_move() as u16;
        cpu.memory.loadb(addr)
    }
    fn store(cpu: &mut CPU, val: u8) {
        let addr = cpu.loadb_move() as u16;
        cpu.memory.storeb(addr, val);
    }
}

struct ZeroPageWBAddressingMode;
impl AddressingMode for ZeroPageWBAddressingMode {
    fn load(cpu: &mut CPU) -> u8 {
        let addr = cpu.memory.loadb(cpu.regs.pc) as u16;
        cpu.memory.loadb(addr)
    }
    fn store(cpu: &mut CPU, val: u8) {
        let addr = cpu.loadb_move() as u16;
        cpu.memory.storeb(addr, val);
    }
}

struct ZeroPageXAddressingMode;
impl AddressingMode for ZeroPageXAddressingMode {
    fn load(cpu: &mut CPU) -> u8 {
        let mut addr = cpu.loadb_move() as u16;
        addr += cpu.regs.x as u16;
        cpu.memory.loadb(addr)
    }
    fn store(cpu: &mut CPU, val: u8) {
        let mut addr = cpu.loadb_move() as u16;
        addr += cpu.regs.x as u16;
        cpu.memory.storeb(addr, val);
    }
}

struct ZeroPageXWBAddressingMode;
impl AddressingMode for ZeroPageXWBAddressingMode {
    fn load(cpu: &mut CPU) -> u8 {
        let mut addr = cpu.memory.loadb(cpu.regs.pc) as u16;
        addr += cpu.regs.x as u16;
        cpu.memory.loadb(addr)
    }
    fn store(cpu: &mut CPU, val: u8) {
        let mut addr = cpu.loadb_move() as u16;
        addr += cpu.regs.x as u16;
        cpu.memory.storeb(addr, val);
    }
}

struct ZeroPageYAddressingMode;
impl AddressingMode for ZeroPageYAddressingMode {
    fn load(cpu: &mut CPU) -> u8 {
        let mut addr = cpu.loadb_move() as u16;
        addr += cpu.regs.y as u16;
        cpu.memory.loadb(addr)
    }
    fn store(cpu: &mut CPU, val: u8) {
        let mut addr = cpu.loadb_move() as u16;
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
        println!("{:#x}: Got opcode ${:x}", self.regs.pc - 1, opcode);
        // Process opcode
        match opcode {
            // Arithmetic
            // -- Adds
            0x69 => { self.adc::<ImmediateAddressingMode>(); },
            0x65 => { self.adc::<ZeroPageAddressingMode>(); },
            0x75 => { self.adc::<ZeroPageXAddressingMode>(); },
            0x6d => { self.adc::<AbsoluteAddressingMode>(); },
            0x7d => { self.adc::<AbsoluteXAddressingMode>(); },
            0x79 => { self.adc::<AbsoluteYAddressingMode>(); },
            // -- Subs
            0xe9 => { self.sbc::<ImmediateAddressingMode>(); },
            0xe5 => { self.sbc::<ZeroPageAddressingMode>(); },
            0xf5 => { self.sbc::<ZeroPageXAddressingMode>(); },
            0xed => { self.sbc::<AbsoluteAddressingMode>(); },
            0xfd => { self.sbc::<AbsoluteXAddressingMode>(); },
            0xf9 => { self.sbc::<AbsoluteYAddressingMode>(); },
            // Comparisons
            // -- Cmp A
            0xc9 => { self.cmp::<ImmediateAddressingMode>(); },
            0xc5 => { self.cmp::<ZeroPageAddressingMode>(); },
            0xd5 => { self.cmp::<ZeroPageXAddressingMode>(); },
            0xcd => { self.cmp::<AbsoluteAddressingMode>(); },
            0xdd => { self.cmp::<AbsoluteXAddressingMode>(); },
            0xd9 => { self.cmp::<AbsoluteYAddressingMode>(); },
            // -- Cmp X
            0xe0 => { self.cpx::<ImmediateAddressingMode>(); },
            0xe4 => { self.cpx::<ZeroPageAddressingMode>(); },
            0xec => { self.cpx::<AbsoluteAddressingMode>(); },
            // -- Cmp Y
            0xc0 => { self.cpy::<ImmediateAddressingMode>(); },
            0xc4 => { self.cpy::<ZeroPageAddressingMode>(); },
            0xcc => { self.cpy::<AbsoluteAddressingMode>(); },
            // Loads
            // -- Load A
            0xa9 => { self.lda::<ImmediateAddressingMode>(); },
            0xa5 => { self.lda::<ZeroPageAddressingMode>(); },
            0xb5 => { self.lda::<ZeroPageXAddressingMode>(); },
            0xad => { self.lda::<AbsoluteAddressingMode>(); },
            0xbd => { self.lda::<AbsoluteXAddressingMode>(); },
            0xb9 => { self.lda::<AbsoluteYAddressingMode>(); },
            // -- Load X
            0xa2 => { self.ldx::<ImmediateAddressingMode>(); },
            0xa6 => { self.ldx::<ZeroPageAddressingMode>(); },
            0xb6 => { self.ldx::<ZeroPageYAddressingMode>(); },
            0xae => { self.ldx::<AbsoluteAddressingMode>(); },
            0xbe => { self.ldx::<AbsoluteYAddressingMode>(); },
            // -- Load Y
            0xa0 => { self.ldy::<ImmediateAddressingMode>(); },
            0xa4 => { self.lda::<ZeroPageAddressingMode>(); },
            0xb4 => { self.lda::<ZeroPageXAddressingMode>(); },
            0xac => { self.ldy::<AbsoluteAddressingMode>(); },
            0xbc => { self.ldy::<AbsoluteXAddressingMode>(); },
            // Stores
            // -- Store A
            0x85 => { self.sta::<ZeroPageAddressingMode>(); },
            0x95 => { self.sta::<ZeroPageXAddressingMode>(); },
            0x8d => { self.sta::<AbsoluteAddressingMode>(); },
            0x9d => { self.sta::<AbsoluteXAddressingMode>(); },
            0x99 => { self.sta::<AbsoluteYAddressingMode>(); },
            // -- Store X
            0x86 => { self.stx::<ZeroPageAddressingMode>(); },
            0x96 => { self.stx::<ZeroPageXAddressingMode>(); },
            0x8e => { self.stx::<AbsoluteAddressingMode>(); },
            // -- Store Y
            0x84 => { self.sty::<ZeroPageAddressingMode>(); },
            0x94 => { self.sty::<ZeroPageXAddressingMode>(); },
            0x8c => { self.sty::<AbsoluteAddressingMode>(); },
            // Nop
            0xea => { self.nop(); },
            // Boolean
            // -- And
            0x29 => { self.and::<ImmediateAddressingMode>(); },
            0x25 => { self.and::<ZeroPageAddressingMode>(); },
            0x35 => { self.and::<ZeroPageXAddressingMode>(); },
            0x2d => { self.and::<AbsoluteAddressingMode>(); },
            0x3d => { self.and::<AbsoluteXAddressingMode>(); },
            0x39 => { self.and::<AbsoluteYAddressingMode>(); },
            // -- Or
            0x09 => { self.ora::<ImmediateAddressingMode>(); },
            0x05 => { self.ora::<ZeroPageAddressingMode>(); },
            0x15 => { self.ora::<ZeroPageXAddressingMode>(); },
            0x0d => { self.ora::<AbsoluteAddressingMode>(); },
            0x1d => { self.ora::<AbsoluteXAddressingMode>(); },
            0x19 => { self.ora::<AbsoluteYAddressingMode>(); },
            // -- Eor
            0x49 => { self.eor::<ImmediateAddressingMode>(); },
            0x45 => { self.eor::<ZeroPageAddressingMode>(); },
            0x55 => { self.eor::<ZeroPageXAddressingMode>(); },
            0x4d => { self.eor::<AbsoluteAddressingMode>(); },
            0x5d => { self.eor::<AbsoluteXAddressingMode>(); },
            0x59 => { self.eor::<AbsoluteYAddressingMode>(); },
            // -- Bit set
            // Shifts
            // -- Asl
            0x0a => { self.asl::<AccumulatorAddressingMode>(); },
            0x06 => { self.asl::<ZeroPageWBAddressingMode>(); },
            0x16 => { self.asl::<ZeroPageXWBAddressingMode>(); },
            0x0e => { self.asl::<AbsoluteWBAddressingMode>(); },
            0x1e => { self.asl::<AbsoluteXWBAddressingMode>(); },
            // -- Rol
            0x2a => { self.rol::<AccumulatorAddressingMode>(); },
            0x26 => { self.rol::<ZeroPageWBAddressingMode>(); },
            0x36 => { self.rol::<ZeroPageXWBAddressingMode>(); },
            0x2e => { self.rol::<AbsoluteWBAddressingMode>(); },
            0x3e => { self.rol::<AbsoluteXWBAddressingMode>(); },
            // -- Lsr
            0x4a => { self.lsr::<AccumulatorAddressingMode>(); },
            0x46 => { self.lsr::<ZeroPageWBAddressingMode>(); },
            0x56 => { self.lsr::<ZeroPageXWBAddressingMode>(); },
            0x4e => { self.lsr::<AbsoluteWBAddressingMode>(); },
            0x5e => { self.lsr::<AbsoluteXWBAddressingMode>(); },
            // -- Ror
            0x6a => { self.ror::<AccumulatorAddressingMode>(); },
            0x66 => { self.ror::<ZeroPageWBAddressingMode>(); },
            0x76 => { self.ror::<ZeroPageXWBAddressingMode>(); },
            0x6e => { self.ror::<AbsoluteWBAddressingMode>(); },
            0x7e => { self.ror::<AbsoluteXWBAddressingMode>(); },
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
                panic!("Illegal/unimplemented opcode {:#02x}", opcode);
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
        println!("OR-ing A {:#x} and {:#x}", self.regs.a, val);
        self.regs.a |= val;
    }

    fn eor<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        println!("EOR-ing A {:#x} and {:#x}", self.regs.a, val);
        self.regs.a ^= val;
    }

    fn and<AM: AddressingMode>(&mut self) {
        let val = AM::load(self);
        println!("AND-ing A {:#x} and {:#x}", self.regs.a, val);
        self.regs.a &= val;
    }

    fn asl<AM: AddressingMode>(&mut self) {
        let mut val = AM::load(self);
        let top_bit = (val & 0x80) != 0;
        val <<= 1;
        self.set_flag(CARRY_FLAG, top_bit);
        AM::store(self, val);
    }

    fn rol<AM: AddressingMode>(&mut self) {
        let mut val = AM::load(self);
        let top_bit = (val & 0x80) != 0;
        val <<= 1;
        val |= self.get_flag(CARRY_FLAG) as u8;
        self.set_flag(CARRY_FLAG, top_bit);
        AM::store(self, val);
    }

    fn lsr<AM: AddressingMode>(&mut self) {
        let mut val = AM::load(self);
        let low_bit = (val & 0x1) != 0;
        val >>= 1;
        self.set_flag(CARRY_FLAG, low_bit);
        AM::store(self, val);
    }

    fn ror<AM: AddressingMode>(&mut self) {
        let mut val = AM::load(self);
        let low_bit = (val & 0x1) != 0;
        val >>= 1;
        val |= (self.get_flag(CARRY_FLAG) as u8) << 7;
        self.set_flag(CARRY_FLAG, low_bit);
        AM::store(self, val);
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

    fn sbc<AM: AddressingMode>(&mut self) {
        let mut result = self.regs.a as u16;
        let val = AM::load(self);
        println!("Subtracting {} from {}", result, val);
        result -= val as u16;
        if self.get_flag(CARRY_FLAG) { result -= 1; }

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
        println!("Storing {:#x} from A", val);
        AM::store(self, val);
    }

    fn stx<AM: AddressingMode>(&mut self) {
        let val = self.regs.x;
        println!("Storing {:#x} from X", val);
        AM::store(self, val);
    }

    fn sty<AM: AddressingMode>(&mut self) {
        let val = self.regs.y;
        println!("Storing {:#x} from Y", val);
        AM::store(self, val);
    }

    fn compare(&mut self, first: u8, second: u8) {
        let result = (first as u16).wrapping_sub(second as u16);
        println!("Comparing {:#x} and {:#x}", first, second);
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
        println!("BPL might branch to +{:#x}", offset);
        if (self.regs.flags & NEG_FLAG) == 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn bmi(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BMI might branch to +{:#x}", offset);
        if (self.regs.flags & NEG_FLAG) != 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn bvc(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BVC might branch to +{:#x}", offset);
        if (self.regs.flags & OVERFLOW_FLAG) == 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn bvs(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BVS might branch to +{:#x}", offset);
        if (self.regs.flags & OVERFLOW_FLAG) != 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn bcc(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BCC might branch to +{:#x}", offset);
        if (self.regs.flags & CARRY_FLAG) == 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn bcs(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BCS might branch to +{:#x}", offset);
        if (self.regs.flags & CARRY_FLAG) != 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn bne(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BNE might branch to +{:#x}", offset);
        if (self.regs.flags & ZERO_FLAG) == 0 {
            println!("Taking the branch!");
            self.regs.pc += offset as u16;
        }
    }

    fn beq(&mut self) {
        let offset = ImmediateAddressingMode::load(self);
        println!("BEQ might branch to +{:#x}", offset);
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
        println!("Jumping to {:#x}", addr);
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
