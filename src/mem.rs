use rom;

pub struct RAM {
    pub data: [u8; 0x800],
}

impl RAM {
    pub fn new() -> RAM { RAM {data: [0; 0x800]} }
    pub fn loadb(&mut self, addr: u16) -> u8 { self.data[addr as usize] }
    pub fn storeb(&mut self, addr: u16, val: u8) { self.data[addr as usize] = val; }
    pub fn loadw(&mut self, addr: u16) -> u16 {
        self.loadb(addr) as u16 | (self.loadb(addr + 1) as u16) << 8
    }
    pub fn storew(&mut self, addr: u16, val: u16) {
        self.storeb(addr, (val & 0xFF) as u8);
        self.storeb(addr + 1, ((val >> 8) & 0xFF) as u8);
    }
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
    pub fn loadb(&mut self, addr: u16) -> u8 {
        // First 0x2000 bytes are 0x800 mirrored 4 times
        if addr < 0x2000 {
            self.ram.loadb(addr & 0x7ff)
        }
        // Next 0x2000 are 8 bytes mirror a ton
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
}
