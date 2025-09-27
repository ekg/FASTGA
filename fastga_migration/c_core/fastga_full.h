#ifndef FASTGA_FULL_H
#define FASTGA_FULL_H

#include <stdint.h>
#include <stddef.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// CORE DATA STRUCTURES
// ============================================================================

// Path/Alignment structures
typedef struct {
    void     *trace;
    int       tlen;
    int       diffs;
    int       abpos, bbpos;
    int       aepos, bepos;
} FastGA_Path;

typedef struct {
    FastGA_Path *path;
    uint32_t     flags;
    char        *aseq;
    char        *bseq;
    int          alen;
    int          blen;
} FastGA_Alignment;

typedef struct {
    FastGA_Path  path;
    uint32_t     flags;
    int          aread;
    int          bread;
} FastGA_Overlap;

// GDB structures
typedef struct {
    int64_t  beg;
    int64_t  end;
} FastGA_Mask;

typedef struct {
    int64_t  clen;
    int64_t  sbeg;
    int64_t  boff;
    int64_t  moff;
    int      scaf;
} FastGA_Contig;

typedef struct {
    int64_t  slen;
    int      fctg;
    int      ectg;
    int64_t  hoff;
} FastGA_Scaffold;

typedef struct {
    int               nscaff;
    FastGA_Scaffold  *scaffolds;
    int               ncontig;
    int64_t           maxctg;
    FastGA_Contig    *contigs;
    int               nmasks;
    FastGA_Mask      *masks;
    int               hdrtot;
    char             *headers;
    char             *srcpath;
    char             *seqpath;
    int64_t           seqtot;
    int               seqstate;
    void             *seqs;
    float             freq[4];
} FastGA_GDB;

// K-mer index structures
typedef struct {
    int      kmer;
    int      freq;
    int64_t  nels;
    void    *table;
    void    *cache;
} FastGA_Index;

// ============================================================================
// GENE_CORE FUNCTIONS
// ============================================================================

void* fastga_malloc(int64_t size, const char* mesg);
void* fastga_realloc(void* p, int64_t size, const char* mesg);
char* fastga_strdup(const char* name, const char* mesg);
FILE* fastga_fopen(const char* name, const char* mode);
char* fastga_path_to(const char* name);
char* fastga_root(const char* name, const char* suffix);
void  fastga_compress_read(int len, char* s);
void  fastga_uncompress_read(int len, char* s);
void  fastga_number_read(char* s);

// ============================================================================
// GDB FUNCTIONS
// ============================================================================

FastGA_GDB* fastga_create_gdb(const char* source_path);
void        fastga_free_gdb(FastGA_GDB* gdb);
int         fastga_open_gdb(FastGA_GDB* gdb, const char* path);
int         fastga_close_gdb(FastGA_GDB* gdb);
char*       fastga_get_contig_seq(FastGA_GDB* gdb, int contig_id);
int         fastga_load_sequences(FastGA_GDB* gdb);

// ============================================================================
// ALIGNMENT FUNCTIONS
// ============================================================================

typedef void FastGA_WorkData;
typedef void FastGA_AlignSpec;

FastGA_WorkData*  fastga_new_work_data(void);
void              fastga_free_work_data(FastGA_WorkData* work);
FastGA_AlignSpec* fastga_new_align_spec(double ave_corr, int trace_space, float* freq);
void              fastga_free_align_spec(FastGA_AlignSpec* spec);

FastGA_Path* fastga_local_alignment(
    FastGA_Alignment* align,
    FastGA_WorkData* work,
    FastGA_AlignSpec* spec,
    int low, int hgh, int anti, int lbord, int hbord
);

int fastga_compute_trace(
    FastGA_Alignment* align,
    FastGA_WorkData* work,
    int trace_spacing,
    int mode
);

// ============================================================================
// K-MER INDEX FUNCTIONS
// ============================================================================

FastGA_Index* fastga_build_index(FastGA_GDB* gdb, int kmer, int freq_cutoff);
void          fastga_free_index(FastGA_Index* index);
int           fastga_save_index(FastGA_Index* index, const char* path);
FastGA_Index* fastga_load_index(const char* path);

// ============================================================================
// MERGE/MATCH FUNCTIONS
// ============================================================================

typedef struct {
    int64_t apos;
    int64_t bpos;
    int     diag;
    int     aread;
    int     bread;
} FastGA_Match;

typedef struct {
    FastGA_Match* matches;
    int64_t       nmatches;
    int64_t       capacity;
} FastGA_MatchList;

FastGA_MatchList* fastga_find_matches(
    FastGA_Index* index1,
    FastGA_Index* index2,
    int min_matches
);

void fastga_free_match_list(FastGA_MatchList* list);

// ============================================================================
// OUTPUT FUNCTIONS
// ============================================================================

void fastga_write_paf(
    FILE* out,
    FastGA_GDB* gdb1,
    FastGA_GDB* gdb2,
    FastGA_Overlap* ovl,
    int flags  // PAFM, PAFX, PAFS, PAFL
);

void fastga_write_psl(
    FILE* out,
    FastGA_GDB* gdb1,
    FastGA_GDB* gdb2,
    FastGA_Overlap* ovl
);

// ============================================================================
// MAIN PIPELINE FUNCTIONS
// ============================================================================

typedef struct {
    int    kmer_size;
    int    freq_cutoff;
    int    chain_break;
    int    chain_min;
    int    align_min;
    double align_rate;
    int    nthreads;
    char*  sort_path;
    int    keep_index;
    int    symmetric;
    int    soft_mask;
    int    output_type;  // 0=PAF, 1=PSL, 2=ONE
    int    output_opt;   // PAF flags
} FastGA_Config;

int fastga_run_alignment(
    const char* source1,
    const char* source2,
    FastGA_Config* config,
    FILE* output
);

#ifdef __cplusplus
}
#endif

#endif // FASTGA_FULL_H