// mod huffman;
mod bits;

// use huffman::HuffmanTree;
use bits::BitData;

fn main() {
    // let data = "aaaaaabbbdddeeeeffdfadskfbbbbbbbbbbbbgdsakfds";
    // let tree = HuffmanTree::from(data.into());

    // println!("Source: \"{}\"", data);
    // tree.print();

    let mut bitdata = BitData::new();

    bitdata.write(&[false, true, true, false, true]);
    bitdata.write(&[true, true]);
    bitdata.write(&[false]);
    bitdata.write(&[true, false, true, true, true, true, true, true, true]);

    bitdata.print();
}