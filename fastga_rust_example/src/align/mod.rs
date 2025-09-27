use anyhow::Result;
use rayon::prelude::*;

pub mod alignment;
pub mod chain;
pub mod traceback;

pub use alignment::{Alignment, AlignmentPath};
pub use chain::AlignmentChain;

use crate::genome::GDB;
use crate::index::{KmerIndex, KmerEntry};

/// Main aligner structure
pub struct Aligner {
    threads: usize,
    min_chain_score: u32,
    min_alignment_length: u32,
    trace_spacing: u32,
}

impl Aligner {
    pub fn new() -> Self {
        Aligner {
            threads: 8,
            min_chain_score: 85,
            min_alignment_length: 100,
            trace_spacing: 100,
        }
    }

    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    pub fn with_min_chain_score(mut self, score: u32) -> Self {
        self.min_chain_score = score;
        self
    }

    pub fn with_min_alignment_length(mut self, length: u32) -> Self {
        self.min_alignment_length = length;
        self
    }

    /// Main alignment function
    pub fn align_genomes(
        &self,
        target: &GDB,
        target_index: &KmerIndex,
        query: &GDB,
        query_index: &KmerIndex,
    ) -> Result<Vec<Alignment>> {
        // Find seed matches between genomes
        let seeds = self.find_seeds(target_index, query_index);

        // Group seeds by contig pairs
        let mut contig_pairs: std::collections::HashMap<(u32, u32), Vec<SeedMatch>> =
            std::collections::HashMap::new();

        for seed in seeds {
            contig_pairs
                .entry((seed.target_contig, seed.query_contig))
                .or_insert_with(Vec::new)
                .push(seed);
        }

        // Process each contig pair in parallel
        let alignments: Vec<Vec<Alignment>> = contig_pairs
            .par_iter()
            .map(|((t_contig, q_contig), seeds)| {
                self.align_contig_pair(
                    target,
                    *t_contig,
                    query,
                    *q_contig,
                    seeds,
                )
            })
            .collect();

        // Flatten results
        Ok(alignments.into_iter().flatten().collect())
    }

    /// Find seed matches between indices
    fn find_seeds(&self, target_index: &KmerIndex, query_index: &KmerIndex) -> Vec<SeedMatch> {
        // This is simplified - real implementation would be more sophisticated
        let mut seeds = Vec::new();

        for target_entry in &target_index.kmers {
            let query_matches = query_index.find_matches(target_entry.kmer);

            for query_entry in query_matches {
                seeds.push(SeedMatch {
                    kmer: target_entry.kmer,
                    target_contig: target_entry.contig,
                    target_pos: target_entry.position,
                    query_contig: query_entry.contig,
                    query_pos: query_entry.position,
                });
            }
        }

        seeds
    }

    /// Align a specific pair of contigs given seed matches
    fn align_contig_pair(
        &self,
        target: &GDB,
        target_contig: u32,
        query: &GDB,
        query_contig: u32,
        seeds: &[SeedMatch],
    ) -> Vec<Alignment> {
        // Sort seeds by diagonal
        let mut seeds = seeds.to_vec();
        seeds.sort_by_key(|s| {
            let diagonal = s.target_pos as i64 - s.query_pos as i64;
            (diagonal, s.target_pos)
        });

        // Chain seeds
        let chains = self.chain_seeds(&seeds);

        // Extend chains into full alignments
        let mut alignments = Vec::new();

        for chain in chains {
            if chain.score >= self.min_chain_score {
                if let Some(alignment) = self.extend_chain(
                    target,
                    target_contig,
                    query,
                    query_contig,
                    &chain,
                ) {
                    if alignment.length() >= self.min_alignment_length {
                        alignments.push(alignment);
                    }
                }
            }
        }

        alignments
    }

    /// Chain seed matches
    fn chain_seeds(&self, seeds: &[SeedMatch]) -> Vec<SeedChain> {
        // Simplified chaining - real implementation would use dynamic programming
        let mut chains = Vec::new();
        let mut used = vec![false; seeds.len()];

        for i in 0..seeds.len() {
            if used[i] {
                continue;
            }

            let mut chain = SeedChain {
                seeds: vec![seeds[i].clone()],
                score: 1,
            };
            used[i] = true;

            // Extend chain
            let diagonal = seeds[i].target_pos as i64 - seeds[i].query_pos as i64;

            for j in i + 1..seeds.len() {
                if used[j] {
                    continue;
                }

                let j_diagonal = seeds[j].target_pos as i64 - seeds[j].query_pos as i64;

                // Check if seed j can extend the chain
                if (diagonal - j_diagonal).abs() < 100
                    && seeds[j].target_pos > chain.seeds.last().unwrap().target_pos
                    && seeds[j].query_pos > chain.seeds.last().unwrap().query_pos
                {
                    chain.seeds.push(seeds[j].clone());
                    chain.score += 1;
                    used[j] = true;
                }
            }

            chains.push(chain);
        }

        chains
    }

    /// Extend a seed chain into a full alignment
    fn extend_chain(
        &self,
        target: &GDB,
        target_contig: u32,
        query: &GDB,
        query_contig: u32,
        chain: &SeedChain,
    ) -> Option<Alignment> {
        // Get sequences
        let target_seq = target.get_sequence(target_contig)?;
        let query_seq = query.get_sequence(query_contig)?;

        // Determine alignment boundaries
        let first_seed = chain.seeds.first()?;
        let last_seed = chain.seeds.last()?;

        // Simple alignment - real implementation would use WFA or banded DP
        Some(Alignment {
            target_id: target_contig,
            target_start: first_seed.target_pos,
            target_end: last_seed.target_pos + target_index.kmer_size() as u32,
            query_id: query_contig,
            query_start: first_seed.query_pos,
            query_end: last_seed.query_pos + target_index.kmer_size() as u32,
            score: chain.score as f32,
            identity: 0.95, // Placeholder
            path: AlignmentPath::Simple,
        })
    }
}

impl Default for Aligner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct SeedMatch {
    kmer: u64,
    target_contig: u32,
    target_pos: u32,
    query_contig: u32,
    query_pos: u32,
}

#[derive(Debug)]
struct SeedChain {
    seeds: Vec<SeedMatch>,
    score: u32,
}