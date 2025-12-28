use std::path::Path;
use std::error::Error;

use huffman::bits::BitData;
use huffman::huffman::{HuffDecoder, HuffEncoder, encode_file};

use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Encoding:");
    let (encoder, encoded) = encode_file("shakespeare.txt")?;
    encoder.write_to_file("test.txt.huff", &encoded)?;

    
    println!("\nDecoding:");
    let (decoder, decoded) = HuffDecoder::decode_file("test.txt.huff")?;

    // Printing
    // println!("source: {}\n", data);
    // println!("tree:\n{}", encoder);
    // println!("encoded:\n{}", encoded);
    // println!("decoded: {}", String::from_utf8_lossy(&decoded));
    // println!("compression ratio: {}%", (data.len() - encoded.data.len()) as f32 / data.len() as f32 * 100.0);

    Ok(())
}