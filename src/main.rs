mod huffman;
mod bits;

use huffman::HuffmanTree;
use bits::BitData;

macro_rules! bits {
    ($($b:expr),* $(,)?) => {
        &[ $( $b != 0 ),* ]
    };
}

fn main() {
    let data = "huffman tree example";
    let tree = HuffmanTree::from(data);

    let encoded: BitData = tree.encode(data.as_bytes());
    let decoded: Vec<u8> = tree.decode(&encoded);

    println!("Source: \"{}\"\n", data);
    println!("Tree:\n{}", tree);
    println!("Encoded:\n{}", encoded);
    println!("Decoded: {}", String::from_utf8(decoded).unwrap());
    println!("Original: {} bytes", data.len());
    println!("Encoded: {} bytes", encoded.data.len());
    println!("Ratio: {}%", encoded.data.len() as f32 / data.len() as f32 * 100.0);
}