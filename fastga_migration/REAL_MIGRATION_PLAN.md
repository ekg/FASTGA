# REAL FastGA Migration Plan

## Current Status: Starting ACTUAL Migration

We previously migrated 11 utility functions. Now we're migrating the actual FastGA genome aligner.

## Code Size Analysis

- **FastGA.c**: 5,264 lines - Main aligner
- **align.c**: 6,050 lines - Alignment algorithms
- **GDB.c**: 1,974 lines - Genome database
- **libfastk.c**: 1,748 lines - K-mer operations
- **GIXmake.c**: 2,050 lines - Index construction

Total: ~17,000 lines of core alignment code

## Migration Order (Inside-Out)

### Phase 1: Foundation Libraries
- [ ] `gene_core.c` (503 lines) - Core utilities
- [ ] `alncode.c` (284 lines) - Alignment encoding
- [ ] `GDB.c` (1,974 lines) - Genome database operations

### Phase 2: K-mer Operations
- [ ] `libfastk.c` (1,748 lines) - K-mer library
- [ ] GIX index operations from `GIXmake.c`

### Phase 3: Alignment Algorithms
- [ ] `align.c` (6,050 lines) - Core alignment
- [ ] Trace operations
- [ ] Chaining logic

### Phase 4: Main Pipeline
- [ ] Merge operations from `FastGA.c`
- [ ] Thread management
- [ ] Memory management

### Phase 5: Output Formats
- [ ] PAF output
- [ ] PSL output
- [ ] ONE-code output

### Phase 6: Entry Point
- [ ] Command-line parsing
- [ ] Main function
- [ ] Replace C binary with Rust

## Key Data Structures to Port

```c
typedef struct {
  void     *trace;
  int       tlen;
  int       diffs;
  int       abpos, bbpos;
  int       aepos, bepos;
} Path;

typedef struct {
  Path   *path;
  uint32  flags;
  char   *aseq;
  char   *bseq;
  int     alen;
  int     blen;
} Alignment;

typedef struct {
  Path    path;
  uint32  flags;
  int     aread;
  int     bread;
} Overlap;
```

## Testing Strategy

1. Each function gets FFI wrapper
2. Dual C/Rust testing like before
3. Integration tests with real genomes
4. PAF output comparison

## Success Metrics

- [ ] Identical PAF output for test genomes
- [ ] Performance within 10% of C version
- [ ] Zero memory leaks
- [ ] All original command-line options work