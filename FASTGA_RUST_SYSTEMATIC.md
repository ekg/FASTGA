# FASTGA Ship of Theseus - Systematic C to Rust Migration

## Strategy: Test-Driven Incremental Replacement

Replace FASTGA piece by piece, testing each function against the C implementation to ensure identical behavior at every step.

## Architecture

```
fastga_migration/
├── c_core/              # C code wrapped for testing
│   ├── libfastga.c      # Consolidated C functions
│   ├── libfastga.h      # C headers
│   └── Makefile         # Build C shared library
├── rust_core/           # Rust implementation
│   ├── Cargo.toml
│   ├── build.rs         # FFI binding generation
│   └── src/
│       ├── lib.rs       # Rust library
│       ├── ffi.rs       # C FFI bindings
│       └── native.rs    # Pure Rust implementations
├── tests/               # Comparison tests
│   ├── harness.rs       # Test framework
│   ├── test_data/       # Common test inputs
│   └── verify.rs        # Verification utilities
└── migrate.py           # Migration progress tracker
```

## Phase 1: Test Harness Setup

### Step 1.1: Create FFI-friendly C library
```c
// c_core/libfastga.h
#ifndef LIBFASTGA_H
#define LIBFASTGA_H

#include <stdint.h>

// Start with core functions that are easy to test

// Sequence encoding functions
uint8_t* encode_2bit(const char* sequence, size_t length);
char* decode_2bit(const uint8_t* encoded, size_t length);

// K-mer operations
uint64_t encode_kmer(const uint8_t* seq, int k);
void decode_kmer(uint64_t kmer, int k, uint8_t* out);
uint64_t kmer_reverse_complement(uint64_t kmer, int k);

// Basic GDB operations
typedef struct {
    int ncontig;
    int64_t* contig_lens;
    char** contig_names;
    uint8_t* sequences;  // 2-bit encoded
} SimpleGDB;

SimpleGDB* load_gdb(const char* path);
void free_gdb(SimpleGDB* gdb);
uint8_t* get_contig_seq(SimpleGDB* gdb, int contig_id);

// Alignment scoring
int score_match(uint8_t a, uint8_t b);
int score_alignment(const uint8_t* seq1, const uint8_t* seq2,
                   int len1, int len2, int* path);

#endif
```

### Step 1.2: Rust FFI bindings
```rust
// rust_core/src/ffi.rs
use std::os::raw::{c_char, c_int};
use std::ffi::{CStr, CString};

#[repr(C)]
pub struct SimpleGDB {
    pub ncontig: c_int,
    pub contig_lens: *mut i64,
    pub contig_names: *mut *mut c_char,
    pub sequences: *mut u8,
}

#[link(name = "fastga")]
extern "C" {
    // Sequence encoding
    pub fn encode_2bit(sequence: *const c_char, length: usize) -> *mut u8;
    pub fn decode_2bit(encoded: *const u8, length: usize) -> *mut c_char;

    // K-mer operations
    pub fn encode_kmer(seq: *const u8, k: c_int) -> u64;
    pub fn decode_kmer(kmer: u64, k: c_int, out: *mut u8);
    pub fn kmer_reverse_complement(kmer: u64, k: c_int) -> u64;

    // GDB operations
    pub fn load_gdb(path: *const c_char) -> *mut SimpleGDB;
    pub fn free_gdb(gdb: *mut SimpleGDB);
    pub fn get_contig_seq(gdb: *mut SimpleGDB, contig_id: c_int) -> *mut u8;

    // Alignment
    pub fn score_match(a: u8, b: u8) -> c_int;
    pub fn score_alignment(
        seq1: *const u8, seq2: *const u8,
        len1: c_int, len2: c_int,
        path: *mut c_int
    ) -> c_int;
}
```

