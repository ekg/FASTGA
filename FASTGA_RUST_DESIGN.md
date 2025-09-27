# FASTGA Rust Rewrite Design

## Overview
A Rust implementation of FASTGA would modernize this genome aligner with memory safety, better concurrency, and improved error handling while maintaining the high performance characteristics.

## Core Architecture

### 1. Module Structure
```
fastga/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library root
│   ├── index/            # K-mer indexing
│   │   ├── mod.rs
│   │   ├── kmer.rs       # K-mer operations
│   │   ├── gix.rs        # GIX index handling
│   │   └── builder.rs    # Index construction
│   ├── genome/           # Genome database
│   │   ├── mod.rs
│   │   ├── gdb.rs        # GDB format handling
│   │   ├── sequence.rs   # Sequence compression/decompression
│   │   └── io.rs         # FASTA/1-code readers
│   ├── align/            # Alignment algorithms
│   │   ├── mod.rs
│   │   ├── local.rs      # Local alignment
│   │   ├── chain.rs      # Alignment chaining
│   │   ├── traceback.rs  # Traceback computation
│   │   └── scoring.rs    # Scoring matrices
│   ├── merge/            # Adaptamer merge
│   │   ├── mod.rs
│   │   ├── matcher.rs    # K-mer matching
│   │   └── parallel.rs   # Parallel merge strategies
│   └── output/           # Output formats
│       ├── mod.rs
│       ├── paf.rs        # PAF format
│       ├── psl.rs        # PSL format
│       └── cigar.rs      # CIGAR string generation
```

## Key Rust Advantages

### 1. Memory Safety
- No manual memory management bugs
- Safe parallelism with Send/Sync traits
- Bounds checking without runtime overhead

### 2. Better Type System
```rust
// Strong typing for genomic coordinates
#[derive(Debug, Clone, Copy)]
struct GenomicPos {
    contig: ContigId,
    position: u64,
}

// Zero-cost abstractions for k-mers
#[derive(Debug, Clone, Copy)]
struct Kmer<const K: usize> {
    data: [u8; K],
}

// Type-safe alignment representation
enum AlignmentTrace {
    Compressed(Vec<u8>),
    Uncompressed(Vec<u16>),
    Exact(Vec<TraceOp>),
}
```

### 3. Modern Concurrency
```rust
use rayon::prelude::*;
use crossbeam::channel;

// Parallel k-mer matching with work-stealing
fn merge_indices(index1: &KmerIndex, index2: &KmerIndex) -> Vec<Match> {
    index1.kmers()
        .par_iter()
        .flat_map(|kmer| {
            index2.find_matches(kmer)
                .map(|pos2| Match {
                    kmer: *kmer,
                    pos1: kmer.position,
                    pos2
                })
        })
        .collect()
}
```

### 4. Error Handling
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum FastGAError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid k-mer size: {0} (must be between 8 and 32)")]
    InvalidKmerSize(usize),

    #[error("Genome database corrupted: {0}")]
    CorruptedGDB(String),

    #[error("Alignment failed: {0}")]
    AlignmentError(String),
}

type Result<T> = std::result::Result<T, FastGAError>;
```

## Core Components Translation

### 1. Genome Database (GDB)
```rust
pub struct GDB {
    scaffolds: Vec<Scaffold>,
    contigs: Vec<Contig>,
    sequences: SequenceStore,
    masks: Vec<SoftMask>,
    metadata: Metadata,
}

enum SequenceStore {
    External(MmapFile),      // Memory-mapped file
    Compressed(Vec<u8>),      // 2-bit encoding
    Numeric(Vec<u8>),         // 0-3 encoding
    Text(Vec<u8>),           // ASCII
}

impl GDB {
    pub fn from_fasta(path: &Path) -> Result<Self> {
        // Use nom or bio-rs for parsing
    }

    pub fn get_sequence(&self, contig: ContigId) -> Sequence {
        // Lazy decompression
    }
}
```

### 2. K-mer Index
```rust
pub struct KmerIndex<const K: usize> {
    kmers: Vec<KmerEntry<K>>,
    positions: Vec<GenomicPos>,
    frequency_cutoff: u32,
}

