use crate::{Config, FastGA};
use crate::native;

// Macro for testing both C and Rust implementations
macro_rules! test_both {
    ($name:ident, $test_body:expr) => {
        #[test]
        fn $name() {
            // Test with C implementation
            let c_config = Config::default();
            let c_fastga = FastGA::new(c_config);
            let c_result = $test_body(&c_fastga);

            // Test with Rust implementation
            let mut rust_config = Config::default();
            match stringify!($name) {
                s if s.contains("encode_2bit") => rust_config.use_rust_encode_2bit = true,
                s if s.contains("decode_2bit") => rust_config.use_rust_decode_2bit = true,
                s if s.contains("encode_kmer") => rust_config.use_rust_encode_kmer = true,
                s if s.contains("decode_kmer") => rust_config.use_rust_decode_kmer = true,
                s if s.contains("reverse_complement") && s.contains("kmer") =>
                    rust_config.use_rust_kmer_reverse_complement = true,
                s if s.contains("hash_kmer") => rust_config.use_rust_hash_kmer = true,
                s if s.contains("score_match") => rust_config.use_rust_score_match = true,
                s if s.contains("edit_distance") => rust_config.use_rust_edit_distance = true,
                s if s.contains("gc_content") => rust_config.use_rust_gc_content = true,
                s if s.contains("count_bases") => rust_config.use_rust_count_bases = true,
                s if s.contains("reverse_complement_seq") =>
                    rust_config.use_rust_reverse_complement_seq = true,
                _ => {}
            }
            let rust_fastga = FastGA::new(rust_config);
            let rust_result = $test_body(&rust_fastga);

            assert_eq!(c_result, rust_result,
                "C and Rust implementations differ for {}:\nC: {:?}\nRust: {:?}",
                stringify!($name), c_result, rust_result
            );
        }
    };
}

// ============================================================================
// SEQUENCE ENCODING TESTS
// ============================================================================

test_both!(test_encode_2bit_simple, |fastga: &FastGA| {
    fastga.encode_2bit("ACGT")
});

test_both!(test_encode_2bit_longer, |fastga: &FastGA| {
    fastga.encode_2bit("ACGTACGTACGTACGT")
});

test_both!(test_encode_2bit_all_same, |fastga: &FastGA| {
    fastga.encode_2bit("AAAAAAAA")
});

test_both!(test_encode_decode_2bit_roundtrip, |fastga: &FastGA| {
    let original = "ACGTACGTACGT";
    let encoded = fastga.encode_2bit(original);
    fastga.decode_2bit(&encoded, original.len())
});

// ============================================================================
// K-MER OPERATIONS TESTS
// ============================================================================

test_both!(test_encode_kmer_4bp, |fastga: &FastGA| {
    let seq = vec![0, 1, 2, 3]; // ACGT
    fastga.encode_kmer(&seq, 4)
});

test_both!(test_encode_kmer_8bp, |fastga: &FastGA| {
    let seq = vec![0, 1, 2, 3, 0, 1, 2, 3]; // ACGTACGT
    fastga.encode_kmer(&seq, 8)
});

test_both!(test_decode_kmer_4bp, |fastga: &FastGA| {
    let kmer = 0b00011011u64; // ACGT
    fastga.decode_kmer(kmer, 4)
});

test_both!(test_kmer_reverse_complement_4bp, |fastga: &FastGA| {
    let kmer = 0b00011011u64; // ACGT -> ACGT (rc)
    fastga.kmer_reverse_complement(kmer, 4)
});

test_both!(test_hash_kmer, |fastga: &FastGA| {
    let kmer = 0x123456789ABCDEF0u64;
    fastga.hash_kmer(kmer)
});

// ============================================================================
// ALIGNMENT SCORING TESTS
// ============================================================================

test_both!(test_score_match_same, |fastga: &FastGA| {
    fastga.score_match(0, 0) // A vs A
});

test_both!(test_score_match_diff, |fastga: &FastGA| {
    fastga.score_match(0, 1) // A vs C
});

test_both!(test_edit_distance_same, |fastga: &FastGA| {
    let seq1 = vec![0, 1, 2, 3]; // ACGT
    let seq2 = vec![0, 1, 2, 3]; // ACGT
    fastga.edit_distance(&seq1, &seq2)
});

test_both!(test_edit_distance_one_diff, |fastga: &FastGA| {
    let seq1 = vec![0, 1, 2, 3]; // ACGT
    let seq2 = vec![0, 1, 2, 0]; // ACGA
    fastga.edit_distance(&seq1, &seq2)
});

