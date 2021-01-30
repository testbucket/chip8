const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub(crate) struct Display {
    screen: [[u8; WIDTH]; HEIGHT],

}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [[0; WIDTH]; HEIGHT],

        }
    }

    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut flipped = false;
        for _ in 0..8 {
            let mut b = byte;

            let coord_x = x as usize;
            let coord_y = y as usize;

            match (b & 0b1000_0000) >> 7 {
                0 => {
                    if self.screen[coord_x][coord_y] == 1 { flipped = true }
                    self.screen[coord_x][coord_y] = 0
                }
                1 => self.screen[coord_x][coord_y] = 1,
                _ => unreachable!()
            }
            b = b << 1;
        }
        flipped
    }
}