impl<const K: usize> KmerIndex<K> {
    pub fn build(genome: &GDB, threads: usize) -> Result<Self> {
        // Parallel index construction
    }

    pub fn find_matches(&self, kmer: &Kmer<K>) -> impl Iterator<Item = GenomicPos> {
        // Binary search in sorted index
    }
}
```

### 3. Alignment Engine
```rust
pub struct Aligner {
    scoring: ScoringMatrix,
    trace_spacing: usize,
    min_chain_score: f64,
}

impl Aligner {
    pub fn align(&self, seq1: &[u8], seq2: &[u8], seeds: &[Match]) -> Alignment {
        // WFA or standard DP with SIMD acceleration
    }

    pub fn chain_alignments(&self, alignments: Vec<Alignment>) -> Vec<Chain> {
        // Dynamic programming chaining
    }
}
```

## Performance Optimizations

### 1. SIMD Operations
```rust
use std::arch::x86_64::*;

// SIMD-accelerated sequence comparison
unsafe fn compare_sequences_avx2(seq1: &[u8], seq2: &[u8]) -> u32 {
    // AVX2 implementation
}
```

### 2. Cache-Efficient Data Structures
```rust
// Structure-of-arrays for better cache locality
struct KmerIndexSOA {
    kmer_high: Vec<u64>,  // High bits of k-mers
    kmer_low: Vec<u64>,   // Low bits of k-mers
    positions: Vec<u32>,  // Positions
}
```

### 3. Lock-Free Data Structures
```rust
use crossbeam::queue::ArrayQueue;

// Lock-free queue for alignment tasks
let queue: ArrayQueue<AlignmentTask> = ArrayQueue::new(1000);
```

## Dependencies
```toml
[dependencies]
clap = "4.0"           # CLI parsing
rayon = "1.7"          # Data parallelism
crossbeam = "0.8"      # Concurrent data structures
memmap2 = "0.9"        # Memory-mapped files
bio = "1.0"            # Bioinformatics primitives
noodles = "0.50"       # File format parsers
thiserror = "1.0"      # Error handling
anyhow = "1.0"         # Error propagation
indicatif = "0.17"     # Progress bars
log = "0.4"            # Logging
env_logger = "0.10"    # Log configuration

[dev-dependencies]
criterion = "0.5"      # Benchmarking
proptest = "1.0"       # Property testing
```

## Migration Strategy

### Phase 1: Core Data Structures
- Implement GDB reader/writer
- K-mer index structures
- Basic sequence operations

### Phase 2: Alignment Algorithms
- Port local alignment with WFA
- Implement traceback
- Add chaining logic

### Phase 3: Parallel Processing
- Parallel index construction
- Concurrent alignment pipeline
- Work-stealing scheduler

### Phase 4: Optimization
- SIMD acceleration
- Memory pool allocation
- Profile-guided optimization

## Benefits Summary

1. **Safety**: Eliminate segfaults and memory leaks
2. **Performance**: Match or exceed C performance with better parallelism
3. **Maintainability**: Modern tooling, testing, and documentation
4. **Portability**: Easy cross-compilation
5. **Ecosystem**: Leverage bio-rs and other bioinformatics crates
6. **Developer Experience**: Cargo, rustfmt, clippy, rust-analyzer

## Example Usage
```rust
use fastga::{GDB, KmerIndex, Aligner, OutputFormat};

fn main() -> Result<()> {
    // Load genomes
    let target = GDB::from_fasta("genome1.fa.gz")?;
    let query = GDB::from_fasta("genome2.fa.gz")?;

    // Build indices
    let index1 = KmerIndex::<14>::build(&target, 8)?;
    let index2 = KmerIndex::<14>::build(&query, 8)?;

    // Find matches and align
    let aligner = Aligner::default();
    let alignments = aligner.run(&index1, &index2)?;

    // Output results
    alignments.write_paf(&mut std::io::stdout())?;

    Ok(())
}
```