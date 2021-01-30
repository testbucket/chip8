pub(crate) struct Display {
}

impl Display{

    pub fn new()-> Display{
        Display{}
    }

    pub fn debug_draw_byte(&self, byte: u8, x: u8, y:u8){
        for _ in 0..8 {
            let mut b = byte;
            match (b & 0b1000_0000) >> 7 {
                0 => print!("_"),
                1 => print!("#"),
                _ => unreachable!()
            }
            b = b << 1;
        }
        print!("\n");
    }

}