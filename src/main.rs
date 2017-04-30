#![allow(dead_code)]
mod cpu;
mod mem;
mod rom;

use std::env;

#[derive(Debug)]
pub struct Args {
    filename: String,
}

impl Args {
    fn parse_args() -> Result<Args, &'static str> {
        let mut args = Args{ filename: "test.nes".to_string() };

        for arg in env::args() {
            match arg {
                _ => {
                    args.filename = arg;
                }
            }
        }
        return Ok(args)
    }
}

fn main() {
    let args = Args::parse_args().unwrap();

    let mut cpu: cpu::CPU = cpu::CPU::new(&args.filename);
    cpu.reset();

    println!("Initializing CPU with state:");
    println!("{:256?}", cpu);
    cpu.print_memory(0, 256);

    println!("Starting CPU");
    loop {
        cpu.emulate_cycle();
    }
}
