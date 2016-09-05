use std::io::prelude::*;
use std::fs::File;

pub struct ROM {
    pub code: [u8; 0x100],
}

impl ROM {
    fn new() -> ROM {
        ROM {code: [0; 0x100]}
    }

    pub fn from_file(filename: &str) -> ROM {
        let mut f = File::open(filename).unwrap();
        let mut rom = ROM::new();
        f.read(&mut rom.code).unwrap();
        return rom
    }
}
