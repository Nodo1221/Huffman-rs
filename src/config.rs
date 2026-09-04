// config.rs
pub const CHUNK_SIZE: usize = 128 * 1024;
pub const PAGE_SIZE: usize = CHUNK_SIZE * 3 / 2 / 8; // 1.5x input bytes, in u64 elements