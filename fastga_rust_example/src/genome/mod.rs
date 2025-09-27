use anyhow::Result;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use flate2::read::GzDecoder;

pub mod sequence;
pub mod contig;
pub mod scaffold;

pub use sequence::{Sequence, SequenceStore};
pub use contig::Contig;
pub use scaffold::Scaffold;

/// Genome database structure
#[derive(Debug, Clone)]
pub struct GDB {
    pub scaffolds: Vec<Scaffold>,
    pub contigs: Vec<Contig>,
    pub sequences: SequenceStore,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub source_path: String,
    pub total_bases: u64,
    pub max_contig_len: u64,
    pub base_frequencies: [f32; 4], // A, C, G, T
}

impl GDB {
    /// Load GDB from a saved database file
    pub fn load(path: &Path) -> Result<Self> {
        // Implementation would deserialize from binary format
        unimplemented!("GDB::load")
    }

    /// Save GDB to a database file
    pub fn save(&self, path: &Path) -> Result<()> {
        // Implementation would serialize to binary format
        unimplemented!("GDB::save")
    }

    /// Build GDB from a source file (FASTA/FASTQ)
    pub fn from_source(path: &Path) -> Result<Self> {
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        match extension {
            "fa" | "fasta" => Self::from_fasta(path, false),
            "gz" => Self::from_fasta(path, true),
            "fq" | "fastq" => Self::from_fastq(path),
            _ => anyhow::bail!("Unsupported file format: {}", extension),
        }
    }

    /// Build from FASTA file
    fn from_fasta(path: &Path, gzipped: bool) -> Result<Self> {
        let file = File::open(path)?;
        let reader: Box<dyn BufRead> = if gzipped {
            Box::new(BufReader::new(GzDecoder::new(file)))
        } else {
            Box::new(BufReader::new(file))
        };

        let mut scaffolds = Vec::new();
        let mut contigs = Vec::new();
        let mut sequences = Vec::new();
        let mut current_header = String::new();
        let mut current_seq = Vec::new();
        let mut total_bases = 0u64;
        let mut base_counts = [0u64; 4];

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('>') {
                // Process previous sequence if exists
                if !current_seq.is_empty() {
                    let contig = Contig {
                        id: contigs.len() as u32,
                        scaffold_id: scaffolds.len() as u32,
                        length: current_seq.len() as u64,
                        sequence_offset: sequences.len() as u64,
                        name: current_header.clone(),
                    };
                    contigs.push(contig);

                    // Count bases for frequency calculation
                    for &base in &current_seq {
                        match base {
                            b'A' | b'a' => base_counts[0] += 1,
                            b'C' | b'c' => base_counts[1] += 1,
                            b'G' | b'g' => base_counts[2] += 1,
                            b'T' | b't' => base_counts[3] += 1,
                            _ => {}
                        }
                        total_bases += 1;
                    }

                    sequences.extend_from_slice(&current_seq);
                    current_seq.clear();
                }

                // Start new sequence
                current_header = line[1..].trim().to_string();
                if scaffolds.is_empty() || !is_part_of_scaffold(&current_header, &scaffolds) {
                    scaffolds.push(Scaffold {
                        id: scaffolds.len() as u32,
                        name: current_header.clone(),
                        length: 0, // Will update later
                        first_contig: contigs.len() as u32,
                        last_contig: 0, // Will update later
                    });
                }
            } else {
                current_seq.extend_from_slice(line.trim().as_bytes());
            }
        }

        // Process last sequence
        if !current_seq.is_empty() {
            let contig = Contig {
                id: contigs.len() as u32,
                scaffold_id: scaffolds.len() as u32 - 1,
                length: current_seq.len() as u64,
                sequence_offset: sequences.len() as u64,
                name: current_header,
            };
            contigs.push(contig);

            for &base in &current_seq {
                match base {
                    b'A' | b'a' => base_counts[0] += 1,
                    b'C' | b'c' => base_counts[1] += 1,
                    b'G' | b'g' => base_counts[2] += 1,
                    b'T' | b't' => base_counts[3] += 1,
                    _ => {}
                }
                total_bases += 1;
            }

            sequences.extend_from_slice(&current_seq);
        }

        // Update scaffold information
        for scaffold in &mut scaffolds {
            let scaffold_contigs: Vec<_> = contigs.iter()
                .filter(|c| c.scaffold_id == scaffold.id)
                .collect();

            scaffold.last_contig = scaffold_contigs.last()
                .map(|c| c.id + 1)
                .unwrap_or(scaffold.first_contig);

            scaffold.length = scaffold_contigs.iter()
                .map(|c| c.length)
                .sum();
        }

        // Calculate base frequencies
        let base_frequencies = if total_bases > 0 {
            [
                base_counts[0] as f32 / total_bases as f32,
                base_counts[1] as f32 / total_bases as f32,
                base_counts[2] as f32 / total_bases as f32,
                base_counts[3] as f32 / total_bases as f32,
            ]
        } else {
            [0.25, 0.25, 0.25, 0.25]
        };

        let max_contig_len = contigs.iter()
            .map(|c| c.length)
            .max()
            .unwrap_or(0);

        Ok(GDB {
            scaffolds,
            contigs,
            sequences: SequenceStore::Uncompressed(sequences),
            metadata: Metadata {
                source_path: path.to_string_lossy().to_string(),
                total_bases,
                max_contig_len,
                base_frequencies,
            },
        })
    }

    /// Build from FASTQ file
    fn from_fastq(_path: &Path) -> Result<Self> {
        unimplemented!("FASTQ support not yet implemented")
    }

    /// Get sequence for a specific contig
    pub fn get_sequence(&self, contig_id: u32) -> Option<Sequence> {
        self.contigs.get(contig_id as usize).map(|contig| {
            self.sequences.extract(contig.sequence_offset, contig.length)
        })
    }

    /// Get total number of bases
    pub fn total_bases(&self) -> u64 {
        self.metadata.total_bases
    }

    /// Get base frequencies
    pub fn base_frequencies(&self) -> [f32; 4] {
        self.metadata.base_frequencies
    }
}

fn is_part_of_scaffold(_header: &str, _scaffolds: &[Scaffold]) -> bool {
    // Simple heuristic - could be improved
    false
}