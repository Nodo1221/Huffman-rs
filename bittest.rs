struct Writer {
    buffer: u8,
    data: Vec<u8>,
    capacity: u8,
}

impl Writer {
    fn add(&mut self, mut byte: u32, mut len: u8) {
        let first = (byte >> 32 - self.capacity) as u8;
        if (len < capacity) {
            buffer |= first;
            self.capacity -= len;
            return;
        }
        
        self.data.push(self.buffer | first);

        println!("{:08b}", first);

        byte <<= self.capacity;
        len -= self.capacity;

        // let octets = (len - self.capacity + 7) / 8;
        let octets = len / 8;

        println!("len - cap: {}", len);
        println!("octets: {}", octets);
        println!("pyte: {:032b}", byte);

        for i in 0..octets {
            let current: u8 = (byte >> (24 - i * 8)) as u8; // (byte.wrapping_shr(32 - 8 - i as u32 * 8) as u8)
            self.data.push(current);

            println!("{:08b}", current);
        }

        let last = (byte >> (24 - octets * 8)) as u8;

        self.buffer = last;

        let newcap = 8 - (len - 8 * octets);

        println!("last: {:-8b}", last);
        // let newcap = 8 * octets - (len - self.capacity); // or 8 - (len - self.capacity) % 8

        println!("new cap: {}", newcap);

        // println!("{:08b}", self.data.last().unwrap());
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