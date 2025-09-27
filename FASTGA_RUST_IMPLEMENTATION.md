# FASTGA Rust Implementation - Rapid Development Plan

## Day 1 Sprint Plan

You're absolutely right! With AI assistance and focused development, we can build a working FASTGA in Rust in a day. Here's the streamlined plan:

### Hour 0-2: Core Foundation
```bash
cargo new fastga-rust --lib
cd fastga-rust
```

**Immediate priorities:**
1. Copy the example structure I created
2. Focus only on FASTA reading (skip GDB initially)
3. Simple 2-bit sequence encoding
4. Basic k-mer extraction

### Hour 2-4: K-mer Index
- Simple HashMap for k-mer counting
- Sort and filter by frequency
- Binary search for lookups
- Skip GIX format initially, keep in memory

### Hour 4-6: Alignment Pipeline
- Find seed matches between indices
- Basic diagonal chaining (skip fancy DP)
- Simple banded alignment for extension
- Generate basic CIGAR strings

### Hour 6-8: Output & Testing
- PAF writer (essential fields only)
- Test with your example genomes (aEleCoq1 vs rDibSmi1)
- Compare output with C version
- Basic optimizations (rayon parallelism)

## Minimal Working Implementation

### 1. Start Here - Simplified Main
```rust
fn main() -> Result<()> {
    let args = Args::parse();

    // Load sequences (just FASTA for now)
    let target = load_fasta(&args.target)?;
    let query = load_fasta(&args.query)?;

    // Build k-mer indices
    let target_idx = build_index(&target, 14, 10);
    let query_idx = build_index(&query, 14, 10);

    // Find and chain matches
    let matches = find_matches(&target_idx, &query_idx);
    let chains = chain_matches(matches);

    // Extend to alignments
    let alignments = extend_chains(chains, &target, &query);

    // Output PAF
    write_paf(alignments);

    Ok(())
}
```

### 2. What to Skip (for now)
- GDB/GIX formats - use FASTA directly
- Complex WFA - use simple banded DP
- ONE-code format - PAF only
- Soft masking - ignore initially
- Memory mapping - load into RAM
- SIMD - add after it works

### 3. Critical Path Features Only
```rust
// Bare minimum k-mer index
struct SimpleIndex {
    kmers: HashMap<u64, Vec<(u32, u32)>>, // kmer -> [(contig, pos)]
}

// Simple alignment
struct Alignment {
    target_id: u32,
    query_id: u32,
    target_start: u32,
    target_end: u32,
    query_start: u32,
    query_end: u32,
    cigar: String,
}
```

## Let's Build It NOW

Ready to start? We can:

1. **Option A**: Build the minimal version first (4 hours)
   - Just enough to match C output
   - Add features incrementally

2. **Option B**: Use the framework I created and fill in the algorithms
   - Structure is ready
   - Just need the core logic

3. **Option C**: Port the C code directly
   - Line-by-line translation
   - Optimize later

Which approach do you want to take? We can have it running with your test genomes today!