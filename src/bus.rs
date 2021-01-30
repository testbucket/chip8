use crate::ram::Ram;
use crate::display::Display;
use crate::keyboard::Keyboard;

pub(crate) struct Bus {
    ram: Ram,
    keyboard: Keyboard,
    display: Display,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: Ram::new(),
            keyboard: Keyboard::new(),
            display: Display::new(),
        }
    }

    pub fn ram_read_byte(&mut self, address: u16) -> u8 {
        self.ram.read_byte(address)
    }

    pub fn ram_write_byte(&mut self, address: u16, value: u8) {
        self.ram.write_byte(address, value)
    }


    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool{
        self.display.debug_draw_byte(byte, x, y)
    }

    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        self.keyboard.key_pressed(key_code)
    }
    pub fn present_screen(&self) {
        self.display.present_screen();
    }

    pub fn clear_screen(&mut self) {
        self.display.clear_screen();
    }
}

