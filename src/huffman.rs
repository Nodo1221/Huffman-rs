use std::collections::HashMap;
use std::path::Path;
use std::fmt;

use std::fs::{read, File};
use std::io::{Write, BufWriter};
use std::convert::TryFrom;

use crate::BitData;

const VERSION: u16 = 1;

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

    // Could return Result / Option for safety (but it should never fail with Huffman)
    fn pop_min(&mut self) -> Box<Node> {
        // Return item at [0], swap with last
        let min = self.heap.swap_remove(0);
        self.heapify(0);
        min
    }

    fn add(&mut self, node: Box<Node>) {
        self.heap.push(node);
        self.heapify_up(self.heap.len() - 1)
    }
}

pub struct HuffmanTree {
    root: Box<Node>, // For decoding
    freqs: [usize; 256], // For encoding header
    lookup: HashMap<u8, Vec<bool>>, // For encoding data
    pub source_data: Vec<u8>, // Could be a &[u8]? Probly unnecesary
}

// Build from &str
impl From<&str> for HuffmanTree {
    fn from(data: &str) -> Self {
        Self::from_vec(data.into())
    }
}

// Build from path
impl From<&Path> for HuffmanTree {
    fn from(path: &Path) -> Self {
        let data: Vec<u8> = read(path).unwrap();
        Self::from_vec(data)
    }
}

impl HuffmanTree {
    // Build from &[u8] by parsing a queue with Self::build()
    pub fn from_vec(data: Vec<u8>) -> Self {
        let mut freqs = [0usize; 256];
        let mut queue = Queue::new();

        for &byte in &data {
            freqs[byte as usize] += 1;
        }

        // .into_iter() creates an iterator of values (not references)
        // They are moved, not referenced, but freqs is of Copy, so they're copied anyway
        freqs.into_iter()
            .enumerate()
            .filter(|&(_, freq)| freq != 0)
            .for_each(|(byte, freq)|
                queue.add(Box::new(Node::new(byte as u8, freq)))
            );

        let root = Self::build(&mut queue);
        let lookup = Self::generate_lookup(&root);

        Self { root, lookup, freqs, source_data: data }
    }

    // Decode a compressed file
    pub fn decode_file(path: &Path) -> Self {
        let raw_data: Vec<u8> = read(path).unwrap();
            let mut data = BitData::new();

            if raw_data.get(..4) != Some(b"HUFF") {
                panic!("invalid header");
            }

            data.offset = raw_data[5] as usize;

            // let mut hashes = 0;

            let mut i = 6;
            let mut queue = Queue::new();

            while i < raw_data.len() {
                if raw_data[i] == b'#' {
                    if let Some(slice) = raw_data.get(i..i+6) {
                        println!("break @ {}", i);
                        break;
                    } else {
                        println!("Not enough elements, ignoring");
                    }

                    // generate a Queue here, then build from queue
                    queue.heap.push(Box::new(Node::new(raw_data[i], raw_data[i+1].into())));
                }
                
                i += 2;
            }

            
            i += 6;
            
            queue.build_heap();
            data.data = (&raw_data[i..]).to_vec();

            // println!("rest of data: {:08b}", &raw_data[i..][0]);

            // definetely wrong
            let root = Self::build(&mut queue);
            let lookup = Self::generate_lookup(&root);
            Self {
                root: root,
                freqs: [0; 256],
                lookup: lookup,
                source_data: Vec::new(),

            }
    }
    
    // Encode from source_data
    // Return BitData (containing offset)
    pub fn encode(&self) -> BitData {
        let mut encoded = BitData::new();

        for raw_byte in &self.source_data {
            let code = self.lookup.get(raw_byte).expect("Broken tree! Missing key in lookup");
            encoded.write(code);
        }

        encoded
    }

    // Decode data, return Vec<u8> with decoded bits
    pub fn decode(&self, data: &BitData) -> Vec<u8> {
        let mut decoded: Vec<u8> = Vec::new();
        let mut head = &self.root;
        let stored_bits = 8 * (data.data.len() - 1) + data.offset;

        for i in 0..stored_bits {
            let current_byte = data.data[i / 8];
            let bit_index = i % 8;

            // Evaluate bit at bit_index of current_byte
            if current_byte & (1 << 7 - bit_index) != 0 {
                // Decoding 1, move head to right Node
                head = head.right.as_ref().unwrap();

                // Found a leaf
                if let Some(byte) = &head.byte {
                    decoded.push(*byte);
                    head = &self.root;
                }
            }

            else {
                // Decoding 0, move head to right Node
                head = head.left.as_ref().unwrap();

                // Found a leaf
                if let Some(byte) = &head.byte {
                    decoded.push(*byte);
                    head = &self.root;
                }
            }
        }
        
        decoded
    }

    // Write encoded &BitData (for offset) to a file
    pub fn write(&self, output: &Path, encoded_data: &BitData) -> std::io::Result<()> {
        let file = File::create(output)?;
        let mut writer = BufWriter::new(file);

        // HUFF header
        writer.write_all(b"HUFF")?;

        // Offset
        writer.write_all(&(encoded_data.offset as u8).to_be_bytes())?;

        // Version number
        writer.write_all(&VERSION.to_be_bytes())?;

        // Byte frequency pairs
        for (byte, &freq) in self.freqs.iter().enumerate() {
            if freq != 0 {
                writer.write_all(&(byte as u8).to_be_bytes())?;
                writer.write_all(&(freq as u32).to_be_bytes())?;   
            }
        }

        // End of table
        writer.write_all(b"######")?;
        
        println!("encoded data:\n{}", encoded_data);

        // Write data
        writer.write_all(&encoded_data.data)?;

        Ok(())
    }

    pub fn read(input: &Path) -> std::io::Result<()> {
        Ok(())
    }

    fn into_str(code: &[bool]) -> String {
        code.iter()
            .map(|&b| if b {'1'} else {'0'})
            .collect()
    }

    // Builds a tree from queue, returns root Box<Node>
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

    // Return a hashtable of codes
    fn generate_lookup(root: &Box<Node>) -> HashMap<u8, Vec<bool>> {
        let mut codes = HashMap::new();
        let mut prefix_buffer = Vec::new();
        Self::lookup_recurse(root, &mut prefix_buffer, &mut codes);        
        codes
    }

    fn lookup_recurse(node: &Node, prefix: &mut Vec<bool>, map: &mut HashMap<u8, Vec<bool>>) {
        // Node is a leaf
        if let Some(b) = node.byte {
            map.insert(b, prefix.clone());
            return;
        }

        // If left exists, recurse
        if let Some(left_node) = &node.left {
            // Run Self::lookup_recurse with a temporarily modified vec (then backtrack -- drop the appendix)
            prefix.push(false);
            Self::lookup_recurse(left_node, prefix, map);
            prefix.pop();
        }

        // If right exists, recurse
        if let Some(right_node) = &node.right {
            prefix.push(true);
            Self::lookup_recurse(right_node, prefix, map);
            prefix.pop();
        }
    }
}

impl fmt::Display for HuffmanTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.lookup.iter()
            .try_for_each(|(byte, code)|
                writeln!(f, "'{}': {}", *byte as char, Self::into_str(code))
            )
    }
}