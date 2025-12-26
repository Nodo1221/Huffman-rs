use std::fmt;

#[allow(dead_code)]
pub struct BitData {
    pub data: Vec<u8>,
    pub offset: usize,
    buffer: u8,
}

impl BitData {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            offset: 0,
            buffer: 0,
        }
    }

    pub fn write(&mut self, data: &[bool]) {
        for &datum in data {
            if self.offset == 8 {
                self.data.push(self.buffer);
                self.offset = 0;
                self.buffer = 0;
            }

            self.buffer |= (datum as u8) << (7 - self.offset);
            self.offset += 1;
        }
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
        writeln!(f, "current offset: {}", self.offset)
    }
}