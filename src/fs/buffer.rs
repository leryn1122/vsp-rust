use core::fmt::{Display, Formatter};
use std::io::Read;

const BUFFER_SIZE: usize = 4;
// const BUFFER_SIZE: usize = 2;

///
/// Buffer, composed of by two char buffer array.
///
#[derive(Debug)]
pub struct Buffer {
    r_array: [u8; 1 << BUFFER_SIZE],
    l_array: [u8; 1 << BUFFER_SIZE],
    offset: usize,     // Read boundary && Write offset.
    limit: usize,      // Read end/limit position.
    pos: usize,        // Read current   position.
    read_flip: bool,
    write_flip: bool,
}

impl Buffer {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            r_array: [0u8; 1 << BUFFER_SIZE],
            l_array: [0u8; 1 << BUFFER_SIZE],
            pos: 0,
            offset: 0,
            limit: 0,
            read_flip: false,
            write_flip: false,
        }
    }

    #[allow(unused_variables)]
    pub fn with_capacity(capacity: usize) -> Self {
        // TODO
        Buffer::new()
    }

    /// Returns the capacity.
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        1 << BUFFER_SIZE
    }

    /// 返回读指针的范围，
    #[inline(always)]
    pub fn range(&self) -> usize {
        if self.is_read_ahead() {
            self.limit - self.pos + self.capacity()
        } else {
            self.limit - self.pos
        }
    }
    
    /// 如果两个 flip 标志不同，那么说明已经读指针到了另一个 array
    fn is_read_ahead(&self) -> bool {
        self.read_flip ^ self.write_flip
    }

    /// 翻转读模式的 flip 标志.
    #[inline(always)]
    fn read_flip(&mut self) {
        self.read_flip = !self.read_flip;
    }

    /// 翻转写模式的 flip 标志.
    #[inline(always)]
    fn write_flip(&mut self) {
        self.write_flip = !self.write_flip;
    }

    pub fn get(&mut self, index: usize) -> u8 {
        if (self.pos + index > self.capacity()) ^ self.read_flip {
            self.r_array[index]
        } else {
            self.l_array[index]
        }
    }

    pub(crate) fn get_read_array(&mut self) -> &mut[u8; 1 << BUFFER_SIZE] {
        if self.read_flip {
            &mut self.r_array
        } else {
            &mut self.l_array
        }
    }

    fn get_write_array(&mut self) -> &mut[u8; 1 << BUFFER_SIZE] {
        if self.write_flip {
            &mut self.r_array
        } else {
            &mut self.l_array
        }
    }

    pub fn get_char(&mut self, index: usize) -> char {
        let c = self.get(index);
        // ASCII ??
        if c & (1<<7) == 0 {
            c as char
        } else {
            // TODO solve non-ascii, for now init as the same as ascii
            c as char
        }
    }

    pub fn forward(&mut self, step: usize) {
        if self.pos + step >= self.capacity() {
            self.pos = (self.pos + step) % self.capacity();
            self.read_flip();
        } else {
            self.pos = self.pos + step;
        }
    }

    pub fn read(&mut self, readable: &mut std::fs::File) -> Result<usize, ()> {
        let offset: usize = readable.read(self.get_write_array()).unwrap();
        self.offset = offset;
        self.write_flip();
        Ok(offset)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&*format!(
            "Buffer[left=[{left}],right=[{right}],\
            pos={pos},offset={offset},limit={limit},\
            read_flip={read_flip},write_flip={write_flip}]",
            left = String::from_utf8(self.l_array.to_vec()).unwrap(),
            right = String::from_utf8(self.r_array.to_vec()).unwrap(),
            pos = self.pos,
            offset = self.offset,
            limit = self.limit,
            read_flip = self.read_flip,
            write_flip = self.write_flip)
        )?;
        Ok(())
    }
}