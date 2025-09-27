# FASTGA Rust Implementation Design Document

## Executive Summary

This document outlines the design for a complete Rust reimplementation of FASTGA, a fast whole genome aligner using adaptive k-mer matching. The Rust version aims to maintain full compatibility with existing file formats while providing memory safety, improved parallelism, and modern development practices.

## 1. Project Overview

### 1.1 Background

FASTGA is a high-performance genome alignment tool that uses adaptive k-mer matching to find alignments between large genomic sequences. The current C implementation consists of approximately 16,000 lines of code across multiple components.

### 1.2 Motivation for Rust Rewrite

- **Memory Safety**: Eliminate segmentation faults, buffer overflows, and memory leaks
- **Concurrency**: Leverage Rust's fearless concurrency for better parallel performance
- **Maintainability**: Modern tooling, package management, and testing framework
- **Type Safety**: Catch more bugs at compile time with Rust's type system
- **Performance**: Match or exceed C performance with zero-cost abstractions

### 1.3 Design Goals

1. **Compatibility**: Support all existing file formats (GDB, GIX, PAF, PSL, ONE)
2. **Performance**: ≥100% of C version speed with better scaling
3. **Safety**: Zero undefined behavior, memory safety guaranteed
4. **Modularity**: Clean separation of concerns with reusable components
5. **Testability**: Comprehensive unit and integration testing

## 2. Architecture

### 2.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────┐
│                   CLI Interface                      │
├─────────────────────────────────────────────────────┤
│                  Core Pipeline                       │
│  ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │
│  │  Index   │→ │  Match   │→ │     Alignment    │  │
│  │  Builder │  │  Finder  │  │     Extension    │  │
│  └──────────┘  └──────────┘  └──────────────────┘  │
├─────────────────────────────────────────────────────┤
│                 Data Structures                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │
│  │   GDB    │  │   GIX    │  │   Alignment      │  │
│  │  Store   │  │  Index   │  │     Store        │  │
│  └──────────┘  └──────────┘  └──────────────────┘  │
├─────────────────────────────────────────────────────┤
│                    I/O Layer                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │
│  │  FASTA   │  │   PAF    │  │      PSL         │  │
│  │  Reader  │  │  Writer  │  │     Writer       │  │
│  └──────────┘  └──────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────┘
```

### 2.2 Module Structure

```
fastga/
├── Cargo.toml                 # Project manifest
├── benches/                   # Benchmarking suite
│   ├── kmer_index.rs
│   ├── alignment.rs
│   └── io.rs
├── src/
│   ├── main.rs               # CLI entry point
│   ├── lib.rs                # Library interface
│   ├── cli/                  # Command-line interface
│   │   ├── mod.rs
│   │   ├── args.rs           # Argument parsing
│   │   └── commands.rs       # Subcommands
│   ├── core/                 # Core algorithms
│   │   ├── mod.rs
│   │   ├── pipeline.rs       # Main alignment pipeline
│   │   ├── scheduler.rs      # Task scheduling
│   │   └── config.rs         # Configuration management
│   ├── genome/               # Genome data structures
│   │   ├── mod.rs
│   │   ├── gdb.rs           # GDB format handling
│   │   ├── sequence.rs      # Sequence operations
│   │   ├── compression.rs   # 2-bit compression
│   │   └── mask.rs          # Soft masking
│   ├── index/                # K-mer indexing
│   │   ├── mod.rs
│   │   ├── kmer.rs          # K-mer operations
│   │   ├── gix.rs           # GIX index format
│   │   ├── builder.rs       # Index construction
│   │   └── search.rs        # K-mer searching
│   ├── align/                # Alignment algorithms
│   │   ├── mod.rs
│   │   ├── seed.rs          # Seed finding
│   │   ├── chain.rs         # Seed chaining
│   │   ├── extend.rs        # Alignment extension
│   │   ├── wfa.rs           # WFA algorithm
│   │   ├── traceback.rs     # Traceback computation
│   │   └── scoring.rs       # Scoring matrices
│   ├── io/                   # File I/O
│   │   ├── mod.rs
│   │   ├── fasta.rs         # FASTA/FASTQ parsing
│   │   ├── paf.rs           # PAF format
│   │   ├── psl.rs           # PSL format
│   │   ├── one.rs           # ONE-code format
│   │   └── compression.rs   # gzip/bzip2 support
│   └── utils/                # Utilities
│       ├── mod.rs
│       ├── parallel.rs      # Parallelization helpers
│       ├── memory.rs        # Memory management
│       ├── progress.rs      # Progress reporting
│       └── logging.rs       # Logging configuration
└── tests/                     # Integration tests
    ├── alignment_tests.rs
    ├── index_tests.rs
    └── io_tests.rs
