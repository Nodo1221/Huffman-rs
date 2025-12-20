mod huffman;

use huffman::HuffmanTree;

fn main() {
    let data = "aaaaaabbbdddeeeeffdfadskfbbbbbbbbbbbbgdsakfds";
    let tree = HuffmanTree::from(data);
    tree.print();
}