#include "libfastga.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

// ============================================================================
// SEQUENCE ENCODING FUNCTIONS
// ============================================================================

uint8_t fastga_base_to_num(char base) {
    switch (base) {
        case 'A': case 'a': return 0;
        case 'C': case 'c': return 1;
        case 'G': case 'g': return 2;
        case 'T': case 't': return 3;
        default: return 0; // Default to A for ambiguous bases
    }
}

char fastga_num_to_base(uint8_t num) {
    static const char bases[] = "ACGT";
    return bases[num & 3];
}

uint8_t* fastga_encode_2bit(const char* sequence, size_t length, size_t* encoded_len) {
    *encoded_len = (length + 3) / 4; // Round up to nearest byte
    uint8_t* encoded = (uint8_t*)calloc(*encoded_len, 1);

    for (size_t i = 0; i < length; i++) {
        uint8_t base = fastga_base_to_num(sequence[i]);
        size_t byte_idx = i / 4;
        size_t bit_offset = (i % 4) * 2;
        encoded[byte_idx] |= (base << (6 - bit_offset));
    }

    return encoded;
}

char* fastga_decode_2bit(const uint8_t* encoded, size_t encoded_len, size_t seq_len) {
    char* sequence = (char*)malloc(seq_len + 1);

    for (size_t i = 0; i < seq_len; i++) {
        size_t byte_idx = i / 4;
        size_t bit_offset = (i % 4) * 2;
        uint8_t base = (encoded[byte_idx] >> (6 - bit_offset)) & 0x03;
        sequence[i] = fastga_num_to_base(base);
    }
    sequence[seq_len] = '\0';

    return sequence;
}

// ============================================================================
// K-MER OPERATIONS
// ============================================================================

uint64_t fastga_encode_kmer(const uint8_t* seq, int k) {
    uint64_t kmer = 0;
    for (int i = 0; i < k; i++) {
        kmer = (kmer << 2) | (seq[i] & 3);
    }
    return kmer;
}

void fastga_decode_kmer(uint64_t kmer, int k, uint8_t* out) {
    for (int i = k - 1; i >= 0; i--) {
        out[i] = kmer & 3;
        kmer >>= 2;
    }
}

uint64_t fastga_kmer_reverse_complement(uint64_t kmer, int k) {
    uint64_t rc = 0;
    for (int i = 0; i < k; i++) {
        uint8_t base = kmer & 3;
        uint8_t comp = 3 - base; // A<->T (0<->3), C<->G (1<->2)
        rc = (rc << 2) | comp;
        kmer >>= 2;
    }
    return rc;
}

uint64_t fastga_hash_kmer(uint64_t kmer) {
    // Simple hash function - can be improved
    kmer ^= kmer >> 33;
    kmer *= 0xff51afd7ed558ccdULL;
    kmer ^= kmer >> 33;
    kmer *= 0xc4ceb9fe1a85ec53ULL;
    kmer ^= kmer >> 33;
    return kmer;
}

// ============================================================================
// BASIC ALIGNMENT SCORING
// ============================================================================

int fastga_score_match(uint8_t a, uint8_t b) {
    return (a == b) ? 1 : -1;
}

int fastga_edit_distance(const uint8_t* seq1, size_t len1,
                         const uint8_t* seq2, size_t len2) {
    // Simple edit distance - not optimized
    int* dp = (int*)calloc((len1 + 1) * (len2 + 1), sizeof(int));

    // Initialize first row and column
    for (size_t i = 0; i <= len1; i++) dp[i * (len2 + 1)] = i;
    for (size_t j = 0; j <= len2; j++) dp[j] = j;

    // Fill DP table
    for (size_t i = 1; i <= len1; i++) {
        for (size_t j = 1; j <= len2; j++) {
            int cost = (seq1[i-1] == seq2[j-1]) ? 0 : 1;
            int replace = dp[(i-1) * (len2+1) + (j-1)] + cost;
            int insert = dp[i * (len2+1) + (j-1)] + 1;
            int delete = dp[(i-1) * (len2+1) + j] + 1;

            dp[i * (len2+1) + j] = (replace < insert) ?
                                   ((replace < delete) ? replace : delete) :
                                   ((insert < delete) ? insert : delete);
        }
    }

    int result = dp[len1 * (len2 + 1) + len2];
    free(dp);
    return result;
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

double fastga_gc_content(const uint8_t* seq, size_t length) {
    if (length == 0) return 0.0;

    size_t gc_count = 0;
    for (size_t i = 0; i < length; i++) {
        if (seq[i] == 1 || seq[i] == 2) { // C or G
            gc_count++;
        }
    }

    return (double)gc_count / length;
}

void fastga_count_bases(const uint8_t* seq, size_t length,
                       uint64_t* count_a, uint64_t* count_c,
                       uint64_t* count_g, uint64_t* count_t) {
    *count_a = *count_c = *count_g = *count_t = 0;

    for (size_t i = 0; i < length; i++) {
        switch (seq[i] & 3) {
            case 0: (*count_a)++; break;
            case 1: (*count_c)++; break;
            case 2: (*count_g)++; break;
            case 3: (*count_t)++; break;
        }
    }
}

void fastga_reverse_complement_seq(uint8_t* seq, size_t length) {
    size_t i = 0;
    size_t j = length - 1;

    while (i < j) {
        uint8_t temp = 3 - seq[i];  // Complement
        seq[i] = 3 - seq[j];         // Complement
        seq[j] = temp;
        i++;
        j--;
    }

    if (i == j) { // Odd length
        seq[i] = 3 - seq[i];
    }
}