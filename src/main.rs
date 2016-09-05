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

struct CPU {
    regs: Registers,
    memory: [u8; 0x800], // 2KB RAM
}

impl CPU {
    fn Initialize(&mut self) {
    }

    fn LoadROM(&mut self, filename: &str) {
    }
}

fn main() {
    let mut cpu: CPU = CPU {
        regs: Registers::new(),
        memory: [0; 0x800]
    };
    cpu.Initialize();
    cpu.LoadROM("filename");

    loop {

    }
}
