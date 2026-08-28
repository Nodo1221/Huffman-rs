use std::fmt;

pub type Block = u8;
pub const PAGE_SIZE: usize = 1024;

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

    pub fn write(&mut self, mut byte: u32, mut len: u8) -> bool {
        let b = Block::BITS as u8;

        // Worst-case space check for a 32-bit write,
        // reserving at least 1 slot for a future flush().
        let max_blocks = 32 / Block::BITS as usize;
        if self.index + max_blocks >= PAGE_SIZE {
            return false;
        }

        let first = (byte >> (32 - self.capacity)) as Block;

        if len < self.capacity {
            self.buffer |= first;
            self.capacity -= len;
            return true;
        }

        self.data[self.index] = self.buffer | first;
        self.index += 1;

        byte <<= self.capacity;
        len -= self.capacity;

        let chunks = len / b;

        for i in 0..chunks {
            let current = (byte >> (32 - b - i * b)) as Block;
            self.data[self.index] = current;
            self.index += 1;
        }

        let last = (byte >> (32 - b - chunks * b)) as Block;
        self.buffer = last;
        self.capacity = b - (len - b * chunks);

        true
    }

    pub fn flush(&mut self) {
        self.data[self.index] = self.buffer;
        self.index += 1;
        self.buffer = 0;
    }
}

impl fmt::Display for BitData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = Block::BITS as usize;
        // Only iterate up to the current index
        for i in 0..self.index {
            writeln!(f, "{:0width$b}", self.data[i], width = width)?;
        }
        write!(f, "current capacity: {}", self.capacity)
    }
}
