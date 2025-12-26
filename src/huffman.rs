use std::collections::HashMap;
use std::fmt;

use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufWriter};

use crate::BitData;

const VERSION: u8 = 1;

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    byte: Option<u8>,
    freq: usize,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

// Create a tree from a queue; return root Box<Node>
fn from_queue(mut queue: Queue) -> Box<Node> {
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

fn generate_lookup(root: &Box<Node>) -> HashMap<u8, Vec<bool>> {
    let mut codes = HashMap::new();
    let mut prefix_buffer = Vec::new();
    lookup_recurse(root, &mut prefix_buffer, &mut codes);        
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
        // Run lookup_recurse with a temporarily modified vec (then backtrack -- drop the appendix)
        prefix.push(false);
        lookup_recurse(left_node, prefix, map);
        prefix.pop();
    }

    // If right exists, recurse
    if let Some(right_node) = &node.right {
        prefix.push(true);
        lookup_recurse(right_node, prefix, map);
        prefix.pop();
    }
}

pub struct HuffEncoder {
    root: Box<Node>,
    freqs: [usize; 256],
    unique_bytes: usize,
    lookup: HashMap<u8, Vec<bool>>,
}

impl HuffEncoder {
    // Create a HuffEncoder from data
    pub fn from_vec(data: &[u8]) -> Self {
        let mut freqs = [0usize; 256];
        let mut queue = Queue::new();

        for &byte in data {
            freqs[byte as usize] += 1;
        }
        
        let mut unique_bytes = 0;

        for (byte, &freq) in freqs.iter().enumerate() {
            if freq != 0 {
                unique_bytes += 1;
                queue.add(Box::new(Node::new(byte as u8, freq)))
            }
        }

        let root = from_queue(queue);
        let lookup = generate_lookup(&root);

        Self { lookup, freqs, root, unique_bytes }
    }

    // Encode data (could differ from srouce data for generality)
    pub fn encode(&self, data: &[u8]) -> BitData {
        let mut encoded = BitData::new();

        for raw_byte in data {
            let code = self.lookup.get(raw_byte).expect("Broken tree! Missing key in lookup");
            encoded.write(code);
        }

        encoded.flush();
        encoded
    }

    // Write encoded to output
    pub fn write_to_file(&self, output: &Path, encoded: &BitData) -> io::Result<()> {
        let file = File::create(output)?;
        let mut writer = BufWriter::new(file);

        // HUFF header
        writer.write_all(b"HUFF")?;

        // Offset
        writer.write_all(&(encoded.offset as u8).to_be_bytes())?;

        // Version number
        writer.write_all(&VERSION.to_be_bytes())?;

        // Number of (byte, freq) pairs
        writer.write_all(&(self.unique_bytes as u16).to_be_bytes())?;

        // Byte frequency pairs
        for (byte, &freq) in self.freqs.iter().enumerate() {
            if freq != 0 {
                writer.write_all(&(byte as u8).to_be_bytes())?;
                writer.write_all(&(freq as u32).to_be_bytes())?;   
            }
        }
        
        // Write data
        writer.write_all(&encoded.data)?;

        Ok(())
    }
}

pub struct HuffDecoder {
    root: Box<Node>,
    offset: u8,
    reader: BufReader<File>,
}

impl HuffDecoder {
    // Create a HuffDecoder from file headers
    pub fn from_file_headers(path: &Path) -> io::Result<Self> {
        let mut reader = BufReader::new(File::open(path)?);

        // 1. Validate "HUFF" header
        let mut header = [0u8; 4];
        reader.read_exact(&mut header)?;
        if &header != b"HUFF" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid HUFF header"));
        }

        // 2. Read Offset
        let mut offset_buf = [0u8; 1];
        reader.read_exact(&mut offset_buf)?;
        let offset = offset_buf[0]; 

        // 3. Read Version (Assuming u8/1 byte based on your writer)
        let mut version_buf = [0u8; 1];
        reader.read_exact(&mut version_buf)?;

        // 4. Read Count (u16)
        let mut count_buf = [0u8; 2];
        reader.read_exact(&mut count_buf)?;
        let count = u16::from_be_bytes(count_buf);

        let mut queue = Queue::new();

        // 5. Loop over pairs
        for _ in 0..count {
            let mut b_buf = [0u8; 1];
            reader.read_exact(&mut b_buf)?;
            let byte = b_buf[0];

            let mut f_buf = [0u8; 4];
            reader.read_exact(&mut f_buf)?;
            let freq = u32::from_be_bytes(f_buf);

            queue.add(Box::new(Node::new(byte, freq as usize)));
        }

        let root = from_queue(queue);

        Ok(Self{
            root,
            offset,
            reader
        })
    }

    // Actually decode file under self.reader
    pub fn decode_file(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.reader.read_to_end(&mut buffer)?;
        let decoded = Self::decode_from_root(&self.root, &buffer, self.offset.into());
        Ok(decoded)
    }

    // Decode data based on root tree (no reader)
    fn decode_from_root(root: &Box<Node>, data: &[u8], offset: usize) -> Vec<u8> {
        let mut decoded: Vec<u8> = Vec::new();
        let mut head = root;
        let stored_bits = 8 * (data.len() - 1) + offset;

        for i in 0..stored_bits {
            let current_byte = data[i / 8];
            let bit_index = i % 8;

            // Evaluate bit at bit_index of current_byte
            if current_byte & (1 << (7 - bit_index)) != 0 {
                // Decoding 1, move head to right Node
                head = head.right.as_ref().unwrap();

                // Found a leaf
                if let Some(byte) = &head.byte {
                    decoded.push(*byte);
                    head = root;
                }
            }

            else {
                // Decoding 0, move head to right Node
                head = head.left.as_ref().unwrap();

                // Found a leaf
                if let Some(byte) = &head.byte {
                    decoded.push(*byte);
                    head = root;
                }
            }
        }
        
        decoded
    }
}

impl fmt::Display for HuffEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.lookup.iter()
            .try_for_each(|(byte, code)| {
                let code: String = code.iter().map(|&b| if b {'1'} else {'0'}).collect();
                writeln!(f, "'{}': {}", *byte as char, code)
            })
    }
}