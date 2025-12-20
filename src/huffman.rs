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

    fn build_heap(&mut self) {
        for i in (0..=(self.heap.len() / 2 - 1)).rev() {
            self.heapify(i);
        }
    }

    // Could return Result / Option     for safety
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
    root: Option<Box<Node>>,
    queue: Queue,
}

impl HuffmanTree {
    // Build a queue from str, root is None
    pub fn from(text: &str) -> Self {
        let mut freqs = [0usize; 256];
        let mut queue = Queue::new();

        for byte in text.bytes() {
            freqs[byte as usize] += 1;
        }

        // .into_iter() creates an iterator of values (not references)
        freqs.into_iter()
            .enumerate()
            .filter(|&(_, freq)| freq != 0)
            .for_each(|(byte, freq)| {
                queue.add(Box::new(Node::new(byte as u8, freq)))
            });

        let mut tree = HuffmanTree {
            root: None,
            queue,
        };

        tree.build();
        tree
    }

    // Build a tree from queue, store root
    // TODO (?): make build consume queue?
    pub fn build(&mut self) {
        // TODO: handle len() == 1
        while self.queue.heap.len() > 1 {
            let left = self.queue.pop_min();
            let right = self.queue.pop_min();
            let freq = left.freq + right.freq;

            let combined = Box::new(Node {
                left: Some(left),
                right: Some(right),
                byte: None,
                freq,
            });

            self.queue.add(combined);
        }
        
        self.root = Some(self.queue.pop_min());
    }

    pub fn print_codes(&self) {
        Self::print_recursive(&self.root, String::new());
    }

    // Recursive helper function
    fn print_recursive(node: &Option<Box<Node>>, prefix: String) {
        // 1. Check if node exists (equivalent to C++ `if (!node) return`)
        if let Some(n) = node {
            
            // 2. Check if it's a leaf node (equivalent to `node->c != '\0'`)
            // In your build() function, internal nodes have `byte: None`
            if let Some(b) = n.byte {
                println!("'{}': {}", b as char, prefix);
            }

            // 3. Recurse left (prefix + "0")
            Self::print_recursive(&n.left, format!("{}0", prefix));

            // 4. Recurse right (prefix + "1")
            Self::print_recursive(&n.right, format!("{}1", prefix));
        }
    }
}