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

    pub fn write(&mut self, data: &[bool]) {
        // &datum dereferences each datum: &bool -> bool
        for &datum in data {
            if self.data.is_empty() || self.offset == 8 {
                self.data.push(0u8);
                self.offset = 0;
            }

            assert!(!self.data.is_empty());

            // Creates a unique mutable borrow (better for performance for some reason over a direct modification in one line)
            let last = self.data.last_mut().unwrap(); 
            *last |= (datum as u8) << (7 - self.offset);

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
