use std::fmt;
use std::time::Instant;

use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

use crate::config::CHUNK_SIZE;
use crate::bits::{BitData};
use crate::queue::{Node, Queue};

const VERSION: u8 = 1;
// const PAGE_SIZE: usize = (128.0 * 1024.0 / u64::BITS as f32) as usize;

pub struct HuffEncoder {
    tree: Box<Node>,
    freqs: [usize; 256],
    unique_bytes: u16,
    lookup: [(u32, u8); 256],
}

impl HuffEncoder {
    // Create a HuffEncoder from data
    #[hotpath::measure]
    pub fn from_data(data: &[u8]) -> Self {
        let mut freqs = [0usize; 256];
        let mut queue = Queue::new();

        for &byte in data {
            freqs[byte as usize] += 1;
        }

        let mut unique_bytes: u16 = 0;

        for (byte, &freq) in freqs.iter().enumerate() {
            if freq != 0 {
                unique_bytes += 1;
                queue.add(Box::new(Node::new(byte as u8, freq)))
            }
        }

        let tree = queue.build_tree();
        let lookup = Self::get_codes(&tree);

        Self {
            lookup,
            freqs,
            tree,
            unique_bytes,
        }
    }
    
    // Encode a single chunk (without flushing!)
    pub fn encode_chunk(&self, data: &[u8], out: &mut BitData) {
        out.reset();
        for &byte in data {
            let (code, len) = self.lookup[byte as usize];
            out.write(code, len);
        }
    }

    // Write a chunk to writer. Precede each with offset (number of written u64's)
    pub fn write_chunk(writer: &mut impl Write, chunk: &mut BitData) -> io::Result<()> {
        let offset = chunk.capacity;
        chunk.flush();
        writer.write_all(&offset.to_be_bytes())?;
        writer.write_all(&(chunk.index as u16).to_be_bytes())?;
        for block in &mut chunk.data[..chunk.index] {
            *block = block.to_be();
        }
        let bytes = unsafe {
            std::slice::from_raw_parts(chunk.data.as_ptr() as *const u8, chunk.index * 8)
        };
        writer.write_all(bytes)
    }

    // Write header to writer
    pub fn write_header(&self, writer: &mut impl Write) -> io::Result<()> {
        // HUFF magic
        writer.write_all(b"HUFF")?;

        // Version number
        writer.write_all(&VERSION.to_be_bytes())?;

        // Number of (byte, freq) pairs
        writer.write_all(&self.unique_bytes.to_be_bytes())?;

        // Byte frequency pairs
        for (byte, &freq) in self.freqs.iter().enumerate() {
            if freq != 0 {
                writer.write_all(&(byte as u8).to_be_bytes())?;
                writer.write_all(&(freq as u32).to_be_bytes())?;
            }
        }

        Ok(())
    }

    // Generate codes
    #[hotpath::measure]
    fn get_codes(tree: &Node) -> [(u32, u8); 256] {
        fn recurse(node: &Node, prefix: u32, depth: u8, codes: &mut [(u32, u8)]) {
            if let Some(char) = node.byte {
                codes[char as usize] = (prefix, depth);
                return;
            }

            if let Some(left) = &node.left {
                recurse(left, prefix, depth + 1, codes);
            }

            if let Some(right) = &node.right {
                recurse(right, prefix | 1u32 << (31 - depth), depth + 1, codes);
            }
        }

        let mut codes = [(0, 0); 256];

        recurse(tree, 0, 0, &mut codes);

        codes
    }

    fn read_chunk(reader: &mut impl Read, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut total = 0;
        while total < buf.len() {
            match reader.read(&mut buf[total..])? {
                0 => break,
                n => total += n,
            }
        }
        Ok(total)
    }

    pub fn encode_all(&self, reader: &mut impl Read, writer: &mut impl Write) -> io::Result<()> {
        let mut in_buf = vec![0u8; CHUNK_SIZE];
        let mut out_buf = BitData::new();
        loop {
            let len = Self::read_chunk(reader, &mut in_buf)?;
            if len == 0 { break; }
            self.encode_chunk(&in_buf[..len], &mut out_buf);
            Self::write_chunk(writer, &mut out_buf)?;
        }
        Ok(())
    }

