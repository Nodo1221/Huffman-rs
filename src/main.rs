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
    let data = "aaaaaabccccccccbbdddeeeeffdfadskfbbbbbbbuuuubbbbbgdsakfds";
    let tree = HuffmanTree::from(data.into());

    println!("Source: \"{}\"", data);
    tree.print();

    let mut bitdata = BitData::new();

    bitdata.write(bits![0, 0, 0]);
    bitdata.write(bits![1, 0, 1, 0]);
    bitdata.write(bits![1, 1, 1]);
    bitdata.write(bits![0, 0, 1, 1, 1]);

    println!("decoding");
    bitdata.print();
    tree.decode(&bitdata.data);
}