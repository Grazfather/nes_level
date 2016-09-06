use std;
use std::io::prelude::*;
use std::fs::File;

const INES_HEADER_MAGIC: u32 = 0x1A53454E; // ELF\x1A

pub struct ROM {
    pub header: iNESHeader,
    pub prg: Vec<u8>,
    pub chr: Vec<u8>,
}

impl ROM {

    pub fn from_file(filename: &str) -> ROM {
        let mut f = File::open(filename).unwrap();
        let mut header: [u8; 16] = [0; 16];
        f.read_exact(&mut header).unwrap();

        let header = iNESHeader::from_array(&header);
        let prg = vec![0; header.size_prg as usize * 16384];
        let chr = vec![0; header.size_chr as usize * 8192];

        println!("Got magic {:x}", header.magic);
        return ROM {
            header: header,
            prg: prg,
            chr: chr,
        }
    }
}

pub struct iNESHeader {
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

impl iNESHeader {
    fn new() -> iNESHeader {
        let header: iNESHeader = unsafe { std::mem::zeroed() };
        return header
    }

    fn from_array(a: &[u8; 16]) -> iNESHeader {
        let mut header: iNESHeader = unsafe { std::mem::zeroed() };

        // Create a mutable slice view
        let as_slice: &mut [u8; 16] = unsafe { std::mem::transmute(&mut header) };
        as_slice.copy_from_slice(a);

        assert!(header.magic == INES_HEADER_MAGIC);

        return header
    }
}
