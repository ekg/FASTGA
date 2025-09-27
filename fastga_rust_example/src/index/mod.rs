use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;
use dashmap::DashMap;

pub mod kmer;
pub use kmer::Kmer;

use crate::genome::GDB;

/// K-mer index for genome searching
#[derive(Debug)]
pub struct KmerIndex {
    kmer_size: usize,
    kmers: Vec<KmerEntry>,
    frequency_cutoff: u32,
}

#[derive(Debug, Clone)]
pub struct KmerEntry {
    pub kmer: u64,  // Encoded k-mer
    pub contig: u32,
    pub position: u32,
}

impl KmerIndex {
    /// Build k-mer index from genome
    pub fn build(genome: &GDB, kmer_size: usize, frequency_cutoff: u32) -> Result<Self> {
        if kmer_size > 32 {
            anyhow::bail!("K-mer size must be <= 32");
        }

        // Count k-mers in parallel
        let kmer_counts = DashMap::new();

        genome.contigs.par_iter().for_each(|contig| {
            if let Some(sequence) = genome.get_sequence(contig.id) {
                let numeric = sequence.to_numeric();

                for i in 0..numeric.len().saturating_sub(kmer_size - 1) {
                    if let Some(encoded) = encode_kmer(&numeric[i..i + kmer_size]) {
                        kmer_counts.entry(encoded).and_modify(|e| *e += 1).or_insert(1u32);
                    }
                }
            }
        });

        // Collect k-mers above frequency threshold
        let mut kmers = Vec::new();

        for contig in &genome.contigs {
            if let Some(sequence) = genome.get_sequence(contig.id) {
                let numeric = sequence.to_numeric();

                for i in 0..numeric.len().saturating_sub(kmer_size - 1) {
                    if let Some(encoded) = encode_kmer(&numeric[i..i + kmer_size]) {
                        if let Some(count) = kmer_counts.get(&encoded) {
                            if *count <= frequency_cutoff {
                                kmers.push(KmerEntry {
                                    kmer: encoded,
                                    contig: contig.id,
                                    position: i as u32,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Sort k-mers for binary search
        kmers.par_sort_unstable_by_key(|e| e.kmer);

        Ok(KmerIndex {
            kmer_size,
            kmers,
            frequency_cutoff,
        })
    }

    /// Find all matches for a given k-mer
    pub fn find_matches(&self, kmer: u64) -> Vec<&KmerEntry> {
        let start = self.kmers.binary_search_by_key(&kmer, |e| e.kmer)
            .unwrap_or_else(|i| i);

        let mut matches = Vec::new();
        for i in start..self.kmers.len() {
            if self.kmers[i].kmer == kmer {
                matches.push(&self.kmers[i]);
            } else {
                break;
            }
        }

        matches
    }

    /// Get k-mer size
    pub fn kmer_size(&self) -> usize {
        self.kmer_size
    }

    /// Get total number of indexed k-mers
    pub fn num_kmers(&self) -> usize {
        self.kmers.len()
    }
}

/// Encode k-mer to u64
fn encode_kmer(seq: &[u8]) -> Option<u64> {
    let mut encoded = 0u64;

    for &base in seq {
        if base > 3 {
            return None; // Ambiguous base
        }
        encoded = (encoded << 2) | (base as u64);
    }

    Some(encoded)
}

/// Decode k-mer from u64
pub fn decode_kmer(encoded: u64, k: usize) -> Vec<u8> {
    let mut seq = Vec::with_capacity(k);

    for i in (0..k).rev() {
        let base = ((encoded >> (i * 2)) & 0b11) as u8;
        seq.push(base);
    }

    seq
}