use mem;

use std;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;

const INES_HEADER_MAGIC: u32 = 0x1A53454E; // ELF\x1A

pub struct ROM {
    pub header: INESHeader,
    pub prg: Vec<u8>,
    pub chr: Vec<u8>,
}

impl ROM {
    pub fn from_file(filename: &str) -> ROM {
        let mut f = File::open(filename).unwrap();
        let mut header: [u8; 16] = [0; 16];
        f.read_exact(&mut header).unwrap();

        let header = INESHeader::from_array(&header);
        println!("Got magic {:x}", header.magic);
        let mut prg = vec![0; header.size_prg as usize * 16384];
        let mut chr = vec![0; header.size_chr as usize * 8192];

        // We want to ignore the trainer, but if it's there we must seek past it.
        if header.has_trainer() { f.seek(SeekFrom::Current(512)).unwrap(); }

        // Read in PRG
        let mut len = prg.len();
        f.read_exact(&mut prg[0..len]).unwrap();

        // Read in CHR
        len = chr.len();
        f.read_exact(&mut chr[0..len]).unwrap();

        return ROM {
            header: header,
            prg: prg,
            chr: chr,
        }
    }
}

impl mem::Addressable for ROM {
    fn loadb(&self, mut addr: u16) -> u8 {
        // TODO: Implement mapper and mirroring
        if self.header.size_prg == 1 && addr >= 0xC000 {
            addr -= 0x4000;
        }
        self.prg[(addr as usize) - 0x8000]
    }
    #[allow(unused_variables)]
    fn storeb(&mut self, addr: u16, val: u8) {
        panic!("You cannot write to PRG");
    }
}

#[derive(Default)]
pub struct INESHeader {
    magic: u32,
    size_prg: u8,
    size_chr: u8,
    flags_6: u8,
    flags_7: u8,
    size_prg_ram: u8,
    flags_9: u8,
    flags_10: u8,
    zero: [u8; 5],
}

impl INESHeader {
    fn new() -> INESHeader {
        let header: INESHeader = INESHeader::default();
        return header
    }

    fn from_array(a: &[u8; 16]) -> INESHeader {
        let mut header: INESHeader = INESHeader::default();

        // Create a mutable slice view
        let as_slice: &mut [u8; 16] = unsafe { std::mem::transmute(&mut header) };
        as_slice.copy_from_slice(a);

        assert!(header.magic == INES_HEADER_MAGIC);

        return header
    }

    fn has_trainer(&self) -> bool {
        self.flags_6 & (1 << 2) != 0
    }
}
