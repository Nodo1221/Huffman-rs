pub struct BitData {
    pub data: Vec<u8>,
    pub offset: usize,
}

impl BitData {
    pub fn new() -> Self {
        Self {
            data: Vec::from([0]),
            offset: 0,
        }
    }

    pub fn write(&mut self, data: &[bool]) {
        // let mut current_byte = 0u8;

        for &datum in data {
            if self.offset == 8 {
                self.data.push(0u8);
                self.offset = 0;
            }

            // TODO: don't write to the heap immediatelly, keep a local buffer with the curret byte
            let last = self.data.len() - 1;
            self.data[last] |= (datum as u8) << (7 - self.offset);
            self.offset += 1;
        }
    }

    pub fn print(&self) {
        for datum in &self.data {
            println!("{:08b}", datum);
        }
        println!("current offset: {}", self.offset);
    }
}