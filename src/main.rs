struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    byte: Option<u8>,
    freq: usize,
}

impl Node {
    // fn new(byte: u8, freq: usize) -> Self {
    //     Self {
    //         left: None,
    //         right: None,
    //         byte, freq
    //     } 
    // }
}

struct Queue {
    heap: Vec<Box<Node>>
}

impl Queue {
    fn heapify(&mut self, i: usize) {
        if self.heap.len() < 2 {
            return;
        }

        let left = 2 * i + 1;
        let right = left + 1;
        let mut min = i;

        if left < self.heap.len() && self.heap[left].freq < self.heap[min].freq {
            min = left;
        }

        if right < self.heap.len() && self.heap[right].freq < self.heap[min].freq {
            min = right;
        }

        if min != i {
            self.heap.swap(min, i);
            self.heapify(min);
        }
    }

    fn heapify_up(&mut self, mut i: usize) {
        while i != 0 {
            let parent = (i - 1) / 2;

            if self.heap[parent].freq <= self.heap[i].freq {
                break;
            }

            self.heap.swap(parent, i);
            i = parent;
        }
    }

    fn build_heap(&mut self) {
        for i in (0..=(self.heap.len() / 2 - 1)).rev() {
            self.heapify(i);
        }
    }

    // Could return Option for safety
    fn pop_min(&mut self) -> Box<Node> {
        // Return item at [0], swap with last
        let min = self.heap.swap_remove(0);
        self.heapify(0);
        return min;
    }

    fn add(&mut self, node: Box<Node>) {
        self.heap.push(node);
        self.heapify_up(self.heap.len() - 1)
    }
}

struct HuffmanTree {
    root: Box<Node>,
    queue: Queue,
}

impl HuffmanTree {
    // Build a queue from str
    fn from(text: &str) -> Self {
        let mut tree = HuffmanTree {
            // temp
            root: Box::new(Node {left: None, right: None, byte: None, freq: 0}),
            queue: Queue {heap: Vec::new()}
        };
        let mut freqs = [0usize; 256];

        for byte in text.bytes() {
            freqs[byte as usize] += 1;
        }

        // .into_iter() creates an iterator of values (not references)
        // TODO: tidy this up
        freqs.into_iter()
            .enumerate()
            .for_each(|(byte, freq)| {
                if freq != 0 {
                    tree.queue.add(Box::new(Node {
                        left: None, right: None, byte: Some(byte as u8), freq: freq
                    }));
                }
            });
        tree
    }

    // Build a tree from queue, store root
    fn build(&mut self) {
        while self.queue.heap.len() > 1{
            let left = self.queue.pop_min();
            let right = self.queue.pop_min();
            let freq = left.freq + right.freq;

            let combined = Box::new(Node {
                left: Some(left),
                right: Some(right),
                byte: None,
                freq,
            });
        }
        self.root = self.queue.pop_min();
    }

    fn print_tree(&mut self, node: &Option<Box<Node>>, prefix: String) {
    // 1. Base case: Handle the "if (!node)" check
    // If 'node' is Some(n), we proceed. If None, we do nothing (return).
    if let Some(n) = node {
        
        // 2. Print if leaf: Handle "if (node->c != '\0')"
        // Assuming your Node struct uses Option<u8> for 'byte'
        if let Some(b) = n.byte {
            println!("{}: {}", b as char, prefix);
        }

        // 3. Recursive calls
        // We use format! to create the new string "prefix + '0'"
        self.print_tree(&n.left, format!("{}0", prefix));
        self.print_tree(&n.right, format!("{}1", prefix));
    }
}
}

fn main() {

}
