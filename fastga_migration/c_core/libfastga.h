#ifndef LIBFASTGA_H
#define LIBFASTGA_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// SEQUENCE ENCODING FUNCTIONS
// ============================================================================

// Convert ASCII sequence to 2-bit encoding (A=0, C=1, G=2, T=3)
// Returns malloc'd buffer, caller must free
uint8_t* fastga_encode_2bit(const char* sequence, size_t length, size_t* encoded_len);

// Convert 2-bit encoding back to ASCII
// Returns malloc'd string, caller must free
char* fastga_decode_2bit(const uint8_t* encoded, size_t encoded_len, size_t seq_len);

// Convert ASCII base to numeric (A=0, C=1, G=2, T=3)
uint8_t fastga_base_to_num(char base);

// Convert numeric to ASCII base
char fastga_num_to_base(uint8_t num);

// ============================================================================
// K-MER OPERATIONS
// ============================================================================

// Encode k-mer from numeric sequence to 64-bit integer
uint64_t fastga_encode_kmer(const uint8_t* seq, int k);

// Decode k-mer from 64-bit integer to numeric sequence
void fastga_decode_kmer(uint64_t kmer, int k, uint8_t* out);

// Compute reverse complement of k-mer
uint64_t fastga_kmer_reverse_complement(uint64_t kmer, int k);

// Hash function for k-mers
uint64_t fastga_hash_kmer(uint64_t kmer);

// ============================================================================
// BASIC ALIGNMENT SCORING
// ============================================================================

// Score a match between two bases
int fastga_score_match(uint8_t a, uint8_t b);

// Compute edit distance between two sequences
int fastga_edit_distance(const uint8_t* seq1, size_t len1,
                         const uint8_t* seq2, size_t len2);

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

// Compute GC content of sequence
double fastga_gc_content(const uint8_t* seq, size_t length);

// Count bases in sequence
void fastga_count_bases(const uint8_t* seq, size_t length,
                       uint64_t* count_a, uint64_t* count_c,
                       uint64_t* count_g, uint64_t* count_t);

// Reverse complement a sequence in place
void fastga_reverse_complement_seq(uint8_t* seq, size_t length);

#ifdef __cplusplus
}
#endif

#endif // LIBFASTGA_H