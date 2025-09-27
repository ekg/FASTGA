# FASTGA C to Rust Migration

## Overview

This directory contains the systematic rewrite of FASTGA from C to Rust. We use an incremental migration approach where C functions are replaced one-by-one with Rust equivalents, with both implementations tested side-by-side to ensure identical behavior.

## Philosophy

Like the Ship of Theseus, we replace each plank (function) of the ship (FASTGA) one at a time. The ship never stops sailing - we always have a working genome aligner throughout the migration process.

## Directory Structure

```
fastga_migration/
├── c_core/              # C library with FFI exports
│   ├── libfastga.h      # C function headers
│   ├── libfastga.c      # C implementations
│   └── Makefile         # Build C shared library
│
├── rust_core/           # Rust implementation
│   ├── src/
│   │   ├── lib.rs       # Main library interface
│   │   ├── ffi.rs       # C FFI bindings
│   │   ├── native.rs    # Pure Rust implementations
│   │   └── tests.rs     # Side-by-side comparison tests
│   ├── build.rs         # Build script for C integration
│   └── Cargo.toml       # Rust project config
│
└── migrate.py           # Migration tracking tool
```

## Quick Start

### 1. Build Everything

```bash
# Build C library
cd c_core
make clean && make

# Build and test Rust
cd ../rust_core
cargo test
```

### 2. Check Migration Status

```bash
cd ..
python3 migrate.py progress
```

Output shows which functions have been migrated:
```
FASTGA Migration Progress
============================================================
Total functions: 11
Migrated:       1 (9.1%)
Tested:         11 (100.0%)

Function                       Status
--------------------------------------------------
encode_2bit                    ✅ Migrated
decode_2bit                    ⏳ Pending
...
```

### 3. Migrate Next Function

```bash
python3 migrate.py next
```

This will:
1. Run tests for the next unmigrated function
2. Verify C and Rust produce identical output
3. Mark the function as migrated
4. Update the configuration

### 4. Run All Tests

```bash
cd rust_core
cargo test
```

All tests compare C and Rust implementations to ensure they produce identical results.

## How It Works

### Dual Implementation

Every function exists in both C and Rust:

**C Implementation** (`c_core/libfastga.c`):
```c
uint64_t fastga_encode_kmer(const uint8_t* seq, int k) {
    uint64_t kmer = 0;
    for (int i = 0; i < k; i++) {
        kmer = (kmer << 2) | (seq[i] & 3);
    }
    return kmer;
}
```

**Rust Implementation** (`rust_core/src/native.rs`):
```rust
pub fn encode_kmer(seq: &[u8], k: usize) -> u64 {
    let mut kmer = 0u64;
    for i in 0..k {
        kmer = (kmer << 2) | (seq[i] as u64 & 3);
    }
    kmer
}
```

### Configuration-Based Switching

The `Config` struct controls which implementation is used:

```rust
pub struct Config {
    pub use_rust_encode_2bit: bool,
    pub use_rust_decode_2bit: bool,
    pub use_rust_encode_kmer: bool,
    // ... etc
}
```

### Test Harness

Every function has tests that verify both implementations produce identical output:

```rust
test_both!(test_encode_kmer_4bp, |fastga: &FastGA| {
    let seq = vec![0, 1, 2, 3]; // ACGT
    fastga.encode_kmer(&seq, 4)
});
```

The `test_both!` macro:
1. Runs the test with C implementation
2. Runs the test with Rust implementation
3. Asserts the results are identical

## Migration Checklist

- [x] Set up C/Rust FFI framework
- [x] Create test harness
- [x] Implement all functions in both C and Rust
- [x] Write comparison tests
- [ ] Migrate each function:
  - [x] encode_2bit
  - [ ] decode_2bit
  - [ ] encode_kmer
  - [ ] decode_kmer
  - [ ] kmer_reverse_complement
  - [ ] hash_kmer
  - [ ] score_match
  - [ ] edit_distance
  - [ ] gc_content
  - [ ] count_bases
  - [ ] reverse_complement_seq
- [ ] Remove C dependencies
- [ ] Optimize Rust implementation
- [ ] Full integration with FASTGA

## Testing Strategy

### Unit Tests
Each function is tested individually with known inputs/outputs.

### Property-Based Tests
Using `proptest` to verify properties hold for random inputs:
```rust
proptest! {
    #[test]
    fn prop_encode_decode_2bit(s in "[ACGT]{0,100}") {
        let encoded = encode_2bit(&s);
        let decoded = decode_2bit(&encoded, s.len());
        prop_assert_eq!(decoded, s);
    }
}
```

### Integration Tests
Full pipeline tests with real genome data to ensure the complete system works.

## Performance

Initial goal: Match C performance
Ultimate goal: Exceed C performance through:
- Better memory layout
- SIMD optimizations
- Parallel processing with Rayon
- Zero-copy operations

## Contributing

1. Pick an unmigrated function from `migrate.py progress`
2. Ensure tests pass for that function
3. Run `migrate.py migrate <function_name>`
4. Commit the change with message: `Migrate <function_name> to Rust`

## Why This Approach?

1. **Never Broken**: System works at every commit
2. **Verifiable**: Every function tested against C original
3. **Incremental**: Can stop/resume migration anytime
4. **Safe**: Rust's memory safety from day one
5. **Measurable**: Clear progress tracking

## Next Steps

After all functions are migrated:
1. Remove C dependencies
2. Integrate with main FASTGA codebase
3. Add Rust-specific optimizations
4. Benchmark against C version
5. Replace C version entirely