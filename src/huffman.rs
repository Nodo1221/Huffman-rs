use std::collections::HashMap;

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    byte: Option<u8>,
    freq: usize,
}

impl Node {
    fn new(byte: u8, freq: usize) -> Self {
        Self {
            left: None,
            right: None,
            byte: Some(byte),
            freq
        } 
    }
}

struct Queue {
    heap: Vec<Box<Node>>,
}

impl Queue {
    fn new() -> Self {
        Self {
            heap: Vec::new()
        }
    }

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

    fn _build_heap(&mut self) {
        for i in (0..=(self.heap.len() / 2 - 1)).rev() {
            self.heapify(i);
        }
    }

    // Could return Result / Option for safety (but it should never fail with Huffman)
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

pub struct HuffmanTree {
    _root: Box<Node>,
    lookup: HashMap<u8, Vec<bool>>,
}

impl HuffmanTree {
    // Build a tree from string using helper Self::build() via queue 
    pub fn from(data: Vec<u8>) -> Self {
        let mut freqs = [0usize; 256];
        let mut queue = Queue::new();

        for byte in data {
            freqs[byte as usize] += 1;
        }

        // .into_iter() creates an iterator of values (not references)
        freqs.into_iter()
            .enumerate()
            .filter(|&(_, freq)| freq != 0)
            .for_each(|(byte, freq)|
                queue.add(Box::new(Node::new(byte as u8, freq)))
            );

        let root = Self::build(&mut queue);
        let lookup = Self::gen_lookup(&root);

        Self { _root: root, lookup }
    }

    pub fn print(&self) {
        // Non owning iter
        self.lookup.iter()
            .for_each(|(byte, code)| {
                println!("'{}': {}", *byte as char, Self::into_str(code));
            });
    }

    fn into_str(code: &Vec<bool>) -> String {
        code.iter()
            .map(|&b| if b {'0'} else {'1'})
            .collect()
    }

    // Builds tree from queue, returns root Box<Node>
    // TODO: handle len() == 1
    fn build(queue: &mut Queue) -> Box<Node> {
        while queue.heap.len() > 1 {
            let left = queue.pop_min();
            let right = queue.pop_min();
            let freq = left.freq + right.freq;

            let combined = Box::new(Node {
                left: Some(left),
                right: Some(right),
                byte: None,
                freq,
            });

            queue.add(combined);
        }

        queue.pop_min()
    }

    fn gen_lookup(root: &Box<Node>) -> HashMap<u8, Vec<bool>> {
        let mut codes = HashMap::new();
        Self::lookup_recurse(root, Vec::new(), &mut codes);
        codes
    }

    fn lookup_recurse(node: &Node, prefix: Vec<bool>, map: &mut HashMap<u8, Vec<bool>>) {
        // Node is a leaf
        if let Some(b) = node.byte {
            map.insert(b, prefix);
            return;
        }

        // If left exists, recurse (new prefix + '0')
        if let Some(left_node) = &node.left {
            let mut new_prefix = prefix.clone();
            new_prefix.push(false);
            Self::lookup_recurse(left_node, new_prefix, map);
        }

        // If left exists, recurse (new prefix + '1')
        if let Some(right_node) = &node.right {
            let mut new_prefix = prefix.clone();
            new_prefix.push(true);
            Self::lookup_recurse(right_node, new_prefix, map);
        }
    }
}