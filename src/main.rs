mod huffman;

use huffman::HuffmanTree;

fn main() {
    let mut tree = HuffmanTree::from("aaaaaabbbdddeeeeffdfadskfbbbbbbbbbbbbgdsakfds");

    tree.build();
    tree.print_codes();
}