test_both!(test_edit_distance_different_lengths, |fastga: &FastGA| {
    let seq1 = vec![0, 1, 2, 3]; // ACGT
    let seq2 = vec![0, 1, 2];    // ACG
    fastga.edit_distance(&seq1, &seq2)
});

// ============================================================================
// UTILITY FUNCTIONS TESTS
// ============================================================================

test_both!(test_gc_content_half, |fastga: &FastGA| {
    let seq = vec![0, 1, 2, 3]; // ACGT - 50% GC
    let gc = fastga.gc_content(&seq);
    (gc * 1000.0).round() as u64 // Convert to integer for exact comparison
});

test_both!(test_gc_content_all_gc, |fastga: &FastGA| {
    let seq = vec![1, 2, 1, 2]; // CGCG - 100% GC
    let gc = fastga.gc_content(&seq);
    (gc * 1000.0).round() as u64
});

test_both!(test_count_bases, |fastga: &FastGA| {
    let seq = vec![0, 0, 1, 1, 2, 2, 3, 3]; // AACCGGTT
    fastga.count_bases(&seq)
});

test_both!(test_reverse_complement_seq, |fastga: &FastGA| {
    let mut seq = vec![0, 1, 2, 3]; // ACGT -> ACGT (rc)
    fastga.reverse_complement_seq(&mut seq);
    seq
});

// ============================================================================
// COMPREHENSIVE INTEGRATION TESTS
// ============================================================================

#[test]
fn test_encode_decode_all_functions() {
    let original_seq = "ACGTACGTACGTACGT";

    // Test with all C
    let c_config = Config::default();
    let c_fastga = FastGA::new(c_config);

    // Test with all Rust
    let rust_config = Config {
        use_rust_encode_2bit: true,
        use_rust_decode_2bit: true,
        use_rust_encode_kmer: true,
        use_rust_decode_kmer: true,
        use_rust_kmer_reverse_complement: true,
        use_rust_hash_kmer: true,
        use_rust_score_match: true,
        use_rust_edit_distance: true,
        use_rust_gc_content: true,
        use_rust_count_bases: true,
        use_rust_reverse_complement_seq: true,
    };
    let rust_fastga = FastGA::new(rust_config);

    // Encode sequence
    let c_encoded = c_fastga.encode_2bit(original_seq);
    let rust_encoded = rust_fastga.encode_2bit(original_seq);
    assert_eq!(c_encoded, rust_encoded);

    // Decode sequence
    let c_decoded = c_fastga.decode_2bit(&c_encoded, original_seq.len());
    let rust_decoded = rust_fastga.decode_2bit(&rust_encoded, original_seq.len());
    assert_eq!(c_decoded, rust_decoded);
    assert_eq!(c_decoded, original_seq);

    // Test k-mer operations
    let seq_numeric = vec![0, 1, 2, 3, 0, 1, 2, 3]; // ACGTACGT
    let c_kmer = c_fastga.encode_kmer(&seq_numeric, 8);
    let rust_kmer = rust_fastga.encode_kmer(&seq_numeric, 8);
    assert_eq!(c_kmer, rust_kmer);

    let c_rc = c_fastga.kmer_reverse_complement(c_kmer, 8);
    let rust_rc = rust_fastga.kmer_reverse_complement(rust_kmer, 8);
    assert_eq!(c_rc, rust_rc);
}

// ============================================================================
// PROPERTY-BASED TESTS
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_encode_decode_2bit(s in "[ACGT]{0,100}") {
            let config = Config {
                use_rust_encode_2bit: true,
                use_rust_decode_2bit: true,
                ..Default::default()
            };
            let fastga = FastGA::new(config);

            let encoded = fastga.encode_2bit(&s);
            let decoded = fastga.decode_2bit(&encoded, s.len());
            prop_assert_eq!(decoded, s);
        }

        #[test]
        fn prop_kmer_reverse_complement_involution(kmer: u64, k in 1usize..=32) {
            let config = Config {
                use_rust_kmer_reverse_complement: true,
                ..Default::default()
            };
            let fastga = FastGA::new(config);

            let rc = fastga.kmer_reverse_complement(kmer, k);
            let rc_rc = fastga.kmer_reverse_complement(rc, k);

            // Mask to only compare relevant bits
            let mask = if k == 32 { !0u64 } else { (1u64 << (2 * k)) - 1 };
            prop_assert_eq!(kmer & mask, rc_rc & mask);
        }
    }
}