### Step 1.3: Test harness
```rust
// tests/harness.rs
use rust_core::{ffi, native};
use std::ffi::CString;

macro_rules! test_both {
    ($name:ident, $test_fn:expr) => {
        #[test]
        fn $name() {
            println!("Testing {}", stringify!($name));

            let c_result = unsafe { $test_fn(ffi::$name) };
            let rust_result = $test_fn(native::$name);

            assert_eq!(c_result, rust_result,
                "C and Rust implementations differ for {}:\nC: {:?}\nRust: {:?}",
                stringify!($name), c_result, rust_result
            );
        }
    };
}

// Test sequence encoding
test_both!(encode_2bit, |encode_fn| {
    let seq = "ACGTACGT";
    encode_fn(seq)
});

test_both!(kmer_reverse_complement, |rc_fn| {
    let kmer = 0x1234;
    let k = 8;
    rc_fn(kmer, k)
});
```

## Phase 2: Function Migration Order

### Priority 1: Pure Functions (no side effects, easy to test)
1. `encode_2bit` / `decode_2bit` - Sequence compression
2. `encode_kmer` / `decode_kmer` - K-mer encoding
3. `kmer_reverse_complement` - K-mer operations
4. `hash_kmer` - K-mer hashing
5. `score_match` - Alignment scoring

### Priority 2: Data Structure Operations
1. `SimpleGDB` - Basic genome database
2. `KmerIndex` - K-mer index structure
3. `Alignment` - Alignment representation
4. `Path` - Traceback path

### Priority 3: Algorithms
1. `find_seeds` - Seed matching
2. `chain_seeds` - Seed chaining
3. `extend_alignment` - Local alignment
4. `compute_traceback` - Traceback

### Priority 4: I/O
1. `read_fasta` - FASTA parsing
2. `write_paf` - PAF output
3. `load_gdb` / `save_gdb` - GDB format

## Phase 3: Migration Process

### For each function:

1. **Extract from C codebase**
```c
// c_core/libfastga.c
uint64_t encode_kmer(const uint8_t* seq, int k) {
    // Original C implementation
    uint64_t kmer = 0;
    for (int i = 0; i < k; i++) {
        kmer = (kmer << 2) | seq[i];
    }
    return kmer;
}
```

2. **Write Rust equivalent**
```rust
// rust_core/src/native.rs
pub fn encode_kmer(seq: &[u8], k: usize) -> u64 {
    // Rust implementation
    let mut kmer = 0u64;
    for i in 0..k {
        kmer = (kmer << 2) | (seq[i] as u64);
    }
    kmer
}
```

3. **Create comparison test**
```rust
// tests/test_kmer.rs
#[test]
fn test_encode_kmer_equivalence() {
    let test_cases = vec![
        (b"ACGT", 4),
        (b"AAAAAAAA", 8),
        (b"TTTTTTTTTTTTTTT", 15),
        (b"ACGTACGTACGTACGT", 16),
    ];

    for (seq, k) in test_cases {
        let c_result = unsafe {
            ffi::encode_kmer(seq.as_ptr(), k as c_int)
        };
        let rust_result = native::encode_kmer(seq, k);

        assert_eq!(c_result, rust_result,
            "encode_kmer mismatch for seq={:?}, k={}",
            seq, k
        );
    }
}
```

4. **Property-based testing**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_encode_decode_kmer(seq in "[0-3]{1,32}") {
        let bytes: Vec<u8> = seq.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let k = bytes.len();

        let c_encoded = unsafe {
            ffi::encode_kmer(bytes.as_ptr(), k as c_int)
        };
        let rust_encoded = native::encode_kmer(&bytes, k);

        prop_assert_eq!(c_encoded, rust_encoded);

        // Test decode
        let mut c_decoded = vec![0u8; k];
        let mut rust_decoded = vec![0u8; k];

        unsafe {
            ffi::decode_kmer(c_encoded, k as c_int, c_decoded.as_mut_ptr());
        }
        native::decode_kmer(rust_encoded, k, &mut rust_decoded);

        prop_assert_eq!(c_decoded, rust_decoded);
        prop_assert_eq!(bytes, rust_decoded);
    }
}
```

## Phase 4: Integration Testing

### End-to-end test with real genomes
```rust
// tests/integration.rs
#[test]
fn test_full_alignment_pipeline() {
    // Use the actual test genomes
    let target = "aEleCoq1.fa.gz";
    let query = "rDibSmi1.fa.gz";

    // Run C version
    let c_output = run_c_fastga(target, query);

    // Run Rust version (using mix of C and Rust functions)
    let rust_output = run_rust_fastga(target, query);

    // Compare outputs
    assert_paf_equivalent(&c_output, &rust_output);
}

