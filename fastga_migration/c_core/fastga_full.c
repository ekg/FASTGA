/**
 * FFI wrapper implementations for actual FastGA functions.
 * This wraps the real FastGA C code to make it callable from Rust.
 */

#define _GNU_SOURCE  // For strdup
#include "fastga_full.h"
#include <string.h>
#include <stdlib.h>

// Include the actual FastGA headers
// NOTE: These paths need to be adjusted based on actual build setup
// #include "../../gene_core.h"
// #include "../../GDB.h"
// #include "../../align.h"

// For now, we'll create stub implementations to get the framework working
// These will be replaced with actual FastGA function calls

// ============================================================================
// GENE_CORE FUNCTION WRAPPERS
// ============================================================================

void* fastga_malloc(int64_t size, const char* mesg) {
    // TODO: Call actual Malloc from gene_core.c
    void* ptr = malloc(size);
    if (!ptr && mesg) {
        fprintf(stderr, "Failed to allocate %lld bytes: %s\n", (long long)size, mesg);
        exit(1);
    }
    return ptr;
}

void* fastga_realloc(void* p, int64_t size, const char* mesg) {
    // TODO: Call actual Realloc from gene_core.c
    void* ptr = realloc(p, size);
    if (!ptr && mesg) {
        fprintf(stderr, "Failed to reallocate %lld bytes: %s\n", (long long)size, mesg);
        exit(1);
    }
    return ptr;
}

char* fastga_strdup(const char* name, const char* mesg) {
    // TODO: Call actual Strdup from gene_core.c
    char* copy = strdup(name);
    if (!copy && mesg) {
        fprintf(stderr, "Failed to duplicate string: %s\n", mesg);
        exit(1);
    }
    return copy;
}

FILE* fastga_fopen(const char* name, const char* mode) {
    // TODO: Call actual Fopen from gene_core.c
    FILE* file = fopen(name, mode);
    if (!file) {
        fprintf(stderr, "Failed to open file: %s\n", name);
        exit(1);
    }
    return file;
}

char* fastga_path_to(const char* name) {
    // TODO: Call actual PathTo from gene_core.c
    // For now, return a copy
    return strdup(name);
}

char* fastga_root(const char* name, const char* suffix) {
    // TODO: Call actual Root from gene_core.c
    // Simplified implementation
    char* root = strdup(name);
    if (suffix && strlen(suffix) > 0) {
        char* dot = strrchr(root, '.');
        if (dot && strcmp(dot, suffix) == 0) {
            *dot = '\0';
        }
    }
    return root;
}

void fastga_compress_read(int len, char* s) {
    // TODO: Call actual Compress_Read from gene_core.c
    // This compresses ACGT to 2-bit encoding in place
    // For now, stub implementation
    (void)len; (void)s; // Suppress unused warnings
}

void fastga_uncompress_read(int len, char* s) {
    // TODO: Call actual Uncompress_Read from gene_core.c
    // This uncompresses 2-bit encoding to ACGT in place
    // For now, stub implementation
    (void)len; (void)s; // Suppress unused warnings
}

void fastga_number_read(char* s) {
    // TODO: Call actual Number_Read from gene_core.c
    // Converts ACGT to 0123
    while (*s) {
        switch (*s) {
            case 'A': case 'a': *s = 0; break;
            case 'C': case 'c': *s = 1; break;
            case 'G': case 'g': *s = 2; break;
            case 'T': case 't': *s = 3; break;
            default: *s = 0; break;
        }
        s++;
    }
}

// ============================================================================
// GDB FUNCTION WRAPPERS
// ============================================================================

FastGA_GDB* fastga_create_gdb(const char* source_path) {
    // TODO: Call actual Create_GDB from GDB.c
    // For now, return a minimal stub
    FastGA_GDB* gdb = (FastGA_GDB*)calloc(1, sizeof(FastGA_GDB));
    gdb->srcpath = strdup(source_path);
    return gdb;
}

void fastga_free_gdb(FastGA_GDB* gdb) {
    // TODO: Call actual Free_GDB from GDB.c
    if (gdb) {
        free(gdb->srcpath);
        free(gdb->seqpath);
        free(gdb->headers);
        free(gdb->scaffolds);
        free(gdb->contigs);
        free(gdb->masks);
        free(gdb);
    }
}