```

## 3. Core Components

### 3.1 Genome Database (GDB)

#### Data Structure
```rust
pub struct GDB {
    metadata: Metadata,
    scaffolds: Vec<Scaffold>,
    contigs: Vec<Contig>,
    sequences: SequenceStore,
    masks: Option<MaskStore>,
}

pub enum SequenceStore {
    External(MmapFile),      // Memory-mapped file
    Compressed(CompressedSeq), // 2-bit encoding
    Numeric(Vec<u8>),        // 0-3 encoding
    Text(Vec<u8>),          // ASCII
}

pub struct CompressedSeq {
    data: Vec<u64>,          // 32 bases per u64
    length: usize,
}
```

#### Key Operations
- Lazy loading with memory-mapped files
- On-demand decompression
- Efficient random access
- Thread-safe concurrent reads

### 3.2 K-mer Index (GIX)

#### Data Structure
```rust
pub struct KmerIndex<const K: usize> {
    header: IndexHeader,
    kmers: SortedKmerStore,
    positions: PositionStore,
    frequency_filter: FrequencyFilter,
}

pub struct SortedKmerStore {
    // Structure-of-arrays for cache efficiency
    high_bits: Vec<u64>,    // Upper 64 bits of k-mer
    low_bits: Vec<u64>,     // Lower 64 bits of k-mer
    count: usize,
}

pub struct PositionStore {
    contigs: Vec<u32>,      // Contig IDs
    positions: Vec<u32>,    // Positions within contigs
}
```

#### Construction Algorithm
```rust
impl KmerIndex {
    pub fn build(genome: &GDB, params: IndexParams) -> Result<Self> {
        // Phase 1: Parallel k-mer extraction
        let kmer_stream = genome.par_iter_kmers(params.k);

        // Phase 2: Frequency counting with concurrent hashmap
        let frequencies = count_kmers_parallel(kmer_stream);

        // Phase 3: Filter by frequency
        let filtered = frequencies.filter(|&count| count <= params.max_freq);

        // Phase 4: Sort for binary search
        let sorted = parallel_radix_sort(filtered);

        // Phase 5: Build index structure
        Self::from_sorted_kmers(sorted)
    }
}
```

### 3.3 Alignment Pipeline

#### Pipeline Stages
```rust
pub struct AlignmentPipeline {
    stages: Vec<Box<dyn PipelineStage>>,
}

pub trait PipelineStage: Send + Sync {
    fn process(&self, input: StageInput) -> Result<StageOutput>;
}

// Stage implementations
pub struct SeedFindingStage { ... }
pub struct SeedChainingStage { ... }
pub struct AlignmentExtensionStage { ... }
pub struct TracebackStage { ... }
```

#### Parallel Execution
```rust
impl AlignmentPipeline {
    pub fn execute(&self, target: &GDB, query: &GDB) -> Result<Vec<Alignment>> {
        // Work-stealing queue for contig pairs
        let work_queue = WorkQueue::new();

        // Spawn worker threads
        let workers = (0..self.num_threads).map(|_| {
            spawn(|| self.worker_loop(work_queue.clone()))
        });

        // Collect and merge results
        workers.join_all()
            .flatten()
            .collect()
    }
}
```

### 3.4 Alignment Algorithms

#### Wavefront Alignment (WFA)
```rust
pub struct WavefrontAligner {
    memory_pool: MemoryPool,
    scoring: ScoringMatrix,
}

