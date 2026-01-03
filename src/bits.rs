use std::fmt;

#[allow(dead_code)]
pub struct BitData {
    pub data: Vec<u8>,
    pub capacity: u8,
    buffer: u8,
}

impl BitData {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            capacity: 0,
            buffer: 0,
        }
    }

    pub fn write(&mut self, mut byte: u32, mut len: u8) {
        let first = (byte >> 32 - self.capacity) as u8;

        if len < self.capacity {
            self.buffer |= first;
            self.capacity -= len;
            return;
        }

        self.data.push(self.buffer | first);

        byte <<= self.capacity;
        len -= self.capacity;

        let octets = len / 8;

        for i in 0..octets {
            let current: u8 = (byte >> (24 - i * 8)) as u8;
            self.data.push(current);
        }

        let last = (byte >> (24 - octets * 8)) as u8;
        self.buffer = last;
        self.capacity = 8 - (len - 8 * octets);
    }

    pub fn flush(&mut self) {
        self.data.push(self.buffer);
        self.buffer = 0;
    }
}

impl fmt::Display for BitData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for datum in &self.data {
            writeln!(f, "{:08b}", datum)?;
        }
        writeln!(f, "current offset: {}", self.capacity)
    }
}