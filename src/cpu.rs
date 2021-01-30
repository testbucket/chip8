use std::fmt;

pub(crate) struct Cpu {
    vx: [u8; 16],
    pc: u16,
    prev_pc: u16,
    i: u16,
}

pub const PROGRAM_START: u16 = 0x200;

use crate::ram::Ram;

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev_pc: 0,
        }
    }

    pub fn run_instruction(&mut self, ram: &mut Ram) {
        let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;
        let instruction: u16 = (hi << 8) | lo;
        println!("instruction read {:#x}: hi:{:#x}, lo:{:#x}", instruction, hi, lo);


        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x0FF) as u8;
        let n = (instruction & 0x00F) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        println!("nnn={:?}, nn={:?}, n={:?} x={}, y={}", nnn, nn, n, x, y);

        if self.prev_pc == self.pc{
            panic!("pc = prev pc")
        }
        self.prev_pc = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x1 => {
                self.pc = nnn;
            }
            0x6 => {
                // vx -- nn
                self.write_reg_vx(x, nn);
                self.pc +2;
            }

            _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instruction),
        }

    }

    pub fn write_reg_vx(&mut self, index: u8, value: u8) {
        println!("writing");
        self.vx[index as usize] = value;
    }

    pub fn read_reg_vx(&mut self, index: u8) -> u8 {
        self.vx[index as usize]
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\npc: {:#X}\n", self.pc)?;
        write!(f, "\nprevious pc: {:#X}\n", self.prev_pc)?;
        write!(f, "vx: ")?;
        for item in &self.vx {
            write!(f, "{:#X} ", *item)?;
        }
        write!(f, "\n")?;
        write!(f, "i: {:#X}\n", self.i)
    }
}