#![allow(dead_code)]
mod cpu;
mod mem;
mod rom;


fn main() {
    let mut cpu: cpu::CPU = cpu::CPU::new();

    cpu.load_rom("test.nes");

    println!("Initializing CPU with state:");
    println!("{:?}", cpu);

    println!("Starting CPU");
    loop {
        cpu.emulate_cycle();
        println!("{}", cpu);
    }
}