fn assert_paf_equivalent(c_paf: &str, rust_paf: &str) {
    let c_alignments = parse_paf(c_paf);
    let rust_alignments = parse_paf(rust_paf);

    assert_eq!(c_alignments.len(), rust_alignments.len());

    for (c_aln, rust_aln) in c_alignments.iter().zip(rust_alignments.iter()) {
        // Allow small differences in scores due to floating point
        assert!((c_aln.score - rust_aln.score).abs() < 0.001);
        assert_eq!(c_aln.target_start, rust_aln.target_start);
        assert_eq!(c_aln.target_end, rust_aln.target_end);
        assert_eq!(c_aln.query_start, rust_aln.query_start);
        assert_eq!(c_aln.query_end, rust_aln.query_end);
    }
}
```

## Phase 5: Progressive Replacement

### Migration tracker
```python
# migrate.py
import json
import subprocess

class MigrationTracker:
    def __init__(self):
        self.functions = self.load_function_list()
        self.status = self.load_status()

    def check_function(self, name):
        """Run tests for a specific function"""
        result = subprocess.run(
            ["cargo", "test", f"test_{name}"],
            capture_output=True
        )
        return result.returncode == 0

    def migrate_function(self, name):
        """Switch from C to Rust implementation"""
        # Update configuration to use Rust version
        config = {
            "use_rust": self.status["use_rust"] + [name]
        }

        # Run all tests
        if self.run_all_tests():
            self.status["migrated"].append(name)
            print(f"✓ Successfully migrated {name}")
        else:
            print(f"✗ Migration of {name} failed tests")
            return False

        return True

    def progress(self):
        total = len(self.functions)
        migrated = len(self.status["migrated"])
        print(f"Migration progress: {migrated}/{total} ({100*migrated/total:.1f}%)")
```

## Phase 6: Continuous Verification

### GitHub Actions CI
```yaml
# .github/workflows/test.yml
name: Test C/Rust Equivalence

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2

    - name: Build C library
      run: |
        cd c_core
        make

    - name: Build Rust
      run: |
        cd rust_core
        cargo build --release

    - name: Run equivalence tests
      run: |
        cargo test --all

    - name: Run integration test
      run: |
        ./test_integration.sh

    - name: Check migration progress
      run: |
        python migrate.py progress
```

## Timeline (Realistic 1-Day Sprint)

### Hour 0-1: Setup
- Create project structure
- Set up C library with FFI exports
- Basic Rust FFI bindings

### Hour 1-3: Core Functions
- Migrate sequence encoding
- Migrate k-mer operations
- Verify with tests

### Hour 3-5: Data Structures
- Port GDB structure
- Port k-mer index
- Integration tests

### Hour 5-7: Algorithms
- Seed finding
- Chaining
- Keep using C alignment initially

### Hour 7-8: Validation
- Run on test genomes
- Compare outputs
- Performance check

## Key Principles

1. **Never break the build** - Mixed C/Rust must always work
2. **Test every function** - Both unit and property tests
3. **Verify equivalence** - Bit-for-bit identical results
4. **Progress tracking** - Know exactly what's migrated
5. **Incremental delivery** - Ship working code continuously

This systematic approach ensures we maintain a working aligner at every step while progressively replacing C with Rust!