    pub fn encode_parallel(&self, mut reader: impl Read + Send, writer: impl Write + Send) -> io::Result<()> {
        use std::sync::mpsc;
        use std::collections::BTreeMap;

        let n = rayon::current_num_threads();
        let lookup = self.lookup;

        // Round-robin work channels: reader -> workers
        let (work_txs, work_rxs): (Vec<_>, Vec<_>) = (0..n)
            .map(|_| mpsc::sync_channel::<(usize, Vec<u8>, usize)>(1))
            .unzip();

        // Input buffer pool: n buffers circulating between reader and workers
        let (in_pool_tx, in_pool_rx) = mpsc::sync_channel::<Vec<u8>>(n);
        for _ in 0..n {
            in_pool_tx.send(vec![0u8; CHUNK_SIZE]).unwrap();
        }

        // Per-worker BitData pool, pre-populated with 2 buffers each
        let (return_txs, return_rxs): (Vec<_>, Vec<_>) = (0..n)
            .map(|_| {
                let (tx, rx) = mpsc::sync_channel::<Box<BitData>>(2);
                tx.send(Box::new(BitData::new())).unwrap();
                tx.send(Box::new(BitData::new())).unwrap();
                (tx, rx)
            })
            .unzip();

        // Workers -> writer
        let (result_tx, result_rx) = mpsc::sync_channel::<(usize, Box<BitData>)>(2 * n);

        std::thread::scope(|ts| {
            let reader_h = ts.spawn(move || -> io::Result<()> {
                let mut idx = 0;
                loop {
                    let mut buf = in_pool_rx.recv().unwrap();
                    let len = Self::read_chunk(&mut reader, &mut buf)?;
                    if len == 0 { break; }
                    if work_txs[idx % n].send((idx, buf, len)).is_err() { break; }
                    idx += 1;
                }
                Ok(())
            });

            let worker_handles: Vec<_> = work_rxs.into_iter()
                .zip(return_rxs)
                .map(|(work_rx, return_rx)| {
                    let result_tx = result_tx.clone();
                    let in_pool_tx = in_pool_tx.clone();
                    ts.spawn(move || {
                        while let Ok((idx, buf, len)) = work_rx.recv() {
                            let mut out = return_rx.recv().unwrap();
                            out.reset();
                            for &byte in &buf[..len] {
                                let (code, code_len) = lookup[byte as usize];
                                out.write(code, code_len);
                            }
                            let _ = in_pool_tx.send(buf);
                            if result_tx.send((idx, out)).is_err() { break; }
                        }
                    })
                })
                .collect();
            drop(result_tx);

            let writer_h = ts.spawn(move || -> io::Result<()> {
                let mut writer = writer;
                let mut pending = BTreeMap::<usize, Box<BitData>>::new();
                let mut next = 0;
                while let Ok((idx, data)) = result_rx.recv() {
                    pending.insert(idx, data);
                    while let Some(mut chunk) = pending.remove(&next) {
                        Self::write_chunk(&mut writer, &mut chunk)?;
                        let _ = return_txs[next % n].send(chunk);
                        next += 1;
                    }
                }
                writer.flush()
            });

            reader_h.join().unwrap()?;
            for h in worker_handles { h.join().unwrap(); }
            writer_h.join().unwrap()?;
            Ok(())
        })
    }
}

pub struct HuffDecoder {
    tree: Box<Node>,
}

impl HuffDecoder {
    // Create a HuffDecoder from file headers and decode file
    #[hotpath::measure]
    pub fn decode_file(path: impl AsRef<Path>) -> io::Result<(Self, Vec<u8>)> {
        let start = Instant::now();
        let mut reader = BufReader::new(File::open(path)?);

        // 1. Validate "HUFF" header
        let mut header = [0u8; 4];
        reader.read_exact(&mut header)?;
        if &header != b"HUFF" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid HUFF header",
            ));
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

        let tree = queue.build_tree();

        println!("read offset: {}", offset);

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let decoded = Self::decode_with_tree(&tree, &buffer, offset.into());

        crate::print_throughput("decoding throughput", decoded.len(), start.elapsed());
        Ok((Self { tree }, decoded))
    }

    // Decode data based on tree tree (no reader)
    #[hotpath::measure]
    pub fn decode_with_tree(tree: &Node, data: &[u8], offset: usize) -> Vec<u8> {
        let mut decoded: Vec<u8> = Vec::new();
        let mut head = tree;
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
                    head = tree;
                }
            }
            
            else {
                // Decoding 0, move head to right Node
                head = head.left.as_ref().unwrap();

                // Found a leaf
                if let Some(byte) = &head.byte {
                    decoded.push(*byte);
                    head = tree;
                }
            }
        }

        decoded
    }
}

impl fmt::Display for HuffEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, &(code, len)) in self.lookup.iter().enumerate() {
            if len != 0 {
                writeln!(
                    f,
                    "'{}' {:0n$b}",
                    index as u8 as char,
                    code >> (32 - len),
                    n = len as usize
                )?;
            }
        }
        Ok(())
    }
}
