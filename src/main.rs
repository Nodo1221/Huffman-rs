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
    let data = " aaaxaaannrbhwuuupuuuuuxhthkkkkxhccrxcccmmfffmxelllecxccx ";
    let tree = HuffmanTree::from(data.into());

    let encoded: BitData = tree.encode(b"huffman tree example");
    let decoded: Vec<u8> = tree.decode(&encoded);

    println!("Source: \"{}\"\n", data);
    println!("Tree:");
    tree.print();

    println!("\nEncoded:");
    encoded.print();

    println!("\nDecoded: {}", decoded.iter().map(|x| char::from(*x)).collect::<String>());
    println!("Original: {} bytes", data.len());
    println!("Encoded: {} bytes", encoded.data.len());
    println!("Ratio: {}%", encoded.data.len() as f32 / data.len() as f32 * 100.0);
}