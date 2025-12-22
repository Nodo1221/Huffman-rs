pub struct BitData {
    data: Vec<u8>,
    offset: usize,
}

impl BitData {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            offset: 0,
        }
    }

    pub fn write(&mut self, data: &Vec<bool>) {
        for datum in data {
            if self.offset == 8 || self.offset == 0 {
                self.data.push(0u8);
                self.offset = 0;
            }

            *self.data.last_mut().expect("No Vec item to populate") |= (*datum as u8) << (7 - self.offset);
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
