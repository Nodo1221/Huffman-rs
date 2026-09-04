use std::fmt;
use crate::config::{CHUNK_SIZE, PAGE_SIZE};

// pub const PAGE_SIZE: usize = (128.0 * 1024.0 * 1.5 / u64::BITS as f32) as usize;

pub struct BitData {
    pub data: [u64; PAGE_SIZE],
    pub index: usize,
    pub capacity: u8,
    buffer: u64,
}

impl BitData {
    pub fn new() -> Self {
        Self {
            data: [0; PAGE_SIZE],
            index: 0,
            capacity: u64::BITS as u8,
            buffer: 0,
        }
    }

    // Write a type agnostic code (u8, u16, u32) to buffer
    pub fn write<T: Into<u64>>(&mut self, code: T, mut len: u8) {
        let block_size = u64::BITS as u8;
        let input_bits = (std::mem::size_of::<T>() * 8) as u32;

        let mut word = code.into() << (u64::BITS - input_bits);
        let first = word >> (block_size - self.capacity);

        if len < self.capacity {
            self.buffer |= first;
            self.capacity -= len;
            return;
        }

        self.data[self.index] = self.buffer | first;
        self.index += 1;

        word <<= self.capacity;
        len -= self.capacity;

        self.buffer = word;
        self.capacity = block_size - len;
    }

    pub fn flush(&mut self) {
        if self.capacity != u64::BITS as u8 {
            self.data[self.index] = self.buffer;
            self.index += 1;
            self.buffer = 0;
            self.capacity = u64::BITS as u8;
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.buffer = 0;
        self.capacity = u64::BITS as u8;
    }
}

impl fmt::Display for BitData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = u64::BITS as usize;
        for i in 0..self.index {
            writeln!(f, "{:0width$b}", self.data[i])?;
        }
        if self.capacity != u64::BITS as u8 {
            let width = (u64::BITS as u8 - self.capacity) as usize;
            writeln!(f, "{:0width$b}", self.buffer >> self.capacity)?;
            write!(f, "(partial, {width} bits)")?;
        }
        Ok(())
    }
}