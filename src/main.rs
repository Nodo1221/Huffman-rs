mod huffman;
mod bits;

use huffman::HuffmanTree;
use bits::BitData;

use std::fs;
use std::path::Path;
use std::error::Error;

macro_rules! bits {
    ($($b:expr),* $(,)?) => {
        &[ $( $b != 0 ),* ]
    };
}

fn encode() -> Result<(), Box<dyn Error>> {
    // Read bytes from file
    let data = fs::read("test.txt")?;

    // Create a tree based on data
    let tree = HuffmanTree::from_vec(&data);

    // Encode data via tree
    let encoded: BitData = tree.encode(&data);

    println!("Encoded data:");
    println!("{encoded}");

    // Write encoded data to .huff file
    tree.write(Path::new("test.txt.huff"), &encoded).unwrap();
    Ok(())
}

fn decode() -> Result<(), Box<dyn Error>> {
    let tree = HuffmanTree::build_from_file(Path::new("test.txt.huff"));

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    encode()?;
    Ok(())
}