impl WavefrontAligner {
    pub fn align(&mut self, seq1: &[u8], seq2: &[u8]) -> Alignment {
        let mut wavefronts = WavefrontSet::new(&mut self.memory_pool);

        while !wavefronts.reached_end(seq1.len(), seq2.len()) {
            wavefronts.extend(seq1, seq2, &self.scoring);
            wavefronts.prune(); // Memory optimization
        }

        wavefronts.traceback()
    }
}
```

#### Seed Chaining
```rust
pub struct SeedChainer {
    gap_penalty: f32,
    chain_bandwidth: u32,
}

impl SeedChainer {
    pub fn chain_seeds(&self, seeds: &[Seed]) -> Vec<Chain> {
        // Dynamic programming for optimal chaining
        let n = seeds.len();
        let mut dp = vec![0.0; n];
        let mut predecessor = vec![None; n];

        for i in 0..n {
            dp[i] = seeds[i].score;

            for j in 0..i {
                if self.can_chain(&seeds[j], &seeds[i]) {
                    let score = dp[j] + seeds[i].score
                              - self.gap_penalty * gap_cost(&seeds[j], &seeds[i]);
                    if score > dp[i] {
                        dp[i] = score;
                        predecessor[i] = Some(j);
                    }
                }
            }
        }

        self.traceback_chains(dp, predecessor)
    }
}
```

## 4. Performance Optimizations

### 4.1 SIMD Operations

```rust
use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
unsafe fn compare_sequences_avx2(seq1: &[u8], seq2: &[u8]) -> u32 {
    let mut matches = 0u32;
    let chunks = seq1.chunks_exact(32);

    for (chunk1, chunk2) in chunks.clone().zip(seq2.chunks_exact(32)) {
        let v1 = _mm256_loadu_si256(chunk1.as_ptr() as *const __m256i);
        let v2 = _mm256_loadu_si256(chunk2.as_ptr() as *const __m256i);
        let cmp = _mm256_cmpeq_epi8(v1, v2);
        matches += _mm256_movemask_epi8(cmp).count_ones();
    }

    // Handle remainder
    matches + chunks.remainder().iter()
        .zip(seq2[chunks.len() * 32..].iter())
        .filter(|(a, b)| a == b)
        .count() as u32
}
```

### 4.2 Cache-Efficient Data Structures

```rust
// Structure-of-arrays for better cache locality
pub struct KmerIndexSOA {
    // Each k-mer component in separate array
    prefix: Vec<u32>,       // First 32 bits
    middle: Vec<u32>,       // Middle 32 bits
    suffix: Vec<u32>,       // Last 32 bits
    positions: Vec<u32>,    // Genomic positions
}

// Prefetching for predictable access patterns
use std::intrinsics::prefetch_read_data;

unsafe fn prefetch_kmer(index: &KmerIndexSOA, i: usize) {
    prefetch_read_data(&index.prefix[i + 8], 3);
    prefetch_read_data(&index.positions[i + 8], 3);
}
```

### 4.3 Memory Pool Allocation

```rust
pub struct MemoryPool {
    chunks: Vec<Vec<u8>>,
    current: AtomicUsize,
    chunk_size: usize,
}

impl MemoryPool {
    pub fn allocate(&self, size: usize) -> &mut [u8] {
        let offset = self.current.fetch_add(size, Ordering::Relaxed);
        let chunk_idx = offset / self.chunk_size;
        let chunk_offset = offset % self.chunk_size;

        unsafe {
            let ptr = self.chunks[chunk_idx].as_ptr().add(chunk_offset);
            std::slice::from_raw_parts_mut(ptr as *mut u8, size)
        }
    }

    pub fn reset(&mut self) {
        self.current.store(0, Ordering::Relaxed);
    }
}
```

### 4.4 Lock-Free Concurrent Data Structures

```rust
use crossbeam::queue::ArrayQueue;
use dashmap::DashMap;

pub struct ConcurrentWorkQueue<T> {
    queue: ArrayQueue<T>,
    active_count: AtomicUsize,
}

pub struct ConcurrentKmerCounter {
    counts: DashMap<u64, AtomicU32>,
}

