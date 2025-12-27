use std::path::Path;
use std::error::Error;

mod huffman;
mod bits;

use huffman::{HuffEncoder, HuffDecoder};
use bits::BitData;

use std::fs;

// macro_rules! bits {
//     ($($b:expr),* $(,)?) => {
//         &[ $( $b != 0 ),* ]
//     };
// }

fn main() -> Result<(), Box<dyn Error>> {
    let data: Vec<u8> = fs::read("shakespeare.txt")?;

    println!("Encoding:");
    let encoder = HuffEncoder::from_file(Path::new("shakespeare.txt"))?;
    let encoded: BitData = encoder.encode(&data);

    // Write result
    encoder.write_to_file(Path::new("test.txt.huff"), &encoded)?;
    
    println!("\nDecoding:");
    let mut decoder = HuffDecoder::from_file_headers(Path::new("test.txt.huff"))?;
    let decoded: Vec<u8> = decoder.decode_file()?;

    // Printing
    // println!("source: {}\n", data);
    // println!("tree:\n{}", encoder);
    // println!("encoded:\n{}", encoded);
    // println!("decoded: {}", String::from_utf8_lossy(&decoded));
    // println!("compression ratio: {}%", (data.len() - encoded.data.len()) as f32 / data.len() as f32 * 100.0);

    Ok(())
}