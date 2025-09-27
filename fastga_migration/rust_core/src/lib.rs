pub mod ffi;
pub mod native;

#[cfg(test)]
mod tests;

// Configuration to control which implementations to use
pub struct Config {
    pub use_rust_encode_2bit: bool,
    pub use_rust_decode_2bit: bool,
    pub use_rust_encode_kmer: bool,
    pub use_rust_decode_kmer: bool,
    pub use_rust_kmer_reverse_complement: bool,
    pub use_rust_hash_kmer: bool,
    pub use_rust_score_match: bool,
    pub use_rust_edit_distance: bool,
    pub use_rust_gc_content: bool,
    pub use_rust_count_bases: bool,
    pub use_rust_reverse_complement_seq: bool,
}

impl Default for Config {
    fn default() -> Self {
        // Start with all C implementations
        Config {
            use_rust_encode_2bit: false,
            use_rust_decode_2bit: false,
            use_rust_encode_kmer: false,
            use_rust_decode_kmer: false,
            use_rust_kmer_reverse_complement: false,
            use_rust_hash_kmer: false,
            use_rust_score_match: false,
            use_rust_edit_distance: false,
            use_rust_gc_content: false,
            use_rust_count_bases: false,
            use_rust_reverse_complement_seq: false,
        }
    }
}

// Unified interface that can call either C or Rust
pub struct FastGA {
    config: Config,
}

impl FastGA {
    pub fn new(config: Config) -> Self {
        FastGA { config }
    }

    pub fn encode_2bit(&self, sequence: &str) -> Vec<u8> {
        if self.config.use_rust_encode_2bit {
            native::encode_2bit(sequence)
        } else {
            unsafe { ffi::encode_2bit_wrapper(sequence) }
        }
    }

    pub fn decode_2bit(&self, encoded: &[u8], seq_len: usize) -> String {
        if self.config.use_rust_decode_2bit {
            native::decode_2bit(encoded, seq_len)
        } else {
            unsafe { ffi::decode_2bit_wrapper(encoded, seq_len) }
        }
    }

    pub fn encode_kmer(&self, seq: &[u8], k: usize) -> u64 {
        if self.config.use_rust_encode_kmer {
            native::encode_kmer(seq, k)
        } else {
            unsafe { ffi::encode_kmer_wrapper(seq, k) }
        }
    }

    pub fn decode_kmer(&self, kmer: u64, k: usize) -> Vec<u8> {
        if self.config.use_rust_decode_kmer {
            native::decode_kmer(kmer, k)
        } else {
            unsafe { ffi::decode_kmer_wrapper(kmer, k) }
        }
    }

    pub fn kmer_reverse_complement(&self, kmer: u64, k: usize) -> u64 {
        if self.config.use_rust_kmer_reverse_complement {
            native::kmer_reverse_complement(kmer, k)
        } else {
            unsafe { ffi::kmer_reverse_complement_wrapper(kmer, k) }
        }
    }

    pub fn hash_kmer(&self, kmer: u64) -> u64 {
        if self.config.use_rust_hash_kmer {
            native::hash_kmer(kmer)
        } else {
            unsafe { ffi::hash_kmer_wrapper(kmer) }
        }
    }

    pub fn score_match(&self, a: u8, b: u8) -> i32 {
        if self.config.use_rust_score_match {
            native::score_match(a, b)
        } else {
            unsafe { ffi::score_match_wrapper(a, b) }
        }
    }

    pub fn edit_distance(&self, seq1: &[u8], seq2: &[u8]) -> usize {
        if self.config.use_rust_edit_distance {
            native::edit_distance(seq1, seq2)
        } else {
            unsafe { ffi::edit_distance_wrapper(seq1, seq2) }
        }
    }

    pub fn gc_content(&self, seq: &[u8]) -> f64 {
        if self.config.use_rust_gc_content {
            native::gc_content(seq)
        } else {
            unsafe { ffi::gc_content_wrapper(seq) }
        }
    }

    pub fn count_bases(&self, seq: &[u8]) -> (u64, u64, u64, u64) {
        if self.config.use_rust_count_bases {
            native::count_bases(seq)
        } else {
            unsafe { ffi::count_bases_wrapper(seq) }
        }
    }

    pub fn reverse_complement_seq(&self, seq: &mut [u8]) {
        if self.config.use_rust_reverse_complement_seq {
            native::reverse_complement_seq(seq)
        } else {
            unsafe { ffi::reverse_complement_seq_wrapper(seq) }
        }
    }
}