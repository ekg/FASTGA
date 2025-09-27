use std::fmt;

/// Type-safe k-mer representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Kmer {
    pub encoded: u64,
    pub size: u8,
}

impl Kmer {
    /// Create a new k-mer from encoded value
    pub fn new(encoded: u64, size: u8) -> Self {
        debug_assert!(size <= 32, "K-mer size must be <= 32");
        Kmer { encoded, size }
    }

    /// Create from sequence slice
    pub fn from_slice(seq: &[u8]) -> Option<Self> {
        if seq.len() > 32 {
            return None;
        }

        let mut encoded = 0u64;
        for &base in seq {
            if base > 3 {
                return None; // Ambiguous base
            }
            encoded = (encoded << 2) | (base as u64);
        }

        Some(Kmer {
            encoded,
            size: seq.len() as u8,
        })
    }

    /// Get reverse complement
    pub fn reverse_complement(&self) -> Self {
        let mut rc = 0u64;
        let mut encoded = self.encoded;

        for _ in 0..self.size {
            let base = encoded & 0b11;
            let comp = 3 - base; // A<->T, C<->G
            rc = (rc << 2) | comp;
            encoded >>= 2;
        }

        Kmer {
            encoded: rc,
            size: self.size,
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.size as usize);
        let mut encoded = self.encoded;

        for _ in 0..self.size {
            let base = ((encoded >> ((self.size - 1) * 2)) & 0b11) as u8;
            let c = match base {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                3 => 'T',
                _ => 'N',
            };
            s.push(c);
            encoded <<= 2;
        }

        s
    }
}

impl fmt::Display for Kmer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}