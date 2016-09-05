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
