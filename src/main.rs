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
    let data = "aaaaaabccccccccbbdddeeeeffdfadskfbbbbbbbuuuubbbmbbgdsakfds";
    let tree = HuffmanTree::from(data.into());

    let encoded: BitData = tree.encode(b"fuck");
    let decoded: Vec<u8> = tree.decode(&encoded.data);


    println!("Source: \"{}\"\n", data);
    println!("Tree:");
    tree.print();

    println!("\nEncoded:");
    encoded.print();

    println!("\nDecoded: {}", decoded.iter().map(|x| char::from(*x)).collect::<String>());
}