impl ConcurrentKmerCounter {
    pub fn increment(&self, kmer: u64) {
        self.counts
            .entry(kmer)
            .or_insert_with(|| AtomicU32::new(0))
            .fetch_add(1, Ordering::Relaxed);
    }
}
```

## 5. File Formats

### 5.1 Format Support Matrix

| Format | Read | Write | Compressed | Indexed |
|--------|------|-------|------------|---------|
| FASTA  | ✓    | ✓     | ✓ (gz)     | ✗       |
| FASTQ  | ✓    | ✗     | ✓ (gz)     | ✗       |
| GDB    | ✓    | ✓     | ✓ (2-bit)  | ✓       |
| GIX    | ✓    | ✓     | ✓          | N/A     |
| PAF    | ✓    | ✓     | ✗          | ✗       |
| PSL    | ✓    | ✓     | ✗          | ✗       |
| ONE    | ✓    | ✓     | ✓          | ✗       |

### 5.2 Binary Format Specifications

#### GDB Format v2.0
```rust
pub struct GdbHeader {
    magic: [u8; 4],        // "GDB\0"
    version: u32,          // 0x00020000
    endianness: u32,       // 0x01234567
    num_scaffolds: u32,
    num_contigs: u32,
    sequence_offset: u64,
    mask_offset: u64,
    header_offset: u64,
}
```

#### GIX Format v2.0
```rust
pub struct GixHeader {
    magic: [u8; 4],        // "GIX\0"
    version: u32,          // 0x00020000
    kmer_size: u32,
    num_kmers: u64,
    frequency_cutoff: u32,
    masked: bool,
    kmer_offset: u64,
    position_offset: u64,
}
```

## 6. Error Handling

### 6.1 Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FastGAError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid format: {0}")]
    Format(String),

    #[error("Index error: {0}")]
    Index(String),

    #[error("Alignment failed: {0}")]
    Alignment(String),

    #[error("Invalid parameter: {param}: {reason}")]
    InvalidParam { param: String, reason: String },

    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
}

pub type Result<T> = std::result::Result<T, FastGAError>;
```

### 6.2 Error Recovery

```rust
pub trait Recoverable {
    fn recover(&mut self) -> Result<()>;
}

impl Recoverable for AlignmentPipeline {
    fn recover(&mut self) -> Result<()> {
        // Reset memory pools
        self.memory_pool.reset();

        // Clear work queues
        self.work_queue.clear();

        // Restart failed workers
        self.restart_workers()?;

        Ok(())
    }
}
```

## 7. Testing Strategy

### 7.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmer_encoding() {
        let seq = b"ACGT";
        let kmer = Kmer::from_slice(seq).unwrap();
        assert_eq!(kmer.encoded, 0b00011011);
    }

    #[test]
    fn test_sequence_compression() {
        let seq = b"ACGTACGT";
        let compressed = compress_sequence(seq);
        let decompressed = decompress_sequence(&compressed, 8);
        assert_eq!(seq, &decompressed[..]);
    }
}
```

### 7.2 Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_alignment_symmetry(seq1 in "[ACGT]{10,100}", seq2 in "[ACGT]{10,100}") {
        let align1 = align(&seq1, &seq2);
        let align2 = align(&seq2, &seq1);
        prop_assert_eq!(align1.score, align2.score);
    }

    #[test]
    fn test_kmer_reverse_complement(seq in "[ACGT]{1,32}") {
        let kmer = Kmer::from_string(&seq).unwrap();
        let rc = kmer.reverse_complement();
        let rc_rc = rc.reverse_complement();
        prop_assert_eq!(kmer, rc_rc);
    }
}
```

### 7.3 Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_kmer_index_build(c: &mut Criterion) {
    let genome = load_test_genome();

    c.bench_function("kmer_index_build", |b| {
        b.iter(|| {
            KmerIndex::build(black_box(&genome), IndexParams::default())
        });
    });
}

criterion_group!(benches, bench_kmer_index_build);
criterion_main!(benches);
```

## 8. Deployment

### 8.1 Build Configuration

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true

[profile.release-with-debug]
inherits = "release"
debug = true
strip = false

[profile.bench]
inherits = "release"
lto = false
```

### 8.2 Platform Support

- **Primary**: Linux x86_64 (Ubuntu 20.04+, RHEL 8+)
- **Secondary**: macOS (11.0+), Windows (WSL2)
- **Architecture**: x86_64 with AVX2, ARM64 with NEON

### 8.3 Dependencies

