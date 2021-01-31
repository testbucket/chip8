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
        println!(
            "Instruction read {:#X}:{:#X}: hi{:#X} lo:{:#X} ",
            self.pc,
            instruction,
            hi,
            lo
        );

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
            0x0 => {
                match nn {
                    0xE0 => {
                        bus.clear_screen();
                        self.pc += 2;
                    }
                    0xEE => {
                        // return from sub routine
                        self.pc = self.ret_stack.pop().unwrap();
                    }
                    _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instruction)
                }
            }
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
                let vy = self.read_reg_vx(y);
                let vx = self.read_reg_vx(x);

                match n {
                    0 => {
                        self.write_reg_vx(x, vy);
                    }

                    2 => {
                        self.write_reg_vx(x, vx & vy);
                    }

                    3 => {
                        self.write_reg_vx(x, vx ^ vy);
                    }
                    4 => {
                        let sum: u16 = vx as u16 + vy as u16;
                        self.write_reg_vx(x, sum as u8);
                        if sum > 0xFF {
                            self.write_reg_vx(0xF, 1)
                        }
                    }
                    5 => {
                        let diff: i8 = vx as i8 - vy as i8;
                        self.write_reg_vx(x, diff as u8);
                        if diff < 0 {
                            self.write_reg_vx(0xF, 1);
                        } else {
                            self.write_reg_vx(0xF, 0);
                        }
                    }
                    0x6 => {
                        // Vx=Vx>>1
                        self.write_reg_vx(0xF, vx & 0x1);
                        self.write_reg_vx(x, vx >> 1);
                    }
                    0x7 => {
                        let diff: i8 = vy as i8 - vx as i8;
                        self.write_reg_vx(x, diff as u8);
                        if diff < 0 {
                            self.write_reg_vx(0xF, 1);
                        } else {
                            self.write_reg_vx(0xF, 0);
                        }
                    }
                    0xE => {
                        // VF is the most significant bit value.
                        // SHR Vx
                        self.write_reg_vx(0xF, (vx & 0x80) >> 7);
                        self.write_reg_vx(x, vx << 1);
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
                        // if(key()!=Vx) then skip the next instruction
                        let key = self.read_reg_vx(x);
                        if !bus.is_key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    0x9E => {
                        // if(key()==Vx) then skip the next instruction
                        let key = self.read_reg_vx(x);
                        if bus.is_key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => panic!(
                        "Unrecognized 0xEX** instruction {:#X}:{:#X}",
                        self.pc,
                        instruction
                    ),
                };
            }
            0xF => {
                match nn {
                    0x07 => {
                        // self.write_reg_vx(x, bus.get_delay_timer());
                        self.pc += 2;
                    }
                    0x0A => {
                        self.pc += 2;

                        // if let Some(val) = bus.get_key_pressed() {
                        //     self.write_reg_vx(x, val);
                        //     self.pc += 2;
                        // }
                    }
                    0x15 => {
                        self.pc += 2;

                        // bus.set_delay_timer(self.read_reg_vx(x));
                        // self.pc += 2;
                    }
                    0x18 => {
                        // TODO Sound timer
                        self.pc += 2;
                    }
                    0x1E => {
                        //I +=Vx
                        let vx = self.read_reg_vx(x);
                        self.i += vx as u16;
                        self.pc += 2;
                    }
                    0x29 => {
                        //i == sprite address for character in Vx
                        //Multiply by 5 because each sprite has 5 lines, each line
                        //is 1 byte.
                        self.i = self.read_reg_vx(x) as u16 * 5;
                        self.pc += 2;
                    }
                    0x33 => {
                        let vx = self.read_reg_vx(x);
                        bus.ram_write_byte(self.i, vx / 100);
                        bus.ram_write_byte(self.i + 1, (vx % 100) / 10);
                        bus.ram_write_byte(self.i + 2, vx % 10);
                        self.pc += 2;
                    }
                    0x55 => {
                        for index in 0..x + 1 {
                            let value = self.read_reg_vx(index);
                            bus.ram_write_byte(self.i + index as u16, value);
                        }
                        self.i += x as u16 + 1;
                        self.pc += 2;
                    }
                    0x65 => {
                        for index in 0..x + 1 {
                            let value = bus.ram_read_byte(self.i + index as u16);
                            self.write_reg_vx(index, value);
                        }
                        self.i += x as u16 + 1;
                        self.pc += 2;
                    }
                    _ => panic!(
                        "Unrecognized 0xF instruction {:#X}:{:#X}",
                        self.pc,
                        instruction
                    ),
                }
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


    fn debug_draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, height: u8) {
        println!("Drawing sprite at ({},{})", x, y);
        let mut should_set_vf = false;
        for y in 0..height {
            let b = bus.ram_read_byte(self.i + y as u16);
            if bus.debug_draw_byte(b, x, y) {
                should_set_vf = true;
            }
        }
        if should_set_vf {
            self.write_reg_vx(0xF, 1);
        } else {
            self.write_reg_vx(0xF, 0);
        }
        bus.present_screen();
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