use std::fmt;

pub type Block = u8;
pub const PAGE_SIZE: usize = 1024;

pub struct PageFull(pub bool);

pub struct BitData {
    pub data: [Block; PAGE_SIZE],
    pub size: usize,
    pub capacity: u8,
    buffer: Block,
}

impl BitData {
    pub fn new() -> Self {
        Self {
            data: [0; PAGE_SIZE],
            size: 0,
            capacity: Block::BITS as u8,
            buffer: 0,
        }
    }

    pub fn write(&mut self, mut byte: u32, mut len: u8) -> PageFull {
        let b = Block::BITS as u8;

        // Worst-case space check for a 32-bit write,
        // reserving at least 1 slot for a future flush().
        let max_blocks = 32 / Block::BITS as usize;
        if self.size + max_blocks >= PAGE_SIZE {
            return PageFull(true);
        }

        let first = (byte >> (32 - self.capacity)) as Block;

        if len < self.capacity {
            self.buffer |= first;
            self.capacity -= len;
            return PageFull(false);
        }

        self.data[self.size] = self.buffer | first;
        self.size += 1;

        byte <<= self.capacity;
        len -= self.capacity;

        let chunks = len / b;

        for i in 0..chunks {
            let current = (byte >> (32 - b - i * b)) as Block;
            self.data[self.size] = current;
            self.size += 1;
        }

        let last = (byte >> (32 - b - chunks * b)) as Block;
        self.buffer = last;
        self.capacity = b - (len - b * chunks);

        PageFull(false)
    }

    pub fn flush(&mut self) {
        // Unconditionally push the last byte to maintain the bit offset structure
        self.data[self.size] = self.buffer;
        self.size += 1;
        self.buffer = 0;
        self.capacity = Block::BITS as u8;
    }
}

impl fmt::Display for BitData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = Block::BITS as usize;
        for i in 0..self.size {
            writeln!(f, "{:0width$b}", self.data[i], width = width)?;
        }
        write!(f, "current capacity: {}", self.capacity)
    }
}