```toml
[dependencies]
# Core
anyhow = "1.0"
thiserror = "1.0"

# CLI
clap = { version = "4.4", features = ["derive"] }
indicatif = "0.17"

# Parallelism
rayon = "1.8"
crossbeam = "0.8"
dashmap = "5.5"
parking_lot = "0.12"

# I/O
memmap2 = "0.9"
flate2 = "1.0"
bzip2 = "0.4"

# Serialization
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
byteorder = "1.5"

# Logging
log = "0.4"
env_logger = "0.11"

[dev-dependencies]
criterion = "0.5"
proptest = "1.4"
tempfile = "3.9"
```

## 9. Migration Plan

### Phase 1: Core Infrastructure (Months 1-2)
- [ ] Set up project structure and CI/CD
- [ ] Implement GDB reader/writer
- [ ] Basic sequence operations
- [ ] Unit test framework

### Phase 2: Indexing (Months 2-3)
- [ ] K-mer encoding/decoding
- [ ] GIX format support
- [ ] Parallel index construction
- [ ] Index serialization

### Phase 3: Alignment (Months 3-5)
- [ ] Seed finding
- [ ] Seed chaining algorithm
- [ ] WFA implementation
- [ ] Traceback computation

### Phase 4: I/O Formats (Month 5)
- [ ] PAF writer with CIGAR
- [ ] PSL writer
- [ ] ONE-code support
- [ ] Compression support

### Phase 5: Optimization (Month 6)
- [ ] SIMD implementations
- [ ] Memory pool optimization
- [ ] Profile-guided optimization
- [ ] Benchmark suite

### Phase 6: Validation (Month 7)
- [ ] Correctness testing against C version
- [ ] Performance benchmarking
- [ ] Real-world dataset testing
- [ ] Documentation

## 10. Performance Targets

### Benchmarks (vs C implementation)

| Operation | Target | Acceptable |
|-----------|--------|------------|
| Index Build | 110% | 100% |
| K-mer Matching | 120% | 100% |
| Alignment | 100% | 95% |
| I/O | 105% | 100% |
| Memory Usage | 90% | 100% |

### Scalability

- Linear scaling to 64 threads
- Efficient handling of genomes up to 10GB
- Sub-linear memory growth with genome size

## 11. Documentation

### 11.1 User Documentation
- Installation guide
- Command-line reference
- File format specifications
- Tutorial with examples
- Performance tuning guide

### 11.2 Developer Documentation
- Architecture overview
- API documentation (rustdoc)
- Contributing guidelines
- Algorithm descriptions
- Debugging guide

## 12. Maintenance and Support

### 12.1 Versioning
- Semantic versioning (MAJOR.MINOR.PATCH)
- Backward compatibility for file formats
- Deprecation policy (2 major versions)

### 12.2 Release Cycle
- Monthly patch releases
- Quarterly minor releases
- Annual major releases
- LTS versions every 2 years

### 12.3 Community
- GitHub repository with issues/discussions
- Documentation wiki
- Benchmark repository
- Test dataset collection

## Appendix A: Algorithm Details

### A.1 Adaptive K-mer Selection
The adaptive k-mer selection algorithm dynamically adjusts k-mer frequency thresholds based on local sequence complexity...

### A.2 Wavefront Alignment
The WFA implementation uses a bidirectional approach with adaptive pruning...

### A.3 Seed Chaining Dynamic Programming
The chaining algorithm uses a modified longest increasing subsequence approach...

## Appendix B: File Format Specifications

### B.1 GDB Binary Format
Complete byte-level specification of the GDB format...

### B.2 GIX Index Format
Detailed specification of the k-mer index format...

### B.3 PAF Extensions
Custom tags and extensions for FASTGA-specific information...

## Appendix C: Benchmark Datasets

### C.1 Small Genomes (< 100MB)
- E. coli reference strains
- Yeast genomes
- Viral genomes

### C.2 Medium Genomes (100MB - 1GB)
- Drosophila melanogaster
- C. elegans
- Arabidopsis thaliana

### C.3 Large Genomes (> 1GB)
- Human genome (GRCh38)
- Mouse genome (GRCm39)
- Plant genomes (wheat, maize)

## Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2024-01-26 | - | Initial draft |