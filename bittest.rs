struct Writer {
    buffer: u8,
    data: Vec<u8>,
    capacity: u8,
}

impl Writer {
    fn add(&mut self, mut byte: u32, mut len: u8) {
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
}

// 01001011 | 11101110 | 11000000

fn main() {
    let mut w1 = Writer {
        buffer: 0b11100000,
        data: Vec::new(),
        capacity: 5,
    };

    // w1.add(0b10101010_01011111_01110110_00000000, 23);
    w1.add(0b11000000_00000000_00000000_00000000, 23);
}