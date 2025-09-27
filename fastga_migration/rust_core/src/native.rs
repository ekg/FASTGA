// Pure Rust implementations of FASTGA functions

// ============================================================================
// SEQUENCE ENCODING FUNCTIONS
// ============================================================================

pub fn base_to_num(base: u8) -> u8 {
    match base {
        b'A' | b'a' => 0,
        b'C' | b'c' => 1,
        b'G' | b'g' => 2,
        b'T' | b't' => 3,
        _ => 0, // Default to A for ambiguous bases
    }
}

pub fn num_to_base(num: u8) -> u8 {
    match num & 3 {
        0 => b'A',
        1 => b'C',
        2 => b'G',
        3 => b'T',
        _ => unreachable!(),
    }
}

pub fn encode_2bit(sequence: &str) -> Vec<u8> {
    let length = sequence.len();
    let encoded_len = (length + 3) / 4;
    let mut encoded = vec![0u8; encoded_len];

    for (i, base) in sequence.bytes().enumerate() {
        let num = base_to_num(base);
        let byte_idx = i / 4;
        let bit_offset = (i % 4) * 2;
        encoded[byte_idx] |= num << (6 - bit_offset);
    }

    encoded
}

pub fn decode_2bit(encoded: &[u8], seq_len: usize) -> String {
    let mut sequence = String::with_capacity(seq_len);

    for i in 0..seq_len {
        let byte_idx = i / 4;
        let bit_offset = (i % 4) * 2;
        let base_num = (encoded[byte_idx] >> (6 - bit_offset)) & 0x03;
        sequence.push(num_to_base(base_num) as char);
    }

    sequence
}

// ============================================================================
// K-MER OPERATIONS
// ============================================================================

pub fn encode_kmer(seq: &[u8], k: usize) -> u64 {
    let mut kmer = 0u64;
    for i in 0..k {
        kmer = (kmer << 2) | (seq[i] as u64 & 3);
    }
    kmer
}

pub fn decode_kmer(kmer: u64, k: usize) -> Vec<u8> {
    let mut out = vec![0u8; k];
    let mut kmer_copy = kmer;

    for i in (0..k).rev() {
        out[i] = (kmer_copy & 3) as u8;
        kmer_copy >>= 2;
    }

    out
}

pub fn kmer_reverse_complement(kmer: u64, k: usize) -> u64 {
    let mut rc = 0u64;
    let mut kmer_copy = kmer;

    for _ in 0..k {
        let base = kmer_copy & 3;
        let comp = 3 - base; // A<->T (0<->3), C<->G (1<->2)
        rc = (rc << 2) | comp;
        kmer_copy >>= 2;
    }

    rc
}

pub fn hash_kmer(kmer: u64) -> u64 {
    // Exact same hash function as C version
    let mut k = kmer;
    k ^= k >> 33;
    k = k.wrapping_mul(0xff51afd7ed558ccdu64);
    k ^= k >> 33;
    k = k.wrapping_mul(0xc4ceb9fe1a85ec53u64);
    k ^= k >> 33;
    k
}

// ============================================================================
// BASIC ALIGNMENT SCORING
// ============================================================================

pub fn score_match(a: u8, b: u8) -> i32 {
    if a == b { 1 } else { -1 }
}

pub fn edit_distance(seq1: &[u8], seq2: &[u8]) -> usize {
    let len1 = seq1.len();
    let len2 = seq2.len();

    // Create DP table
    let mut dp = vec![vec![0usize; len2 + 1]; len1 + 1];

    // Initialize first row and column
    for i in 0..=len1 {
        dp[i][0] = i;
    }
    for j in 0..=len2 {
        dp[0][j] = j;
    }

    // Fill DP table
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if seq1[i - 1] == seq2[j - 1] { 0 } else { 1 };
            dp[i][j] = std::cmp::min(
                std::cmp::min(
                    dp[i - 1][j - 1] + cost, // Replace
                    dp[i][j - 1] + 1,        // Insert
                ),
                dp[i - 1][j] + 1,            // Delete
            );
        }
    }

    dp[len1][len2]
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

pub fn gc_content(seq: &[u8]) -> f64 {
    if seq.is_empty() {
        return 0.0;
    }

    let gc_count = seq.iter()
        .filter(|&&base| base == 1 || base == 2) // C or G
        .count();

    gc_count as f64 / seq.len() as f64
}

pub fn count_bases(seq: &[u8]) -> (u64, u64, u64, u64) {
    let mut count_a = 0u64;
    let mut count_c = 0u64;
    let mut count_g = 0u64;
    let mut count_t = 0u64;

    for &base in seq {
        match base & 3 {
            0 => count_a += 1,
            1 => count_c += 1,
            2 => count_g += 1,
            3 => count_t += 1,
            _ => unreachable!(),
        }
    }

    (count_a, count_c, count_g, count_t)
}

pub fn reverse_complement_seq(seq: &mut [u8]) {
    let len = seq.len();
    for i in 0..len / 2 {
        let j = len - 1 - i;
        let temp = 3 - seq[i];  // Complement
        seq[i] = 3 - seq[j];    // Complement
        seq[j] = temp;
    }

    // Handle middle element for odd-length sequences
    if len % 2 == 1 {
        seq[len / 2] = 3 - seq[len / 2];
    }
}