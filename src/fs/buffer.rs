use std::io::Read;

const BUFFER_SIZE: usize = 8;
// const BUFFER_SIZE: i8 = 2;

///
/// Buffer, composed of by two char buffer array.
///
pub struct Buffer {
    r_buffer: [u8; 1 << BUFFER_SIZE],
    l_buffer: [u8; 1 << BUFFER_SIZE],
    offset: usize,
    pos: usize,
    end: usize,
    flip: bool,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            r_buffer: [0u8; 1 << BUFFER_SIZE],
            l_buffer: [0u8; 1 << BUFFER_SIZE],
            offset: 0,
            pos: 0,
            end: 0,
            flip: false,
        }
    }

    #[allow(unused_variables)]
    pub fn with_capacity(capacity: usize) -> Self {
        // TODO
        Buffer::new()
    }

    /// Returns the capacity
    pub fn capacity(&self) -> usize {
        BUFFER_SIZE
    }

    /// flip the left & right buffer
    pub fn flip(&mut self) {
        self.flip = !self.flip;
    }

    fn get(&mut self) -> &mut[u8; 1 << BUFFER_SIZE] {
        if self.flip {
            &mut self.r_buffer
        } else {
            &mut self.l_buffer
        }
    }

    #[allow(unused_variables)]
    fn put(&mut self, byte: u8) {
    }

    pub fn at(&mut self, index: usize) -> u8 {
        if self.flip {
            self.r_buffer[index]
        } else {
            self.l_buffer[index]
        }
    }

    pub fn char_at(&mut self, index: usize) -> char {
        let c = self.at(index);
        // ASCII ??
        if c & (1<<7) == 0 {
            c as char
        } else {
            // TODO solve non-ascii, for now init as the same as ascii
            c as char
        }
    }

    //
    pub fn read(&mut self, readable: &mut std::fs::File) -> Result<usize, ()> {
        let offset: usize = readable.read(self.get()).unwrap();
        self.pos = offset;
        Ok(offset)
    }
}