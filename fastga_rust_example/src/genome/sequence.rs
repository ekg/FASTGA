/// Sequence representation with various encoding schemes
#[derive(Debug, Clone)]
pub enum SequenceStore {
    /// Uncompressed ASCII sequences
    Uncompressed(Vec<u8>),
    /// 2-bit compressed sequences (4 bases per byte)
    Compressed(Vec<u8>),
    /// Numeric encoding (0=A, 1=C, 2=G, 3=T)
    Numeric(Vec<u8>),
}

impl SequenceStore {
    /// Extract a sequence segment
    pub fn extract(&self, offset: u64, length: u64) -> Sequence {
        match self {
            SequenceStore::Uncompressed(data) => {
                let start = offset as usize;
                let end = (offset + length) as usize;
                Sequence::from_bytes(&data[start..end])
            }
            SequenceStore::Compressed(data) => {
                // Decompress 2-bit encoding
                let mut seq = Vec::with_capacity(length as usize);
                let start_byte = (offset / 4) as usize;
                let start_bit = ((offset % 4) * 2) as usize;

                let mut byte_idx = start_byte;
                let mut bit_idx = start_bit;

                for _ in 0..length {
                    let byte = data[byte_idx];
                    let base_bits = (byte >> (6 - bit_idx)) & 0b11;
                    let base = match base_bits {
                        0 => b'A',
                        1 => b'C',
                        2 => b'G',
                        3 => b'T',
                        _ => unreachable!(),
                    };
                    seq.push(base);

                    bit_idx += 2;
                    if bit_idx >= 8 {
                        bit_idx = 0;
                        byte_idx += 1;
                    }
                }

                Sequence::from_bytes(&seq)
            }
            SequenceStore::Numeric(data) => {
                let start = offset as usize;
                let end = (offset + length) as usize;
                let numeric = &data[start..end];
                let mut seq = Vec::with_capacity(length as usize);

                for &val in numeric {
                    let base = match val {
                        0 => b'A',
                        1 => b'C',
                        2 => b'G',
                        3 => b'T',
                        _ => b'N',
                    };
                    seq.push(base);
                }

                Sequence::from_bytes(&seq)
            }
        }
    }

    /// Compress sequences to 2-bit encoding
    pub fn compress(&mut self) {
        if let SequenceStore::Uncompressed(data) = self {
            let mut compressed = Vec::with_capacity((data.len() + 3) / 4);
            let mut current_byte = 0u8;
            let mut bit_pos = 0;

            for &base in data.iter() {
                let bits = match base {
                    b'A' | b'a' => 0b00,
                    b'C' | b'c' => 0b01,
                    b'G' | b'g' => 0b10,
                    b'T' | b't' => 0b11,
                    _ => 0b00, // Default to A for ambiguous bases
                };

                current_byte |= bits << (6 - bit_pos);
                bit_pos += 2;

                if bit_pos == 8 {
                    compressed.push(current_byte);
                    current_byte = 0;
                    bit_pos = 0;
                }
            }

            if bit_pos > 0 {
                compressed.push(current_byte);
            }

            *self = SequenceStore::Compressed(compressed);
        }
    }
}

/// Individual sequence
#[derive(Debug, Clone)]
pub struct Sequence {
    pub data: Vec<u8>,
}

impl Sequence {
    pub fn from_bytes(data: &[u8]) -> Self {
        Sequence {
            data: data.to_vec(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get reverse complement
    pub fn reverse_complement(&self) -> Self {
        let mut rc = Vec::with_capacity(self.data.len());

        for &base in self.data.iter().rev() {
            let comp = match base {
                b'A' | b'a' => b'T',
                b'C' | b'c' => b'G',
                b'G' | b'g' => b'C',
                b'T' | b't' => b'A',
                b'N' | b'n' => b'N',
                _ => base,
            };
            rc.push(comp);
        }

        Sequence { data: rc }
    }

    /// Convert to numeric encoding
    pub fn to_numeric(&self) -> Vec<u8> {
        self.data.iter().map(|&base| {
            match base {
                b'A' | b'a' => 0,
                b'C' | b'c' => 1,
                b'G' | b'g' => 2,
                b'T' | b't' => 3,
                _ => 4, // N or ambiguous
            }
        }).collect()
    }
}