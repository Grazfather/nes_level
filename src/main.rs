#![allow(dead_code)]
mod cpu;
mod mem;
mod rom;

use std::env;


fn main() {
    let filename = if env::args().len() > 1 {
        env::args().nth(1).unwrap()
    }
    else {
        "test.nes".to_string()
    };

    let mut cpu: cpu::CPU = cpu::CPU::new(&filename);
    cpu.reset();

    println!("Initializing CPU with state:");
    println!("{:256?}", cpu);
    cpu.print_memory(0, 256);

    println!("Starting CPU");
    loop {
        cpu.emulate_cycle();
        println!("{:64?}", cpu);
    }
}
