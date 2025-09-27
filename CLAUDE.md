# FASTGA Rust Rewrite Project

## Current Status: Systematic C to Rust Migration

We're rewriting FASTGA in Rust using an incremental "Ship of Theseus" migration strategy where we replace C functions one-by-one while maintaining a working system at all times. Both implementations are tested side-by-side to ensure identical behavior.

### Branch: `rewrite`

All migration work is happening on the `rewrite` branch:
```bash
git checkout rewrite
```

### Migration Framework Location
```
fastga_migration/
├── c_core/          # C functions exported via FFI
│   ├── libfastga.h  # FFI-friendly function signatures
│   ├── libfastga.c  # C implementations
│   └── Makefile     # Build shared library
├── rust_core/       # Rust implementations
│   ├── src/
│   │   ├── ffi.rs   # C FFI bindings
│   │   ├── native.rs # Pure Rust implementations
│   │   ├── tests.rs  # Side-by-side comparison tests
│   │   └── lib.rs    # Main library with Config switching
│   ├── build.rs     # Build script for C integration
│   └── Cargo.toml   # Rust dependencies
└── migrate.py       # Migration tracker script
```

### How It Works

1. **Dual Implementation**: Every function exists in both C and Rust
2. **Configuration Switching**: A `Config` struct controls which implementation is used
3. **Test Harness**: The `test_both!` macro runs tests with both C and Rust, asserting identical results
4. **Incremental Migration**: Functions are migrated one at a time, with full testing at each step

### How to Use

1. **Initial Setup:**
   ```bash
   cd fastga_migration

   # Build C library
   cd c_core
   make clean && make

   # Build and test Rust
   cd ../rust_core
   cargo test
   ```

2. **Check migration progress:**
   ```bash
   cd fastga_migration
   python3 migrate.py progress
   ```

   Output shows:
   ```
   FASTGA Migration Progress
   ============================================================
   Total functions: 11
   Migrated:       1 (9.1%)

   Function                       Status
   --------------------------------------------------
   encode_2bit                    ✅ Migrated
   decode_2bit                    ⏳ Pending
   ...
   ```

3. **Migrate next function:**
   ```bash
   python3 migrate.py next
   ```

   This will:
   - Run tests for the next unmigrated function
   - Verify C and Rust produce identical output
   - Mark function as migrated if tests pass
   - Update configuration to use Rust version

4. **Run all tests:**
   ```bash
   cd rust_core
   cargo test
   ```

   All 23 tests compare C and Rust implementations

5. **Test specific function:**
   ```bash
   cargo test test_encode_2bit
   ```

### Migration Progress

| Function | Status | Notes |
|----------|--------|-------|
| encode_2bit | ✅ Migrated | First function successfully migrated |
| decode_2bit | ⏳ Pending | |
| encode_kmer | ⏳ Pending | |
| decode_kmer | ⏳ Pending | |
| kmer_reverse_complement | ⏳ Pending | |
| hash_kmer | ⏳ Pending | |
| score_match | ⏳ Pending | |
| edit_distance | ⏳ Pending | |
| gc_content | ⏳ Pending | |
| count_bases | ⏳ Pending | |
| reverse_complement_seq | ⏳ Pending | |

### Key Files to Review

1. **Test Harness**: `rust_core/src/tests.rs` - See how C/Rust equivalence is tested
2. **Migration Config**: `rust_core/src/lib.rs` - Config struct that switches implementations
3. **Native Rust**: `rust_core/src/native.rs` - Pure Rust implementations
4. **FFI Bindings**: `rust_core/src/ffi.rs` - Safe wrappers around C functions

### Why This Approach?

1. **Never Broken**: System works at every commit - we always have a functioning genome aligner
2. **Verifiable**: Every function tested against C original for bit-identical results
3. **Incremental**: Can stop/resume migration anytime without breaking anything
4. **Safe**: Get Rust's memory safety benefits immediately for migrated functions
5. **Measurable**: Clear progress tracking with migration script

### Next Steps

1. Continue migrating functions one by one using `migrate.py next`
2. After all functions migrated, remove C dependencies
3. Integrate with main FASTGA codebase
4. Add Rust-specific optimizations (SIMD, better parallelism)
5. Benchmark against C version

---

# FASTGA Quick Start Guide

## Base Case Alignment: aEleCoq1 vs rDibSmi1

This documents the standard workflow for running FASTGA alignments between two genomes.

### Prerequisites
1. Build FASTGA: `make clean && make -j8`
2. Ensure FASTGA tools are in PATH: `export PATH=$PATH:$(pwd)`

### Input Genomes
- **Target (reference):** aEleCoq1.fa.gz (967MB) - Elephantulus edwardii (Cape elephant shrew)
- **Query:** rDibSmi1.fa.gz (712MB) - Dibamus smithi (Smith's blind skink)

### Basic Alignment (PAF without CIGAR)

```bash
# Run FASTGA with default PAF output
PATH=$PATH:$(pwd) ./FastGA -v -T8 aEleCoq1.fa.gz rDibSmi1.fa.gz > aEleCoq1_vs_rDibSmi1.paf 2> fastga.log

# Output: aEleCoq1_vs_rDibSmi1.paf (2.0MB)
# Runtime: ~5 minutes
# Alignments found: 15,458 with average length 729bp
```

### Alignment with CIGAR Strings (PAF with detailed alignment)

```bash
# Run FASTGA with CIGAR strings using X/= format
PATH=$PATH:$(pwd) ./FastGA -v -T8 -pafx aEleCoq1.fa.gz rDibSmi1.fa.gz > aEleCoq1_vs_rDibSmi1_cigar.paf 2> fastga_cigar.log

# Output: aEleCoq1_vs_rDibSmi1_cigar.paf (8.8MB)
# Runtime: ~5 minutes
# Includes: CIGAR strings with = (match), X (mismatch), I (insertion), D (deletion)
```

## Key Options

- `-v`: Verbose mode (progress to stderr)
- `-T8`: Use 8 threads
- `-pafx`: PAF with CIGAR using X/= operators
- `-pafm`: PAF with CIGAR using M operator
- `-pafS`: PAF with long CS strings
- `-pafs`: PAF with short CS strings
- `-psl`: PSL format output
- `-1:output.1aln`: Save in efficient binary format

## Performance Notes

- Index building: ~1 minute per genome (one-time cost with `-k` option to keep)
- Alignment search: ~3-4 minutes for these genome sizes
- CIGAR generation adds minimal overhead
- Memory usage: ~3-4GB peak

## Output Fields

Standard PAF plus:
- `dv:f`: Divergence fraction (e.g., dv:f:.0629 = 6.29% divergent)
- `df:i`: Number of differences in optimal alignment
- `cg:Z`: CIGAR string (when using -pafx or -pafm options)