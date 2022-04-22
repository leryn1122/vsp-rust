use std::io::Read;

const BUFFER_SIZE: usize = 5;
// const BUFFER_SIZE: i8 = 2;

/// Buffer, composed of by two char buffer array.
#[warn(dead_code)]
pub struct Buffer {
    buffer_r: [u8; 1 << BUFFER_SIZE],
    buffer_l: [u8; 1 << BUFFER_SIZE],
    pos: usize,
    end: usize,
    flip: bool,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            buffer_r: [0u8; 1 << BUFFER_SIZE],
            buffer_l: [0u8; 1 << BUFFER_SIZE],
            pos: 0,
            end: 0,
            flip: false,
        }
    }

    /// Returns the capacity
    pub fn capacity(&self) -> usize {
        BUFFER_SIZE
    }

    /// flip the left & right buffer
    pub fn flip(&mut self) {
        self.flip = !self.flip;
    }

    pub fn get(&mut self) -> &mut[u8; 1 << BUFFER_SIZE] {
        if self.flip {
            &mut self.buffer_r
        } else {
            &mut self.buffer_l
        }
    }

    pub fn char_at(&mut self, index: usize) -> u8 {
        if self.flip {
            self.buffer_r[index]
        } else {
            self.buffer_l[index]
        }
    }

    pub fn read(&mut self, readable: &mut std::fs::File) -> Result<usize, ()> {
        let offset: usize = readable.read(self.get()).unwrap();
        self.pos = offset;

        println!("{:?}", self.get());

        Ok(offset)
    }
}
