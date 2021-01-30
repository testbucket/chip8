mod ram;
mod chip8;
mod cpu;

use std::fs::File;
use std::io::prelude::*;

use crate::ram::Ram;
use crate::chip8::Chip8;
use crate::cpu::Cpu;

fn main() {

    let mut file = File::open("INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom (&data);

    loop {
        chip8.run_instruction();
    }
    print!("hello")
}
