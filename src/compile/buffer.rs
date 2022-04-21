const BUFFER_SIZE: i8 = 5;

/// Buffer, composed of by two char buffer array.
#[warn(dead_code)]
pub struct Buffer {
    buffer_r: [u8; 1 << BUFFER_SIZE],
    buffer_l: [u8; 1 << BUFFER_SIZE],
    read: u16,
    end: u16,
    flip: bool,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            buffer_r: [0u8; 1 << BUFFER_SIZE],
            buffer_l: [0u8; 1 << BUFFER_SIZE],
            read: 0,
            end: 0,
            flip: false,
        }
    }

    pub fn get(&mut self) -> &mut[u8; 1 << BUFFER_SIZE] {
        if self.flip {
            &mut self.buffer_r
        } else {
            &mut self.buffer_l
        }
    }

    /// flip the left & right buffer
    pub fn flip(&mut self) {
        self.flip = !self.flip;
    }
}
