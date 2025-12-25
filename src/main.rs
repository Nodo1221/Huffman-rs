mod huffman;
mod bits;

use huffman::HuffmanTree;
use bits::BitData;

use std::fs::{File, read};
use std::io::{BufReader, Read, Seek, Result};

use std::path::Path;
use std::error::Error;

macro_rules! bits {
    ($($b:expr),* $(,)?) => {
        &[ $( $b != 0 ),* ]
    };
}

fn encode() -> Result<()> {
    // Read bytes from file
    let data = read("test.txt")?;

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

fn decode() -> Result<()> {
    let file = File::open("test.txt.huff")?;
    let mut reader = BufReader::new(file);
    let tree = HuffmanTree::parse_headers(&mut reader)?;

    tree.decode_file(&mut reader);

    Ok(())
}

fn main() -> Result<()> {
    decode()?;
    Ok(())
}