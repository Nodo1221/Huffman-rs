use std::fmt;

pub type Block = u8;

pub struct BitData {
    pub data: Vec<Block>,
    pub capacity: u8,
    buffer: Block,
}

impl BitData {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            capacity: Block::BITS as u8,
            buffer: 0,
        }
    }

    pub fn write(&mut self, mut byte: u32, mut len: u8) {
        let b = Block::BITS as u8;
        let first = (byte >> (32 - self.capacity)) as Block;

        if len < self.capacity {
            self.buffer |= first;
            self.capacity -= len;
            return;
        }

        self.data.push(self.buffer | first);

        byte <<= self.capacity;
        len -= self.capacity;

        let chunks = len / b;

        for i in 0..chunks {
            let current = (byte >> (32 - b - i * b)) as Block;
            self.data.push(current);
        }

        let last = (byte >> (32 - b - chunks * b)) as Block;
        self.buffer = last;
        self.capacity = b - (len - b * chunks);
    }

    pub fn flush(&mut self) {
        self.data.push(self.buffer);
        self.buffer = 0;
    }
}

impl fmt::Display for BitData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = Block::BITS as usize;
        for datum in &self.data {
            writeln!(f, "{:0width$b}", datum, width = width)?;
        }
        write!(f, "current capacity: {}", self.capacity)
    }
}