int fastga_open_gdb(FastGA_GDB* gdb, const char* path) {
    // TODO: Call actual Open_GDB from GDB.c
    // Returns 0 on success, 1 on failure
    return 1; // Stub: always fails for now
}

int fastga_close_gdb(FastGA_GDB* gdb) {
    // TODO: Call actual Close_GDB from GDB.c
    return 0; // Success
}

char* fastga_get_contig_seq(FastGA_GDB* gdb, int contig_id) {
    // TODO: Implement actual contig sequence retrieval
    return NULL;
}

int fastga_load_sequences(FastGA_GDB* gdb) {
    // TODO: Call actual Load_Sequences from GDB.c
    return 0;
}

// ============================================================================
// ALIGNMENT FUNCTION WRAPPERS
// ============================================================================

FastGA_WorkData* fastga_new_work_data(void) {
    // TODO: Call actual New_Work_Data from align.c
    return (FastGA_WorkData*)calloc(1, 1024); // Stub
}

void fastga_free_work_data(FastGA_WorkData* work) {
    // TODO: Call actual Free_Work_Data from align.c
    free(work);
}

FastGA_AlignSpec* fastga_new_align_spec(double ave_corr, int trace_space, float* freq) {
    // TODO: Call actual New_Align_Spec from align.c
    return (FastGA_AlignSpec*)calloc(1, 256); // Stub
}

void fastga_free_align_spec(FastGA_AlignSpec* spec) {
    // TODO: Call actual Free_Align_Spec from align.c
    free(spec);
}

FastGA_Path* fastga_local_alignment(
    FastGA_Alignment* align,
    FastGA_WorkData* work,
    FastGA_AlignSpec* spec,
    int low, int hgh, int anti, int lbord, int hbord
) {
    // TODO: Call actual Local_Alignment from align.c
    return NULL; // Stub
}

int fastga_compute_trace(
    FastGA_Alignment* align,
    FastGA_WorkData* work,
    int trace_spacing,
    int mode
) {
    // TODO: Call actual Compute_Trace_PTS or Compute_Trace_MID from align.c
    return 1; // Failure
}

// ============================================================================
// INDEX FUNCTION WRAPPERS
// ============================================================================

FastGA_Index* fastga_build_index(FastGA_GDB* gdb, int kmer, int freq_cutoff) {
    // TODO: Build actual k-mer index
    FastGA_Index* index = (FastGA_Index*)calloc(1, sizeof(FastGA_Index));
    index->kmer = kmer;
    index->freq = freq_cutoff;
    return index;
}

void fastga_free_index(FastGA_Index* index) {
    if (index) {
        free(index->table);
        free(index->cache);
        free(index);
    }
}

int fastga_save_index(FastGA_Index* index, const char* path) {
    // TODO: Implement index serialization
    return 1; // Failure
}

FastGA_Index* fastga_load_index(const char* path) {
    // TODO: Implement index deserialization
    return NULL;
}

// ============================================================================
// MERGE FUNCTION WRAPPERS
// ============================================================================

FastGA_MatchList* fastga_find_matches(
    FastGA_Index* index1,
    FastGA_Index* index2,
    int min_matches
) {
    // TODO: Implement actual adaptamer matching
    FastGA_MatchList* list = (FastGA_MatchList*)calloc(1, sizeof(FastGA_MatchList));
    return list;
}

void fastga_free_match_list(FastGA_MatchList* list) {
    if (list) {
        free(list->matches);
        free(list);
    }
}

// ============================================================================
// OUTPUT FUNCTION WRAPPERS
// ============================================================================

void fastga_write_paf(
    FILE* out,
    FastGA_GDB* gdb1,
    FastGA_GDB* gdb2,
    FastGA_Overlap* ovl,
    int flags
) {
    // TODO: Implement actual PAF output
    fprintf(out, "# PAF output not yet implemented\n");
}

void fastga_write_psl(
    FILE* out,
    FastGA_GDB* gdb1,
    FastGA_GDB* gdb2,
    FastGA_Overlap* ovl
) {
    // TODO: Implement actual PSL output
    fprintf(out, "# PSL output not yet implemented\n");
}

// ============================================================================
// PIPELINE FUNCTION WRAPPER
// ============================================================================

int fastga_run_alignment(
    const char* source1,
    const char* source2,
    FastGA_Config* config,
    FILE* output
) {
    // TODO: Implement actual alignment pipeline
    fprintf(output, "# Alignment pipeline not yet implemented\n");
    return 1; // Failure
}