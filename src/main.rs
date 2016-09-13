#![allow(dead_code)]
mod cpu;
mod mem;
mod rom;

use std::env;


fn main() {
    let mut filename: String;
    if std::env::args().len() > 1 {
        filename = std::env::args().nth(1).unwrap();
    } else {
        filename = "test.nes".to_string();
    }
    let mut cpu: cpu::CPU = cpu::CPU::new(&filename);

    println!("Initializing CPU with state:");
    println!("{:?}", cpu);

    println!("Starting CPU");
    loop {
        cpu.emulate_cycle();
        println!("{}", cpu);
    }
}
