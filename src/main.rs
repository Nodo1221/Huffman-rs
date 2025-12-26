mod huffman;
mod bits;

use huffman::{HuffEncoder, HuffDecoder};
use bits::BitData;
use std::path::Path;

macro_rules! bits {
    ($($b:expr),* $(,)?) => {
        &[ $( $b != 0 ),* ]
    };
}

fn main() {
    let data = b"abbbeeeddcdfdjfahfdkjhfjdahfkjhjkhekjhfkad";

    // Encoding
    let encoder = HuffEncoder::from_vec(data);
    let encoded: BitData = encoder.encode(data);
    encoder.write_to_file(Path::new("test.txt.huff"), &encoded).unwrap();
    
    // Deocding
    let decoder = HuffDecoder::from_file_headers(Path::new("test.txt.huff")).unwrap();
    let decoded: Vec<u8> = decoder.decode_file(Path::new("test.txt.huff")).unwrap();

    println!("encoded:");
    println!("{}", encoded);
    println!("{}", String::from_utf8_lossy(&decoded));

    println!("decoded: {}", String::from_utf8_lossy(&decoded));
}