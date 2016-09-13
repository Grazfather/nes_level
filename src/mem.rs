use rom;

pub trait Addressable {
    fn loadb(&self, addr: u16) -> u8;
    fn storeb(&mut self, addr: u16, val: u8);

    fn loadw(&self, addr: u16) -> u16 {
        self.loadb(addr) as u16 | (self.loadb(addr + 1) as u16) << 8
    }

    fn storew(&mut self, addr: u16, val: u16) {
        self.storeb(addr, (val & 0xFF) as u8);
        self.storeb(addr + 1, ((val >> 8) & 0xFF) as u8);
    }
}
pub struct RAM {
    pub data: [u8; 0x800],
}

impl RAM {
    pub fn new() -> RAM { RAM {data: [0; 0x800]} }
    pub fn loadw(&self, addr: u16) -> u16 {
        self.loadb(addr) as u16 | (self.loadb(addr + 1) as u16) << 8
    }
    pub fn storew(&mut self, addr: u16, val: u16) {
        self.storeb(addr, (val & 0xFF) as u8);
        self.storeb(addr + 1, ((val >> 8) & 0xFF) as u8);
    }
}

impl Addressable for RAM {
    fn loadb(&self, addr: u16) -> u8 { self.data[addr as usize] }
    fn storeb(&mut self, addr: u16, val: u8) { self.data[addr as usize] = val; }
}

pub struct Memory {
    pub ram: RAM,
    // ppu: PPU,
    // apu: APU,
    rom: rom::ROM,
}

impl Memory {
    pub fn from_rom(rom: rom::ROM) -> Memory {
        Memory {
            ram: RAM::new(),
            // ppu
            // apu
            rom: rom,
        }
    }
}

impl Addressable for Memory {
    fn loadb(&self, addr: u16) -> u8 {
        // First 0x2000 bytes are 0x800 mirrored 4 times
        if addr < 0x2000 {
            self.ram.loadb(addr & 0x7ff)
        }
        // Next 0x2000 are 8 bytes mirrored a ton
        else if addr >= 0x2000 && addr < 0x4000 {
            //self.ppu.loadb((addr & 0x7) + 0x2000)
            0u8
        }
        // Next 0x20 are APU
        else if addr >= 0x4000 && addr < 0x4020 {
            //self.apu.loadb(addr)
            0u8
        }
        // The rest is mapped to the cartridge
        else {
            self.rom.loadb(addr)
        }
    }

    fn storeb(&mut self, addr: u16, val: u8) {
        // First 0x2000 bytes are 0x800 mirrored 4 times
        if addr < 0x2000 {
            self.ram.storeb(addr & 0x7ff, val);
        }
        // Next 0x2000 are 8 bytes mirrored a ton
        else if addr >= 0x2000 && addr < 0x4000 {
            //self.ppu.storeb((addr & 0x7) + 0x2000, val);
        }
        // Next 0x20 are APU
        else if addr >= 0x4000 && addr < 0x4020 {
            //self.apu.storeb(addr, val);
        }
        // The rest is mapped to the cartridge
        // TODO: Will the ROM panic if the writes are to ROM?
        else {
            //self.rom.storeb(addr, val);
        }

    }
}
