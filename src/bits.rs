use std::fmt;

pub type Block = u8;
pub const PAGE_SIZE: usize = 1024 * 2;

pub struct BitData {
    pub data: [Block; PAGE_SIZE],
    pub index: usize,
    pub capacity: u8,
    buffer: Block,
}

impl BitData {
    pub fn new() -> Self {
        Self {
            data: [0; PAGE_SIZE],
            index: 0,
            capacity: Block::BITS as u8,
            buffer: 0,
        }
    }

    pub fn write(&mut self, mut byte: u32, mut len: u8) {
        let b = Block::BITS as u8;
        debug_assert!(self.index + if len < self.capacity { 0 } else { 1 + ((len - self.capacity) / b) as usize } <= PAGE_SIZE);
        let first = (byte >> (32 - self.capacity)) as Block;
        if len < self.capacity {
            self.buffer |= first;
            self.capacity -= len;
            return;
        }
        self.data[self.index] = self.buffer | first;
        self.index += 1;
        byte <<= self.capacity;
        len -= self.capacity;
        let chunks = len / b;
        for i in 0..chunks {
            self.data[self.index] = (byte >> (32 - b - i * b)) as Block;
            self.index += 1;
        }
        self.buffer = (byte >> (32 - b - chunks * b)) as Block;
        self.capacity = b - (len - b * chunks);
    }

    pub fn flush(&mut self) {
        if self.capacity != Block::BITS as u8 {
            self.data[self.index] = self.buffer;
            self.index += 1;
            self.buffer = 0;
            self.capacity = Block::BITS as u8;
        }
    }
}

impl fmt::Display for BitData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = Block::BITS as usize;
        for i in 0..self.index {
            writeln!(f, "{:0width$b}", self.data[i], width = width)?;
        }
        write!(f, "current capacity: {}", self.capacity)
    }
}
