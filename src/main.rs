use std::error::Error;
use huffman::huffman::{HuffDecoder, HuffEncoder};


fn main() -> Result<(), Box<dyn Error>> {
    println!("Encoding:");
    let (encoder, encoded) = HuffEncoder::encode_file("shakespeare.txt")?;

    println!("\nSaved to file");
    encoder.write_file("test.txt.huff", &encoded)?;

    println!("\nDecoding:");
    let (decoder, decoded) = HuffDecoder::decode_file("test.txt.huff")?;
    
    Ok(())
}