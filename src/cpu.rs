use std::fmt;
use crate::bus::Bus;

pub(crate) struct Cpu {
    vx: [u8; 16],
    pc: u16,
    prev_pc: u16,
    i: u16,
    ret_stack: Vec<u16>,

}

pub const PROGRAM_START: u16 = 0x200;


impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev_pc: 0,
            ret_stack: Vec::<u16>::new(),

        }
    }

    pub fn run_instruction(&mut self, bus: &mut Bus) {
        let hi = bus.ram_read_byte(self.pc) as u16;
        let lo = bus.ram_read_byte(self.pc + 1) as u16;
        let instruction: u16 = (hi << 8) | lo;
        println!("** instruction read {:#x}: hi:{:#x}, lo:{:#x}", instruction, hi, lo);


        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x0FF) as u8;
        let n = (instruction & 0x00F) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        println!("nnn={:?}, nn={:?}, n={:?} x={}, y={}", nnn, nn, n, x, y);

        if self.prev_pc == self.pc {
            panic!("You need to change the Program Counter")
        }
        self.prev_pc = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x1 => {
                self.pc = nnn;
            }
            0x2 => {
                // call subroutine
                self.ret_stack.push(self.pc + 2);
                self.pc = nnn;
            }
            0x6 => {
                // vx -- nn
                self.write_reg_vx(x, nn);
                self.pc += 2;
            }
            0x3 => {
                // skips the next instruction is vx==nn
                let vx = self.read_reg_vx(x);
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x7 => {
                let vx = self.read_reg_vx(x);
                self.write_reg_vx(x, vx.wrapping_add(nn));

                self.pc += 2;
            }

            0x8 => {
                match n {
                    0 => {
                        let vy = self.read_reg_vx(y);
                        self.write_reg_vx(x, vy);
                    }

                    _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instruction)
                }

                self.pc += 2;
            }
            0xA => {
                // I -> nnn
                self.i = nnn;
                self.pc += 2;
            }
            0xD => {
                self.debug_draw_sprite(bus, x, y, n);
                self.pc += 2;
            }
            0xE => {
                match nn {
                    0xA1 => {
                        let key = self.read_reg_vx(x);

                    }
                    _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instruction)
                }
                // self.pc += 2;
            }
            0xF => {
                // I +=Vx
                let vx = self.read_reg_vx(x);
                self.i += vx as u16;
                self.pc += 2;
            }
            _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instruction),
        }


    }


    pub fn write_reg_vx(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value;
    }

    pub fn read_reg_vx(&mut self, index: u8) -> u8 {
        self.vx[index as usize]
    }


    fn debug_draw_sprite(&self, bus: &mut Bus, x: u8, y: u8, height: u8) {
        println!("Drawing sprite at ({},{})", x, y);
        for y in 0..height {
            let b = bus.ram_read_byte(self.i + y as u16);
            bus.debug_draw_byte(b, x, y);
        }
        print!("\n");
    }


}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\npc: {:#X}\n", self.pc)?;
        write!(f, "vx: ")?;
        for item in &self.vx {
            write!(f, "{:#X} ", *item)?;
        }
        write!(f, "\n")?;
        write!(f, "i: {:#X}\n", self.i)
    }
}