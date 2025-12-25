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
    let data = "aaeee";
    // let data = "huffman tree example";
    let tree = HuffmanTree::from(&data);

    // let encoded: BitData = tree.encode();
    // let decoded: Vec<u8> = tree.decode(&encoded);

    // println!("Source: \"{}\"\n", data);
    // println!("Tree:\n{}", tree);
    // println!("Encoded:\n{}", encoded);
    // println!("Decoded: {}", String::from_utf8(decoded).unwrap());
    // println!("Original: {} bytes", data.len());
    // println!("Encoded: {} bytes", encoded.data.len());
    // println!("Ratio: {}%", encoded.data.len() as f32 / data.len() as f32 * 100.0);

    // match tree.write(Path::new("test.txt.huff")) {
    //     Ok(()) => println!("ok!"),
    //     Err(e) => println!("error: {}", e.kind()),
    // }

    // let tree = HuffmanTree::from(Path::new("test.txt.huff"));
}