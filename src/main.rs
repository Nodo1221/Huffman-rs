mod huffman;
mod bits;

use huffman::HuffmanTree;
use bits::BitData;
use std::path::Path;

macro_rules! bits {
    ($($b:expr),* $(,)?) => {
        &[ $( $b != 0 ),* ]
    };
}

fn main() {
    // Build a tree based on a regular uncompressed file
    // let tree = HuffmanTree::from(Path::new("test.txt"));

    // Compress data
    // let encoded: BitData = tree.encode();

    // Write compressed data to file
    // tree.write(Path::new("test.txt.compressed"), &encoded).unwrap();


    let tree = HuffmanTree::decode_file(Path::new("test.txt.compressed"));
    println!("{:?}", tree.source_data);
}